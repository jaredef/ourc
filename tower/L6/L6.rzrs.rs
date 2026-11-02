/* rzrs.rs -- a self-hosting slice of rzrs, WRITTEN IN the Rust subset and compiled by rzrs itself.
   Reads a Rust program on stdin, emits rzir. Surface: multiple fns (params, calls, i32/bool),
   let/assign, if/else, while+break, return, arithmetic (+ - * / %), comparison (== != < <= > >=),
   logical (&& || !), parens, unary minus. Observable = the exit code (main's tail/return).
   Written in exactly the subset it compiles, so it can eventually compile ITSELF. */

static mut SRC: [u8; 1500000] = [0; 1500000];
static mut LEN: i32 = 0;
static mut POS: i32 = 0;
static mut NC: i32 = 0;
static mut ID_O: i32 = 0;
static mut ID_L: i32 = 0;
static mut LBL: i32 = 0;

/* per-function local symbol table (name span -> vN slot) */
static mut SO: [i32; 8192] = [0; 8192];
static mut SL: [i32; 8192] = [0; 8192];
static mut SV: [i32; 8192] = [0; 8192];
static mut NSYM: i32 = 0;

/* C-S0-I64-INDEXED-32: is the just-read type identifier an 8-byte integer (i64/u64/usize/isize)? A module-level
   static declared with one of these needs an 8-byte slot and store8/load8, exactly as a struct field does via FI64.
   Without this every such static -- scalar OR array -- was a 4-byte cell and silently truncated to i32 range, which
   is why `static mut GSADDR: [i64; N]` in rzir-ld.rs could not hold an address past 2GB. */
fn id_w8() -> i32 { unsafe {
    if ID_L == 3 { if at(ID_O) == 105 || at(ID_O) == 117 { if at(ID_O + 1) == 54 { if at(ID_O + 2) == 52 { return 1; } } } }
    if ID_L == 5 { if at(ID_O) == 105 || at(ID_O) == 117 { if at(ID_O + 1) == 115 { if at(ID_O + 2) == 105 { if at(ID_O + 3) == 122 { if at(ID_O + 4) == 101 { return 1; } } } } } }
    0
} }
/* module-level statics (name span -> gdef). GK 0=scalar 1=array; GE=element stride (1 u8 / 4 i32 / 8 i64) */
static mut GO: [i32; 2048] = [0; 2048];
static mut GL: [i32; 2048] = [0; 2048];
static mut GK: [i32; 2048] = [0; 2048];
static mut GFOLD: [i32; 2048] = [0; 2048];   /* CONSTEVAL: 1 iff this global is a genuinely-immutable `const` array whose elements may be const-folded by VALUE (E0080). `static [mut]` arrays are runtime-mutated (or zero-filled with no replayed inits) -> NEVER value-foldable; GCNT is only their replay-count, not a logical length. */
static mut GE: [i32; 2048] = [0; 2048];
static mut NGLOB: i32 = 0;
/* R2af: static initializer consts; GSTART/GCNT index GIV per static */
static mut GSTART: [i32; 2048] = [0; 2048];
static mut GCNT: [i32; 2048] = [0; 2048];
static mut GIV: [i32; 8192] = [0; 8192];
static mut NGIV: i32 = 0;
static mut SK: [i32; 8192] = [0; 8192];   /* symbol kind: 0 scalar, 1 &str */
static mut EISSTR: i32 = 0;               /* set by a producer when the just-parsed value is a &str descriptor */
static mut EISVEC: i32 = 0;               /* R4: set when the just-parsed value is a Vec descriptor (let binds SK=2) */
static mut HAS_RVEC: i32 = 0;             /* std-hoist P4.1: set when the program DEFINES `struct RVec` (std prepended) -> route user Vec/vec!/.push to the std RVec_* fns (dispatch_vec retirement begins) */
static mut HAS_RSTRING: i32 = 0;          /* std-hoist P4.3: set when the program DEFINES `struct RString` (std prepended) -> route user String::new/.push_str/.push to the std RString_* fns (reads .len/Display stay on dispatch_string via the compat {buf@0,len@4} layout) */
static mut EISSTRING: i32 = 0;            /* R6: set when the just-parsed value is an owned String (let binds SK=3) */
static mut EISARR: i32 = 0;               /* R11: set when the just-parsed value is a local fixed array (let binds SK=5); buffer pointer, a[i]=*(a+i*4) */
static mut EANNARR64: i32 = 0;            /* C-S0I64-LOCALARR: the `let a: [i64;N]` annotation flag, threaded into the RHS array-literal build for 8-byte stride */
static mut AV: [i32; 16] = [0; 16];       /* println! arg vNs */
/* CL-A7 (2026-08-04-rustc-frontend-compilation/locales/call-abi-arity): scratch for call arguments past the
   EIGHTH, used ONLY inside emit_call's tail loop. XTOP is a stack top, not a count: parse_expr re-enters
   emit_call, and a nested >=9-argument call runs its own tail loop over this same array. The outer loop
   therefore recomputes its own top from its LOCALS before every store, which erases anything a nested call
   left behind; the nested call always works above the outer's region, so the outer's slots survive. */
static mut XCA: [i32; 32] = [0; 32];
static mut XTOP: i32 = 0;
static mut AS: [i32; 16] = [0; 16];       /* println! arg is-str flags */
static mut AC: [i32; 16] = [0; 16];       /* R12: println! arg is-char flags (print via putc) */
static mut A64: [i32; 16] = [0; 16];      /* R17: println! arg is-i64 flags (print via __print_int64) */
static mut AB: [i32; 16] = [0; 16];       /* A4: println! arg is-bool flags (print true/false) */
static mut AFLT: [i32; 16] = [0; 16];     /* SOFTFLOAT: println! arg is a fixed-point f64 (scaled i64) */
static mut AFPREC: [i32; 16] = [0 - 1; 16];   /* SOFTFLOAT: per-hole `.N` precision (-1 = default trim) */
static mut AITYP: [i32; 16] = [0; 16];    /* long-tail: println!/format! arg struct type (for {:?} Debug), -1 if none */
static mut AVEC: [i32; 16] = [0; 16];     /* R2cg: println! arg is a Vec descriptor (for {:?} -> [e0, e1, ..]) */
static mut AVELT: [i32; 16] = [0 - 1; 16];  /* DBG: element struct-type of a Vec arg (for {:?} on Vec<Struct> -> [P { .. }, ..]), -1 if scalar */
static mut AVELTY: [i32; 16] = [0 - 1; 16]; /* DBG: element INTERNED Ty of a Vec arg (TYTAG==11 tuple -> {:?} prints [(a, b), ..] instead of element pointers) */
static mut ATUP: [i32; 16] = [0; 16];     /* R2cg: println! arg is a tuple cell (for {:?} -> (e0, e1, ..)) */
static mut ATUPN: [i32; 16] = [0; 16];    /* R2cg: tuple arity for {:?} */
static mut ATUPTY: [i32; 16] = [0 - 1; 16];  /* DBG: tuple-type id of a tuple arg (for per-field kind via tupty_fk -> {:?} quotes &str/String fields instead of printing a pointer) */
static mut ETUPN: i32 = 0;                /* R2cg: element count of the just-parsed tuple */
static mut ETUPSLICE: i32 = 0;            /* ADAPT: the just-parsed tuple's elements are Vec/slice descriptors (split_at) -> destructure binds them sym_kind=2 */
static mut ETUPTY: i32 = 0 - 1;           /* tuple-field-kind: tuple-type id of the just-parsed tuple (index into TUPTY_*) */
static mut STUPTY: [i32; 8192] = [0 - 1; 8192];   /* tuple-field-kind: per-symbol tuple-type id */
static mut NTUPTY: i32 = 0;               /* tuple-field-kind: number of registered tuple types */
static mut TUPTY_BASE: [i32; 2048] = [0; 2048];   /* tuple-field-kind: base offset of this type's field-kinds in TFK */
static mut TUPTY_N: [i32; 2048] = [0; 2048];      /* tuple-field-kind: arity */
static mut TFK: [i32; 16384] = [0; 16384];        /* tuple-field-kind: flat field-kind pool (0 scalar/1 &str/2 Vec/3 String/4 char/5 i64) */
static mut TFKCUR: i32 = 0;                       /* tuple-field-kind: TFK pool cursor */
/* TI0 (type-inference substrate): ONE interned type table `Ty`. Bases at fixed indices 0..6; composites
   interned from 7 up. TYTAG kind (1 i32/2 i64/3 &str/4 String/5 char/6 bool/7 unit/8 Vec/9 Option/10 array/
   11 tuple/12 struct); TYELT element-Ty (Vec/Option/array); TYAUX width/struct-id/tuple-registry-id. The
   flag<->Ty codec (ty_of_flags/apply_ty) is the byte-stable seam: producers set STY/EITY in parallel with
   the ~51-flag side-band; consumers still read the flags. Migration (retire the flags) happens in rzrs-next
   after the freeze. See apparatus/arcs/2026-07-18-rustc-parity/type-inference/arc.md. */
static mut TYTAG: [i32; 4096] = [0; 4096];
static mut TYELT: [i32; 4096] = [0; 4096];
static mut TYAUX: [i32; 4096] = [0; 4096];
static mut NTY: i32 = 0;
static mut EITY: i32 = 0;                          /* TI0: interned Ty of the just-parsed expression (0 = i32) */
static mut EITY_PIN: i32 = 0 - 1;                  /* FLAG-RETIRE inversion scaffold: a producer sets this to pin EITY (bypass ty_of_flags) for one expression; -1 = no pin. The seam that makes EITY producer-authoritative so a flag's write can be deleted. */
static mut STY: [i32; 8192] = [0; 8192];           /* TI0: per-symbol interned Ty */
static mut SVELTY: [i32; 8192] = [0 - 1; 8192];    /* TI3: per-Vec element Ty (Vec(eltTy)) */
static mut DVELTY: i32 = 0 - 1;                     /* TI3: element Ty threaded into dispatch_vec (parallels DVELT) */
static mut VELTY: i32 = 0 - 1;                      /* TI3: element Ty of the just-built vec! literal */
static mut EOPTTY: i32 = 0 - 1;                     /* TI3: Option payload Ty (min_by_key etc. -> unwrap restores it) */
static mut EISCHAR: i32 = 0;              /* R12: the just-parsed value is a char (print as a character) */
static mut EIS64: i32 = 0;                /* R17: the just-parsed value is a 64-bit (i64) value */
static mut SI64: [i32; 8192] = [0; 8192]; /* R17: per-symbol i64 flag */
static mut AU: [i32; 16] = [0; 16];       /* MEET-3: println! arg is-u64 (unsigned) flag */
static mut EISU: i32 = 0;                 /* MEET-3: the just-parsed value is unsigned (u64) */
static mut WUNS: i32 = 0;                 /* value-width: next i64op widens a 32-bit operand UNSIGNED (zero-extend) instead of the signed default (sign-extend); set by unsigned compares, consumed+cleared by i64op */
static mut SU64: [i32; 8192] = [0; 8192]; /* MEET-3: per-symbol u64 flag */
static mut SMOVED: [i32; 8192] = [0; 8192]; /* R18: per-symbol moved flag (move-checking: struct consumed by value) */
static mut MVLOG: [i32; 16384] = [0; 16384]; /* MOVE-FLOW: transition log of symbol indices marked 0->1 moved, so a branch's moves can be UNDONE to the pre-branch state (flow-sensitive move check: mutually-exclusive if/else + match arms each start from the pre-branch moved-set; the post-branch set is their union). Reset per function. */
static mut MVLOGN: i32 = 0;                   /* MOVE-FLOW: live length of MVLOG */
static mut MVSTASH: [i32; 16384] = [0; 16384]; /* MOVE-FLOW: stash of a branch's OUTER-symbol moves, replayed after sibling branches compile to UNION the post-branch moved-set */
static mut MVSTASHN: i32 = 0;                 /* MOVE-FLOW: live length of MVSTASH */
static mut TCOPY: [i32; 256] = [0; 256];    /* P2: per-type Copy flag (from #[derive(..Copy..)]) -- a Copy struct is never moved */
static mut TPEQ: [i32; 256] = [0; 256];     /* R2: per-type PartialEq flag -- struct `==` is only valid when derived (else reject like rustc E0369) */
static mut EB1: [u8; 1] = [0; 1];           /* B1: 1-byte scratch for writing a diagnostic to stderr (fd 2) */
static mut PATHBUF: [u8; 512] = [0; 512];   /* LAYER4 P-MOD-FILE-SPLICE: NUL-terminated "NAME.rs" path scratch for the module open() syscall */
static mut MODBUF: [u8; 262144] = [0; 262144];   /* LAYER4 P-MOD-FILE-SPLICE: per-file read scratch (256KB) for a `mod NAME;` sibling before grow-splicing it into SRC */
static mut SFORREF: [i32; 8192] = [0; 8192]; /* R19: per-symbol for-ref flag (loop var of `for x in &v` binds a reference; *x is a no-op) */
static mut SELEMREF: [i32; 8192] = [0; 8192];   /* S1-AST P-FORREF-STRUCT-ELEM-MOVE-EXEMPT: a struct-typed loop var bound from a REFERENCE iteration (`for p in &vec_of_struct` / `.iter()`) is a `&Struct` -- rustc never lets you move OUT of it, so it must be MOVE-EXEMPT. Distinct from SFORREF (which is 0 here so `p.field` dispatches via SST); SELEMREF affects ONLY the move-check (die_moved/mark_moved at the struct-read path), nothing else. Was: passing such a loop var to a fn inside the loop (`for seg in segs.iter() { dump_seg(seg) }`) marked it moved -> the next iteration die_moved'd (a FALSE-REJECT rustc accepts). */
static mut SREF1: [i32; 8192] = [0; 8192];   /* D004 IC-3 P-BYREF-MATCH-PAYLOAD-DEREF-DEPTH: per-symbol by-ref MATCH/IFLET payload flag -- the binder holds the payload as stored at cell+4, so the source's leading `*`s are (1 ergonomic skip) + (N real loads): the FIRST `*` is a no-op, each SUBSEQUENT `*` a real eload. Distinct from SFORREF (all-skip -- a filter closure `|x| **x` over &&T, or a scalar for-ref, needs EVERY `*` no-op). Set alongside SFORREF at the by-ref payload binder; read -> EWASBOXREF; consumed skip-one-then-load in the unary-`*` emit. Closes `match &o{Some(e)=>**e}` / `if let Some(e)=&o{**e}` on Option<Box<T>>/Option<&T> (the 2nd `*` was swallowed by SFORREF -> the Box/ref pointer leaked raw). */
static mut SBYREF: [i32; 8192] = [0; 8192];  /* R2bg: per-symbol by-ref flag -- SV holds the ADDRESS of the real cell (FnMut mutable capture); reads eload(SV), writes store SV */
static mut SBOOL: [i32; 8192] = [0; 8192];   /* A4: per-symbol bool flag (Display prints true/false) */
static mut SCHAR: [i32; 8192] = [0; 8192];   /* R12: per-symbol char flag (Display prints the character via __putc) */
static mut SINTO: [i32; 8192] = [0 - 1; 8192];   /* From/Into: per-symbol `.into()` target struct type (an `impl Into<T>` param) -> reading it sets INTO_TARGET */
static mut SFLT: [i32; 8192] = [0; 8192];    /* R2ci: per-symbol float flag -- holds a truncated int; only `NAME as <int>` uses are allowed (no runtime soft-float) */
static mut EIS8: i32 = 0;                     /* R1: the just-read value is a u8 (mask wrapping arithmetic to 0xFF) */
static mut SU8: [i32; 8192] = [0; 8192];      /* R1: per-symbol u8 flag */
static mut EMUTBOR: i32 = 0;                  /* R3: the just-parsed value is `&mut X` on a scalar local X */
static mut EMUTBOR_O: i32 = 0;                /* R3: X's name span */
static mut EMUTBOR_L: i32 = 0;
static mut MREFK: i32 = 0;                    /* MEM: sym_kind of the last `&mut NAME` parsed by mref_slot (2=Vec) */
static mut MREFSTY: i32 = 0 - 1;              /* MEM: struct-type of that target (-1 none) */
static mut MREFADDR: i32 = 0;                 /* MEM: 1 if mref_slot returned a memory ADDRESS (`&mut p.field`) to eload/store, 0 if a scalar slot to ecopy */
static mut FLV_I64: i32 = 0;                  /* value-width: the field_lvalue target field is i64/u64 (64-bit op=/store) */
static mut SMBORO: [i32; 8192] = [0; 8192];   /* R3: per-local the live mut-borrower binding's name span (-1 = unborrowed) */
static mut SMBORL: [i32; 8192] = [0; 8192];
static mut SENUM: [i32; 8192] = [0; 8192];   /* A4: per-symbol enum flag (== compares discriminants) */
static mut EWASREF: i32 = 0;                 /* R19: set when the last parse_atom returned a bare for-ref loop var */
static mut EWASBOXREF: i32 = 0;              /* D004 IC-3: set when the last read symbol is a by-ref MATCH/IFLET payload (SREF1). The unary-`*` emit consumes it skip-one-then-load: the first `*` clears it (+EWASREF) and no-ops, so a further `*` does a real eload. Reset in the EI-reset block like EWASREF. */
static mut EOPTREF: i32 = 0;                  /* R2: the just-built Option payload is a &T (iter().find element) -> `*x` is a no-op */
static mut EISITER: i32 = 0;                  /* STATEFUL-ITER: the just-parsed value is a bare `.iter()`/`.peekable()` chain terminus -> `let mut it = ..` builds a stateful cursor [buf,len,pos] (SK=9) */
static mut ESPLIT: i32 = 0;                   /* STATEFUL-ITER: the just-parsed value is a `splitn(n,sep)` str-cursor [buf,len,pos,remaining,sep] -> `let mut it = ..` binds SK=10 */
static mut EISBYTES: i32 = 0;                /* R25: the just-parsed value is a &[u8] byte slice ([ptr,len] over raw bytes) */
static mut EISFLT: i32 = 0;                   /* SOFTFLOAT: the just-parsed value is a fixed-point f64 -- a scaled i64 = round(value * 1e9), backend stays float-free (deviation from the seed's IEEE-754 telos; corpus-exact fixed-point) */
static mut SFLTV: [i32; 8192] = [0; 8192];    /* SOFTFLOAT: per-symbol 1 if the local holds a fixed-point f64 (scaled i64) */
static mut EARRLEN: i32 = 0;                 /* E3: element count of the last array literal (for &[..] slice descriptors) */
static mut EARRELT: i32 = 0 - 1;             /* R2ar: element struct-type of the last array literal (-1 = none) */
static mut EARRVTY: i32 = 0 - 1;             /* TI3e-f: element INTERNED Ty of the last array literal (tag 11 for tuples, etc.) -- feeds the for-loop dvelty so `for (a,b) in [(..),..]` destructures instead of enumerating. */
static mut SARRELT: [i32; 8192] = [0 - 1; 8192];   /* R2ar: per-symbol fixed-array element struct-type (mirror of SVELT for Vecs) */
static mut SARRVTY: [i32; 8192] = [0 - 1; 8192];   /* TI3e-m: per-symbol fixed-array element INTERNED Ty (tag 3 &str etc.) -- mirror of SVELTY; threads a bound array's element kind to `for w in a.iter()` so the loop var is tagged. */
static mut EISBOOL: i32 = 0;                 /* A4: the just-parsed value is a bool (Display prints true/false) */
static mut EISUNIT: i32 = 0;                 /* Wave-2 bug 3: the just-parsed value is unit/() (a void method-call) -- a block/fn tail discards it instead of `ret` */
static mut EISENUM: i32 = 0;                 /* A4: the just-parsed value is an enum cell (== compares discriminants) */
static mut EIENUMTY: i32 = 0 - 1;            /* long-tail: enum index of the just-parsed enum value (for {:?}), -1 if none */
static mut EISHM: i32 = 0;                    /* S3: the just-parsed value is a HashMap descriptor [len@0, entries@4] */
static mut EISVDQ: i32 = 0;                   /* STDCOLL N1: the just-parsed value is a VecDeque descriptor [buf@0, len@4, head@8] (FIFO; pop_front advances head) */
static mut EISSET: i32 = 0;                   /* STDCOLL N2: the just-parsed value is a HashSet -- a keys-only HashMap [len@0, entries@4] (pairs [key,0]) */
static mut TUPV: [i32; 16] = [0; 16];        /* R23: scratch for tuple-constructor element vNs */
static mut TNO: [i32; 16] = [0; 16];         /* R23: scratch for let-destructure name offsets */
static mut TNL: [i32; 16] = [0; 16];         /* R23: scratch for let-destructure name lengths */
static mut TDSL: [i32; 16] = [0; 16];        /* R2: per-field slot vNs for `for &(a,b)` tuple destructure */
static mut BNO: [i32; 16] = [0; 16];         /* R2ce: struct-pattern bind-name offsets (`Ty { field: bind }`) */
static mut BNL: [i32; 16] = [0; 16];         /* R2ce: struct-pattern bind-name lengths */

/* R1: struct registry. types: TSO/TSL name span, TSZ size(bytes). fields: FSST owner-type, FSO/FSL name
   span, FSB byte offset, FSTY field's struct-type (-1 scalar). SST = a local/param's struct type (-1). */
static mut TSO: [i32; 128] = [0; 128];
static mut TSL: [i32; 128] = [0; 128];
static mut TSZ: [i32; 128] = [0; 128];
static mut NTYP: i32 = 0;
static mut FSST: [i32; 1024] = [0; 1024];
static mut FSO: [i32; 1024] = [0; 1024];
static mut FSL: [i32; 1024] = [0; 1024];
static mut FSB: [i32; 1024] = [0; 1024];
static mut FSTY: [i32; 1024] = [0; 1024];
static mut FSEN: [i32; 1024] = [0 - 1; 1024];   /* RES-3 P-STRUCT-ENUM-FIELD-READ: per-field TYPE-NAME span OFFSET (-1 = none). Resolved to an enum index LAZILY at the field read via enum_find (NOT at registration -- prepass_structs runs BEFORE prepass_enums, so enum_find is -1 then). A field of enum type reads as an untagged pointer at the scalar fallthrough; tag EISENUM from the lazy enum_find so `{:?}`/a let-binding/`s.e.method()` treat `s.e` as an enum (the enum dual of FSTY's struct-field descend). */
static mut FSENL: [i32; 1024] = [0; 1024];      /* RES-3: per-field type-name span LENGTH (0 = none) */
static mut FI64: [i32; 1024] = [0; 1024];   /* value-width: field is i64/u64 -> 8-byte slot, store8/load8, EIS64 on read */
static mut FBOOL: [i32; 1024] = [0; 1024];  /* LAYER5 P-STRUCT-BOOL-FIELD: field is `bool` -> restore EISBOOL on read so `{}`/`{:?}` print true/false, not 1/0 */
static mut FVELO: [i32; 1024] = [0; 1024];   /* LAYER6 P-SLICE-ELEM-ENUM: a `&[Enum]`/`Vec<Enum>` FIELD's element-type NAME span offset (resolved via enum_find at DISPATCH time -- prepass_structs runs BEFORE prepass_enums). */
static mut FVELL: [i32; 1024] = [0; 1024];   /* LAYER6 P-SLICE-ELEM-ENUM: element-type name span length; `p.toks[i]` -> DVELENUM = enum_find(FVELO[fi],FVELL[fi]) so the element is tagged EISENUM. Vec-local analogue: SVELENUM. */
static mut FFK: [i32; 1024] = [0; 1024];   /* R7: field container-kind (0 scalar/struct, 1 &str, 2 Vec, 3 String, 4 [T;N] array) */
static mut FARR64: [i32; 1024] = [0; 1024];   /* value-width: [T;N] field element is i64/u64 */
static mut FGEN: [i32; 1024] = [0; 1024];  /* std-hoist P0.1: 1 if the field's declared type IS the struct's generic type-param (e.g. `v: T`) */
static mut NFLD: i32 = 0;
/* std-hoist P0.1: propagate the CONCRETE type-arg KIND across the generic/monomorph boundary.
   TGPO/TGPL = a struct's generic-param name span (0/0 if none). Per generic-struct VALUE we record its
   concrete type-arg kind (SGAK: 0 scalar, 1 &str, 2 Vec, 3 String) + struct type (SGAT, -1 if not a struct).
   A field of type T (FGEN) or a method returning T (MRTS==-3) resolves to that recorded kind. */
static mut TGPO: [i32; 128] = [0; 128];
static mut TGPL: [i32; 128] = [0; 128];
static mut TGPO2: [i32; 128] = [0; 128];   /* std-hoist GAP2: struct's SECOND generic-param name span (V in `struct P<K,V>`), 0/0 if none */
static mut TGPFN: [i32; 128] = [0; 128];   /* R10: struct's param#1 is `Fn`-bounded (`<F: Fn(..)->..>`) -> a `f: F` field is a closure fat-ptr cell (ffk=5) */
static mut TGPL2: [i32; 128] = [0; 128];
static mut SGAK: [i32; 8192] = [0; 8192];
static mut SGAT: [i32; 8192] = [0 - 1; 8192];
static mut EGAK: i32 = 0;        /* pending type-arg kind of the just-built generic-struct literal */
static mut EGAT: i32 = 0 - 1;    /* pending type-arg struct type */
static mut EWASGARG: i32 = 0;    /* 1 if the just-parsed atom was a generic-struct literal that recorded a type-arg */
static mut DSGAK: i32 = 0 - 1;   /* dispatch: the self value's recorded type-arg kind (-1 = unknown) */
static mut DSGAT: i32 = 0 - 1;   /* dispatch: the self value's recorded type-arg struct type */
/* std-hoist P0.3: one NESTED level of type-arg kind (T-of-T). SGAK2/SGAT2 per-local; EGAK2/EGAT2/EWASGARG2
   pending; DSGAK2/DSGAT2 dispatch-time; ANNGAK/ANNGAT = a let-annotation/fn-return's `<..>` type-arg kind. */
static mut SGAK2: [i32; 8192] = [0; 8192];
static mut SGAT2: [i32; 8192] = [0 - 1; 8192];
static mut EGAK2: i32 = 0;
static mut EGAT2: i32 = 0 - 1;
static mut EWASGARG2: i32 = 0;
static mut DSGAK2: i32 = 0 - 1;
static mut DSGAT2: i32 = 0 - 1;
static mut ANNGAK: i32 = 0;
static mut ANNGAT: i32 = 0 - 1;
static mut RET_GAK: i32 = 0 - 99;   /* scan_ret_kind side channel: a `-> B<..>` return's type-arg kind (-99 = none) */
static mut RET_GAT: i32 = 0 - 1;
static mut FRGAK: [i32; 1024] = [0 - 99; 1024];   /* per-fn generic-return type-arg kind (-99 = not a generic-return) */
static mut FRGAT: [i32; 1024] = [0 - 1; 1024];
static mut SST: [i32; 8192] = [0; 8192];   /* per-symbol struct type (-1 if not a struct) */
static mut SARRLEN: [i32; 8192] = [0; 8192];   /* R1: per-symbol local-fixed-array element count (for a.len()) */
static mut SARR64: [i32; 8192] = [0; 8192];   /* C-S0I64-LOCALARR: 1 if a local `[i64;N]`/`[u64;N]` array -> 8-byte element (store8/eload8 + EIS64 on read) */
static mut SCKNOWN: [i32; 8192] = [0; 8192];   /* CONSTEVAL R2/R3: per-symbol 1 if an IMMUTABLE `let x = <const int>` -> x's compile-time value is known (const-let-propagation) */
static mut SCKVAL: [i32; 8192] = [0; 8192];    /* CONSTEVAL R2/R3: the known compile-time value (only when SCKNOWN==1) */
static mut EITYP: i32 = 0;                 /* struct type of the just-parsed expr (-1 if none) */
static mut TAILDST: i32 = 0;               /* R3b if-expr: holds dst+1 (0 = unset); tail expr copies to dst-1 not `ret`. 0-default so it self-hosts without a non-zero static init */

/* R2: enum registry. ENO/ENL = enum name span. per-variant: VEN owner-enum, VO/VL variant name span,
   VDISC discriminant, VHAS 1 if the variant carries a single i32 payload. an enum VALUE is an 8-byte heap
   cell: discriminant at offset 0, payload at offset 4. */
static mut ENO: [i32; 64] = [0; 64];
static mut ENL: [i32; 64] = [0; 64];
static mut ENGPO: [i32; 64] = [0; 64];   /* std-hoist GAP1: generic enum's type-param name span (e.g. T in `enum ROpt<T>`) */
static mut ENGPL: [i32; 64] = [0; 64];
static mut NEN: i32 = 0;
static mut VEN: [i32; 256] = [0; 256];
static mut VO: [i32; 256] = [0; 256];
static mut VL: [i32; 256] = [0; 256];
static mut VDISC: [i32; 256] = [0; 256];
static mut VHAS: [i32; 256] = [0; 256];
static mut VNF: [i32; 256] = [0; 256];   /* R13: number of payload fields (0, 1, or more) */
static mut VSTRUCT: [i32; 256] = [0; 256];   /* R22b: 1 if the variant is a struct-shaped `V { f: T }` (named fields) vs tuple `V(T)` -- for {:?} */
static mut VFNO: [i32; 2048] = [0; 2048];    /* R22b: struct-variant field-NAME span offset, flat index vi*8 + fi (<=8 fields) */
static mut VFNL: [i32; 2048] = [0; 2048];    /* R22b: struct-variant field-NAME span length */
static mut VFSTR: [i32; 2048] = [0; 2048];   /* R34: 1 if variant field fi is a String/&str (print quoted in {:?}), flat index vi*8 + fi */
static mut VFKIND: [i32; 2048] = [0; 2048];  /* LAYER5 P-DERIVE-RECURSIVE-EQ: per variant-field structural-eq kind (flat vi*8+fi). 0 scalar (`eq`), 1 Box<ThisEnum> recursive (`call <Enum>_eq`), 2 String/&str (byte-eq), 3 UNSUPPORTED for derived eq (nested struct/Vec/foreign-Box -> the `==` site rejects rather than miscompiles). */
static mut VFVEC: [i32; 2048] = [0; 2048];
static mut VFVELT: [i32; 2048] = [0 - 1; 2048];   /* LAYER8 P-VARIANT-VEC-STRUCT-ELEM element struct Ty */   /* LAYER6 P-STRUCT-VARIANT-FIELD-KIND: 1 if variant field fi is a `Vec<..>`, flat vi*8+fi. The struct-variant match binder reads it to tag the bound field SK=2 so `match e { E::C{ args } => args.len() }` dispatches the Vec (was untagged -> .len() returned the descriptor pointer / crashed). Per-field descriptor metadata (mirrors VFSTR/VFI64). */
static mut VFI64: [i32; 2048] = [0; 2048];   /* LAYER6 P-ENUM-I64-PAYLOAD-WIDTH: 1 if variant field fi is a signed 64-bit type (i64/isize), flat vi*8+fi. The match payload binder reads it to tag the bound symbol SI64 so `match n { N::Lit(v) => *v + BIG }` computes 64-bit (was untagged -> 32-bit arithmetic truncation on the eval result, e.g. a recursive `-ev(a)`/`ev(a)+ev(b)` node evaluator returned the value mod 2^32). Per-field descriptor metadata (mirrors VFSTR), NOT a per-value type flag. */
static mut VFPSTO: [i32; 2048] = [0 - 1; 2048];   /* DEFECT-c P-VARIANT-STRUCT-PAYLOAD-BINDER: the tuple-variant field fi's payload TYPE-NAME span OFFSET (through Box), flat vi*8+fi (-1 = none). LAZILY resolved to a struct index at the match binder via ty_find + enum_find<0 (NOT at the enum prepass -- a Box<SelfEnum> field is in the type table but not yet the enum registry there, so struct-vs-enum discrimination is unreliable; at match time all prepasses have run so enum_find is authoritative). Mirrors FSEN/FVELO's lazy span-capture-then-resolve convention. Per-variant-field span metadata, NOT a per-value type flag. */
static mut VFPSTL: [i32; 2048] = [0; 2048];   /* DEFECT-c: the VFPSTO span LENGTH (0 = none). */
static mut VFVELTY: [i32; 2048] = [0 - 1; 2048];   /* DEFECT-g P-VARIANT-VEC-STRING-ELEM: a tuple-variant `Vec<String>`/`Vec<&str>` payload field's element display-kind code (3 String, 2 &str; -1 none), flat vi*8+fi. The match binder threads it to the bound Vec symbol's SVELTY so `match e { E::V(v) => for p in v.iter() { .. p .. } }` binds p String/&str (was: element untagged -> Displayed the descriptor POINTER). Name-based (is_string_ty), so resolved at the prepass (no enum ambiguity). Mirrors the struct-FIELD DVELTY=3/2 convention + VFVELT's element metadata. NOT a per-value flag. */
static mut NVAR: i32 = 0;

/* R6: free-fn return-kind registry -> propagate String/Vec-ness of a call result (FRK: 0 scalar, 2 Vec, 3 String) */
static mut FNO: [i32; 1024] = [0; 1024];
static mut FNL: [i32; 1024] = [0; 1024];
static mut FRK: [i32; 1024] = [0; 1024];
static mut FRETREF: [i32; 1024] = [0; 1024];   /* R19: 1 if the fn returns `-> &T` -- a call `*f(..)` is then a no-op (f already returns the value; the & is transparent in the value model) */
static mut RET_REF: i32 = 0;                  /* scan_ret_kind side channel: the return type had a leading `&` */
static mut FDYNPOS: [i32; 1024] = [0; 1024];   /* exotica: position of a fn's first &dyn param (-1 if none) */
static mut FARRPOS: [i32; 1024] = [0; 1024];   /* A: per-fn bitmask of params that are const-generic `[T; N]` arrays (arg coerced to a [ptr,len] descriptor) */
static mut FNAC: [i32; 1024] = [0; 1024];      /* W1b: per-fn param count (for fn-pointer thunks) */
static mut FRST: [i32; 1024] = [0 - 1; 1024];  /* R2cj: per-fn return struct-type (-1 = not a user struct) -- lets `let p = mk(..)` bind p as struct-typed */
static mut RET_STY: i32 = 0 - 1;             /* R2cj: scan_ret_kind side channel -- the return type's user-struct index (-1 = none) */
static mut FRST2: [i32; 1024] = [0 - 1; 1024]; /* LAYER6 P-PRESULT-ERR-STRUCT-PAYLOAD: per-fn `-> Result<_, E>` ERR (2nd type-arg) struct-type (-1 = none) -- binds an `Err(e)=>e.field` match arm struct-typed (the Err-side dual of FRST) */
static mut RET_STY2: i32 = 0 - 1;            /* LAYER6 P-PRESULT-ERR-STRUCT-PAYLOAD: scan_ret_kind side channel -- the Result Err (2nd type-arg) struct index */
static mut EOPTERRTY: i32 = 0 - 1;           /* LAYER6 P-PRESULT-ERR-STRUCT-PAYLOAD: the Result value's ERR-payload struct Ty (tag 12) -> parse_match binds the Err (disc 0) arm struct-typed */
static mut RET_ENUM: i32 = 0 - 1;            /* LAYER5 P-FN-RETURNS-ENUM: scan_ret_kind side channel -- the return type's user-ENUM index (-1 = none), so `let x = f()` / `Box::new(f())` / a variant payload tag the enum-returning call's result EISENUM (else it was untagged -> Box::new(call) double-boxed the enum pointer -> a fold `Add(Box::new(dbl(a)),..)` built a broken tree) */
static mut FREN: [i32; 1024] = [0 - 1; 1024];  /* LAYER5 P-FN-RETURNS-ENUM: per-fn return enum index (-1 = not an enum) */
static mut FROPTK: [i32; 1024] = [0; 1024];    /* SPAN-4 P-OPTION-PAYLOAD-DISPLAY-KIND: per-fn `-> Option<P>`/`-> Result<P,_>` payload DISPLAY-kind (0 scalar, 1 &str/String, 3 char). Read at the call site so a `match f(){Some(x)=>..}` binder gets its display-kind from the DECLARED return type, not from whichever branch the body emitted last -- a `fn(..)->Option<&str>{ if b {Some(s)} else {None} }` lost the str-kind when the None branch left EOPTK stale (str Displayed as an int). Mirrors FRST/fnret_st + the generic EOPTK path. */
static mut RET_OPTK: i32 = 0;                 /* SPAN-4 scan_ret_kind side channel -- the `-> Option<P>`/`-> Result<P,_>` payload display-kind (0/1/3) */
static mut FROPTREF: [i32; 1024] = [0; 1024];   /* AST-1 P-OPTION-PAYLOAD-BYREF: per-fn `-> Option<&T>`/`-> Option<Box<T>>` payload-is-a-reference flag. Read at the call site to set EOPTREF so a `match f(){Some(x)=>*x}` binder treats `*x` as a no-op (the payload is already a reference). The declared-return dual of the inline find/get/last EOPTREF; the tag was lost across the fn/method-return boundary (`fn last(&self)->Option<&i32>{self.s.last()}` -> caller `*x` over-derefed -> crash). Mirrors FROPTK. */
static mut RET_OPTREF: i32 = 0;               /* AST-1 scan_ret_kind side channel -- 1 if the `-> Option<P>`/`-> Result<P,_>` payload P is `&T` or `Box<T>` (by-ref) */
static mut FROPTENUM: [i32; 1024] = [0 - 1; 1024];   /* D005 P-PRESULT-BOX-ENUM-PAYLOAD-BINDER: per-fn `-> Result<Box<Enum>,_>`/`-> Option<Box<Enum>>`/`-> Option<Enum>` success-payload ENUM index (-1 = none). Read at the call site -> EOPTTY = enum Ty (tag 13) so a match `Ok(e)=>e.method()`/`Some(e)=>e.method()` binder tags e SENUM (the enum dual of FRST/fnret_st's struct-payload binder). */
static mut MROPTENUM: [i32; 1024] = [0 - 1; 1024];   /* D005 (method arm): this method's `-> Result<Box<Enum>,_>`/`-> Option<Box<Enum>>` success-payload ENUM index (-1 = none). The free-fn analogue is FROPTENUM. */
static mut RET_OPTENUM: i32 = 0 - 1;          /* D005 scan_ret_kind side channel -- the `-> Option<P>`/`-> Result<P,_>` success payload's ENUM index (-1 = none), Box<Enum> seen-through */
static mut RET_VELENUM: i32 = 0 - 1;         /* R40: scan_ret_kind side channel -- a `-> Vec<Enum>` return's element enum-type (-1 = none) */
static mut RET_VELT: i32 = 0 - 1;            /* R40: scan_ret_kind side channel -- a `-> Vec<Struct>` return's element struct-type (-1 = none) */
static mut RET_VELSTR: i32 = 0;              /* R14 P-CF-ACC: scan_ret_kind side channel -- a `-> Vec<String>`(3)/`-> Vec<&str>`(2) return's element DISPLAY-kind (0 none). str/String dual of RET_VELT. */
static mut EVELSTR_RET: i32 = 0;             /* R14 P-CF-ACC: NON-leaking per-let-RHS channel -- the just-emitted RHS call's fn-return Vec<String>/Vec<&str> element display-kind (3/2, 0 none). RESET to 0 before each let-RHS parse_expr, set only in the rk==2 call block, read ONLY at the Vec binding. Unlike R7's global VELTY it cannot leak into other bindings (a struct binding is evec==0 and ignores it; a fresh reset precedes every let-RHS). */
static mut FRVELENUM: [i32; 1024] = [0 - 1; 1024];   /* R40: per-fn `-> Vec<Enum>` element enum-type */
static mut FRVELT: [i32; 1024] = [0 - 1; 1024];      /* R40: per-fn `-> Vec<Struct>` element struct-type */
static mut FRVELSTR: [i32; 1024] = [0; 1024];        /* R14 P-CF-ACC: per-fn `-> Vec<String>`(3)/`-> Vec<&str>`(2) element display-kind (0 none) */
static mut NFRN: i32 = 0;
/* E1: compile-time const registry -- `const NAME: T = <integer const expr>;` folded to a value + substituted */
static mut CNO: [i32; 1024] = [0; 1024];
static mut CNL: [i32; 1024] = [0; 1024];
static mut CVAL: [i32; 1024] = [0; 1024];
static mut NCONST: i32 = 0;
static mut CVALID: i32 = 0;
static mut COVFL: i32 = 0;   /* CONSTEVAL R2: set when an i32 add/sub/mul in a const-foldable expr leaves the i32 range (provable overflow) */
static mut C64SEEN: i32 = 0;   /* CONSTEVAL R2: set when the const expr involves an i64/u64 operand or any `as` cast -> the arithmetic is NOT i32, so an i32-range excess is NOT an overflow (guards `let c: i64 = a as i64 * b`) */
static mut TCTY: [i32; 1024] = [0; 1024];   /* TRAIT R3: trait-impl assoc const registry -- owner type index */
static mut TBALO: i32 = 0 - 1;   /* LAYER7 P-BOX-ALIAS: name span of a `type NAME<T> = Box<T>` alias (rustc's P<T>) -- treated as Box in the transparent-pointer see-through sites */
static mut TBALL: i32 = 0;
static mut TVALO: i32 = 0 - 1;   /* LAYER7 P-VEC-ALIAS: name span of a `type NAME<T> = Vec<T>` alias (rustc's ThinVec<T>) -- treated as Vec by is_vec_ty */
static mut TVALL: i32 = 0;
static mut TCNO: [i32; 1024] = [0; 1024];   /* const name span */
static mut TCNL: [i32; 1024] = [0; 1024];
static mut TCVAL: [i64; 1024] = [0; 1024];  /* const value (i64) */
static mut NTCONST: i32 = 0;
static mut ITERTY: [i32; 64] = [0; 64];   /* TRAIT R5: struct types with a user `impl Iterator` (drive .sum()/.filter()/.map() by materializing next() into a Vec) */
static mut NITER: i32 = 0;
/* CONST-FN: registry of `const fn NAME(p..) -> T { EXPR }` for compile-time eval in const initializers. */
static mut CFNO: [i32; 64] = [0; 64];        /* const-fn name span offset */
static mut CFNL: [i32; 64] = [0; 64];        /* const-fn name span length */
static mut CFNP: [i32; 64] = [0; 64];        /* param count */
static mut CFBODY: [i32; 64] = [0; 64];      /* POS of the body return-expr (after `{`, past optional `return`) */
static mut CFPO: [i32; 256] = [0; 256];      /* param name offsets, flat cf*4+i (<=4 params) */
static mut CFPL: [i32; 256] = [0; 256];      /* param name lengths, flat cf*4+i */
static mut NCFN: i32 = 0;
static mut PBNO: [i32; 64] = [0; 64];        /* active param-binding name-offset stack (nested const-fn calls) */
static mut PBNL: [i32; 64] = [0; 64];
static mut PBV: [i32; 64] = [0; 64];         /* bound value */
static mut PBDEPTH: i32 = 0;
/* A1: trait default methods -- trait name, method name, and re-parse POS (at the method name). When an
   `impl T for S` omits a default method, it is emitted as S_method from this body with self=S. */
static mut TRO: [i32; 256] = [0; 256];
static mut TRL: [i32; 256] = [0; 256];
static mut TDMO: [i32; 256] = [0; 256];
static mut TDML: [i32; 256] = [0; 256];
static mut TDPOS: [i32; 256] = [0; 256];
static mut TDMSTR: [i32; 256] = [0; 256];   /* S1 L1: default method's return kind -- 3 String, 1 &str, 0 other */
static mut NTDM: i32 = 0;
static mut IMPLMO: [i32; 64] = [0; 64];   /* method names an impl defined (to detect overrides) */
static mut IMPLML: [i32; 64] = [0; 64];
static mut NIMPLM: i32 = 0;
/* long-tail: persistent (type, method-name) registry for dyn trait-object dispatch (type-tag switch) */
static mut MITY: [i32; 1024] = [0; 1024];
static mut MIMO: [i32; 1024] = [0; 1024];
static mut MIML: [i32; 1024] = [0; 1024];
static mut NMI: i32 = 0;
static mut WANTDYN: i32 = 0;  /* A: the current `let` is typed Vec<Box<dyn ..>> -- vec! elements must be coerced to fat pointers */
static mut INTO_TARGET: i32 = 0 - 1;  /* R2cc: user struct type from a `let x: T =` annotation (or `impl Into<T>` param) -- `.into()` dispatches to T_from */
static mut ANNSTR: i32 = 0;  /* TI3e-i: 1 when a `let s: String =` annotation is in scope for the RHS -> a char-iterator `.collect()` builds a String, not a Vec. Set at the annotation, cleared after the RHS. */
static mut EISDYNVEC: i32 = 0;/* A: the just-parsed value is a Vec of dyn fat pointers (loop var gets SK=8) */
static mut SVECDYN: [i32; 8192] = [0; 8192]; /* A: per-symbol -- this Vec local holds dyn fat pointers */
static mut SVELT: [i32; 8192] = [0; 8192];   /* R1: per-symbol Vec element struct-type (-1 = none); for `for p in &v { p.field }` */
static mut VELT: i32 = 0 - 1;                 /* R1: element struct-type of the just-built vec! literal */
static mut VELENUM: i32 = 0 - 1;              /* R23b: element ENUM-type of the just-built vec! literal (-1 = none) */
static mut SVELENUM: [i32; 8192] = [0 - 1; 8192];   /* R23b: per-symbol Vec element enum-type */
static mut AVELENUM: [i32; 16] = [0 - 1; 16];       /* R23b: element enum-type of a Vec {:?} arg -> [A(1), B, ..] */
static mut DVELENUM: i32 = 0 - 1;             /* R23b: element enum-type entering dispatch_vec (index/for read) */
static mut VELVEC: i32 = 0;                    /* R2bx: the just-built vec! literal has Vec elements (Vec<Vec<..>>) -> SVELT sentinel -3 */
static mut EVELT: i32 = 0 - 1;               /* R1: element struct-type of the just-parsed &vec */
static mut EVELENUM: i32 = 0 - 1;            /* R24b: element enum-type of the just-parsed &vec (for `for e in &v` loop-var tagging) */
static mut EVELSTR: i32 = 0;                 /* R2ax: the just-parsed Vec has &str elements (split().collect()) -> SVELT=-2 */
static mut EHMVVEC: i32 = 0;                  /* nestgen: the HashMap being dispatched has Vec values -> m[k] yields a Vec descriptor */
static mut DVELT: i32 = 0 - 1;               /* R2an: element struct-type of the Vec entering dispatch_vec (so pop()->Option carries the elem type) */
static mut EOPTK: i32 = 0;                    /* container-kind K1: payload-kind code of the just-produced Option/Result (0 scalar, 2 Vec, 3 String, 1 &str); restored at unwrap/? */
static mut EOPTVELT: i32 = 0 - 1;             /* container-kind K1: Vec-payload element struct-type (feeds DVELT after unwrap) */
static mut SOPTK: [i32; 8192] = [0; 8192];    /* container-kind K1: per-symbol Option payload-kind (a `let`-bound Option remembers its payload kind) */
static mut SOPTVELT: [i32; 8192] = [0 - 1; 8192];
static mut EISOPT: i32 = 0;
static mut EOPTRES: i32 = 0;
static mut SISOPT: [i32; 8192] = [0; 8192];
static mut SOPTRES: [i32; 8192] = [0; 8192];
static mut SOPTREF: [i32; 8192] = [0; 8192];   /* RUSTC-FE LEX: per-symbol Option-payload-is-a-reference flag (mirrors SISOPT). `let r = v.get(i)/find()/max()` binds an Option<&T>; persist EOPTREF here so a later `*r.unwrap()`/`if let Some(x)=r` deref stays a no-op (was lost at the let-binding -> the bound Option<&T> segfaulted on deref, while the INLINE form worked). */
static mut AOPT: [i32; 16] = [0; 16];
static mut AOPTRES: [i32; 16] = [0; 16];
static mut AOPTK: [i32; 16] = [0; 16];   /* DBG: Option payload kind of an arg (EOPTK: 1=&str/3=String -> {:?} quotes the payload; else int) */
/* A: macro_rules! registry (single rule, expr params). MACBO/MACBL = body span; MACPO/MACPL[i*4+j] = param j name */
static mut MACO: [i32; 128] = [0; 128];
static mut MACL: [i32; 128] = [0; 128];
static mut MACBO: [i32; 128] = [0; 128];
static mut MACBL: [i32; 128] = [0; 128];
static mut MACNP: [i32; 128] = [0; 128];
static mut MACPO: [i32; 1024] = [0; 1024];
static mut MACPL: [i32; 1024] = [0; 1024];
static mut NMAC: i32 = 0;
static mut SETF: [i32; 1024] = [0; 1024];       /* R1: struct-literal explicitly-set field indices (stack, for `..base`) */
static mut NSETF: i32 = 0;
static mut LRES: [i32; 64] = [0; 64];         /* R1: per-nested-loop break-value result slot (loop as an expression) */
static mut NLRES: i32 = 0;
static mut LBLO: [i32; 16] = [0; 16];         /* R1: active loop-label name spans (stack) */
static mut LBLL: [i32; 16] = [0; 16];
static mut LBLFLAG: [i32; 16] = [0; 16];      /* R1: per-label break flag vreg (set by `break 'l`, propagated out at each loop body end) */
static mut NLBL: i32 = 0;
static mut LBLCFLAG: [i32; 16] = [0; 16];     /* R2az: per-label continue flag vreg (set by `continue 'l`) */
static mut LBLLVL: [i32; 16] = [0; 16];       /* R2az: loop-nesting level of each label's target loop */
static mut LBLRES: [i32; 16] = [0; 16];       /* R2ch: LRES slot index of each label's target value-loop (for `break 'l value`) */
static mut LDEPTH: i32 = 0;                   /* R2az: current loop-nesting depth */
static mut LVFB: [i32; 64] = [0; 64];         /* R2ch: per-LRES-slot break-value type flags (the loop-value's Display type comes from `break value`, routed to the TARGET loop's slot, not leftover body flags) */
static mut LVF64: [i32; 64] = [0; 64];
static mut LVFU: [i32; 64] = [0; 64];
static mut LVFSTR: [i32; 64] = [0; 64];
static mut LVFSTRING: [i32; 64] = [0; 64];
static mut LVFVEC: [i32; 64] = [0; 64];
static mut LVFCHAR: [i32; 64] = [0; 64];
static mut LVFTYP: [i32; 64] = [0 - 1; 64];
static mut LVFENUM: [i32; 64] = [0; 64];
static mut LVFENUMTY: [i32; 64] = [0 - 1; 64];
static mut LVFARR: [i32; 64] = [0; 64];
static mut LVF8: [i32; 64] = [0; 64];
static mut CURNP: i32 = 0;                    /* A: bindings for the macro currently expanding */
static mut CURPO: [i32; 4] = [0; 4];
static mut CURPL: [i32; 4] = [0; 4];
static mut CURAO: [i32; 4] = [0; 4];
static mut CURAL: [i32; 4] = [0; 4];

/* R15: impl-method return-struct registry (owner type, method name span, return struct type or -1) */
static mut MRTOWN: [i32; 1024] = [0; 1024];
static mut MRTO: [i32; 1024] = [0; 1024];
static mut MRTL: [i32; 1024] = [0; 1024];
static mut MRTS: [i32; 1024] = [0; 1024];
static mut MRTB: [i32; 1024] = [0; 1024];   /* R2bv: 1 if this method's declared return type is `bool` */
static mut MRTI64: [i32; 1024] = [0; 1024];   /* value-width: 1 if this method's declared return type is i64/u64 -> caller tags EIS64 */
static mut MRTV: [i32; 1024] = [0; 1024];   /* Wave-2 bug 3: 1 if this method has NO `-> T` (returns unit/()) */
static mut MRTSTR: [i32; 1024] = [0; 1024];   /* S1 L1: this method's declared return kind -- 3 String, 1 &str, 0 neither -> caller tags EISSTRING/EISSTR (else a str result prints as a pointer) */
static mut MREN: [i32; 1024] = [0 - 1; 1024];   /* LAYER5 P-METHOD-RETURNS-ENUM: this method's `-> UserEnum` / `-> &UserEnum` return enum index (-1 = none) -> caller tags EISENUM so `match p.peek() { .. }` dispatches (was untagged -> a `-> &Tok` slice-element peek matched a scalar -> disc mis-read -> crash). The free-fn analogue is FREN (R8). */
static mut MROPTK: [i32; 1024] = [0; 1024];     /* SPAN-4 P-OPTION-PAYLOAD-DISPLAY-KIND (method arm): this method's `-> Option<P>`/`-> Result<P,_>` payload display-kind (0 scalar, 1 str, 3 char) -> the caller's `match p.m(){Some(x)=>..}` binder Displays x by the DECLARED return type, not by whichever body branch emitted last. The free-fn analogue is FROPTK. */
static mut MROPTREF: [i32; 1024] = [0; 1024];   /* AST-1 P-OPTION-PAYLOAD-BYREF (method arm): this method's `-> Option<&T>`/`-> Option<Box<T>>` payload-is-a-reference flag -> the caller's `match p.m(){Some(x)=>*x}` binder treats `*x` as a no-op (`fn last(&self)->Option<&i32>{self.s.last()}`). The free-fn analogue is FROPTREF. */
static mut MRTREF: [i32; 1024] = [0; 1024];   /* LAYER5 P-METHOD-RETURNS-SLICE-ENUM-REF: 1 if this method returns a REFERENCE (`-> &T`) -> caller sets EWASREF so a match payload binder is by-ref (`*n` a no-op), else `*n` derefs the scalar payload -> crash. The free-fn analogue is FRETREF (fnret_ref). */
static mut MRVEC: [i32; 1024] = [0; 1024];   /* LAYER7 P-METHOD-RETURNS-VEC: 1 if this method returns `-> Vec<T>`/`-> ThinVec<T>` -> caller tags the result EISVEC (SK=2 binding) so a bound `let v = c.parse()` / a chained `.method().len()` dispatch the Vec (was untagged -> `.len()` garbage, `for x in v.iter()` parse-drift). The free-fn analogue is scan_ret_kind's RET_VEC. */
static mut MRVELT: [i32; 1024] = [0 - 1; 1024];   /* LAYER7 P-METHOD-RETURNS-VEC: element STRUCT-type of a `-> Vec<Struct>` method return (-1 none) -> threads SVELT to the caller binding. */
static mut MRVELENUM: [i32; 1024] = [0 - 1; 1024];   /* LAYER7 P-METHOD-RETURNS-VEC: element ENUM-type of a `-> Vec<Enum>` method return (-1 none). */
static mut NMRT: i32 = 0;

/* R9: generic-fn registry (monomorphization). GFO/GFL name span, GFTPO/GFTPL the type-param name (e.g. T),
   GFSIG the pos of the `(` param list, GFARG the index of the first param whose type is T. Instances to emit:
   (IGF fn-index, ISTYP concrete struct type; -2 = scalar i32). GSUB_* = active T-substitution while emitting. */
static mut GFO: [i32; 128] = [0; 128];
static mut GFL: [i32; 128] = [0; 128];
static mut GFTPO: [i32; 128] = [0; 128];
static mut GFTPL: [i32; 128] = [0; 128];
static mut GFSIG: [i32; 128] = [0; 128];
static mut GFGS: [i32; 128] = [0; 128];   /* TRAIT: generic-header span start (after `<`) -- re-scanned in emit_generic_inst so is_fn_generic sees a 2nd-position `F: Fn` param */
static mut GFGE: [i32; 128] = [0; 128];   /* TRAIT: generic-header span end (the closing `>`) */
static mut GFARG: [i32; 128] = [0; 128];
static mut TURBO_STYP: i32 = 0 - 1;   /* TRAIT R3: a call's `::<Type>` turbofish type-arg (user struct index) -> monomorph key when no param carries T (e.g. `m::<A>()`) */
static mut GFRETT: [i32; 128] = [0; 128];   /* KP0: 1 if the generic fn's return type IS the bare type-param T -> result kind = the T-arg's kind */
static mut GFRETOPT: [i32; 128] = [0; 128]; /* KP0/K1: 1 if return is Option<T>, 2 if Result<T,..> (T = type-param) -> payload kind = the T-arg's kind */
static mut NGF: i32 = 0;
static mut IGF: [i32; 256] = [0; 256];
static mut ISTYP: [i32; 256] = [0; 256];
static mut NINST: i32 = 0;
static mut GSUB_ACT: i32 = 0;
static mut GSUB_TPO: i32 = 0;
static mut GSUB_TPL: i32 = 0;
static mut GSUB_STYP: i32 = 0;
static mut IMPL_SELF_ENUM: i32 = 0 - 1;   /* R2: while emitting an `impl EnumName` method, bind self as this enum */
static mut BLANK_PRIM_SELF: i32 = 0;      /* R1 keystone: emitting a prim-concrete impl monomorph (`impl Tr for i32`) -- bind `self` as a scalar param (value, not a struct pointer); `*self` is identity */
/* S3 R1: generic-inherent-impl method registry (impl<T> Wrap<T> { fn m }) -- for on-demand per-T
   monomorphization so self.v (a field of type T) resolves the concrete type-arg kind. */
static mut GIMOWN: [i32; 256] = [0; 256];   /* owner struct type */
static mut GIMMO: [i32; 256] = [0; 256];    /* method name span */
static mut GIML: [i32; 256] = [0; 256];
static mut GIMTPO: [i32; 256] = [0; 256];   /* impl generic-param name (T) */
static mut GIMTPL: [i32; 256] = [0; 256];
static mut GIMBODY: [i32; 256] = [0; 256];  /* re-parse pos (at the method name) */
static mut NGIM: i32 = 0;
static mut GIMINST: [i32; 1024] = [0; 1024];  /* emitted (owner*..)+styp instances, to emit each once */
static mut GIMINSTM: [i32; 1024] = [0; 1024];
static mut NGIMINST: i32 = 0;
static mut GIM_ACT: i32 = 0;   /* S3 R1: 1 while emitting a generic-impl method monomorph (self.v resolves via GSUB_STYP) */
static mut CUR_RET_I64: i32 = 0;          /* R2: the current fn returns i64/u64 -- sign/zero-extend a 32-bit return value */
static mut CUR_RET_RES: i32 = 0;          /* CHECK N4: current fn return kind -- 1 = Result<..>, 2 = Option<..>, 0 = other (for the `?`-type-mismatch E0277 check) */
static mut CUR_RET_U64: i32 = 0;
/* R2: blanket impl<T: Bound> Trait for T -- method bodies monomorphized per concrete struct (mirrors GF) */
static mut BIMO: [i32; 128] = [0; 128];   /* blanket method name span */
static mut BIML: [i32; 128] = [0; 128];
static mut BITPO: [i32; 128] = [0; 128];  /* type-param token (T) span */
static mut BITPL: [i32; 128] = [0; 128];
static mut BIBODY: [i32; 128] = [0; 128]; /* POS of the method's `(` signature */
static mut BIPRIM: [i32; 128] = [0; 128]; /* R1 keystone: 0 = true blanket `impl<T> Tr for T`; else the primitive code (-2=i32) for a CONCRETE `impl Tr for i32` mis-bucketed as blanket (ty_find<0). A prim-concrete impl dispatches only onto a scalar receiver, never a struct. */
static mut BIRET: [i32; 128] = [0; 128];  /* R16b: blanket/prim-impl method return kind -- 3=String, 1=&str, else 0 -- so a prim-receiver `42.g()` returning a String is tagged (was a pointer) */
static mut NBI: i32 = 0;
static mut IBI: [i32; 256] = [0; 256];    /* instance: blanket-method index */
static mut IBTYP: [i32; 256] = [0; 256];  /* instance: concrete struct type */
static mut NBINST: i32 = 0;

/* R10: closures. Each `|params| body` is lifted to a top-level fn __closure_N(env, params...) emitted
   deferred; captures = enclosing locals referenced in the body. A closure VALUE is a heap {code@0, env@4}
   fat pointer (code = faddr(__closure_N)); called via calli. Locals/params of closure type are SK=4.
   Registry arrays flattened: params CPO/CPL[cid*4+i] (<=4), captures CAO/CAL[cid*8+i] (<=8). */
static mut CBO: [i32; 256] = [0; 256];
static mut CBL: [i32; 256] = [0; 256];
static mut CBLK: [i32; 256] = [0; 256];
static mut CNP: [i32; 256] = [0; 256];
static mut CNC: [i32; 256] = [0; 256];
static mut CRETCLO: [i32; 256] = [0; 256];
static mut SCRETCLO: [i32; 8192] = [0; 8192];
static mut ECRETCLO: i32 = 0;
static mut CPO: [i32; 1024] = [0; 1024];
static mut CPL: [i32; 1024] = [0; 1024];
static mut CAO: [i32; 2048] = [0; 2048];
static mut CAL: [i32; 2048] = [0; 2048];
static mut CAK: [i32; 2048] = [0; 2048];   /* capture symbol-kind (SK) */
static mut CAS: [i32; 2048] = [0; 2048];   /* capture struct type (SST) */
static mut CABR: [i32; 2048] = [0; 2048];  /* R2bg: per-capture by-ref flag (mutated scalar capture -> env holds &outer_cell) */
static mut CFORREF: [i32; 256] = [0; 256]; /* std-hoist P4.2: this closure's params are iter-element for-refs (value passed, `*x` a no-op) -- set when the closure is routed to the std iter library */
static mut CLO_FORREF: i32 = 0;            /* std-hoist P4.2: mark the NEXT parse_closure's params for-ref (iter-adapter routing) */
static mut NCLO: i32 = 0;
static mut TESTO: [i32; 128] = [0; 128];     /* K5: #[test] fn name offsets into SRC */
static mut TESTL: [i32; 128] = [0; 128];     /* K5: #[test] fn name lengths */
static mut NTEST: i32 = 0;                    /* K5: number of #[test] fns discovered */
static mut MAIN_SEEN: i32 = 0;                /* K5: a user `fn main` exists -> don't synthesize a test runner */
static mut INMAIN: i32 = 0;                   /* R3: emitting fn main's body -> flush buffered output before each ret (cf turns main's ret into exit) */
static mut EISCLO: i32 = 0;
static mut GFNO: [i32; 8] = [0; 8];   /* W1b: Fn-bound generic param names (e.g. `F` in `<F: Fn(..)>`) -> tag `f: F` params SK=4 */
static mut GFNL: [i32; 8] = [0; 8];
static mut GFNRB: [i32; 8] = [0; 8];   /* std-hoist: this Fn-bound generic's return type is `bool` -> a `f(x)` calli result is a bool (so `!f(x)` uses ==0) */
static mut GNFN: i32 = 0;
/* W1b: scan a generic-param region [gs,ge) for `Name: ...Fn...` bounds; record each Fn-bound param name. */
fn scan_fn_generics(gs: i32, ge: i32) {
    unsafe {
        GNFN = 0;
        let mut q = gs;
        loop {
            if q >= ge { break; }
            while q < ge && (at(q) == 32 || at(q) == 9 || at(q) == 10 || at(q) == 13 || at(q) == 44) { q = q + 1; }
            if q >= ge { break; }
            let s = q;
            while q < ge && is_idc(at(q)) { q = q + 1; }
            let nlen = q - s;
            let mut hasfn = 0; let mut retb = 0; let mut d = 0; let mut r = q;
            loop {
                if r >= ge { break; }
                let c = at(r);
                if c == 60 || c == 40 { d = d + 1; }
                else if c == 62 || c == 41 { if d == 0 { break; } d = d - 1; }
                else if c == 44 && d == 0 { break; }
                else if d == 0 && c == 45 && r + 1 < ge && at(r + 1) == 62 { let mut w = r + 2; while w < ge && (at(w) == 32 || at(w) == 9) { w = w + 1; } if w + 3 < ge && at(w) == 98 && at(w + 1) == 111 && at(w + 2) == 111 && at(w + 3) == 108 { retb = 1; } }   /* `Fn(..) -> bool` return type (depth 0 within this generic-param region) */
                else if d == 0 && c == 70 && r + 1 < ge && at(r + 1) == 110 { hasfn = 1; }   /* `Fn` at depth 0 in the bound */
                r = r + 1;
            }
            if hasfn == 1 && nlen > 0 && GNFN < 8 { GFNO[GNFN as usize] = s; GFNL[GNFN as usize] = nlen; GFNRB[GNFN as usize] = retb; GNFN = GNFN + 1; }
            q = r + 1;
        }
    }
}
/* R-either: scan a `where` clause span for `Ident: Fn/FnOnce/FnMut(..)[-> bool]` bounds and APPEND them to
   the Fn-generic registry (does NOT reset GNFN -- runs after scan_fn_generics has scanned the inline `<..>`).
   Same shape as a `<..>` param list: comma-separated `Ident: Bound` clauses. */
fn scan_where_fn_bounds(ws: i32, we: i32) {
    unsafe {
        let mut q = ws;
        loop {
            if q >= we { break; }
            while q < we && (at(q) == 32 || at(q) == 9 || at(q) == 10 || at(q) == 13 || at(q) == 44) { q = q + 1; }
            if q >= we { break; }
            let s = q;
            while q < we && is_idc(at(q)) { q = q + 1; }
            let nlen = q - s;
            let mut hasfn = 0; let mut retb = 0; let mut d = 0; let mut r = q;
            loop {
                if r >= we { break; }
                let c = at(r);
                if c == 60 || c == 40 { d = d + 1; }
                else if c == 62 || c == 41 { if d == 0 { break; } d = d - 1; }
                else if c == 44 && d == 0 { break; }
                else if d == 0 && c == 45 && r + 1 < we && at(r + 1) == 62 { let mut w = r + 2; while w < we && (at(w) == 32 || at(w) == 9) { w = w + 1; } if w + 3 < we && at(w) == 98 && at(w + 1) == 111 && at(w + 2) == 111 && at(w + 3) == 108 { retb = 1; } }   /* `-> bool` */
                else if d == 0 && c == 70 && r + 1 < we && at(r + 1) == 110 { hasfn = 1; }   /* `Fn` at depth 0 */
                r = r + 1;
            }
            if hasfn == 1 && nlen > 0 && GNFN < 8 { GFNO[GNFN as usize] = s; GFNL[GNFN as usize] = nlen; GFNRB[GNFN as usize] = retb; GNFN = GNFN + 1; }
            q = r + 1;
        }
    }
}
fn is_fn_generic(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < GNFN { if span_eq_at(GFNO[i as usize], GFNL[i as usize], o, l) { return 1; } i = i + 1; } 0 }
}
fn fn_generic_ret_bool(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < GNFN { if span_eq_at(GFNO[i as usize], GFNL[i as usize], o, l) { return GFNRB[i as usize]; } i = i + 1; } 0 }
}
static mut FPTO: [i32; 32] = [0; 32];   /* W1b: address-taken fn names -> emit an env-dropping thunk __fp_<name> */
static mut FPTL: [i32; 32] = [0; 32];
static mut FPTAC: [i32; 32] = [0; 32];
static mut NFPT: i32 = 0;
fn reg_fnptr(o: i32, l: i32, ac: i32) {
    unsafe { let mut i = 0; while i < NFPT { if span_eq_at(FPTO[i as usize], FPTL[i as usize], o, l) { return; } i = i + 1; }
        if NFPT < 32 { FPTO[NFPT as usize] = o; FPTL[NFPT as usize] = l; FPTAC[NFPT as usize] = ac; NFPT = NFPT + 1; } }
}

fn ty_find(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NTYP { if span_eq_at(TSO[i as usize], TSL[i as usize], o, l) { return i; } i = i + 1; } -1 }
}
fn fld_find(ty: i32, o: i32, l: i32) -> i32 {   /* -> field index, or -1 */
    unsafe { let mut i = 0; while i < NFLD { if FSST[i as usize] == ty && span_eq_at(FSO[i as usize], FSL[i as usize], o, l) { return i; } i = i + 1; } -1 }
}
fn fld_nth(ty: i32, n: i32) -> i32 {            /* A: -> the field-table index of the n-th field of `ty` (tuple-struct .N access), or -1 */
    unsafe { let mut i = 0; let mut c = 0; while i < NFLD { if FSST[i as usize] == ty { if c == n { return i; } c = c + 1; } i = i + 1; } -1 }
}

fn is_print() -> bool { unsafe { ID_L == 5 && at(ID_O) == 112 && at(ID_O + 1) == 114 && at(ID_O + 2) == 105 && at(ID_O + 3) == 110 && at(ID_O + 4) == 116 } }
fn is_println() -> bool { unsafe { ID_L == 7 && at(ID_O) == 112 && at(ID_O + 4) == 116 && at(ID_O + 5) == 108 && at(ID_O + 6) == 110 } }
fn is_assert() -> bool { unsafe { ID_L == 6 && at(ID_O) == 97 && at(ID_O + 1) == 115 && at(ID_O + 2) == 115 && at(ID_O + 3) == 101 && at(ID_O + 4) == 114 && at(ID_O + 5) == 116 } }   /* assert */
fn is_assert2() -> bool { unsafe { ID_L == 9 && at(ID_O) == 97 && at(ID_O + 1) == 115 && at(ID_O + 2) == 115 && at(ID_O + 3) == 101 && at(ID_O + 4) == 114 && at(ID_O + 5) == 116 && at(ID_O + 6) == 95 } }   /* assert_eq / assert_ne */

/* emit the fixed rzir prelude: __putc / __print_str / __print_uint / __print_int (host-free I/O). */
fn emit_prelude() {
    /* R3 buffered output: __obuf(1MB)+__obuf_n; __putc appends & flushes on full; __flush writes+resets;
       __print_str flushes then writes. `call __flush` is emitted before every process exit (main ret via the
       INMAIN-gated flush in emit_tail/return, plus process::exit + panic sites). */
    println!("fn __flush");
    println!("  v0: i32 = gref __obuf_n");
    println!("  v1: i32 = load v0");
    println!("  v2: i32 = const 1");
    println!("  v3: i32 = const 1");
    println!("  v4: i32 = gref __obuf");
    println!("  v5: i32 = syscall v2 v3 v4 v1");
    println!("  v6: i32 = const 0");
    println!("  store v0 v6");
    println!("  ret v6");
    println!("fn __putc");
    println!("  v0: i32 = param");
    println!("  v1: i32 = gref __obuf_n");
    println!("  v2: i32 = load v1");
    println!("  v3: i32 = gref __obuf");
    println!("  v4: i32 = add v3 v2");
    println!("  storeb v4 v0");
    println!("  v5: i32 = const 1");
    println!("  v6: i32 = add v2 v5");
    println!("  store v1 v6");
    println!("  v7: i32 = const 1048576");
    println!("  v8: i32 = ge v6 v7");
    println!("  brz v8"); ckreset();
    println!("  v9: i32 = call __flush");
    println!("  pool"); ckreset();
    println!("  v10: i32 = const 0");
    println!("  ret v10");
    println!("fn __print_str");
    println!("  v0: i32 = param");
    println!("  v1: i32 = param");
    println!("  v2: i32 = call __flush");
    println!("  v3: i32 = const 1");
    println!("  v4: i32 = const 1");
    println!("  v5: i32 = syscall v3 v4 v0 v1");
    println!("  v6: i32 = const 0");
    println!("  ret v6");
    println!("fn __print_uint");
    println!("  v0: i32 = param");
    println!("  v1: i32 = const 0");
    println!("  v2: i32 = const 10");
    println!("  v3: i32 = ge v0 v2");
    println!("  brz v3"); ckreset();
    println!("  v4: i32 = const 10");
    println!("  v5: i32 = div v0 v4");
    println!("  v6: i32 = call __print_uint v5");
    println!("  pool"); ckreset();
    println!("  v7: i32 = const 48");
    println!("  v8: i32 = const 10");
    println!("  v9: i32 = mod v0 v8");
    println!("  v10: i32 = add v7 v9");
    println!("  v11: i32 = call __putc v10");
    println!("  ret v1");
    println!("fn __ndig");
    println!("  v0: i32 = param");
    println!("  v1: i32 = const 10"); println!("  v2: i32 = ge v0 v1"); println!("  brz v2"); ckreset();
    println!("  v3: i32 = const 10"); println!("  v4: i32 = div v0 v3");
    println!("  v5: i32 = call __ndig v4"); println!("  v6: i32 = const 1"); println!("  v7: i32 = add v5 v6"); println!("  ret v7");
    println!("  pool"); ckreset();
    println!("  v8: i32 = const 1"); println!("  ret v8");
    println!("fn __ppow10");
    println!("  v0: i32 = param");
    println!("  v1: i32 = const 0"); println!("  v2: i32 = gt v0 v1"); println!("  brz v2"); ckreset();
    println!("  v3: i32 = const 1"); println!("  v4: i32 = sub v0 v3");
    println!("  v5: i32 = call __ppow10 v4"); println!("  v6: i32 = const 10"); println!("  v7: i32 = mul v5 v6"); println!("  ret v7");
    println!("  pool"); ckreset();
    println!("  v8: i32 = const 1"); println!("  ret v8");
    println!("fn __striptz");
    println!("  v0: i32 = param");
    println!("  v1: i32 = const 10"); println!("  v2: i32 = mod v0 v1");
    println!("  v3: i32 = const 0"); println!("  v4: i32 = eq v2 v3"); println!("  brz v4"); ckreset();
    println!("  v5: i32 = const 10"); println!("  v6: i32 = div v0 v5");
    println!("  v7: i32 = call __striptz v6"); println!("  ret v7");
    println!("  pool"); ckreset();
    println!("  ret v0");
    println!("fn __pzeros");
    println!("  v0: i32 = param");
    println!("  v1: i32 = const 0"); println!("  v2: i32 = gt v0 v1"); println!("  brz v2"); ckreset();
    println!("  v3: i32 = const 48"); println!("  v4: i32 = call __putc v3");
    println!("  v5: i32 = const 1"); println!("  v6: i32 = sub v0 v5"); println!("  v7: i32 = call __pzeros v6");
    println!("  pool"); ckreset();
    println!("  v8: i32 = const 0"); println!("  ret v8");
    println!("fn __print_padz");
    println!("  v0: i32 = param"); println!("  v1: i32 = param");
    println!("  v2: i32 = call __ndig v0");
    println!("  v3: i32 = sub v1 v2"); println!("  v4: i32 = call __pzeros v3");
    println!("  v5: i32 = call __print_uint v0");
    println!("  v6: i32 = const 0"); println!("  ret v6");
    /* SOFTFLOAT: print the significant fractional digits of f (a td-digit value with leading zeros),
       stripping trailing zeros: if f%10==0 recurse on (f/10, td-1), else print_padz(f, td). */
    println!("fn __strip_frac");
    println!("  v0: i32 = param"); println!("  v1: i32 = param");
    println!("  v2: i32 = const 10"); println!("  v3: i32 = mod v0 v2");
    println!("  v4: i32 = const 0"); println!("  v5: i32 = eq v3 v4"); println!("  brz v5"); ckreset();
    println!("  v6: i32 = const 10"); println!("  v7: i32 = div v0 v6");
    println!("  v8: i32 = const 1"); println!("  v9: i32 = sub v1 v8");
    println!("  v10: i32 = call __strip_frac v7 v9"); println!("  ret v10");
    println!("  pool"); ckreset();
    println!("  v11: i32 = call __print_padz v0 v1");
    println!("  v12: i32 = const 0"); println!("  ret v12");
    println!("fn __print_exp");
    println!("  v50: i32 = param");
    println!("  v51: i32 = const 0"); println!("  v52: i32 = lt v50 v51");
    println!("  brz v52"); ckreset(); println!("  v53: i32 = const 45"); println!("  v54: i32 = call __putc v53"); println!("  pool"); ckreset();
    println!("  v55: i32 = const 1"); println!("  v56: i32 = const 2");
    println!("  v57: i32 = mul v56 v52"); println!("  v58: i32 = sub v55 v57"); println!("  v0: i32 = mul v50 v58");
    println!("  v1: i32 = const 0"); println!("  v2: i32 = eq v0 v1"); println!("  brz v2"); ckreset();
    println!("  v3: i32 = const 48"); println!("  v4: i32 = call __putc v3");
    println!("  v5: i32 = const 101"); println!("  v6: i32 = call __putc v5");
    println!("  v7: i32 = const 48"); println!("  v8: i32 = call __putc v7");
    println!("  v9: i32 = const 0"); println!("  ret v9");
    println!("  pool"); ckreset();
    println!("  v10: i32 = call __ndig v0");
    println!("  v11: i32 = call __striptz v0");
    println!("  v12: i32 = call __ndig v11");
    println!("  v13: i32 = const 1"); println!("  v14: i32 = sub v12 v13");
    println!("  v15: i32 = call __ppow10 v14");
    println!("  v16: i32 = div v11 v15");
    println!("  v17: i32 = const 48"); println!("  v18: i32 = add v16 v17"); println!("  v19: i32 = call __putc v18");
    println!("  v20: i32 = const 1"); println!("  v21: i32 = gt v12 v20"); println!("  brz v21"); ckreset();
    println!("  v22: i32 = const 46"); println!("  v23: i32 = call __putc v22");
    println!("  v24: i32 = mod v11 v15"); println!("  v25: i32 = call __print_padz v24 v14");
    println!("  pool"); ckreset();
    println!("  v26: i32 = const 101"); println!("  v27: i32 = call __putc v26");
    println!("  v28: i32 = const 1"); println!("  v29: i32 = sub v10 v28"); println!("  v30: i32 = call __print_uint v29");
    println!("  v31: i32 = const 0"); println!("  ret v31");
    println!("fn __print_hex");                     /* R6: unsigned hex printer for {:x} (digit = 48+d + 39*(d>=10)) */
    println!("  v0: i32 = param");
    println!("  v1: i32 = const 0");
    println!("  v2: i32 = const 16");
    println!("  v3: i32 = ge v0 v2");
    println!("  brz v3"); ckreset();
    println!("  v4: i32 = const 16");
    println!("  v5: i32 = div v0 v4");
    println!("  v6: i32 = call __print_hex v5");
    println!("  pool"); ckreset();
    println!("  v7: i32 = const 16");
    println!("  v8: i32 = mod v0 v7");
    println!("  v9: i32 = const 10");
    println!("  v10: i32 = ge v8 v9");
    println!("  v11: i32 = const 39");
    println!("  v12: i32 = mul v10 v11");
    println!("  v13: i32 = const 48");
    println!("  v14: i32 = add v13 v8");
    println!("  v15: i32 = add v14 v12");
    println!("  v16: i32 = call __putc v15");
    println!("  ret v1");
    println!("fn __print_hexup");                   /* fmt: {:X} uppercase hex (A-F = 48+d+7) */
    println!("  v0: i32 = param");
    println!("  v1: i32 = const 0");
    println!("  v2: i32 = const 16");
    println!("  v3: i32 = ge v0 v2");
    println!("  brz v3"); ckreset();
    println!("  v4: i32 = const 16");
    println!("  v5: i32 = div v0 v4");
    println!("  v6: i32 = call __print_hexup v5");
    println!("  pool"); ckreset();
    println!("  v7: i32 = const 16");
    println!("  v8: i32 = mod v0 v7");
    println!("  v9: i32 = const 10");
    println!("  v10: i32 = ge v8 v9");
    println!("  v11: i32 = const 7");
    println!("  v12: i32 = mul v10 v11");
    println!("  v13: i32 = const 48");
    println!("  v14: i32 = add v13 v8");
    println!("  v15: i32 = add v14 v12");
    println!("  v16: i32 = call __putc v15");
    println!("  ret v1");
    println!("fn __print_radix");                   /* R2: unsigned printer in an arbitrary base 2..16 (for {:b}/{:o}); v0=n v1=base */
    println!("  v0: i32 = param");
    println!("  v1: i32 = param");
    println!("  v2: i32 = ge v0 v1");
    println!("  brz v2"); ckreset();
    println!("  v3: i32 = div v0 v1");
    println!("  v4: i32 = call __print_radix v3 v1");
    println!("  pool"); ckreset();
    println!("  v5: i32 = mod v0 v1");
    println!("  v6: i32 = const 10");
    println!("  v7: i32 = ge v5 v6");
    println!("  v8: i32 = const 39");
    println!("  v9: i32 = mul v7 v8");
    println!("  v10: i32 = const 48");
    println!("  v11: i32 = add v10 v5");
    println!("  v12: i32 = add v11 v9");
    println!("  v13: i32 = call __putc v12");
    println!("  ret v13");
    println!("fn __digits");                        /* R6: decimal digit count of an unsigned value (0 -> 1) */
    println!("  v0: i32 = param");
    println!("  v1: i32 = const 10");
    println!("  v2: i32 = lt v0 v1");
    println!("  brz v2"); ckreset();
    println!("  v3: i32 = const 1");
    println!("  ret v3");
    println!("  pool"); ckreset();
    println!("  v4: i32 = const 10");
    println!("  v5: i32 = div v0 v4");
    println!("  v6: i32 = call __digits v5");
    println!("  v7: i32 = const 1");
    println!("  v8: i32 = add v6 v7");
    println!("  ret v8");
    println!("fn __emit_pad");                       /* R6: emit `n` copies of char `c` (recursive) */
    println!("  v0: i32 = param");
    println!("  v1: i32 = param");
    println!("  v2: i32 = const 0");
    println!("  v3: i32 = gt v0 v2");
    println!("  brz v3"); ckreset();
    println!("  v4: i32 = call __putc v1");
    println!("  v5: i32 = const 1");
    println!("  v6: i32 = sub v0 v5");
    println!("  v7: i32 = call __emit_pad v6 v1");
    println!("  pool"); ckreset();
    println!("  ret v2");
    println!("fn __dbg_sep");                        /* R2cg: print ", " before element i when i>0 (Debug element separator) */
    println!("  v0: i32 = param");
    println!("  v1: i32 = const 0");
    println!("  v2: i32 = gt v0 v1");
    println!("  brz v2"); ckreset();
    println!("  v3: i32 = const 44");
    println!("  v4: i32 = call __putc v3");
    println!("  v5: i32 = const 32");
    println!("  v6: i32 = call __putc v5");
    println!("  pool"); ckreset();
    println!("  v7: i32 = const 0");
    println!("  ret v7");
    println!("fn __dbg_els");                        /* R2cg: recursively print buf[i..n] as `e0, e1, ..` (i32 elements) for {:?} */
    println!("  v0: i32 = param");                   /* buf (element base ptr) */
    println!("  v1: i32 = param");                   /* i */
    println!("  v2: i32 = param");                   /* n */
    println!("  v3: i32 = lt v1 v2");
    println!("  brz v3"); ckreset();
    println!("  v4: i32 = call __dbg_sep v1");
    println!("  v5: i32 = const 4");
    println!("  v6: i32 = mul v1 v5");
    println!("  v7: i32 = add v0 v6");
    println!("  v8: i32 = load v7");
    println!("  v9: i32 = call __print_int v8");
    println!("  v10: i32 = const 1");
    println!("  v11: i32 = add v1 v10");
    println!("  v12: i32 = call __dbg_els v0 v11 v2");
    println!("  pool"); ckreset();
    println!("  v13: i32 = const 0");
    println!("  ret v13");
    println!("fn __str_count_sep");                  /* R6: count occurrences of byte `sep` in ptr[0..len] (recursive) */
    println!("  v0: i32 = param");                   /* ptr */
    println!("  v1: i32 = param");                   /* len */
    println!("  v2: i32 = param");                   /* sep */
    println!("  v3: i32 = const 0");
    println!("  v4: i32 = gt v1 v3");
    println!("  brz v4"); ckreset();
    println!("  v5: i32 = loadb v0");
    println!("  v6: i32 = eq v5 v2");
    println!("  v7: i32 = const 1");
    println!("  v8: i32 = add v0 v7");
    println!("  v9: i32 = const 1");
    println!("  v10: i32 = sub v1 v9");
    println!("  v11: i32 = call __str_count_sep v8 v10 v2");
    println!("  v12: i32 = add v6 v11");
    println!("  ret v12");
    println!("  pool"); ckreset();
    println!("  ret v3");
    println!("fn __str_seglen");                     /* R2ax: length of p[0..ln] until the first `sep` byte (or ln) (recursive) */
    println!("  v0: i32 = param");                   /* p */
    println!("  v1: i32 = param");                   /* ln */
    println!("  v2: i32 = param");                   /* sep */
    println!("  v3: i32 = const 0");
    println!("  v4: i32 = gt v1 v3");
    println!("  brz v4"); ckreset();
    println!("  v5: i32 = loadb v0");
    println!("  v6: i32 = eq v5 v2");
    println!("  brz v6"); ckreset();
    println!("  ret v3");                            /* p[0]==sep -> seglen 0 */
    println!("  pool"); ckreset();
    println!("  v7: i32 = const 1");
    println!("  v8: i32 = add v0 v7");
    println!("  v9: i32 = sub v1 v7");
    println!("  v10: i32 = call __str_seglen v8 v9 v2");
    println!("  v11: i32 = add v10 v7");
    println!("  ret v11");
    println!("  pool"); ckreset();
    println!("  ret v3");                            /* ln<=0 -> 0 */
    println!("fn __str_split_into");                 /* R2ax: split p[0..ln] on `sep` into str descriptors; store desc-ptrs in vbuf, desc storage in dbuf; -> field count (recursive) */
    println!("  v0: i32 = param");                   /* vbuf (element buffer of the Vec) */
    println!("  v1: i32 = param");                   /* dbuf (8-byte descriptor storage) */
    println!("  v2: i32 = param");                   /* p */
    println!("  v3: i32 = param");                   /* ln */
    println!("  v4: i32 = param");                   /* sep */
    println!("  v5: i32 = call __str_seglen v2 v3 v4");   /* k = field length */
    println!("  store v1 v2");                       /* dbuf[0] = field ptr */
    println!("  v6: i32 = const 4");
    println!("  v7: i32 = add v1 v6");
    println!("  store v7 v5");                        /* dbuf[4] = field len */
    println!("  store v0 v1");                        /* vbuf[0] = dbuf (the descriptor) */
    println!("  v9: i32 = lt v5 v3");                 /* k < ln  -> a sep follows */
    println!("  brz v9"); ckreset();
    println!("  v10: i32 = const 1");
    println!("  v11: i32 = add v2 v5");
    println!("  v12: i32 = add v11 v10");            /* p + k + 1 */
    println!("  v13: i32 = sub v3 v5");
    println!("  v14: i32 = sub v13 v10");            /* ln - k - 1 */
    println!("  v15: i32 = const 4");
    println!("  v16: i32 = add v0 v15");             /* vbuf + 4 */
    println!("  v17: i32 = const 8");
    println!("  v18: i32 = add v1 v17");             /* dbuf + 8 */
    println!("  v19: i32 = call __str_split_into v16 v18 v12 v14 v4");
    println!("  v20: i32 = const 1");
    println!("  v21: i32 = add v20 v19");
    println!("  ret v21");
    println!("  pool"); ckreset();
    println!("  v22: i32 = const 1");
    println!("  ret v22");                            /* no more sep -> this is the last field */
    println!("fn __str_match");                      /* R2: 1 if the first nl bytes of hp equal np, else 0 (recursive) */
    println!("  v0: i32 = param");                   /* hp */
    println!("  v1: i32 = param");                   /* np */
    println!("  v2: i32 = param");                   /* nl */
    println!("  v3: i32 = const 0");
    println!("  v4: i32 = gt v2 v3");
    println!("  brz v4"); ckreset();
    println!("  v5: i32 = loadb v0");
    println!("  v6: i32 = loadb v1");
    println!("  v7: i32 = eq v5 v6");
    println!("  brz v7"); ckreset();
    println!("  v8: i32 = const 1");
    println!("  v9: i32 = add v0 v8");
    println!("  v10: i32 = add v1 v8");
    println!("  v11: i32 = sub v2 v8");
    println!("  v12: i32 = call __str_match v9 v10 v11");
    println!("  ret v12");
    println!("  pool"); ckreset();
    println!("  ret v3");
    println!("  pool"); ckreset();
    println!("  v13: i32 = const 1");
    println!("  ret v13");
    println!("fn __str_find");                       /* R2: index of needle (np,nl) in haystack (hp,hl), or -1 (recursive) */
    println!("  v0: i32 = param");                   /* hp */
    println!("  v1: i32 = param");                   /* hl */
    println!("  v2: i32 = param");                   /* np */
    println!("  v3: i32 = param");                   /* nl */
    println!("  v4: i32 = ge v1 v3");
    println!("  brz v4"); ckreset();
    println!("  v5: i32 = call __str_match v0 v2 v3");
    println!("  brz v5"); ckreset();
    println!("  v6: i32 = const 0");
    println!("  ret v6");
    println!("  pool"); ckreset();
    println!("  v7: i32 = const 1");
    println!("  v8: i32 = add v0 v7");
    println!("  v9: i32 = sub v1 v7");
    println!("  v10: i32 = call __str_find v8 v9 v2 v3");
    println!("  v11: i32 = const 0");
    println!("  v12: i32 = lt v10 v11");
    println!("  brz v12"); ckreset();
    println!("  v13: i32 = const 1");
    println!("  v14: i32 = sub v11 v13");
    println!("  ret v14");
    println!("  pool"); ckreset();
    println!("  v15: i32 = const 1");
    println!("  v16: i32 = add v10 v15");
    println!("  ret v16");
    println!("  pool"); ckreset();
    println!("  v17: i32 = const 1");
    println!("  v18: i32 = const 0");
    println!("  v19: i32 = sub v18 v17");
    println!("  ret v19");
    println!("fn __pair_minidx");                    /* LAYER2 P6: 0-based index of the min KEY among n 8-byte [key,val] pairs at base (recursive) */
    println!("  v0: i32 = param");                   /* base */
    println!("  v1: i32 = param");                   /* n */
    println!("  v2: i32 = const 1");
    println!("  v3: i32 = le v1 v2");                /* n <= 1 -> index 0 */
    println!("  brz v3"); ckreset();
    println!("  v4: i32 = const 0");
    println!("  ret v4");
    println!("  pool"); ckreset();
    println!("  v5: i32 = const 8");
    println!("  v6: i32 = add v0 v5");               /* base+8 */
    println!("  v7: i32 = const 1");
    println!("  v8: i32 = sub v1 v7");               /* n-1 */
    println!("  v9: i32 = call __pair_minidx v6 v8");
    println!("  v10: i32 = const 1");
    println!("  v11: i32 = add v9 v10");             /* candidate index relative to base */
    println!("  v12: i32 = load v0");                /* key[0] */
    println!("  v13: i32 = const 8");
    println!("  v14: i32 = mul v11 v13");
    println!("  v15: i32 = add v0 v14");
    println!("  v16: i32 = load v15");               /* key[cand] */
    println!("  v17: i32 = le v12 v16");             /* key0 <= keycand -> keep 0 */
    println!("  brz v17"); ckreset();
    println!("  v18: i32 = const 0");
    println!("  ret v18");
    println!("  pool"); ckreset();
    println!("  ret v11");
    println!("fn __pair_sort");                      /* LAYER2 P6: in-place selection sort of n 8-byte [key,val] pairs at base by key asc (recursive) */
    println!("  v0: i32 = param");                   /* base */
    println!("  v1: i32 = param");                   /* n */
    println!("  v2: i32 = const 1");
    println!("  v3: i32 = gt v1 v2");                /* n > 1 */
    println!("  brz v3"); ckreset();
    println!("  v4: i32 = call __pair_minidx v0 v1");
    println!("  v5: i32 = const 8");
    println!("  v6: i32 = mul v4 v5");
    println!("  v7: i32 = add v0 v6");               /* addr of min pair */
    println!("  v8: i32 = load v0");                 /* key0 */
    println!("  v9: i32 = const 4");
    println!("  v10: i32 = add v0 v9");
    println!("  v11: i32 = load v10");               /* val0 */
    println!("  v12: i32 = load v7");                /* keymin */
    println!("  v13: i32 = add v7 v9");
    println!("  v14: i32 = load v13");               /* valmin */
    println!("  store v0 v12");
    println!("  store v10 v14");
    println!("  store v7 v8");
    println!("  store v13 v11");
    println!("  v15: i32 = add v0 v5");              /* base+8 */
    println!("  v16: i32 = const 1");
    println!("  v17: i32 = sub v1 v16");             /* n-1 */
    println!("  v18: i32 = call __pair_sort v15 v17");
    println!("  ret v18");
    println!("  pool"); ckreset();
    println!("  v19: i32 = const 0");
    println!("  ret v19");
    println!("fn __str_lead_ws");                    /* R2: count leading ASCII whitespace in p[0..n] (recursive) */
    println!("  v0: i32 = param");   println!("  v1: i32 = param");
    println!("  v2: i32 = const 0");
    println!("  v3: i32 = gt v1 v2");
    println!("  brz v3"); ckreset();
    println!("  v4: i32 = loadb v0");
    println!("  v5: i32 = const 32");  println!("  v6: i32 = eq v4 v5");
    println!("  v7: i32 = const 9");   println!("  v8: i32 = eq v4 v7");
    println!("  v9: i32 = const 10");  println!("  v10: i32 = eq v4 v9");
    println!("  v11: i32 = const 13"); println!("  v12: i32 = eq v4 v11");
    println!("  v13: i32 = add v6 v8");
    println!("  v14: i32 = add v13 v10");
    println!("  v15: i32 = add v14 v12");
    println!("  brz v15"); ckreset();
    println!("  v16: i32 = const 1");
    println!("  v17: i32 = add v0 v16");
    println!("  v18: i32 = sub v1 v16");
    println!("  v19: i32 = call __str_lead_ws v17 v18");
    println!("  v20: i32 = add v16 v19");
    println!("  ret v20");
    println!("  pool"); ckreset();
    println!("  ret v2");
    println!("  pool"); ckreset();
    println!("  ret v2");
    println!("fn __str_trail_ws");                   /* R2: count trailing ASCII whitespace in p[0..n] (recursive) */
    println!("  v0: i32 = param");   println!("  v1: i32 = param");
    println!("  v2: i32 = const 0");
    println!("  v3: i32 = gt v1 v2");
    println!("  brz v3"); ckreset();
    println!("  v4: i32 = const 1");
    println!("  v5: i32 = sub v1 v4");
    println!("  v6: i32 = add v0 v5");
    println!("  v7: i32 = loadb v6");
    println!("  v8: i32 = const 32");  println!("  v9: i32 = eq v7 v8");
    println!("  v10: i32 = const 9");  println!("  v11: i32 = eq v7 v10");
    println!("  v12: i32 = const 10"); println!("  v13: i32 = eq v7 v12");
    println!("  v14: i32 = const 13"); println!("  v15: i32 = eq v7 v14");
    println!("  v16: i32 = add v9 v11");
    println!("  v17: i32 = add v16 v13");
    println!("  v18: i32 = add v17 v15");
    println!("  brz v18"); ckreset();
    println!("  v19: i32 = call __str_trail_ws v0 v5");
    println!("  v20: i32 = add v4 v19");
    println!("  ret v20");
    println!("  pool"); ckreset();
    println!("  ret v2");
    println!("  pool"); ckreset();
    println!("  ret v2");
    println!("fn __str_casemap");                    /* R2: map p[lo..lo+25]+delta over s[0..n] into d; lo=0,delta=0 = byte-copy */
    println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = param");
    println!("  v3: i32 = param"); println!("  v4: i32 = param");
    println!("  v5: i32 = const 0");
    println!("  v6: i32 = gt v2 v5");
    println!("  brz v6"); ckreset();
    println!("  v7: i32 = loadb v1");
    println!("  v8: i32 = ge v7 v3");
    println!("  v9: i32 = const 26");
    println!("  v10: i32 = add v3 v9");
    println!("  v11: i32 = lt v7 v10");
    println!("  v12: i32 = mul v8 v11");
    println!("  v13: i32 = mul v12 v4");
    println!("  v14: i32 = add v7 v13");
    println!("  storeb v0 v14");
    println!("  v15: i32 = const 1");
    println!("  v16: i32 = add v0 v15");
    println!("  v17: i32 = add v1 v15");
    println!("  v18: i32 = sub v2 v15");
    println!("  v19: i32 = call __str_casemap v16 v17 v18 v3 v4");
    println!("  pool"); ckreset();
    println!("  ret v5");
    println!("fn __str_repeat_into");                /* R2: write `count` copies of s[0..slen] into d */
    println!("  v0: i32 = param"); println!("  v1: i32 = param");
    println!("  v2: i32 = param"); println!("  v3: i32 = param");
    println!("  v4: i32 = const 0");
    println!("  v5: i32 = gt v3 v4");
    println!("  brz v5"); ckreset();
    println!("  v6: i32 = const 0");
    println!("  v7: i32 = call __str_casemap v0 v1 v2 v4 v6");
    println!("  v8: i32 = add v0 v2");
    println!("  v9: i32 = const 1");
    println!("  v10: i32 = sub v3 v9");
    println!("  v11: i32 = call __str_repeat_into v8 v1 v2 v10");
    println!("  pool"); ckreset();
    println!("  ret v4");
    println!("fn __str_rev");                        /* R2am: write s[0..n] reversed into d; d[i]=s[n-1-i] */
    println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = param");
    println!("  v3: i32 = const 0");
    println!("  v4: i32 = gt v2 v3");
    println!("  brz v4"); ckreset();
    println!("  v5: i32 = const 1");
    println!("  v6: i32 = sub v2 v5");
    println!("  v7: i32 = add v1 v6");
    println!("  v8: i32 = loadb v7");
    println!("  storeb v0 v8");
    println!("  v9: i32 = add v0 v5");
    println!("  v10: i32 = call __str_rev v9 v1 v6");
    println!("  pool"); ckreset();
    println!("  ret v3");
    println!("fn __str_replace_into");               /* R2: replace every `from`(desc) with `to`(desc) in h[0..hl] into d; -> total written */
    println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = param");
    println!("  v3: i32 = param"); println!("  v4: i32 = param");
    println!("  v5: i32 = const 0");
    println!("  v6: i32 = gt v2 v5");
    println!("  brz v6"); ckreset();
    println!("  v7: i32 = const 4");
    println!("  v8: i32 = add v3 v7");
    println!("  v9: i32 = load v8");
    println!("  v10: i32 = ge v2 v9");
    println!("  v11: i32 = gt v9 v5");
    println!("  v12: i32 = mul v10 v11");
    println!("  v13: i32 = load v3");
    println!("  v14: i32 = call __str_match v1 v13 v9");
    println!("  v15: i32 = mul v12 v14");
    println!("  brz v15"); ckreset();
    println!("  v16: i32 = load v4");
    println!("  v17: i32 = add v4 v7");
    println!("  v18: i32 = load v17");
    println!("  v19: i32 = call __str_casemap v0 v16 v18 v5 v5");
    println!("  v20: i32 = add v0 v18");
    println!("  v21: i32 = add v1 v9");
    println!("  v22: i32 = sub v2 v9");
    println!("  v23: i32 = call __str_replace_into v20 v21 v22 v3 v4");
    println!("  v24: i32 = add v18 v23");
    println!("  ret v24");
    println!("  pool"); ckreset();
    println!("  v25: i32 = loadb v1");
    println!("  storeb v0 v25");
    println!("  v26: i32 = const 1");
    println!("  v27: i32 = add v0 v26");
    println!("  v28: i32 = add v1 v26");
    println!("  v29: i32 = sub v2 v26");
    println!("  v30: i32 = call __str_replace_into v27 v28 v29 v3 v4");
    println!("  v31: i32 = add v26 v30");
    println!("  ret v31");
    println!("  pool"); ckreset();
    println!("  ret v5");
    println!("fn __popcount");                       /* R2: count set bits of x over n low bits (recursive) */
    println!("  v0: i32 = param"); println!("  v1: i32 = param");
    println!("  v2: i32 = const 0");
    println!("  v3: i32 = eq v1 v2");
    println!("  brz v3"); ckreset();
    println!("  v4: i32 = const 1");
    println!("  v5: i32 = and v0 v4");
    println!("  v6: i32 = const 1");
    println!("  v7: i32 = shu v0 v6");
    println!("  v8: i32 = const 1");
    println!("  v9: i32 = sub v1 v8");
    println!("  v10: i32 = call __popcount v7 v9");
    println!("  v11: i32 = add v5 v10");
    println!("  ret v11");
    println!("  pool"); ckreset();
    println!("  ret v2");
    println!("fn __popcount64");                      /* R2: count set bits of a 64-bit x over n low bits (recursive, i64 shu8/and8; recurse while n>0) */
    println!("  v0: i64 = param"); println!("  v1: i32 = param");   /* x (i64), n */
    println!("  v2: i32 = const 0");
    println!("  v3: i32 = gt v1 v2"); println!("  brz v3"); ckreset();          /* n>0 -> recurse; else 0 */
    println!("  v4: i32 = const 1"); println!("  v5: i64 = conv v4");   /* i64 1 */
    println!("  v6: i64 = and8 v0 v5");                                  /* low bit (0/1) */
    println!("  v7: i64 = shu8 v0 v5");                                  /* x >> 1 (logical) */
    println!("  v8: i32 = const 1"); println!("  v9: i32 = sub v1 v8");  /* n-1 */
    println!("  v10: i64 = call __popcount64 v7 v9");
    println!("  v11: i64 = add8 v6 v10"); println!("  ret v11");
    println!("  pool"); ckreset();
    println!("  v12: i32 = const 0"); println!("  v13: i64 = conv v12"); println!("  ret v13");
    println!("fn __atoi_acc");                        /* R2: fold digits p[i..n) into acc (recursive) */
    println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = param"); println!("  v3: i32 = param");
    println!("  v4: i32 = lt v1 v2"); println!("  brz v4"); ckreset();
    println!("  v5: i32 = add v0 v1"); println!("  v6: i32 = loadb v5");
    println!("  v7: i32 = const 48"); println!("  v8: i32 = sub v6 v7");
    println!("  v9: i32 = const 10"); println!("  v10: i32 = mul v3 v9");
    println!("  v11: i32 = add v10 v8");
    println!("  v12: i32 = const 1"); println!("  v13: i32 = add v1 v12");
    println!("  v14: i32 = call __atoi_acc v0 v13 v2 v11"); println!("  ret v14");
    println!("  pool"); ckreset(); println!("  ret v3");
    println!("fn __str_atoi");                        /* R2: p[0..n) -> i32 (optional leading '-') */
    println!("  v0: i32 = param"); println!("  v1: i32 = param");
    println!("  v2: i32 = const 0");
    println!("  v3: i32 = const 45"); println!("  v4: i32 = loadb v0");
    println!("  v5: i32 = eq v4 v3"); println!("  brz v5"); ckreset();
    println!("  v6: i32 = const 1");
    println!("  v7: i32 = call __atoi_acc v0 v6 v1 v2");
    println!("  v8: i32 = const 0"); println!("  v9: i32 = sub v8 v7"); println!("  ret v9");
    println!("  pool"); ckreset();
    println!("  v10: i32 = const 0");
    println!("  v11: i32 = call __atoi_acc v0 v10 v1 v2"); println!("  ret v11");
    println!("fn __valid_dig");                       /* P-STRING: 1 if every byte of p[i..n) is an ASCII digit (vacuously 1 when i>=n) */
    println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = param");
    println!("  v3: i32 = lt v1 v2"); println!("  brz v3"); ckreset();                  /* i>=n -> exhausted -> 1 */
    println!("  v4: i32 = add v0 v1"); println!("  v5: i32 = loadb v4");     /* c = p[i] */
    println!("  v6: i32 = const 48"); println!("  v7: i32 = ge v5 v6");      /* c>=48 */
    println!("  v8: i32 = const 57"); println!("  v9: i32 = le v5 v8");      /* c<=57 */
    println!("  v10: i32 = mul v7 v9");                                      /* isdigit */
    println!("  v11: i32 = const 1"); println!("  v12: i32 = add v1 v11");   /* i+1 */
    println!("  v13: i32 = call __valid_dig v0 v12 v2");                     /* rest */
    println!("  v14: i32 = mul v10 v13"); println!("  ret v14");
    println!("  pool"); ckreset();
    println!("  v15: i32 = const 1"); println!("  ret v15");
    println!("fn __str_valid_int");                   /* P-STRING: 1 if p[0..n) is a valid i32 literal (non-empty; optional leading '-'; rest all digits). No overflow check (bounded). */
    println!("  v0: i32 = param"); println!("  v1: i32 = param");           /* p, n */
    println!("  v2: i32 = loadb v0");                                        /* c0 */
    println!("  v3: i32 = const 45"); println!("  v4: i32 = eq v2 v3");      /* start = (c0=='-') ? 1 : 0 */
    println!("  v5: i32 = gt v1 v4");                                        /* hasdig = n > start (>=1 digit after sign) */
    println!("  v6: i32 = call __valid_dig v0 v4 v1");                       /* all p[start..n) digits */
    println!("  v7: i32 = mul v5 v6"); println!("  ret v7");
    println!("fn __radix_acc");                       /* R2: p[i..n) -> acc in base b; digit d = (c-48) - (c>57)*7 - (c>96)*32 (0-9 / A-F / a-f) */
    println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = param"); println!("  v3: i32 = param"); println!("  v4: i32 = param");
    println!("  v5: i32 = lt v1 v2"); println!("  brz v5"); ckreset();
    println!("  v6: i32 = add v0 v1"); println!("  v7: i32 = loadb v6");
    println!("  v8: i32 = const 48"); println!("  v9: i32 = sub v7 v8");
    println!("  v10: i32 = const 57"); println!("  v11: i32 = gt v7 v10");
    println!("  v12: i32 = const 7"); println!("  v13: i32 = mul v11 v12"); println!("  v14: i32 = sub v9 v13");
    println!("  v15: i32 = const 96"); println!("  v16: i32 = gt v7 v15");
    println!("  v17: i32 = const 32"); println!("  v18: i32 = mul v16 v17"); println!("  v19: i32 = sub v14 v18");
    println!("  v20: i32 = mul v4 v3"); println!("  v21: i32 = add v20 v19");
    println!("  v22: i32 = const 1"); println!("  v23: i32 = add v1 v22");
    println!("  v24: i32 = call __radix_acc v0 v23 v2 v3 v21"); println!("  ret v24");
    println!("  pool"); ckreset();
    println!("  ret v4");
    println!("fn __from_radix");                      /* R2: buf[0..n) parsed in the given base -> i32 */
    println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = param");
    println!("  v3: i32 = const 0");
    println!("  v4: i32 = call __radix_acc v0 v3 v1 v2 v3"); println!("  ret v4");
    println!("fn __vec_idxof");                      /* R2: index of x in ptr[0..n] (4-byte elems), or -1 (recursive) */
    println!("  v0: i32 = param");                   /* ptr */
    println!("  v1: i32 = param");                   /* n */
    println!("  v2: i32 = param");                   /* x */
    println!("  v3: i32 = const 0");
    println!("  v4: i32 = gt v1 v3");
    println!("  brz v4"); ckreset();
    println!("  v5: i32 = load v0");
    println!("  v6: i32 = eq v5 v2");
    println!("  brz v6"); ckreset();
    println!("  ret v3");
    println!("  pool"); ckreset();
    println!("  v7: i32 = const 4");
    println!("  v8: i32 = add v0 v7");
    println!("  v9: i32 = const 1");
    println!("  v10: i32 = sub v1 v9");
    println!("  v11: i32 = call __vec_idxof v8 v10 v2");
    println!("  v12: i32 = lt v11 v3");
    println!("  brz v12"); ckreset();
    println!("  v13: i32 = const 1");
    println!("  v14: i32 = sub v3 v13");
    println!("  ret v14");
    println!("  pool"); ckreset();
    println!("  v15: i32 = add v11 v9");
    println!("  ret v15");
    println!("  pool"); ckreset();
    println!("  v16: i32 = const 1");
    println!("  v17: i32 = const 0");
    println!("  v18: i32 = sub v17 v16");
    println!("  ret v18");
    println!("fn __vec_pass_step");                  /* R6: one bubble-pass step at index i (recursive): swap ptr[i],ptr[i+1] if out of order, recurse i+1 */
    println!("  v0: i32 = param");                   /* ptr */
    println!("  v1: i32 = param");                   /* i */
    println!("  v2: i32 = param");                   /* len */
    println!("  v3: i32 = const 0");
    println!("  v4: i32 = const 1");
    println!("  v5: i32 = sub v2 v4");               /* len-1 */
    println!("  v6: i32 = lt v1 v5");                /* i < len-1 */
    println!("  brz v6"); ckreset();
    println!("  v7: i32 = const 4");
    println!("  v8: i32 = mul v1 v7");
    println!("  v9: i32 = add v0 v8");               /* &ptr[i] */
    println!("  v10: i32 = load v9");
    println!("  v11: i32 = const 4");
    println!("  v12: i32 = add v9 v11");             /* &ptr[i+1] */
    println!("  v13: i32 = load v12");
    println!("  v14: i32 = gt v10 v13");             /* a > b */
    println!("  brz v14"); ckreset();
    println!("  store v9 v13");
    println!("  store v12 v10");
    println!("  pool"); ckreset();
    println!("  v15: i32 = const 1");
    println!("  v16: i32 = add v1 v15");
    println!("  v17: i32 = call __vec_pass_step v0 v16 v2");
    println!("  pool"); ckreset();
    println!("  ret v3");
    println!("fn __vec_sort");                       /* R6: bubble sort (recursive): one pass, then sort len-1 */
    println!("  v0: i32 = param");                   /* ptr */
    println!("  v1: i32 = param");                   /* len */
    println!("  v2: i32 = const 0");
    println!("  v3: i32 = const 1");
    println!("  v4: i32 = gt v1 v3");                /* len > 1 */
    println!("  brz v4"); ckreset();
    println!("  v5: i32 = const 0");
    println!("  v6: i32 = call __vec_pass_step v0 v5 v1");
    println!("  v7: i32 = const 1");
    println!("  v8: i32 = sub v1 v7");
    println!("  v9: i32 = call __vec_sort v0 v8");
    println!("  pool"); ckreset();
    println!("  ret v2");
    /* value-width: sign-extend to i64 and delegate to __print_int64 so i32::MIN (-2147483648) prints
       (32-bit `0 - v` overflowed for MIN -> signed div/mod garbage "-("). Byte-identical decimal for all
       other i32; self-host-safe (rzrs.rs's own {} emission only prints small positives). */
    println!("fn __print_int");
    println!("  v0: i32 = param");
    println!("  v1: i64 = convs v0");
    println!("  v2: i32 = call __print_int64 v1");
    println!("  v3: i32 = const 0");
    println!("  ret v3");
    /* VEC-GROW: growable built-in Vec. Descriptor {buf@0,len@4,cap@8}. __vec_reserve(dv) ensures room for
       one more element; when len>=cap it calls __vec_grow, which allocates a 2x buffer at runtime, LOOP-copies
       the live len*4 bytes (loop, NOT recursion -- avoids the D8 __str_casemap-style stack overflow on large
       vecs), and updates buf@0 + cap@8. emit_vec_push emits `call __vec_reserve dv` before each store and
       reloads buf, so a moved buffer is picked up. Element values are stable across grow (an i32 or a heap
       pointer, never a pointer INTO the vec buffer). */
    println!("fn __vec_grow");
    println!("  v0: i32 = param");                 /* dv (descriptor) */
    println!("  v1: i32 = const 4");
    println!("  v2: i32 = add v0 v1");
    println!("  v3: i32 = load v2");               /* len */
    println!("  v4: i32 = const 8");
    println!("  v5: i32 = add v0 v4");
    println!("  v6: i32 = load v5");               /* cap */
    println!("  v7: i32 = const 2");
    println!("  v8: i32 = mul v6 v7");             /* cap*2 */
    println!("  v40: i32 = const 8");
    println!("  v41: i32 = add v8 v40");           /* R18 P-CF-ACC facet-2: newcap = cap*2 + 8 -- guard cap==0 (mirrors __str_grow's cap*2+4096). Without it, a cap==0 Vec (fresh {buf@0,len@4,cap@8}=0 descriptor reaching __vec_reserve) grew to newcap=cap*2=0 -> a 0-capacity buffer -> push_raw stored buf[i] MARCHING FORWARD through the heap, clobbering adjacent Vec descriptors' buf@0 to 0 -> null-buf deref at scale (the acc-crash). +8 guarantees progress + room for the +1 push. */
    println!("  v9: i32 = const 4");
    println!("  v10: i32 = mul v41 v9");           /* newbytes = newcap*4 */
    println!("  v11: i32 = gref __heap_cursor");
    println!("  v12: i32 = load v11");             /* cur */
    println!("  v13: i32 = gref __heap");
    println!("  v14: i32 = add v13 v12");          /* newbuf = __heap + cur */
    println!("  v15: i32 = add v12 v10");
    println!("  store v11 v15");                   /* __heap_cursor = cur + newbytes */
    println!("  v16: i32 = load v0");              /* oldbuf */
    println!("  v17: i32 = const 4");
    println!("  v18: i32 = mul v3 v17");           /* lenbytes = len*4 */
    println!("  v19: i32 = const 0");              /* i (mutable via re-def below) */
    println!("  loop"); ckreset();
    println!("  v20: i32 = lt v19 v18");
    println!("  brz v20"); ckreset();                         /* exit loop when i >= lenbytes */
    println!("  v21: i32 = add v16 v19");
    println!("  v22: i32 = loadb v21");
    println!("  v23: i32 = add v14 v19");
    println!("  storeb v23 v22");
    println!("  cont"); ckreset();
    println!("  v24: i32 = const 1");
    println!("  v25: i32 = add v19 v24");
    println!("  v19: i32 = or v25 v25");           /* i = i + 1 */
    println!("  jmp"); ckreset();
    println!("  endloop"); ckreset();
    println!("  store v0 v14");                    /* buf@0 = newbuf */
    println!("  store v5 v41");                    /* cap@8 = newcap (R18: v41 = cap*2+8, cap==0-guarded) */
    println!("  v26: i32 = const 0");
    println!("  ret v26");
    println!("fn __vec_reserve");
    println!("  v0: i32 = param");                 /* dv */
    println!("  v1: i32 = const 4");
    println!("  v2: i32 = add v0 v1");
    println!("  v3: i32 = load v2");               /* len */
    println!("  v4: i32 = const 8");
    println!("  v5: i32 = add v0 v4");
    println!("  v6: i32 = load v5");               /* cap */
    println!("  v7: i32 = ge v3 v6");              /* len >= cap */
    println!("  brz v7"); ckreset();                          /* if len < cap, skip grow */
    println!("  v8: i32 = call __vec_grow v0");
    println!("  pool"); ckreset();
    println!("  v9: i32 = const 0");
    println!("  ret v9");
    /* STR-GROW: growable String buffer. Descriptor {buf@0,len@4,cap@8} (cap in BYTES). __str_grow doubles the
       buffer, LOOP-copies the live len bytes (no recursion -- D8 trap), updates buf@0 + cap@8. __str_reserve
       LOOP-doubles until len+add <= cap (so a bulk push_str of `add` bytes larger than one doubling still fits).
       The `format!`-accumulation dump (`s = format!("{}\n{}", s, ..)`) grows `s` past the old fixed 4096 buffer;
       without this it overflows into adjacent live heap (layout-dependent segfault). */
    println!("fn __str_grow");
    println!("  v0: i32 = param");                 /* desc */
    println!("  v1: i32 = const 4");
    println!("  v2: i32 = add v0 v1");
    println!("  v3: i32 = load v2");               /* len */
    println!("  v4: i32 = const 8");
    println!("  v5: i32 = add v0 v4");
    println!("  v6: i32 = load v5");               /* cap */
    println!("  v7: i32 = const 2");
    println!("  v27: i32 = mul v6 v7");            /* cap*2 */
    println!("  v28: i32 = const 4096");
    println!("  v8: i32 = add v27 v28");           /* newcap = cap*2 + 4096 (always > cap AND >= 4096: progress guaranteed even if cap==0, no infinite reserve loop) */
    println!("  v11: i32 = gref __heap_cursor");
    println!("  v12: i32 = load v11");
    println!("  v13: i32 = gref __heap");
    println!("  v14: i32 = add v13 v12");          /* newbuf = __heap + cur */
    println!("  v15: i32 = add v12 v8");
    println!("  store v11 v15");                   /* __heap_cursor += newcap */
    println!("  v16: i32 = load v0");              /* oldbuf */
    println!("  v19: i32 = const 0");              /* i */
    println!("  loop"); ckreset();
    println!("  v20: i32 = lt v19 v3");            /* i < len */
    println!("  brz v20"); ckreset();
    println!("  v21: i32 = add v16 v19");
    println!("  v22: i32 = loadb v21");
    println!("  v23: i32 = add v14 v19");
    println!("  storeb v23 v22");
    println!("  cont"); ckreset();
    println!("  v24: i32 = const 1");
    println!("  v25: i32 = add v19 v24");
    println!("  v19: i32 = or v25 v25");
    println!("  jmp"); ckreset();
    println!("  endloop"); ckreset();
    println!("  store v0 v14");                    /* buf@0 = newbuf */
    println!("  store v5 v8");                     /* cap@8 = newcap */
    println!("  v26: i32 = const 0");
    println!("  ret v26");
    println!("fn __str_reserve");
    println!("  v0: i32 = param");                 /* desc */
    println!("  v1: i32 = param");                 /* add */
    println!("  v2: i32 = const 4");
    println!("  v3: i32 = add v0 v2");
    println!("  v4: i32 = load v3");               /* len */
    println!("  v5: i32 = add v4 v1");             /* need = len + add */
    println!("  loop"); ckreset();
    println!("  v6: i32 = const 8");
    println!("  v7: i32 = add v0 v6");
    println!("  v8: i32 = load v7");               /* cap (reloaded each iter) */
    println!("  v9: i32 = gt v5 v8");              /* need > cap -> keep growing */
    println!("  brz v9"); ckreset();                          /* exit when need <= cap */
    println!("  v10: i32 = call __str_grow v0");
    println!("  cont"); ckreset();
    println!("  jmp"); ckreset();
    println!("  endloop"); ckreset();
    println!("  v11: i32 = const 0");
    println!("  ret v11");
    /* S2 format!: append into a String descriptor [buf@0,len@4] instead of stdout */
    println!("fn __push_byte");
    println!("  v0: i32 = param");
    println!("  v1: i32 = param");
    println!("  v3: i32 = const 4");
    println!("  v4: i32 = add v0 v3");
    println!("  v5: i32 = load v4");               /* len */
    println!("  v9: i32 = const 8");               /* STR-GROW: grow-on-full guard */
    println!("  v10: i32 = add v0 v9");
    println!("  v11: i32 = load v10");             /* cap */
    println!("  v12: i32 = ge v5 v11");            /* len >= cap */
    println!("  brz v12"); ckreset();
    println!("  v13: i32 = call __str_grow v0");
    println!("  pool"); ckreset();
    println!("  v2: i32 = load v0");               /* buf (reloaded after possible grow) */
    println!("  v6: i32 = add v2 v5");
    println!("  storeb v6 v1");
    println!("  v7: i32 = const 1");
    println!("  v8: i32 = add v5 v7");
    println!("  store v4 v8");
    println!("  v14: i32 = const 0");
    println!("  ret v14");
    println!("fn __push_uint");
    println!("  v0: i32 = param");
    println!("  v1: i32 = param");
    println!("  v2: i32 = const 0");
    println!("  v3: i32 = const 10");
    println!("  v4: i32 = ge v1 v3");
    println!("  brz v4"); ckreset();
    println!("  v5: i32 = const 10");
    println!("  v6: i32 = div v1 v5");
    println!("  v7: i32 = call __push_uint v0 v6");
    println!("  pool"); ckreset();
    println!("  v8: i32 = const 48");
    println!("  v9: i32 = const 10");
    println!("  v10: i32 = mod v1 v9");
    println!("  v11: i32 = add v8 v10");
    println!("  v12: i32 = call __push_byte v0 v11");
    println!("  ret v2");
    println!("fn __push_int");
    println!("  v0: i32 = param");
    println!("  v1: i32 = param");
    println!("  v2: i32 = const 0");
    println!("  v3: i32 = add v1 v2");
    println!("  v4: i32 = const 0");
    println!("  v5: i32 = lt v3 v4");
    println!("  brz v5"); ckreset();
    println!("  v6: i32 = const 45");
    println!("  v7: i32 = call __push_byte v0 v6");
    println!("  v8: i32 = ref v3");
    println!("  v9: i32 = const 0");
    println!("  v10: i32 = sub v9 v3");
    println!("  store v8 v10");
    println!("  pool"); ckreset();
    println!("  v11: i32 = call __push_uint v0 v3");
    println!("  v12: i32 = const 0");
    println!("  ret v12");
    /* S2 deferrals: __push_radix(desc,n,base,upper) for {:x}/{:X}/{:b}/{:o}; __push_int64 for i64 args */
    println!("fn __push_radix");
    println!("  v0: i32 = param");
    println!("  v1: i32 = param");
    println!("  v2: i32 = param");
    println!("  v3: i32 = param");
    println!("  v4: i32 = ge v1 v2");
    println!("  brz v4"); ckreset();
    println!("  v5: i32 = div v1 v2");
    println!("  v6: i32 = call __push_radix v0 v5 v2 v3");
    println!("  pool"); ckreset();
    println!("  v7: i32 = mod v1 v2");
    println!("  v8: i32 = const 10");
    println!("  v9: i32 = ge v7 v8");
    println!("  v10: i32 = const 32");
    println!("  v11: i32 = mul v3 v10");
    println!("  v12: i32 = const 87");
    println!("  v13: i32 = sub v12 v11");
    println!("  v14: i32 = const 48");
    println!("  v15: i32 = sub v13 v14");
    println!("  v16: i32 = mul v9 v15");
    println!("  v17: i32 = add v14 v16");
    println!("  v18: i32 = add v17 v7");
    println!("  v19: i32 = call __push_byte v0 v18");
    println!("  v20: i32 = const 0");
    println!("  ret v20");
    println!("fn __push_uint64");
    println!("  v0: i32 = param");
    println!("  v1: i32 = param");
    println!("  v2: i32 = const 10");
    println!("  v3: i64 = conv v2");
    println!("  v4: i64 = ge8 v1 v3");
    println!("  brz v4"); ckreset();
    println!("  v5: i32 = const 10");
    println!("  v6: i64 = conv v5");
    println!("  v7: i64 = div8 v1 v6");
    println!("  v8: i32 = call __push_uint64 v0 v7");
    println!("  pool"); ckreset();
    println!("  v9: i32 = const 10");
    println!("  v10: i64 = conv v9");
    println!("  v11: i64 = mod8 v1 v10");
    println!("  v12: i32 = const 48");
    println!("  v13: i64 = conv v12");
    println!("  v14: i64 = add8 v13 v11");
    println!("  v15: i32 = call __push_byte v0 v14");
    println!("  v16: i32 = const 0");
    println!("  ret v16");
    println!("fn __push_int64");
    println!("  v0: i32 = param");
    println!("  v1: i32 = param");
    println!("  v2: i64 = or8 v1 v1");
    println!("  v3: i32 = const 0");
    println!("  v4: i64 = conv v3");
    println!("  v5: i64 = lt8 v2 v4");
    println!("  brz v5"); ckreset();
    println!("  v6: i32 = const 45");
    println!("  v7: i32 = call __push_byte v0 v6");
    println!("  v8: i32 = ref v2");
    println!("  v9: i32 = const 0");
    println!("  v10: i64 = conv v9");
    println!("  v11: i64 = sub8 v10 v2");
    println!("  store8 v8 v11");
    println!("  pool"); ckreset();
    println!("  v12: i32 = call __push_uint64 v0 v2");
    println!("  v13: i32 = const 0");
    println!("  ret v13");
    /* R17: __print_uint64 / __print_int64 (i64 decimal via div8/mod8) */
    /* value-width: UNSIGNED 64-bit decimal. Signed div8/mod8 are wrong for values with the high bit set
       (u64::MAX = -1 signed). Use q = (n >>u 1) / 5 == floor(n/10) EXACTLY (n=10k+r: (n>>1)/5 = k), via
       the logical-shift op shu8; digit = n - q*10; recurse while q>0. Correct for all u64. */
    println!("fn __print_uint64");
    println!("  v0: i32 = param");
    println!("  v1: i32 = const 1");
    println!("  v2: i64 = conv v1");
    println!("  v3: i64 = shu8 v0 v2");
    println!("  v4: i32 = const 5");
    println!("  v5: i64 = conv v4");
    println!("  v6: i64 = div8 v3 v5");
    println!("  v7: i32 = const 0");
    println!("  v8: i64 = conv v7");
    println!("  v9: i64 = gt8 v6 v8");
    println!("  brz v9"); ckreset();
    println!("  v10: i32 = call __print_uint64 v6");
    println!("  pool"); ckreset();
    println!("  v11: i32 = const 10");
    println!("  v12: i64 = conv v11");
    println!("  v13: i64 = mul8 v6 v12");
    println!("  v14: i64 = sub8 v0 v13");
    println!("  v15: i32 = const 48");
    println!("  v16: i64 = conv v15");
    println!("  v17: i64 = add8 v16 v14");
    println!("  v18: i32 = call __putc v17");
    println!("  v19: i32 = const 0");
    println!("  ret v19");
    println!("fn __print_int64");
    println!("  v0: i32 = param");
    println!("  v1: i32 = const 0");
    println!("  v2: i64 = or8 v0 v0");
    println!("  v3: i32 = const 0");
    println!("  v4: i64 = conv v3");
    println!("  v5: i64 = lt8 v2 v4");
    println!("  brz v5"); ckreset();
    println!("  v6: i32 = const 45");
    println!("  v7: i32 = call __putc v6");
    println!("  v8: i32 = ref v2");
    println!("  v9: i32 = const 0");
    println!("  v10: i64 = conv v9");
    println!("  v11: i64 = sub8 v10 v2");
    println!("  store8 v8 v11");
    println!("  pool"); ckreset();
    println!("  v12: i32 = call __print_uint64 v2");
    println!("  ret v1");
}

/* a string literal spanning SRC[o..o+l] (raw content, no escapes needed for self-hosting) -> bytes +
   a {ptr@0, len@4} descriptor in the __str_arena; returns the descriptor vN, sets EISSTR. */
fn emit_str_literal(o: i32, l: i32) -> i32 {
    unsafe {
        let cur = eload(egref_str_cursor());
        let base = ebin("add", egref_str_arena(), cur);
        let mut i = 0;
        let mut n = 0;   /* LAYER4 P-STR-ESCAPE: the OUTPUT byte count (processed), <= the source span `l` */
        while i < l {
            let mut c = at(o + i);
            if c == 92 && i + 1 < l {   /* backslash escape: emit the DECODED byte, not the raw `\x` pair (was stored verbatim -> `\"`/`\n`/... counted as 2 bytes; rustc_lexer differential over-read strings by the escape count) */
                i = i + 1; let e = at(o + i);
                if e == 110 { c = 10; }        /* \n */
                else if e == 116 { c = 9; }    /* \t */
                else if e == 114 { c = 13; }   /* \r */
                else if e == 48 { c = 0; }     /* \0 */
                else { c = e; }                /* \" \\ \' and the rest -> the escaped char itself */
            }
            let ad = ebin("add", base, ec(n));
            println!("  storeb v{} v{}", ad, ec(c));
            n = n + 1; i = i + 1;
        }
        let doff = ((n + 3) / 4) * 4;
        let dc = ec(doff);
        let desc = ebin("add", base, dc);
        println!("  store v{} v{}", desc, base);
        let c4 = ecr(4);
        let da = ebin("add", desc, c4);
        let lv = ec(n);
        println!("  store v{} v{}", da, lv);
        let nc = ec(doff + 8);
        let newcur = ebin("add", cur, nc);
        let gc = egref_str_cursor();
        println!("  store v{} v{}", gc, newcur);
        EISSTR = 1;
        desc
    }
}
fn egref_str_arena() -> i32 { unsafe { println!("  v{}: i32 = gref __str_arena", NC); NC = NC + 1; NC - 1 } }
fn egref_str_cursor() -> i32 { unsafe { println!("  v{}: i32 = gref __str_cursor", NC); NC = NC + 1; NC - 1 } }

fn at(i: i32) -> i32 { unsafe { SRC[i as usize] as i32 } }
fn is_sp(c: i32) -> bool { c == 32 || c == 9 || c == 10 || c == 13 }
fn is_dig(c: i32) -> bool { c >= 48 && c <= 57 }
fn is_idst(c: i32) -> bool { (c >= 97 && c <= 122) || (c >= 65 && c <= 90) || c == 95 }
fn is_idc(c: i32) -> bool { is_idst(c) || is_dig(c) }

fn skipws() {
    unsafe {
        let mut go = true;
        while go {
            go = false;
            while POS < LEN && is_sp(at(POS)) { POS = POS + 1; }
            if POS + 1 < LEN && at(POS) == 47 && at(POS + 1) == 47 {
                while POS < LEN && at(POS) != 10 { POS = POS + 1; }
                go = true;
            }
            if POS + 1 < LEN && at(POS) == 47 && at(POS + 1) == 42 {
                POS = POS + 2;
                while POS + 1 < LEN && !(at(POS) == 42 && at(POS + 1) == 47) { POS = POS + 1; }
                POS = POS + 2;
                go = true;
            }
        }
    }
}

/* skip ws+comments then ONE token (string/char literal, whole ident, or punct) -- top-level scans use
   this so `fn`/`static` INSIDE a string or comment (e.g. println!("fn __putc")) aren't matched. */
fn skip_tok() {
    unsafe {
        skipws();
        if POS >= LEN { return; }
        let c = at(POS);
        if c == 34 {
            POS = POS + 1;
            while POS < LEN && at(POS) != 34 { if at(POS) == 92 { POS = POS + 1; } POS = POS + 1; }
            if POS < LEN { POS = POS + 1; }
            return;
        }
        if c == 39 {
            POS = POS + 1;
            if POS < LEN && at(POS) == 92 { POS = POS + 1; }
            if POS < LEN { POS = POS + 1; }
            if POS < LEN && at(POS) == 39 { POS = POS + 1; }
            return;
        }
        if is_idst(c) { let io = POS; while POS < LEN && is_idc(at(POS)) { POS = POS + 1; }
            if POS - io == 11 && at(io) == 109 && at(io + 1) == 97 && at(io + 2) == 99 && at(io + 3) == 114 && at(io + 4) == 111 && at(io + 5) == 95 && at(io + 6) == 114 && at(io + 7) == 117 && at(io + 8) == 108 && at(io + 9) == 101 && at(io + 10) == 115 {
                /* RUSTC-FE FC: skip a whole `macro_rules! name { .. }` as a balanced token tree, so the item-prepasses do NOT step into the macro body and mis-parse an inner keyword token (`struct`/`fn`/`enum`/..) as a real item (was an infinite HANG: e.g. `macro_rules! m { (struct) => {}; }` -> prepass_structs parsed the matcher's `struct`). macro_rules is unambiguously an item, so skipping it wholesale here is safe. */
                skipws(); if POS < LEN && at(POS) == 33 { POS = POS + 1; }                 /* ! */
                skipws(); while POS < LEN && is_idc(at(POS)) { POS = POS + 1; }             /* the macro name */
                skipws();
                if POS < LEN && (at(POS) == 123 || at(POS) == 40 || at(POS) == 91) {        /* the balanced body { } / ( ) / [ ] */
                    let mut d = 0;
                    loop {
                        if POS >= LEN { break; }
                        let ch = at(POS);
                        if ch == 34 { POS = POS + 1; while POS < LEN && at(POS) != 34 { if at(POS) == 92 { POS = POS + 1; } POS = POS + 1; } if POS < LEN { POS = POS + 1; } }   /* skip a string literal (its delimiters do not count) */
                        else if ch == 39 { POS = POS + 1; if POS < LEN && at(POS) == 92 { POS = POS + 1; } if POS < LEN { POS = POS + 1; } if POS < LEN && at(POS) == 39 { POS = POS + 1; } }   /* skip a char literal / lifetime */
                        else { if ch == 123 || ch == 40 || ch == 91 { d = d + 1; } if ch == 125 || ch == 41 || ch == 93 { d = d - 1; } POS = POS + 1; if d == 0 { break; } }
                    }
                }
            }
            return; }
        POS = POS + 1;
    }
}
fn peek() -> i32 { unsafe { skipws(); if POS < LEN { at(POS) } else { -1 } } }
fn eat_ch(c: i32) -> bool { unsafe { skipws(); if POS < LEN && at(POS) == c { POS = POS + 1; true } else { false } } }
/* eat a two-char operator like ==, <=, &&; returns true if matched */
fn eat2(a: i32, b: i32) -> bool { unsafe { skipws(); if POS + 1 < LEN && at(POS) == a && at(POS + 1) == b { POS = POS + 2; true } else { false } } }

fn rd_ident() -> bool {
    unsafe {
        skipws();
        if POS >= LEN { return false; }
        if at(POS) == 36 && CURNP > 0 {   /* R-either: macro metavar `$name` in a NON-expression position (a pattern `Some($p)`, a variant name) -> substitute the bound arg's ident span. Expression-position `$name` is intercepted earlier by parse_atom (splice-parse), so this only fires where rd_ident reads the metavar directly. */
            let dp = POS; POS = POS + 1;
            if POS < LEN && is_idst(at(POS)) {
                let no = POS; while POS < LEN && is_idc(at(POS)) { POS = POS + 1; }
                let nl = POS - no;
                let mut k = 0;
                while k < CURNP { if span_eq_at(CURPO[k as usize], CURPL[k as usize], no, nl) { ID_O = CURAO[k as usize]; ID_L = CURAL[k as usize]; return true; } k = k + 1; }
            }
            POS = dp;   /* not a known metavar -> fall through (read `$` literally, harmless) */
        }
        if !is_idst(at(POS)) { return false; }
        ID_O = POS;
        while POS < LEN && is_idc(at(POS)) { POS = POS + 1; }
        ID_L = POS - ID_O;
        true
    }
}

/* does the just-read ident [ID_O,ID_L] equal a keyword given by span [ko,kl] in SRC? we instead pass
   a small keyword code; span compare against literals is done with byte matches. */
fn id_is(a: i32, b: i32, c: i32, d: i32, e: i32, n: i32) -> bool {
    /* compare [ID_O,ID_L] to up to 5 byte codes a..e of length n */
    unsafe {
        if ID_L != n { return false; }
        if n >= 1 && at(ID_O) != a { return false; }
        if n >= 2 && at(ID_O + 1) != b { return false; }
        if n >= 3 && at(ID_O + 2) != c { return false; }
        if n >= 4 && at(ID_O + 3) != d { return false; }
        if n >= 5 && at(ID_O + 4) != e { return false; }
        true
    }
}
fn is_kw_let() -> bool { id_is(108, 101, 116, 0, 0, 3) }       /* let */
fn is_kw_if() -> bool { id_is(105, 102, 0, 0, 0, 2) }          /* if */
fn is_kw_else() -> bool { id_is(101, 108, 115, 101, 0, 4) }    /* else */
fn is_kw_while() -> bool { id_is(119, 104, 105, 108, 101, 5) } /* while */
fn is_kw_return() -> bool { id_is(114, 101, 116, 117, 114, 6) }/* retur(n) -- first 5 + len 6 */
fn is_kw_break() -> bool { id_is(98, 114, 101, 97, 107, 5) }   /* break */
fn is_kw_mut() -> bool { id_is(109, 117, 116, 0, 0, 3) }       /* mut */
fn is_kw_fn() -> bool { id_is(102, 110, 0, 0, 0, 2) }          /* fn */
fn is_kw_true() -> bool { id_is(116, 114, 117, 101, 0, 4) }    /* true */
fn is_kw_false() -> bool { id_is(102, 97, 108, 115, 101, 5) }  /* false */
fn is_kw_loop() -> bool { id_is(108, 111, 111, 112, 0, 4) }   /* loop */
fn is_kw_static() -> bool { id_is(115, 116, 97, 116, 105, 6) } /* stati(c) len 6 */
fn is_kw_const() -> bool { id_is(99, 111, 110, 115, 116, 5) }  /* const */
fn is_kw_trait() -> bool { id_is(116, 114, 97, 105, 116, 5) }  /* trait */
fn is_kw_struct() -> bool { id_is(115, 116, 114, 117, 99, 6) } /* struc(t) len 6 */
fn is_kw_impl() -> bool { id_is(105, 109, 112, 108, 0, 4) }    /* impl */
fn is_kw_self() -> bool { id_is(115, 101, 108, 102, 0, 4) }    /* self */
fn is_kw_enum() -> bool { id_is(101, 110, 117, 109, 0, 4) }    /* enum */
fn is_kw_type() -> bool { id_is(116, 121, 112, 101, 0, 4) }    /* type */
fn is_box_ident(o: i32, l: i32) -> bool { unsafe { (l == 3 && at(o) == 66 && at(o + 1) == 111 && at(o + 2) == 120) || (TBALL > 0 && span_eq_at(TBALO, TBALL, o, l)) } }   /* LAYER7 P-BOX-ALIAS: literal `Box` OR a recorded `type NAME<T>=Box<T>` alias name */
fn prepass_aliases() { unsafe {   /* LAYER7 P-BOX-ALIAS: record a `type NAME<T> = Box<T>` alias (rustc's P<T>) so the transparent-pointer see-through sites treat NAME like Box */
    POS = 0;
    loop {
        skipws();
        if POS >= LEN { break; }
        let save = POS;
        if rd_ident() && is_kw_type() {
            rd_ident(); let no = ID_O; let nl = ID_L;
            skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }   /* skip `<T,..>` generic header */
            skipws();
            if POS < LEN && at(POS) == 61 { POS = POS + 1; skipws();
                if POS + 2 < LEN && at(POS) == 66 && at(POS + 1) == 111 && at(POS + 2) == 120 { TBALO = no; TBALL = nl; }   /* `= Box..` -> NAME is a Box alias */
                if POS + 2 < LEN && at(POS) == 86 && at(POS + 1) == 101 && at(POS + 2) == 99 { TVALO = no; TVALL = nl; }   /* LAYER7 P-VEC-ALIAS: `= Vec..` -> NAME is a Vec alias (ThinVec) */
            }
        } else { POS = save; skip_tok(); }
    }
} }
fn is_kw_match() -> bool { id_is(109, 97, 116, 99, 104, 5) }   /* match */
fn is_kw_for() -> bool { id_is(102, 111, 114, 0, 0, 3) }       /* for */
fn is_kw_continue() -> bool { id_is(99, 111, 110, 116, 105, 8) }  /* continu(e) len 8 */

fn enum_find(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NEN { if span_eq_at(ENO[i as usize], ENL[i as usize], o, l) { return i; } i = i + 1; } -1 }
}
fn var_find(o: i32, l: i32) -> i32 {   /* -> variant index (searches all enums), or -1 */
    unsafe { let mut i = 0; while i < NVAR { if span_eq_at(VO[i as usize], VL[i as usize], o, l) { return i; } i = i + 1; } -1 }
}
fn var_find_in(en: i32, o: i32, l: i32) -> i32 {   /* LAYER6 P-QUALIFIED-VARIANT-RESOLUTION: variant index WITHIN enum `en` (or -1). A qualified `E::V` pattern must resolve V among E's variants, not by the global first-name-match var_find -- else `Expr::Num` found `Tok::Num` (an earlier enum sharing the name), setting menum=Tok so a 4-arm Expr match read as non-exhaustive against Tok's 11 variants. */
    unsafe { let mut i = 0; while i < NVAR { if VEN[i as usize] == en && span_eq_at(VO[i as usize], VL[i as usize], o, l) { return i; } i = i + 1; } -1 }
}
fn is_std(o: i32, l: i32) -> bool { unsafe { l == 3 && at(o) == 115 && at(o + 1) == 116 && at(o + 2) == 100 } }   /* std */

/* R5: built-in Option/Result variants (bare Some/None/Ok/Err). UNIFIED convention: success variant disc=1
   (Some/Ok), failure disc=0 (None/Err), so ? / unwrap / is_* behave identically for both. cell = [disc,payload]. */
fn nm_some(o: i32, l: i32) -> bool { unsafe { l == 4 && at(o) == 83 && at(o + 1) == 111 && at(o + 2) == 109 && at(o + 3) == 101 } }
fn nm_none(o: i32, l: i32) -> bool { unsafe { l == 4 && at(o) == 78 && at(o + 1) == 111 && at(o + 2) == 110 && at(o + 3) == 101 } }
fn nm_ok(o: i32, l: i32) -> bool { unsafe { l == 2 && at(o) == 79 && at(o + 1) == 107 } }
fn nm_err(o: i32, l: i32) -> bool { unsafe { l == 3 && at(o) == 69 && at(o + 1) == 114 && at(o + 2) == 114 } }
fn opt_disc(o: i32, l: i32) -> i32 {   /* built-in variant discriminant, or -1 if not built-in */
    unsafe { if nm_some(o, l) || nm_ok(o, l) { 1 } else if nm_none(o, l) || nm_err(o, l) { 0 } else { -1 } }
}
fn opt_has(o: i32, l: i32) -> bool { unsafe { nm_some(o, l) || nm_ok(o, l) || nm_err(o, l) } }   /* None carries no payload */

/* TI3e-g: std::cmp::Ordering. `.cmp()` returns a BARE i32 (-1/0/1, not a heap enum), so an
   `Ordering` match must compare the scrutinee directly (no eload) and map the variants to -1/0/1. */
fn is_ordering(o: i32, l: i32) -> bool { unsafe { l == 8 && at(o) == 79 && at(o + 1) == 114 && at(o + 2) == 100 && at(o + 3) == 101 && at(o + 4) == 114 && at(o + 5) == 105 && at(o + 6) == 110 && at(o + 7) == 103 } }
fn ord_disc(o: i32, l: i32) -> i32 {   /* Less -> -1, Equal -> 0, Greater -> 1, else -99 */
    unsafe {
        if l == 4 && at(o) == 76 && at(o + 1) == 101 && at(o + 2) == 115 && at(o + 3) == 115 { return 0 - 1; }   /* Less */
        if l == 5 && at(o) == 69 && at(o + 1) == 113 && at(o + 2) == 117 && at(o + 3) == 97 && at(o + 4) == 108 { return 0; }   /* Equal */
        if l == 7 && at(o) == 71 && at(o + 1) == 114 && at(o + 2) == 101 && at(o + 3) == 97 && at(o + 4) == 116 && at(o + 5) == 101 && at(o + 6) == 114 { return 1; }   /* Greater */
        0 - 99
    }
}

/* R1: register every `struct Name { f: T, ... }` (fields, offsets, size). Every field is 4 bytes
   (a scalar value or a pointer to a heap struct). */
fn prepass_structs() {
    unsafe {
        POS = 0;
        let mut pendingcopy = 0; let mut pendingpeq = 0;                           /* P2/R2: #[derive(..Copy..)] / #[derive(..PartialEq..)] seen just before the next struct */
        loop {
            skipws();
            if POS >= LEN { break; }
            let save = POS;
            if POS < LEN && at(POS) == 35 {                                        /* #[ .. ] attribute -- scan for Copy, then carry to the next struct */
                POS = POS + 1; if POS < LEN && at(POS) == 33 { POS = POS + 1; }     /* #! inner */
                if POS < LEN && at(POS) == 91 {
                    let ao = POS + 1; let mut d = 1; POS = POS + 1;
                    while POS < LEN && d > 0 { if at(POS) == 91 { d = d + 1; } if at(POS) == 93 { d = d - 1; } POS = POS + 1; }
                    let ae = POS - 1;
                    let mut j = ao;
                    while j + 3 < ae { if at(j) == 67 && at(j + 1) == 111 && at(j + 2) == 112 && at(j + 3) == 121 { pendingcopy = 1; } j = j + 1; }   /* "Copy" */
                    let mut jp = ao; while jp + 8 < ae { if at(jp) == 80 && at(jp + 1) == 97 && at(jp + 2) == 114 && at(jp + 3) == 116 && at(jp + 4) == 105 && at(jp + 5) == 97 && at(jp + 6) == 108 && at(jp + 7) == 69 && at(jp + 8) == 113 { pendingpeq = 1; } jp = jp + 1; }   /* R2: "PartialEq" */
                }
                continue;
            }
            if rd_ident() && is_kw_struct() {
                rd_ident();
                let no = ID_O; let nl = ID_L;
                if nl == 4 && at(no) == 82 && at(no + 1) == 86 && at(no + 2) == 101 && at(no + 3) == 99 { HAS_RVEC = 1; }   /* std-hoist P4.1: program defines `struct RVec` -> route built-in Vec ops to RVec_* */
                if nl == 7 && at(no) == 82 && at(no + 1) == 83 && at(no + 2) == 116 && at(no + 3) == 114 && at(no + 4) == 105 && at(no + 5) == 110 && at(no + 6) == 103 { HAS_RSTRING = 1; }   /* std-hoist P4.3: program defines `struct RString` -> route built-in String ops to RString_* */
                let mut tgpo = 0; let mut tgpl = 0;                                       /* std-hoist P0.1: capture the generic type-param name (first non-lifetime ident inside `<>`) */
                let mut tgpo2 = 0; let mut tgpl2 = 0;                                     /* std-hoist GAP2: second generic-param name */
                let mut pgpfn = 0;
                skipws(); if POS < LEN && at(POS) == 60 {
                    let gp = POS + 1; POS = gp; skipws();
                    while POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; skipws(); } }   /* skip leading lifetimes */
                    if POS < LEN && is_idst(at(POS)) { let sp = POS; if rd_ident() { tgpo = ID_O; tgpl = ID_L; } POS = sp;
                        rd_ident(); skipws();                                            /* consume param#1 name + any `: Bound` up to the top-level comma */
                        let mut dd = 0;
                        while POS < LEN { let ch = at(POS); if ch == 70 && POS + 2 < LEN && at(POS + 1) == 110 && at(POS + 2) == 40 { pgpfn = 1; } if ch == 60 || ch == 40 { dd = dd + 1; POS = POS + 1; } else if ch == 62 || ch == 41 { if dd == 0 { break; } dd = dd - 1; POS = POS + 1; } else if dd == 0 && ch == 44 { break; } else { POS = POS + 1; } }   /* R10: `Fn(` in param#1's bound -> closure-bounded param */
                        if POS < LEN && at(POS) == 44 { POS = POS + 1; skipws();          /* std-hoist GAP2: second type-param */
                            while POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; skipws(); } }
                            if POS < LEN && is_idst(at(POS)) { let sp2 = POS; if rd_ident() { tgpo2 = ID_O; tgpl2 = ID_L; } POS = sp2; } } }
                    POS = gp; let mut d = 1; while POS < LEN && d > 0 { if POS + 1 < LEN && at(POS) == 45 && at(POS + 1) == 62 { POS = POS + 2; } else { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }   /* R2: `->`-aware generic-param skip so `struct H<F: Fn()->T>` doesn't miscount (was a HANG) */
                let ty = NTYP;
                TSO[ty as usize] = no; TSL[ty as usize] = nl; TGPO[ty as usize] = tgpo; TGPL[ty as usize] = tgpl; TGPO2[ty as usize] = tgpo2; TGPL2[ty as usize] = tgpl2; TGPFN[ty as usize] = pgpfn; TCOPY[ty as usize] = pendingcopy; pendingcopy = 0; TPEQ[ty as usize] = pendingpeq; pendingpeq = 0; NTYP = NTYP + 1;
                skipws();
                if POS < LEN && at(POS) == 59 { POS = POS + 1; TSZ[ty as usize] = 0; }   /* unit struct `struct S;` -> zero fields; DON'T enter the field loop (it would over-scan into a following item, hanging codegen). Behaves like `struct S{}`. */
                else if POS < LEN && at(POS) == 40 {                                     /* A: tuple struct `struct S(T0, T1, ..)` -- positional fields at offsets 0,4,8,.. (accessed via .N) */
                    POS = POS + 1; let mut off = 0;
                    loop {
                        skipws();
                        if POS >= LEN { break; }
                        if at(POS) == 41 { POS = POS + 1; break; }
                        if at(POS) == 38 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); } }
                        let fo = POS; rd_ident();
                        let fsty = ty_find(ID_O, ID_L);
                        let mut fgen = 0;
                        if TGPL[ty as usize] > 0 && span_eq_at(TGPO[ty as usize], TGPL[ty as usize], ID_O, ID_L) { fgen = 1; }   /* std-hoist P0.1: tuple field's type is the generic param T */
                        if TGPL2[ty as usize] > 0 && span_eq_at(TGPO2[ty as usize], TGPL2[ty as usize], ID_O, ID_L) { fgen = 2; }   /* std-hoist GAP2: tuple field's type is the 2nd generic param V */
                        let mut ffk = 0;
                        if is_option_ty(ID_O, ID_L) { ffk = 8; }   /* P-STRUCT-FIELD-OPTION: tuple-struct Option<T> field (`S(Option<T>)`) */
                        if is_vec_ty(ID_O, ID_L) { ffk = 2; }
                        if is_string_ty(ID_O, ID_L) { ffk = 3; }
                        if ID_L == 5 && at(ID_O) == 67 && at(ID_O + 1) == 104 && at(ID_O + 2) == 97 && at(ID_O + 3) == 114 && at(ID_O + 4) == 115 { ffk = 13; }   /* LAYER4 P-CHARS-ITER-CURSOR-FIELD: a `Chars` field is a char-cursor cell (FFK=13, mirrors SK=13) */
                        if ID_L == 3 && at(ID_O) == 115 && at(ID_O + 1) == 116 && at(ID_O + 2) == 114 { ffk = 1; }
                        let mut d = 0;
                        loop {
                            if POS >= LEN { break; }
                            let ch = at(POS);
                            if ch == 60 || ch == 91 || ch == 40 { d = d + 1; POS = POS + 1; }
                            else if ch == 62 || ch == 93 || ch == 41 { if d == 0 { break; } d = d - 1; POS = POS + 1; }
                            else if d == 0 && ch == 44 { break; }
                            else { POS = POS + 1; }
                        }
                        FSST[NFLD as usize] = ty; FSO[NFLD as usize] = fo; FSL[NFLD as usize] = 0; FSB[NFLD as usize] = off; FSTY[NFLD as usize] = fsty; FFK[NFLD as usize] = ffk; FGEN[NFLD as usize] = fgen; NFLD = NFLD + 1;
                        off = off + 4;
                        skipws();
                        if POS < LEN && at(POS) == 44 { POS = POS + 1; }
                    }
                    TSZ[ty as usize] = off;
                    skipws(); if POS < LEN && at(POS) == 59 { POS = POS + 1; }
                }
                else {
                eat_ch(123);
                let mut off = 0;
                loop {
                    skipws();
                    if POS >= LEN { break; }
                    if at(POS) == 125 { POS = POS + 1; break; }
                    if POS + 2 < LEN && at(POS) == 112 && at(POS + 1) == 117 && at(POS + 2) == 98 && (POS + 3 >= LEN || !is_idc(at(POS + 3))) { POS = POS + 3; skipws(); if POS < LEN && at(POS) == 40 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } POS = POS + 1; } skipws(); } }   /* LAYER4: skip `pub` / `pub(crate)` field visibility (`pub v: T`) */
                    rd_ident();
                    let fo = ID_O; let fl = ID_L;
                    eat_ch(58); skipws();
                    let mut pref = 0; let mut arru8 = 0;   /* LAYER5 P-SLICE-FIELD: `pref` = the field type began with `&` (a borrowed slice `&[T]`, NOT an owned `[T;N]` array) */
                    if POS < LEN && at(POS) == 38 { pref = 1; POS = POS + 1; skipws(); if POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); skipws(); } }
                    let mut arrfk = 0; let mut arr64f = 0; let mut felo = 0; let mut fell = 0;   /* LAYER6 P-SLICE-ELEM-ENUM: element-type name span for a `&[Enum]`/`Vec<Enum>` field */
                    if POS < LEN && at(POS) == 91 {                                     /* aggregate element-address: `[T; N]` array field -> a 4-byte pointer slot to a heap array; `.d[i]` indexes it */
                        arrfk = 1; let mut aps = POS + 1; while aps < LEN && at(aps) == 32 { aps = aps + 1; }
                        if aps + 2 < LEN && (at(aps) == 105 || at(aps) == 117) && at(aps + 1) == 54 && at(aps + 2) == 52 { arr64f = 1; }   /* [i64;N]/[u64;N] element */
                        if aps + 1 < LEN && at(aps) == 117 && at(aps + 1) == 56 && (aps + 2 >= LEN || is_idc(at(aps + 2)) == false) { arru8 = 1; }   /* [u8] element -> byte slice */
                        { let mut ae = aps; while ae < LEN && is_idc(at(ae)) { ae = ae + 1; } felo = aps; fell = ae - aps; }   /* LAYER6 P-SLICE-ELEM-ENUM: `&[T]` slice-field element type name span */
                    }
                    rd_ident();
                    while POS < LEN && at(POS) == 58 && POS + 1 < LEN && at(POS + 1) == 58 { POS = POS + 2; skipws(); rd_ident(); }   /* LAYER6 P-QUALIFIED-FIELD-TYPE: a qualified field type `std::str::Chars<'a>` / `std::vec::Vec<T>` -> resolve on the LAST path segment (rd_ident read only `std`), so the Chars(FFK=13)/Vec/String/&str kind checks below see the real type. Was: ID stayed `std` -> a `chars: std::str::Chars` field got FFK=0 (scalar) -> self.chars.next() mis-dispatched -> crash. */
                    let fsty = ty_find(ID_O, ID_L);
                    let fseno = ID_O; let fsenl = ID_L;   /* RES-3 P-STRUCT-ENUM-FIELD-READ: capture the field TYPE-NAME span (enum_find is deferred to the read -- prepass_structs precedes prepass_enums) before the type-tail scan below moves POS */
                    let mut fgen = 0;
                    if TGPL[ty as usize] > 0 && span_eq_at(TGPO[ty as usize], TGPL[ty as usize], ID_O, ID_L) { fgen = 1; }   /* std-hoist P0.1: field's type is the struct's generic param T */
                    if TGPL2[ty as usize] > 0 && span_eq_at(TGPO2[ty as usize], TGPL2[ty as usize], ID_O, ID_L) { fgen = 2; }   /* std-hoist GAP2: field's type is the 2nd generic param V */
                    let mut ffk = 0;
                    if fgen == 1 && TGPFN[ty as usize] == 1 { ffk = 5; fgen = 0; }   /* R10: field is the `Fn`-bounded generic param (`struct H<F: Fn(..)>{ f: F }`) -> closure field (fat-ptr cell); clear fgen so the FGEN type-arg read path doesn't intercept */
                    if is_option_ty(ID_O, ID_L) { ffk = 8; }   /* P-STRUCT-FIELD-OPTION: named Option<T> field */
                    if is_vec_ty(ID_O, ID_L) { ffk = 2; }
                    if ffk == 2 { let mut q = POS; while q < LEN && at(q) == 32 { q = q + 1; } if q < LEN && at(q) == 60 { q = q + 1; while q < LEN && at(q) == 32 { q = q + 1; } if q + 1 < LEN && at(q) == 117 && at(q + 1) == 56 { ffk = 6; } { let mut qe = q; while qe < LEN && is_idc(at(qe)) { qe = qe + 1; } felo = q; fell = qe - q; } } }   /* BVK N3: Vec<u8> field -> byte-vec kind (FFK=6, 1-byte dispatch_bytes -- as_bytes().to_vec() into a field is 1-byte packed, so the field must read 1-byte; a Vec<u8> LOCAL built 4-byte then moved into the field is the residual byte-vec-repr-unification deferral). LAYER6 P-SLICE-ELEM-ENUM: `Vec<Enum>` field element type name span */
                    if is_string_ty(ID_O, ID_L) { ffk = 3; }
                    if ID_L == 5 && at(ID_O) == 67 && at(ID_O + 1) == 104 && at(ID_O + 2) == 97 && at(ID_O + 3) == 114 && at(ID_O + 4) == 115 { ffk = 13; }   /* LAYER4 P-CHARS-ITER-CURSOR-FIELD: a `Chars` field is a char-cursor cell (FFK=13, mirrors SK=13) */
                    if (ID_L == 7 && at(ID_O) == 72 && at(ID_O + 1) == 97 && at(ID_O + 2) == 115 && at(ID_O + 3) == 104 && at(ID_O + 4) == 77 && at(ID_O + 5) == 97 && at(ID_O + 6) == 112) || (ID_L == 8 && at(ID_O) == 66 && at(ID_O + 1) == 84 && at(ID_O + 2) == 114 && at(ID_O + 3) == 101 && at(ID_O + 4) == 101 && at(ID_O + 5) == 77 && at(ID_O + 6) == 97 && at(ID_O + 7) == 112) { ffk = 7; }   /* LAYER7 P-HASHMAP-FIELD: a `HashMap`/`BTreeMap` field -> FFK=7 (mirrors sk==7 HashMap local) so `self.m.get/insert/contains_key/len` route to dispatch_hashmap. */
                    if ID_L == 3 && at(ID_O) == 115 && at(ID_O + 1) == 116 && at(ID_O + 2) == 114 { ffk = 1; }   /* P: &str field -> descriptor (prints as str, .len works) */
                    if arrfk == 1 { if pref == 1 { if arru8 == 1 { ffk = 6; } else { ffk = 2; } } else { ffk = 4; } }   /* LAYER5 P-SLICE-FIELD: a `&[T]` BORROWED-SLICE field is a [ptr,len] descriptor (route `.len()`/`[i]`/iter through dispatch_vec FFK=2, or dispatch_bytes FFK=6 for `&[u8]`), NOT an owned `[T;N]` array field (FFK=4, a heap-array pointer). Was: `&` consumed then `[` -> arrfk=1 -> ffk=4 -> `.len()` fell to `call len` (undefined). rustc parsers hold `&[Token]` cursors. */
                    /* R24: consume the REST of the field type up to a top-level , or } -- handles
                       [T; N] arrays, &[u8] slices, Vec<..>, (T,T) tuples, nested generics. Guarantees
                       progress so a bracketed/parenthesized field type can't spin the loop. */
                    let mut d = 0;
                    let mut fclo = 0;
                    loop {
                        if POS >= LEN { break; }
                        let ch = at(POS);
                        if ch == 70 && POS + 2 < LEN && at(POS + 1) == 110 && at(POS + 2) == 40 { fclo = 1; }   /* `Fn(` in the field type (`Box<dyn Fn(..)->..>`) -> closure field: holds a {code,env} fat-ptr cell */
                        if ch == 45 && POS + 1 < LEN && at(POS + 1) == 62 { POS = POS + 2; }   /* `->` (return arrow inside a `Box<dyn Fn(i32)->i32>` field type) is a unit -- its `>` is NOT a generic close */
                        else if ch == 60 || ch == 91 || ch == 40 { d = d + 1; POS = POS + 1; }
                        else if ch == 62 || ch == 93 || ch == 41 { if d == 0 { break; } d = d - 1; POS = POS + 1; }
                        else if d == 0 && (ch == 44 || ch == 125) { break; }
                        else { POS = POS + 1; }
                    }
                    if fclo == 1 { ffk = 5; }   /* R10: closure field -- `s.f` read yields the fat-ptr cell (EISCLO); `(s.f)(args)` calls through */
                    let fi64 = ID_L == 3 && at(ID_O + 1) == 54 && at(ID_O + 2) == 52 && (at(ID_O) == 105 || at(ID_O) == 117);   /* value-width: `i64`/`u64` field */
                    let fbool = ID_L == 4 && at(ID_O) == 98 && at(ID_O + 1) == 111 && at(ID_O + 2) == 111 && at(ID_O + 3) == 108;   /* LAYER5 P-STRUCT-BOOL-FIELD: `bool` field */
                    FSST[NFLD as usize] = ty; FSO[NFLD as usize] = fo; FSL[NFLD as usize] = fl; FSB[NFLD as usize] = off; FSTY[NFLD as usize] = fsty; FSEN[NFLD as usize] = fseno; FSENL[NFLD as usize] = fsenl; FFK[NFLD as usize] = ffk; FGEN[NFLD as usize] = fgen; if fi64 { FI64[NFLD as usize] = 1; } else { FI64[NFLD as usize] = 0; } if fbool { FBOOL[NFLD as usize] = 1; } else { FBOOL[NFLD as usize] = 0; } FVELO[NFLD as usize] = felo; FVELL[NFLD as usize] = fell; FARR64[NFLD as usize] = arr64f; NFLD = NFLD + 1;
                    if fi64 { off = off + 8; } else { off = off + 4; }   /* value-width: i64/u64 field takes an 8-byte slot (array field = a 4-byte pointer slot) */
                    skipws();
                    if POS < LEN && at(POS) == 44 { POS = POS + 1; }
                }
                TSZ[ty as usize] = off;
                }
            } else { POS = save; skip_tok(); pendingcopy = 0; }                    /* a non-struct item ends any pending derive(Copy) */
        }
    }
}
/* heap bump-alloc `n` bytes -> pointer vN (structs live on the heap so a returned struct survives) */
fn halloc(n: i32) -> i32 {
    unsafe {
        let cur = eload(egref_heap_cursor());
        let base = ebin("add", egref_heap(), cur);
        let cn = ec(n);
        let nc = ebin("add", cur, cn);
        let gc = egref_heap_cursor();
        println!("  store v{} v{}", gc, nc);
        base
    }
}
fn egref_heap() -> i32 { unsafe { println!("  v{}: i32 = gref __heap", NC); NC = NC + 1; NC - 1 } }
fn egref_heap_cursor() -> i32 { unsafe { println!("  v{}: i32 = gref __heap_cursor", NC); NC = NC + 1; NC - 1 } }
fn is_kw_unsafe() -> bool { id_is(117, 110, 115, 97, 102, 6) }/* unsaf(e) len 6 */

fn g_find(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NGLOB { if span_eq_at(GO[i as usize], GL[i as usize], o, l) { return i; } i = i + 1; } -1 }
}
fn emit_span(o: i32, l: i32) { unsafe { __flush(); __syscall(1, 1, &SRC[o as usize], l); } }   /* R3: flush buffered output before this direct fd-1 write (ordering when the front-end itself is buffered) */
fn emit_flush_call() { unsafe { println!("  v{}: i32 = call __flush", NC); NC = NC + 1; } }   /* R3: emit `call __flush` into the compiled program before a ret->exit / process::exit / panic */
fn egref(o: i32, l: i32) -> i32 { unsafe { print!("  v{}: i32 = gref ", NC); emit_span(o, l); println!(""); NC = NC + 1; NC - 1 } }
fn eload(a: i32) -> i32 { unsafe { println!("  v{}: i32 = load v{}", NC, a); NC = NC + 1; NC - 1 } }
fn eload8(a: i32) -> i32 { unsafe { println!("  v{}: i64 = load8 v{}", NC, a); NC = NC + 1; NC - 1 } }   /* value-width: 64-bit load (i64/u64 field) */
fn eloadb(a: i32) -> i32 { unsafe { println!("  v{}: i32 = loadb v{}", NC, a); NC = NC + 1; NC - 1 } }

fn sym_add(o: i32, l: i32, v: i32) { unsafe { if NSYM >= 8192 { die_internal_limit(); } SO[NSYM as usize] = o; SL[NSYM as usize] = l; SV[NSYM as usize] = v; SK[NSYM as usize] = 0; SST[NSYM as usize] = -1; SI64[NSYM as usize] = 0; SMOVED[NSYM as usize] = 0; SFORREF[NSYM as usize] = 0; SREF1[NSYM as usize] = 0; SELEMREF[NSYM as usize] = 0; SBYREF[NSYM as usize] = 0; SBOOL[NSYM as usize] = 0; SENUM[NSYM as usize] = 0; SMBORO[NSYM as usize] = -1; SMBORL[NSYM as usize] = 0; SVELT[NSYM as usize] = -1; SVELENUM[NSYM as usize] = -1; SARRELT[NSYM as usize] = -1; SARRVTY[NSYM as usize] = -1; SARR64[NSYM as usize] = 0; SGAK[NSYM as usize] = 0; SGAT[NSYM as usize] = -1; SGAK2[NSYM as usize] = 0; SGAT2[NSYM as usize] = -1; SCRETCLO[NSYM as usize] = 0; SISOPT[NSYM as usize] = 0; SOPTREF[NSYM as usize] = 0; SCKNOWN[NSYM as usize] = 0; SU8[NSYM as usize] = 0; NSYM = NSYM + 1; } }   /* LAYER4 P-ENUM-VALUE-WIDTH: reset SU8 (the u8 value-width flag) for the new symbol -- it was the ONE per-symbol flag sym_add forgot to clear, so a symbol reused at the same index (NSYM reset per fn) inherited a STALE SU8=1 from a prior fn's `u8` param. Then reading that symbol masked its value `& 255`; for a `let e = E::V` enum binding at a poisoned index this truncated the enum-cell POINTER -> load(garbage) -> crash (rustc_lexer: unescape `impl From<u8> for MixedUnit` before `token.kind != TokenKind::Eof`). std-hoist: reset generic type-arg kind (scalar/no-struct) so a param without recorded kind (e.g. &RVec<i32>) does not inherit a stale SGAT>=0 -> get()->T mis-tagged as a moved struct */
fn sym_idx(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return i; } i = i - 1; } -1 } }
fn mut_used_later(o: i32, l: i32) -> bool {                /* R3: is the identifier [o,l] used anywhere in the rest of the enclosing block (from POS to its close)? */
    unsafe {
        let save = POS; let mut depth = 0; let mut found = false;
        loop {
            skipws();
            if POS >= LEN { break; }
            let c = at(POS);
            if c == 123 { depth = depth + 1; POS = POS + 1; }
            else if c == 125 { if depth == 0 { break; } depth = depth - 1; POS = POS + 1; }
            else if is_idst(c) { let io = POS; while POS < LEN && is_idc(at(POS)) { POS = POS + 1; } if span_eq_at(io, POS - io, o, l) { found = true; break; } }
            else { POS = POS + 1; }
        }
        POS = save; found
    }
}
fn die_borrow() { unsafe {                                 /* R3: aliasing violation -> REJECT like rustc, exit 1 */
    eputc(101); eputc(114); eputc(114); eputc(111); eputc(114); eputc(58); eputc(32);   /* "error: " */
    eputc(99); eputc(97); eputc(110); eputc(110); eputc(111); eputc(116); eputc(32);    /* "cannot " */
    eputc(98); eputc(111); eputc(114); eputc(114); eputc(111); eputc(119); eputc(32);   /* "borrow " */
    eputc(109); eputc(117); eputc(116); eputc(97); eputc(98); eputc(108); eputc(121); eputc(32);   /* "mutably " */
    eputc(116); eputc(119); eputc(105); eputc(99); eputc(101); eputc(10);               /* "twice\n" */
    __syscall(60, 1, 0, 0);
} }
fn sym_forref(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SFORREF[i as usize]; } i = i - 1; } 0 } }
fn sym_byref(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SBYREF[i as usize]; } i = i - 1; } 0 } }
fn sym_sref1(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SREF1[i as usize]; } i = i - 1; } 0 } }   /* D004 IC-3: is this symbol a by-ref match/iflet payload (skip-one-then-load on `*`)? */
fn sym_elemref(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SELEMREF[i as usize]; } i = i - 1; } 0 } }   /* S1-AST: is this a struct loop var bound from a `&vec`/`.iter()` reference iteration -> move-exempt? */
fn sym_bool(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SBOOL[i as usize]; } i = i - 1; } 0 } }
fn sym_char(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SCHAR[i as usize]; } i = i - 1; } 0 } }
fn sym_into(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SINTO[i as usize]; } i = i - 1; } 0 - 1 } }
/* TI0: ensure the 7 base Tys occupy indices 0..6 (i32=0,i64=1,&str=2,String=3,char=4,bool=5,unit=6) */
fn ty_bases() { unsafe { if NTY == 0 { TYTAG[0] = 1; TYTAG[1] = 2; TYTAG[2] = 3; TYTAG[3] = 4; TYTAG[4] = 5; TYTAG[5] = 6; TYTAG[6] = 7; NTY = 7; } } }
/* TI0: intern a composite Ty (tag,elt,aux) -- dedup then append */
fn ty_intern(tag: i32, elt: i32, aux: i32) -> i32 { unsafe { ty_bases(); let mut i = 7; while i < NTY { if TYTAG[i as usize] == tag && TYELT[i as usize] == elt && TYAUX[i as usize] == aux { return i; } i = i + 1; } TYTAG[NTY as usize] = tag; TYELT[NTY as usize] = elt; TYAUX[NTY as usize] = aux; NTY = NTY + 1; NTY - 1 } }
/* TI0: codec -- build a Ty from the current E-flag side-band (the just-parsed value's kind) */
fn ty_of_flags() -> i32 { unsafe { ty_bases();
    if EISSTRING == 1 { return 3; }
    if EISSTR == 1 { return 2; }
    if EISCHAR == 1 { return 4; }
    if EISBOOL == 1 { return 5; }
    if EISFLT == 1 { return ty_intern(17, 0, 0); }             /* FLAG-RETIRE P1: f64 (softfloat) Ty */
    if EIS8 == 1 { return ty_intern(16, 0, 0); }               /* FLAG-RETIRE P1: u8/i8 Ty */
    if EISU == 1 && EIS64 == 1 { return ty_intern(15, 0, 0); } /* FLAG-RETIRE P1: u64 Ty */
    if EISU == 1 { return ty_intern(14, 0, 0); }               /* FLAG-RETIRE P1: u32 (unsigned) Ty */
    if EIS64 == 1 { return 1; }
    if EISVEC == 1 { return ty_intern(8, 0, 0); }
    if EISOPT == 1 { return ty_intern(9, 0, 0); }
    if EISENUM == 1 { return ty_intern(13, 0, EIENUMTY); }   /* FLAG-RETIRE P0: enum Ty (tag 13, aux = enum index) -- the alphabet gap that blocked reading enum-ness from the Ty channel (was: an enum value's EITY fell through to 0/i32) */
    if EITYP >= 0 { return ty_intern(12, 0, EITYP); }
    0 } }
/* TI0: codec -- set the E-flags from a Ty (inverse of ty_of_flags; used when consumers migrate to Ty) */
fn apply_ty(t: i32) { unsafe { if t < 0 || t >= NTY { return; } let tag = TYTAG[t as usize];
    if tag == 2 { EIS64 = 1; } else if tag == 3 { EISSTR = 1; } else if tag == 4 { EISSTRING = 1; } else if tag == 5 { EISCHAR = 1; } else if tag == 6 { EISBOOL = 1; } else if tag == 8 { EISVEC = 1; } else if tag == 9 { EISOPT = 1; } else if tag == 11 { ETUPTY = TYAUX[t as usize]; EITY_PIN = t; } else if tag == 12 { EITYP = TYAUX[t as usize]; } else if tag == 13 { EISENUM = 1; EIENUMTY = TYAUX[t as usize]; } else if tag == 14 { EISU = 1; } else if tag == 15 { EISU = 1; EIS64 = 1; } else if tag == 16 { EIS8 = 1; } else if tag == 17 { EISFLT = 1; } } }   /* FLAG-RETIRE P0/P1: enum + scalar-width Tys -> restore the flags (inverse of ty_of_flags) */
fn sym_ty(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return STY[i as usize]; } i = i - 1; } 0 } }
fn sym_tupty(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return STUPTY[i as usize]; } i = i - 1; } 0 - 1 } }
/* tuple-field-kind: encode the just-parsed value's KIND into one code (0 scalar/1 &str/2 Vec/3 String/4 char/5 i64) */
fn cur_kind() -> i32 { unsafe { if EISSTRING == 1 { return 3; } if EISSTR == 1 { return 1; } if EISVEC == 1 { return 2; } if EISCHAR == 1 { return 4; } if EIS64 == 1 { return 5; } 0 } }
/* tuple-field-kind: set the E-flags from a kind code (for a restored tuple field / destructured binding) */
fn apply_kind(k: i32) { unsafe { if k == 3 { EISSTRING = 1; } else if k == 1 { EISSTR = 1; } else if k == 2 { EISVEC = 1; } else if k == 4 { EISCHAR = 1; } else if k == 5 { EIS64 = 1; } } }
/* tuple-field-kind: field kind of tuple-type `ty`, field `fi` (0 if unknown) */
fn tupty_fk(ty: i32, fi: i32) -> i32 { unsafe { if ty < 0 || ty >= NTUPTY { return 0; } if fi < 0 || fi >= TUPTY_N[ty as usize] { return 0; } TFK[(TUPTY_BASE[ty as usize] + fi) as usize] } }
fn sym_flt(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SFLT[i as usize]; } i = i - 1; } 0 } }
fn sym_fltv(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SFLTV[i as usize]; } i = i - 1; } 0 } }   /* SOFTFLOAT: is NAME a fixed-point f64 local? */
fn sym_enum(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SENUM[i as usize]; } i = i - 1; } 0 } }
fn mark_forref() { unsafe { SFORREF[(NSYM - 1) as usize] = 1; } }
fn sym_i64(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SI64[i as usize]; } i = i - 1; } 0 } }
fn sym_u8(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SU8[i as usize]; } i = i - 1; } 0 } }
fn sym_u(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SU64[i as usize]; } i = i - 1; } 0 } }
fn sym_moved(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SMOVED[i as usize]; } i = i - 1; } 0 } }
fn mv_mark_i(i: i32) { unsafe { if SMOVED[i as usize] == 0 { SMOVED[i as usize] = 1; if MVLOGN < 16384 { MVLOG[MVLOGN as usize] = i; MVLOGN = MVLOGN + 1; } } } }   /* MOVE-FLOW: mark symbol i moved + log the 0->1 transition (idempotent per live range) */
fn mv_undo(base: i32) { unsafe { while MVLOGN > base { MVLOGN = MVLOGN - 1; let i = MVLOG[MVLOGN as usize]; SMOVED[i as usize] = 0; } } }   /* MOVE-FLOW: undo all move transitions back to `base` (restore the pre-branch moved-set) */
fn mv_stash(base: i32, thresh: i32) -> i32 { unsafe { let s0 = MVSTASHN; let mut k = base; while k < MVLOGN { let i = MVLOG[k as usize]; if i < thresh && MVSTASHN < 16384 { MVSTASH[MVSTASHN as usize] = i; MVSTASHN = MVSTASHN + 1; } k = k + 1; } s0 } }   /* MOVE-FLOW: stash a branch's OUTER-symbol (index < thresh) moves for post-branch union; inner (pattern/block-local) moves are excluded -- their indices are reused across arms */
fn mv_replay(s0: i32) { unsafe { let mut k = s0; while k < MVSTASHN { mv_mark_i(MVSTASH[k as usize]); k = k + 1; } MVSTASHN = s0; } }   /* MOVE-FLOW: re-apply stashed outer-symbol moves (union) into the enclosing scope, then pop the stash */
fn mark_moved(o: i32, l: i32) { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { mv_mark_i(i); i = 0 - 1; } else { i = i - 1; } } } }
fn mark_unmoved(o: i32, l: i32) { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { SMOVED[i as usize] = 0; i = 0 - 1; } else { i = i - 1; } } } }   /* MOVEBORROW: reassignment `NAME = expr` un-moves NAME (a mut local rebind is legal after a move) */
fn eputc(c: i32) { unsafe { EB1[0] = c as u8; __syscall(1, 2, &EB1[0], 1); } }   /* B1: write one byte to stderr */
fn eputnum(n: i32) { unsafe { if n < 0 { eputc(45); eputnum(0 - n); return; } if n >= 10 { eputnum(n / 10); } eputc(48 + (n % 10)); } }   /* decimal to stderr (recursive), for a located diagnostic */
fn eput_at(off: i32) { unsafe { eputc(32); eputc(64); eputnum(off); }   /* " @<byte-offset>" -- a rustc-style location marker on a terminal diagnostic; the reach instrument maps this offset to a source line */ }
/* R0.5: reject a construct we cannot faithfully lower (rather than mis-lower it into a silent miscompile).
   Prints `error: unsupported: <span>` to stderr and exits 1, like rustc rejects. The span [o,l] is the
   offending token, written straight from SRC (no byte-slice read -> tier-0-safe). */
fn die_unsupported(o: i32, l: i32) { unsafe {
    eputc(101); eputc(114); eputc(114); eputc(111); eputc(114); eputc(58); eputc(32);                       /* "error: " */
    eputc(117); eputc(110); eputc(115); eputc(117); eputc(112); eputc(112); eputc(111); eputc(114); eputc(116); eputc(101); eputc(100); eputc(58); eputc(32);   /* "unsupported: " */
    __syscall(1, 2, &SRC[o as usize], l);                /* the offending construct */
    eput_at(o);
    eputc(10);
    __syscall(60, 1, 0, 0);
} }
fn die_nonexhaustive() { unsafe {                        /* R4: non-exhaustive enum match -> REJECT like rustc, exit 1 */
    eputc(101); eputc(114); eputc(114); eputc(111); eputc(114); eputc(58); eputc(32);   /* "error: " */
    eputc(110); eputc(111); eputc(110); eputc(45); eputc(101); eputc(120); eputc(104); eputc(97); eputc(117); eputc(115); eputc(116); eputc(105); eputc(118); eputc(101);   /* "non-exhaustive" */
    eputc(32); eputc(112); eputc(97); eputc(116); eputc(116); eputc(101); eputc(114); eputc(110); eputc(115); eputc(10);   /* " patterns\n" */
    __syscall(60, 1, 0, 0);
} }
fn die_nofield(o: i32, l: i32) { unsafe {                /* R2ba: E0609 -- field access `.X` on a KNOWN struct type where X is not a field -> REJECT (was a silent wrong-offset no-op), exit 1 */
    eputc(101); eputc(114); eputc(114); eputc(111); eputc(114); eputc(58); eputc(32);   /* "error: " */
    eputc(110); eputc(111); eputc(32);                                                   /* "no " */
    eputc(102); eputc(105); eputc(101); eputc(108); eputc(100); eputc(32);               /* "field " */
    eputc(110); eputc(97); eputc(109); eputc(101); eputc(100); eputc(32);                /* "named " */
    __syscall(1, 2, &SRC[o as usize], l);                                                /* the missing field name */
    eput_at(o);
    eputc(10);
    __syscall(60, 1, 0, 0);
} }
fn die_nomethod(o: i32, l: i32) { unsafe {              /* R2bf: E0599 -- `.X(..)` method call on a KNOWN struct type where X is not a defined method -> REJECT (was an undefined `call Type_X` link-fail/miscompile), exit 1 */
    eputc(101); eputc(114); eputc(114); eputc(111); eputc(114); eputc(58); eputc(32);   /* "error: " */
    eputc(110); eputc(111); eputc(32);                                                   /* "no " */
    eputc(109); eputc(101); eputc(116); eputc(104); eputc(111); eputc(100); eputc(32);   /* "method " */
    eputc(110); eputc(97); eputc(109); eputc(101); eputc(100); eputc(32);                /* "named " */
    __syscall(1, 2, &SRC[o as usize], l);                                                /* the missing method name */
    eput_at(o);
    eputc(10);
    __syscall(60, 1, 0, 0);
} }
fn die_mismatch() { unsafe {                            /* R2bn: E0308 -- comparing/storing two DIFFERENT struct types -> REJECT (was a silent field-wise compare on mismatched layouts), exit 1 */
    eputc(101); eputc(114); eputc(114); eputc(111); eputc(114); eputc(58); eputc(32);   /* "error: " */
    eputc(109); eputc(105); eputc(115); eputc(109); eputc(97); eputc(116); eputc(99); eputc(104); eputc(101); eputc(100); eputc(32);   /* "mismatched " */
    eputc(116); eputc(121); eputc(112); eputc(101); eputc(115);                          /* "types" */
    eputc(10);
    __syscall(60, 1, 0, 0);
} }
/* R2bf: does struct type `cty` have a callable method `mo/ml`? Conservative -- ANY positive signal = exists.
   Combines: prepass method registry (MRTOWN, order-independent), dyn/default registry (MITY), blanket impls,
   the current-impl override list, and the trait-default NAME registry (coarse, to never false-reject a default). */
fn method_exists(cty: i32, mo: i32, ml: i32) -> i32 {
    unsafe {
        let mut i = 0;
        while i < NMRT { if MRTOWN[i as usize] == cty && span_eq_at(MRTO[i as usize], MRTL[i as usize], mo, ml) { return 1; } i = i + 1; }
        i = 0;
        while i < NMI { if MITY[i as usize] == cty && span_eq_at(MIMO[i as usize], MIML[i as usize], mo, ml) { return 1; } i = i + 1; }
        if blanket_find(mo, ml) >= 0 { return 1; }
        if impl_has_method(mo, ml) == 1 { return 1; }
        i = 0;
        while i < NTDM { if span_eq_at(TDMO[i as usize], TDML[i as usize], mo, ml) { return 1; } i = i + 1; }
        0
    }
}
fn die_moved() { unsafe {                                /* R18/B1: use-of-moved value -> REJECT with a real diagnostic on stderr, exit 1, like rustc */
    eputc(101); eputc(114); eputc(114); eputc(111); eputc(114); eputc(58); eputc(32);   /* "error: " */
    eputc(117); eputc(115); eputc(101); eputc(32);                                       /* "use " */
    eputc(111); eputc(102); eputc(32);                                                   /* "of " */
    eputc(109); eputc(111); eputc(118); eputc(101); eputc(100); eputc(32);               /* "moved " */
    eputc(118); eputc(97); eputc(108); eputc(117); eputc(101); eputc(10);                /* "value\n" */
    __syscall(60, 1, 0, 0);
} }
fn die_const_oob() { unsafe {                             /* CONSTEVAL R1: E0080 -- const array index out of bounds -> REJECT at fold time, exit 1, like rustc */
    eputc(101); eputc(114); eputc(114); eputc(111); eputc(114); eputc(58); eputc(32);   /* "error: " */
    eputc(105); eputc(110); eputc(100); eputc(101); eputc(120); eputc(32);               /* "index " */
    eputc(111); eputc(111); eputc(98); eputc(10);                                        /* "oob\n" */
    __syscall(60, 1, 0, 0);
} }
fn die_undefined(o: i32, l: i32) { unsafe {               /* NAMERES: E0425 -- call to a function not defined anywhere -> REJECT, exit 1, like rustc */
    eputc(101); eputc(114); eputc(114); eputc(111); eputc(114); eputc(58); eputc(32);   /* "error: " */
    eputc(99); eputc(97); eputc(110); eputc(110); eputc(111); eputc(116); eputc(32);     /* "cannot " */
    eputc(102); eputc(105); eputc(110); eputc(100); eputc(32);                           /* "find " */
    eputc(102); eputc(110); eputc(32);                                                   /* "fn " */
    __syscall(1, 2, &SRC[o as usize], l);                                                /* the undefined fn name */
    eput_at(o);
    eputc(10);
    __syscall(60, 1, 0, 0);
} }
fn die_internal_limit() { unsafe {   /* DEFENSIVE: a fixed compiler table (symbol scope) overflowed -> a mis-parse ran away. Fail CLEANLY (exit 1) instead of writing OOB and corrupting adjacent statics (silent UB, a hang). Never trips on valid input: >8000 locals in one scope is always a runaway. */
    eputc(101); eputc(114); eputc(114); eputc(111); eputc(114); eputc(58); eputc(32);   /* "error: " */
    eputc(105); eputc(110); eputc(116); eputc(101); eputc(114); eputc(110); eputc(97); eputc(108); eputc(32);   /* "internal " */
    eputc(108); eputc(105); eputc(109); eputc(105); eputc(116); eputc(10);               /* "limit\n" */
    __syscall(60, 1, 0, 0);
} }
fn preceded_by_dot(o: i32) -> i32 {   /* NAMERES: is the identifier at `o` immediately preceded (over ws) by a `.`? -> a leaked METHOD name, not a free call */
    unsafe { let mut i = o - 1; while i >= 0 && (at(i) == 32 || at(i) == 9 || at(i) == 10 || at(i) == 13) { i = i - 1; } if i >= 0 && at(i) == 46 { 1 } else { 0 } }
}
fn die_overflow() { unsafe {                              /* CONSTEVAL R2: arithmetic overflow in a const-foldable i32 expr -> REJECT at fold time, exit 1, like rustc */
    eputc(101); eputc(114); eputc(114); eputc(111); eputc(114); eputc(58); eputc(32);   /* "error: " */
    eputc(111); eputc(118); eputc(101); eputc(114); eputc(102); eputc(108); eputc(111); eputc(119); eputc(10);   /* "overflow\n" */
    __syscall(60, 1, 0, 0);
} }
fn die_qmark() { unsafe {                                 /* CHECK N4: E0277 -- `?` on an Option in a `-> Result` fn -> REJECT, exit 1, like rustc */
    eputc(101); eputc(114); eputc(114); eputc(111); eputc(114); eputc(58); eputc(32);   /* "error: " */
    eputc(96); eputc(63); eputc(96); eputc(32);                                          /* "`?` " */
    eputc(111); eputc(110); eputc(32);                                                   /* "on " */
    eputc(79); eputc(112); eputc(116); eputc(105); eputc(111); eputc(110); eputc(10);    /* "Option\n" */
    __syscall(60, 1, 0, 0);
} }
fn sym_styp(o: i32, l: i32) -> i32 {
    unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SST[i as usize]; } i = i - 1; } -1 }
}
fn sym_kind(o: i32, l: i32) -> i32 {
    unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SK[i as usize]; } i = i - 1; } 0 }
}
fn sym_ckknown(o: i32, l: i32) -> i32 {   /* CONSTEVAL: 1 if NAME is a const-known immutable local (most-recent binding wins -> shadowing safe) */
    unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SCKNOWN[i as usize]; } i = i - 1; } 0 }
}
fn sym_ckval(o: i32, l: i32) -> i32 {   /* CONSTEVAL: the const-known value of NAME (guard with sym_ckknown) */
    unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SCKVAL[i as usize]; } i = i - 1; } 0 }
}
fn sym_arrlen(o: i32, l: i32) -> i32 {   /* R1: element count of a local fixed array, or 0 */
    unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SARRLEN[i as usize]; } i = i - 1; } 0 }
}
fn span_eq_at(o1: i32, l1: i32, o2: i32, l2: i32) -> bool {
    if l1 != l2 { return false; }
    let mut j = 0;
    let mut eq = true;
    while j < l1 { if at(o1 + j) != at(o2 + j) { eq = false; } j = j + 1; }
    eq
}
fn sym_find(o: i32, l: i32) -> i32 {
    unsafe {
        let mut i = NSYM - 1;
        while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SV[i as usize]; } i = i - 1; }
        -1
    }
}

/* C-L6-CONST-DUP: consts are 35% of what L6 emits and 90% repeat a value already live in the same function.
   `ec` cannot be cached directly: it has THREE responsibilities and callers depend on all of them --
   (1) emit a constant, (2) allocate a fresh slot that callers assign through, (3) ADVANCE NC, which
   neighbouring statements read directly (`let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;`).
   `ecr` is the read-only variant. It handles (2) by being routed only through bindings whose name is provably
   never a write destination, and (3) by BURNING A VREG NUMBER on a cache hit -- NC advances exactly as it would
   have, so every caller that reads NC around the call still sees what it expects. An earlier attempt cached
   without burning and broke L6 outright (the stage gate hung on `v.iter().sum()`).
   The cache is STRAIGHT-LINE, reset immediately after EVERY control-flow emission, per STATEMENT: an
   end-of-line reset let an ec() later on the same line reuse a pre-branch const and regressed six probes. */
static mut CKK: [i32; 64] = [0; 64];
static mut CKV: [i32; 64] = [0; 64];
static mut CKN: i32 = 0;
fn ckreset() { unsafe { CKN = 0; } }
fn ckfind(k: i32) -> i32 { unsafe { let mut i = 0; while i < CKN { if CKK[i as usize] == k { return CKV[i as usize]; } i = i + 1; } 0 - 1 } }
fn ckput(k: i32, v: i32) { unsafe { if CKN < 64 { CKK[CKN as usize] = k; CKV[CKN as usize] = v; CKN = CKN + 1; } } }
fn ecr(v: i32) -> i32 { unsafe { let hit = ckfind(v); if hit >= 0 { EISSTR = 0; EIS64 = 0; NC = NC + 1; return hit; } let r = ec(v); ckput(v, r); r } }
fn ec(v: i32) -> i32 { unsafe { EISSTR = 0; EIS64 = 0; println!("  v{}: i32 = const {}", NC, v); NC = NC + 1; NC - 1 } }
fn ebin(op: &str, a: i32, b: i32) -> i32 { unsafe { EISSTR = 0; EIS64 = 0; println!("  v{}: i32 = {} v{} v{}", NC, op, a, b); NC = NC + 1; NC - 1 } }
fn eref(v: i32) -> i32 { unsafe { println!("  v{}: i32 = ref v{}", NC, v); NC = NC + 1; NC - 1 } }
fn ecopy(dst: i32, src: i32) { unsafe { println!("  v{}: i32 = or v{} v{}", dst, src, src); } }   /* K2: single-def copy (or src src == src) -- the old `add src (ec 0)` reused NC into a self-referential double-def of dst, which opt/ra mis-handle for descriptor pointers (byte-slice / as_bytes reads returned 0). Mirrors ecopy64's or8-identity. */
fn ecopy64(dst: i32, src: i32) { unsafe { println!("  v{}: i64 = or8 v{} v{}", dst, src, src); } }   /* R17: 64-bit copy */
fn ebin8(op: i32, a: i32, b: i32) -> i32 { unsafe { print!("  v{}: i64 = ", NC); if op == 0 { print!("add8"); } else if op == 1 { print!("sub8"); } else if op == 2 { print!("mul8"); } else if op == 3 { print!("div8"); } else { print!("mod8"); } println!(" v{} v{}", a, b); NC = NC + 1; NC - 1 } }   /* SOFTFLOAT: emit an i64 binary op (0 add/1 sub/2 mul/3 div/4 mod) */
fn econv32(v: i32) -> i32 { unsafe { println!("  v{}: i32 = conv v{}", NC, v); NC = NC + 1; NC - 1 } }   /* SOFTFLOAT: i64 -> i32 (value known to fit) */
/* R17: i64 helpers */
fn emit_conv(v: i32) -> i32 { unsafe { println!("  v{}: i64 = conv v{}", NC, v); NC = NC + 1; NC - 1 } }
fn emit_convs(v: i32) -> i32 { unsafe { println!("  v{}: i64 = convs v{}", NC, v); NC = NC + 1; NC - 1 } }   /* R2: sign-extend i32->i64 (for arithmetic >>) */
fn w64(a: i32, b: i32) -> i32 { let w = if a == 1 || b == 1 { 1 } else { 0 }; w }   /* 1 if either operand is i64 -- natural if-|| (tier-1: beyond the C seed, built by tier-0) */
fn ec64raw(v: i64) -> i32 { unsafe { EISSTR = 0; EIS64 = 0; println!("  v{}: i32 = const {}", NC, v); NC = NC + 1; NC - 1 } }   /* const from a 32-bit-range i64 */
fn emit_i64_lit(val: i64) -> i32 {
    unsafe {
        if val < 4294967296 && val >= 0 {                       /* fits in low 32 bits: const(lo signed) -> conv -> or8 */
            let mut lo = val; if lo >= 2147483648 { lo = lo - 4294967296; }
            let c = ec64raw(lo); let cv = emit_conv(c);
            println!("  v{}: i64 = or8 v{} v{}", NC, cv, cv); let r = NC; NC = NC + 1; EIS64 = 1; r
        } else {
            let mut lo = val % 4294967296; if lo < 0 { lo = lo + 4294967296; }   /* value-width: NON-negative low 32 bits so negative/u64>i63 vals emit correct two's-complement bits (was: hi=0 for -1 -> 4294967295) */
            let hi = (val - lo) / 4294967296;
            let mut los = lo; if los >= 2147483648 { los = los - 4294967296; }
            let cl = ec64raw(los); let clv = emit_conv(cl);
            let mut his = hi; if his >= 2147483648 { his = his - 4294967296; }   /* fold hi to signed i32; conv zero-extends its low 32 bits */
            let ch = ec64raw(his); let chv = emit_conv(ch);
            let c32 = ec64raw(32); let c32v = emit_conv(c32);
            println!("  v{}: i64 = shl8 v{} v{}", NC, chv, c32v); let sh = NC; NC = NC + 1;
            println!("  v{}: i64 = or8 v{} v{}", NC, sh, clv); let r = NC; NC = NC + 1; EIS64 = 1; r
        }
    }
}
/* value-width: primitive int type tag (1=i8 2=i16 3=i32 4=i64 5=u8 6=u16 7=u32 8=u64 9=usize 10=isize; 0=not-prim) -- for `Type::MAX/MIN` assoc consts */
fn vfield_eq_kind(fo: i32, fl: i32, en: i32) -> i32 {   /* LAYER5 P-DERIVE-RECURSIVE-EQ: classify a variant field's TYPE for derived structural eq. 0 scalar, 1 Box<ThisEnum> (recurse), 2 String/&str, 3 unsupported. */
    unsafe {
        if is_string_ty(fo, fl) || (fl == 3 && at(fo) == 115 && at(fo + 1) == 116 && at(fo + 2) == 114) { return 2; }   /* LAYER6 P-DERIVE-EQ-STRING-PAYLOAD: String/&str payload field IS derivable (kind 2 -> emit_str_desc_eq). The payload word at cell+off IS a String descriptor pointer (same as `s3` manual-match binds it), so byte-compare via __str_match works. R7's kind-3 deferral (reject) is lifted now that the `==` site routes enum operands to the enum-eq branch (not the leaked-EISSTRING inline string path). */
        if fl == 3 && at(fo) == 66 && at(fo + 1) == 111 && at(fo + 2) == 120 {   /* Box */
            let mut q = fo + 3; while q < LEN && (at(q) == 32 || at(q) == 9) { q = q + 1; }
            if q < LEN && at(q) == 60 { q = q + 1; while q < LEN && (at(q) == 32 || at(q) == 38 || at(q) == 9) { q = q + 1; }
                if q < LEN && at(q) == 39 { q = q + 1; while q < LEN && is_idc(at(q)) { q = q + 1; } while q < LEN && at(q) == 32 { q = q + 1; } }   /* skip a lifetime `&'a` */
                let io = q; while q < LEN && is_idc(at(q)) { q = q + 1; } let il = q - io;
                if span_eq_at(io, il, ENO[en as usize], ENL[en as usize]) { return 1; }   /* Box<ThisEnum> -> recurse via <Enum>_eq */
            }
            return 3;   /* Box<other> -- would need another type's eq; unsupported for now (reject, not miscompile) */
        }
        if prim_int_ty(fo, fl) > 0 { return 0; }
        if fl == 4 && at(fo) == 98 && at(fo + 1) == 111 && at(fo + 2) == 111 && at(fo + 3) == 108 { return 0; }   /* bool */
        if fl == 4 && at(fo) == 99 && at(fo + 1) == 104 && at(fo + 2) == 97 && at(fo + 3) == 114 { return 0; }   /* char */
        if fl == 5 && at(fo) == 117 && at(fo + 1) == 115 && at(fo + 2) == 105 && at(fo + 3) == 122 && at(fo + 4) == 101 { return 0; }   /* usize */
        if fl == 5 && at(fo) == 105 && at(fo + 1) == 115 && at(fo + 2) == 105 && at(fo + 3) == 122 && at(fo + 4) == 101 { return 0; }   /* isize */
        return 3;   /* nested struct/enum/Vec/foreign -- unsupported for derived eq: the `==` site rejects (E: no silent miscompile) */
    }
}
fn prim_int_ty(o: i32, l: i32) -> i32 {
    unsafe {
        if l == 2 && at(o) == 105 && at(o + 1) == 56 { return 1; }
        if l == 2 && at(o) == 117 && at(o + 1) == 56 { return 5; }
        if l == 3 && at(o) == 105 && at(o + 1) == 49 && at(o + 2) == 54 { return 2; }
        if l == 3 && at(o) == 105 && at(o + 1) == 51 && at(o + 2) == 50 { return 3; }
        if l == 3 && at(o) == 105 && at(o + 1) == 54 && at(o + 2) == 52 { return 4; }
        if l == 3 && at(o) == 117 && at(o + 1) == 49 && at(o + 2) == 54 { return 6; }
        if l == 3 && at(o) == 117 && at(o + 1) == 51 && at(o + 2) == 50 { return 7; }
        if l == 3 && at(o) == 117 && at(o + 1) == 54 && at(o + 2) == 52 { return 8; }
        if l == 5 && at(o) == 117 && at(o + 1) == 115 { return 9; }
        if l == 5 && at(o) == 105 && at(o + 1) == 115 { return 10; }
        0
    }
}
/* R17: i64-aware binop. opis64 -> append 8 + i64 type; resis64 -> the result's EIS64 (0 for compares). */
fn ebin8raw(op: &str, a: i32, b: i32) -> i32 { unsafe { EISSTR = 0; println!("  v{}: i64 = {}8 v{} v{}", NC, op, a, b); NC = NC + 1; NC - 1 } }
fn emit_binop(op: &str, a: i32, b: i32, opis64: i32, resis64: i32) -> i32 {
    unsafe {
        let mut r = 0;
        if opis64 == 1 { r = ebin8raw(op, a, b); } else { r = ebin(op, a, b); }
        EIS64 = resis64; r
    }
}
/* R17c: emit a binop, CONV'ing any i32 operand to i64 when the op is 64-bit -- an i32 const stored as a
   32-bit DWORD but read as a QWORD leaves stale high bits (e.g. `n <= 1` collapsing to true after a print). */
fn i64op(op: &str, a: i32, a64: i32, b: i32, b64: i32, resis64: i32) -> i32 {
    unsafe {
        let w = w64(a64, b64);
        let uns = WUNS; WUNS = 0;                                             /* value-width: signed widening by default (sign-extend); WUNS=1 -> unsigned (zero-extend). Was ALWAYS zero-extend -> a negative i32 operand (e.g. `x < -1000000000`) became a huge positive -> wrong signed compare. */
        let mut aa = a; let mut bb = b;
        if w == 1 { if a64 == 0 { if uns == 1 { aa = emit_conv(a); } else { aa = emit_convs(a); } } if b64 == 0 { if uns == 1 { bb = emit_conv(b); } else { bb = emit_convs(b); } } }
        emit_binop(op, aa, bb, w, resis64)
    }
}
fn emit_tail(v: i32) { unsafe { if TAILDST > 0 { if EIS64 == 1 { ecopy64(TAILDST - 1, v); } else { ecopy(TAILDST - 1, v); } } else if TAILDST < 0 { let d = v; } else if EISUNIT == 1 { let d = v; } else { let mut rv = v; if CUR_RET_I64 == 1 && EIS64 == 0 { if CUR_RET_U64 == 1 { rv = emit_conv(v); } else { rv = emit_convs(v); } } if INMAIN == 1 { emit_flush_call(); } println!("  ret v{}", rv); } } }   /* TAILDST<0 = loop body: discard; EISUNIT = a void-call tail (Wave-2 bug 3): discard, never `ret`; R2: widen a 32-bit tail into an i64/u64-returning fn; R3: flush buffered output before main's tail ret */
fn parse_loop_body() { unsafe { let s = TAILDST; TAILDST = -1;
    LDEPTH = LDEPTH + 1; let lvl = LDEPTH;
    /* LAYER2 P4: inline the loop body so a labeled break/continue set by a NESTED loop short-circuits the
       REST of this body. Was: parse_block ran EVERY statement, then checked the flag only at body-END -- so a
       statement after a nested loop (e.g. a `println!` following the inner `for`) still ran even when
       `break 'outer` had already fired. The per-statement check runs ONLY inside labeled scopes (NLBL>0), so
       non-labeled loops emit byte-identically. */
    let blkopen = POS; let blkend = find_block_end(POS); eat_ch(123); let mark = NSYM;
    loop {
        skipws();
        if POS >= LEN { break; }
        if at(POS) == 125 { POS = POS + 1; break; }
        if blkend >= 0 && POS > blkend { die_parse_drift(blkopen); }
        let before = POS;
        parse_stmt();
        if POS == before { POS = POS + 1; }
        if NLBL > 0 {
            let idx = (NLBL - 1) as usize;
            let f = LBLFLAG[idx]; println!("  brz v{}", f); ckreset(); println!("  brk"); println!("  pool"); ckreset();   /* break 'l set -> break this loop, skipping the rest of the body (propagates outward to the target) */
            let cf = LBLCFLAG[idx];
            if lvl == LBLLVL[idx] { println!("  brz v{}", cf); ckreset(); let z4 = ecr(0); ecopy(cf, z4); println!("  cnt"); println!("  pool"); ckreset(); }   /* continue 'l at the TARGET loop -> clear + continue, skipping the rest */
            else { println!("  brz v{}", cf); ckreset(); println!("  brk"); println!("  pool"); ckreset(); }              /* continue 'l targeting an OUTER loop -> break this (inner) loop to propagate */
        }
    }
    NSYM = mark; TAILDST = s;
    LDEPTH = LDEPTH - 1; } }
/* R2ch: the loop-value's Display type comes from `break value`, not from leftover loop-body flags.
   capture the value's type flags at a `break value`, apply them to the loop result after the body. */
fn lvf_capture(slot: i32) { unsafe { let s = slot as usize; LVFB[s] = EISBOOL; LVF64[s] = EIS64; LVFU[s] = EISU; LVFSTR[s] = EISSTR; LVFSTRING[s] = EISSTRING; LVFVEC[s] = EISVEC; LVFCHAR[s] = EISCHAR; LVFTYP[s] = EITYP; LVFENUM[s] = EISENUM; LVFENUMTY[s] = EIENUMTY; LVFARR[s] = EISARR; LVF8[s] = EIS8; } }
fn lvf_apply(slot: i32) { unsafe { let s = slot as usize; EISBOOL = LVFB[s]; EIS64 = LVF64[s]; EISU = LVFU[s]; EISSTR = LVFSTR[s]; EISSTRING = LVFSTRING[s]; EISVEC = LVFVEC[s]; EISCHAR = LVFCHAR[s]; EITYP = LVFTYP[s]; EISENUM = LVFENUM[s]; EIENUMTY = LVFENUMTY[s]; EISARR = LVFARR[s]; EIS8 = LVF8[s]; } }
fn lvf_reset(slot: i32) { unsafe { let s = slot as usize; LVFB[s] = 0; LVF64[s] = 0; LVFU[s] = 0; LVFSTR[s] = 0; LVFSTRING[s] = 0; LVFVEC[s] = 0; LVFCHAR[s] = 0; LVFTYP[s] = 0 - 1; LVFENUM[s] = 0; LVFENUMTY[s] = 0 - 1; LVFARR[s] = 0; LVF8[s] = 0; } }
/* R1: `loop { .. break v; .. }` as a value -- infinite loop with a break-value result slot. */
fn emit_loop_val() -> i32 {
    unsafe {
        let myslot = NLRES;
        lvf_reset(myslot);                        /* default this loop's result to plain int until a `break value` routes flags here */
        let res = NC; NC = NC + 1;
        let z = ecr(0); ecopy(res, z);
        LRES[NLRES as usize] = res; NLRES = NLRES + 1;
        println!("  loop"); ckreset();
        let one = ecr(1); println!("  brz v{}", one); ckreset();
        parse_loop_body();
        println!("  cont"); ckreset(); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        NLRES = NLRES - 1;
        lvf_apply(myslot);                        /* this loop's result takes the type of its `break value` */
        res
    }
}

/* a call: name ( args ) -- emit `call NAME arg...` printing the name span */
fn arg_kind() -> i32 { unsafe { if EISVEC == 1 { 2 } else if EISSTRING == 1 { 3 } else if EISSTR == 1 { 1 } else { 0 } } }   /* KP0: the just-parsed value's kind code (0 scalar/1 &str/2 Vec/3 String) */
fn emit_call(no: i32, nl: i32) -> i32 {
    unsafe {
        let turbo_styp = TURBO_STYP; TURBO_STYP = 0 - 1;   /* TRAIT R3: capture this call's turbofish type-arg before arg parsing clobbers the global */
        let rk = fnret(no, nl);
        let mut a0 = -1; let mut a1 = -1; let mut a2 = -1; let mut a3 = -1;
        let mut a4 = -1; let mut a5 = -1; let mut a6 = -1; let mut a7 = -1;
        let mut t0 = -1; let mut t1 = -1; let mut t2 = -1; let mut t3 = -1;
        let mut t4 = -1; let mut t5 = -1; let mut t6 = -1; let mut t7 = -1;
        let mut na = 0;
        let mut ar0 = 0; let mut ar1 = 0; let mut ar2 = 0; let mut ar3 = 0;   /* A: per-arg array-literal flag + length (for const-generic [T;N] coercion) */
        let mut al0 = 0; let mut al1 = 0; let mut al2 = 0; let mut al3 = 0;
        let mut k0 = 0; let mut k1 = 0; let mut k2 = 0; let mut k3 = 0;   /* KP0: per-arg kind code (0 scalar / 1 &str / 2 Vec / 3 String) */
        let mut k4 = 0; let mut k5 = 0; let mut k6 = 0; let mut k7 = 0;
        let mut vl0 = 0 - 1; let mut vl1 = 0 - 1; let mut vl2 = 0 - 1; let mut vl3 = 0 - 1;   /* R31c: per-arg element struct-type (a `&[Struct]`/`&Vec<Struct>` arg -> the generic T is the ELEMENT struct, not i32) */
        /* CL-A7: arguments past the EIGHTH. The parse below is a hard-coded 8-deep `if eat_ch(44)` chain, so a
           ninth argument's comma was never consumed -- it was neither parsed nor emitted and the call silently
           lost it. The eight named slots stay exactly as they are, because the monomorphization logic between
           the parse and the emit reads t0..t7 / k0..k7 / vl0..vl3 BY NAME.
           The overflow is parsed into XCA, then copied into a CONTIGUOUS run of fresh vregs, so only the first
           vreg number and the count have to reach the emit. Copying is not optional: an earlier revision
           recorded each argument's own vreg and assumed they were consecutive, and they are not -- parse_expr
           emits its own instructions between arguments, so a `&str` argument's setup took the number the next
           argument was assumed to be at. Measured: a 10-argument call whose last argument is a `&str` returned
           0 instead of 3. */
        let mut xfirst = 0 - 1; let mut xcount = 0; let mut xbase = 0;
        skipws();
        if peek() != 41 {
            a0 = parse_expr(); t0 = EITYP; ar0 = EISARR; al0 = EARRLEN; k0 = arg_kind(); vl0 = EVELT; na = 1;
            if eat_ch(44) { a1 = parse_expr(); t1 = EITYP; ar1 = EISARR; al1 = EARRLEN; k1 = arg_kind(); vl1 = EVELT; na = 2;
            if eat_ch(44) { a2 = parse_expr(); t2 = EITYP; ar2 = EISARR; al2 = EARRLEN; k2 = arg_kind(); vl2 = EVELT; na = 3;
            if eat_ch(44) { a3 = parse_expr(); t3 = EITYP; ar3 = EISARR; al3 = EARRLEN; k3 = arg_kind(); vl3 = EVELT; na = 4;
            if eat_ch(44) { a4 = parse_expr(); t4 = EITYP; k4 = arg_kind(); na = 5;
            if eat_ch(44) { a5 = parse_expr(); t5 = EITYP; k5 = arg_kind(); na = 6;
            if eat_ch(44) { a6 = parse_expr(); t6 = EITYP; k6 = arg_kind(); na = 7;
            if eat_ch(44) { a7 = parse_expr(); t7 = EITYP; k7 = arg_kind(); na = 8;
            xbase = XTOP;
            while eat_ch(44) { skipws(); if peek() == 41 { break; } let ax = parse_expr(); XTOP = xbase + xcount; if XTOP < 32 { XCA[XTOP as usize] = ax; XTOP = XTOP + 1; xcount = xcount + 1; na = na + 1; } }
            if xcount > 0 { xfirst = NC; let mut xw = 0; while xw < xcount { let xv = XCA[(xbase + xw) as usize]; println!("  v{}: i64 = or8 v{} v{}", NC, xv, xv); NC = NC + 1; xw = xw + 1; } XTOP = xbase; }
            } } } } } } }
        }
        eat_ch(41);
        if nl == 12 && at(no) == 105 && at(no + 1) == 115 && at(no + 2) == 95 && at(no + 3) == 120 && at(no + 4) == 105 && at(no + 5) == 100 && at(no + 6) == 95 && at(no + 7) == 115 && at(no + 8) == 116 && at(no + 9) == 97 && at(no + 10) == 114 && at(no + 11) == 116 {   /* LAYER4 P-EXT-CRATE-DEP: unicode_xid::UnicodeXID::is_xid_start(c) -> ASCII (a-z|A-Z); deterministic name-intercept (trait-shim was unsound R34). */
            let az = ebin("mul", ebin("ge", a0, ec(97)), ebin("le", a0, ec(122)));
            let auz = ebin("mul", ebin("ge", a0, ec(65)), ebin("le", a0, ec(90)));
            let r = ebin("add", az, auz); EISCHAR = 0; EISBOOL = 1; return r;
        }
        if nl == 15 && at(no) == 105 && at(no + 1) == 115 && at(no + 2) == 95 && at(no + 3) == 120 && at(no + 4) == 105 && at(no + 5) == 100 && at(no + 6) == 95 && at(no + 7) == 99 {   /* is_xid_continue -> ASCII (a-z|A-Z|0-9|_) */
            let az = ebin("mul", ebin("ge", a0, ec(97)), ebin("le", a0, ec(122)));
            let auz = ebin("mul", ebin("ge", a0, ec(65)), ebin("le", a0, ec(90)));
            let d09 = ebin("mul", ebin("ge", a0, ec(48)), ebin("le", a0, ec(57)));
            let us = ebin("eq", a0, ec(95));
            let r = ebin("add", ebin("add", ebin("add", az, auz), d09), us); EISCHAR = 0; EISBOOL = 1; return r;
        }
        let dmask = fn_dynpos(no, nl);                                             /* exotica: wrap each &c arg at a &dyn param into a fat pointer [data, type] (mask: bit p = param p) */
        if dmask % 2 == 1 { let f = halloc(8); println!("  store v{} v{}", f, a0); let c4 = ecr(4); let tc = ec(t0); println!("  store v{} v{}", ebin("add", f, c4), tc); a0 = f; }
        if (dmask / 2) % 2 == 1 { let f = halloc(8); println!("  store v{} v{}", f, a1); let c4 = ecr(4); let tc = ec(t1); println!("  store v{} v{}", ebin("add", f, c4), tc); a1 = f; }
        if (dmask / 4) % 2 == 1 { let f = halloc(8); println!("  store v{} v{}", f, a2); let c4 = ecr(4); let tc = ec(t2); println!("  store v{} v{}", ebin("add", f, c4), tc); a2 = f; }
        if (dmask / 8) % 2 == 1 { let f = halloc(8); println!("  store v{} v{}", f, a3); let c4 = ecr(4); let tc = ec(t3); println!("  store v{} v{}", ebin("add", f, c4), tc); a3 = f; }
        let arm = fn_arrmask(no, nl);                                              /* A: coerce a bare `[..]` array-literal arg at a const-generic [T;N] param into a [ptr,len] descriptor */
        if arm % 2 == 1 && ar0 == 1 { let f = halloc(8); println!("  store v{} v{}", f, a0); let c4 = ecr(4); let lc = ec(al0); println!("  store v{} v{}", ebin("add", f, c4), lc); a0 = f; }
        if (arm / 2) % 2 == 1 && ar1 == 1 { let f = halloc(8); println!("  store v{} v{}", f, a1); let c4 = ecr(4); let lc = ec(al1); println!("  store v{} v{}", ebin("add", f, c4), lc); a1 = f; }
        if (arm / 4) % 2 == 1 && ar2 == 1 { let f = halloc(8); println!("  store v{} v{}", f, a2); let c4 = ecr(4); let lc = ec(al2); println!("  store v{} v{}", ebin("add", f, c4), lc); a2 = f; }
        if (arm / 8) % 2 == 1 && ar3 == 1 { let f = halloc(8); println!("  store v{} v{}", f, a3); let c4 = ecr(4); let lc = ec(al3); println!("  store v{} v{}", ebin("add", f, c4), lc); a3 = f; }
        /* __syscall intrinsic -> `syscall args` (no callee name) */
        let is_sc = nl == 9 && at(no) == 95 && at(no + 1) == 95 && at(no + 2) == 115 && at(no + 3) == 121 && at(no + 4) == 115 && at(no + 5) == 99 && at(no + 6) == 97 && at(no + 7) == 108 && at(no + 8) == 108;
        /* K0: __argc() -> argc ; __argb(i,j) -> byte j of argv[i]. The entry rsp is saved at 0x4000000 (see cf.rs).
           argc = [[0x4000000]] ; argv[i] = [[0x4000000] + 8 + 8i] ; byte = low byte of [argv[i] + j]. All via i64
           ops because the args live at 64-bit stack addresses. */
        let is_argc = nl == 6 && at(no) == 95 && at(no + 1) == 95 && at(no + 2) == 97 && at(no + 3) == 114 && at(no + 4) == 103 && at(no + 5) == 99;   /* __argc */
        let is_argb = nl == 6 && at(no) == 95 && at(no + 1) == 95 && at(no + 2) == 97 && at(no + 3) == 114 && at(no + 4) == 103 && at(no + 5) == 98;   /* __argb */
        if is_argc {
            let kb = ecr(67108864); println!("  v{}: i64 = load8 v{}", NC, kb); let sp = NC; NC = NC + 1;
            println!("  v{}: i64 = load8 v{}", NC, sp); let r = NC; NC = NC + 1;   /* argc (high 32 bits zero) */
            return r;
        }
        if is_argb {
            let kb = ecr(67108864); println!("  v{}: i64 = load8 v{}", NC, kb); let sp = NC; NC = NC + 1;   /* saved rsp */
            let c8a = ecr(8); let i8 = ebin("mul", a0, c8a); let c8b = ecr(8); let off = ebin("add", i8, c8b);   /* 8 + i*8 (i32) */
            println!("  v{}: i64 = conv v{}", NC, off); let offc = NC; NC = NC + 1;
            println!("  v{}: i64 = add8 v{} v{}", NC, sp, offc); let pp = NC; NC = NC + 1;                 /* &argv[i] */
            println!("  v{}: i64 = load8 v{}", NC, pp); let ptr = NC; NC = NC + 1;                         /* argv[i] (64-bit ptr) */
            println!("  v{}: i64 = conv v{}", NC, a1); let jc = NC; NC = NC + 1;
            println!("  v{}: i64 = add8 v{} v{}", NC, ptr, jc); let ba = NC; NC = NC + 1;                  /* &byte j */
            println!("  v{}: i64 = load8 v{}", NC, ba); let word = NC; NC = NC + 1;                        /* 8 bytes at byte j */
            let m = ecr(255); println!("  v{}: i64 = and8 v{} v{}", NC, word, m); let r = NC; NC = NC + 1;  /* low byte */
            return r;
        }
        /* std-hoist P1.0: raw-memory intrinsics (word=4B) so a real Vec<T> can be written in Rust.
           __alloc(bytes)->ptr (runtime bump-alloc), __store(ptr,i,v)=ptr[i]=v, __load(ptr,i)->ptr[i]. */
        let is_alloc = nl == 7 && at(no) == 95 && at(no + 1) == 95 && at(no + 2) == 97 && at(no + 3) == 108 && at(no + 4) == 108 && at(no + 5) == 111 && at(no + 6) == 99;   /* __alloc */
        let is_store = nl == 7 && at(no) == 95 && at(no + 1) == 95 && at(no + 2) == 115 && at(no + 3) == 116 && at(no + 4) == 111 && at(no + 5) == 114 && at(no + 6) == 101;   /* __store */
        let is_load = nl == 6 && at(no) == 95 && at(no + 1) == 95 && at(no + 2) == 108 && at(no + 3) == 111 && at(no + 4) == 97 && at(no + 5) == 100;   /* __load */
        if is_alloc {
            let cur = eload(egref_heap_cursor());
            let base = ebin("add", egref_heap(), cur);
            let nc = ebin("add", cur, a0);
            let gc = egref_heap_cursor(); println!("  store v{} v{}", gc, nc);
            return base;
        }
        if is_store {
            let c4 = ecr(4); let off = ebin("mul", a1, c4); let ad = ebin("add", a0, off);
            println!("  store v{} v{}", ad, a2);
            return ec(0);
        }
        if is_load {
            let c4 = ecr(4); let off = ebin("mul", a1, c4); let ad = ebin("add", a0, off);
            println!("  v{}: i32 = load v{}", NC, ad); let r = NC; NC = NC + 1; return r;
        }
        let gi = gf_find(no, nl);
        if gi >= 0 {                                                              /* R9: monomorphized call NAME_TypeName */
            let ai = GFARG[gi];
            let mut styp = -1;
            if ai == 0 { styp = t0; } else if ai == 1 { styp = t1; } else if ai == 2 { styp = t2; }
            else if ai == 3 { styp = t3; } else if ai == 4 { styp = t4; } else if ai == 5 { styp = t5; }
            else if ai == 6 { styp = t6; } else { styp = t7; }
            if turbo_styp >= 0 { styp = turbo_styp; }   /* TRAIT R3: an explicit `::<Type>` turbofish is authoritative (`m::<A>()` -> m_A, not the no-arg i32 default) */
            if styp < 0 { let vv = if ai == 0 { vl0 } else if ai == 1 { vl1 } else if ai == 2 { vl2 } else if ai == 3 { vl3 } else { 0 - 1 }; if vv >= 0 { styp = vv; } }   /* R31c: a `&[Struct]`/`&Vec<Struct>` arg -> T is the element struct-type (else it fell to the i32 default -> total_i32 monomorph, wrong method dispatch) */
            if styp < 0 { let kk = if ai == 0 { k0 } else if ai == 1 { k1 } else if ai == 2 { k2 } else if ai == 3 { k3 } else if ai == 4 { k4 } else if ai == 5 { k5 } else if ai == 6 { k6 } else { k7 }; if kk == 1 { styp = -3; } else if kk == 3 { styp = -4; } else { styp = -2; } }   /* KP0.5: a &str/String T-arg -> a DISTINCT instance (NAME_str/NAME_string) so the param carries the kind in the body */
            reg_inst(gi, styp);
            print!("  v{}: i32 = call ", NC); emit_span(no, nl); print!("_");
            if styp == -2 { print!("i32"); } else if styp == -3 { print!("str"); } else if styp == -4 { print!("string"); } else { emit_span(TSO[styp as usize], TSL[styp as usize]); }
        }
        else if is_sc { print!("  v{}: i32 = syscall", NC); }
        else { print!("  v{}: i32 = call ", NC); emit_span(no, nl); }
        if na >= 1 { print!(" v{}", a0); }
        if na >= 2 { print!(" v{}", a1); }
        if na >= 3 { print!(" v{}", a2); }
        if na >= 4 { print!(" v{}", a3); }
        if na >= 5 { print!(" v{}", a4); }
        if na >= 6 { print!(" v{}", a5); }
        if na >= 7 { print!(" v{}", a6); }
        if na >= 8 { print!(" v{}", a7); }
        if xcount > 0 { let mut xci = 0; while xci < xcount { print!(" v{}", xfirst + xci); xci = xci + 1; } }   /* CL-A7 */
        println!("");
        NC = NC + 1;
        EISARR = 0; EISVEC = 0; EISCLO = 0; EISSTR = 0; EISSTRING = 0; EISCHAR = 0; EISBOOL = 0; EISENUM = 0; EIENUMTY = 0 - 1; EISBYTES = 0;   /* LAYER11 P-ASBYTES-ARG-LEAK-STRUCT-RET: also clear EISBYTES -- a `.as_bytes()`/`b".."` ARG (a &[u8] byte slice) left EISBYTES=1, which leaked past the call and mis-tagged a struct-returning free fn's result SK=6 (byte slice), so `let t = mk(src.as_bytes())` bound t byte-typed not struct-typed -> `t.field.len()` emitted `__print_int(t)` + a stray `call len` -> garbage. The free-fn dual of the emit-2456 method-path clear (which already clears EISBYTES). No rk re-sets EISBYTES, so a `-> &[u8]` fn is unaffected. LAYER5 P-MATCH-GUARD-MULTIARM: also clear EISENUM -- an ENUM arg (esp. an R10 SENUM-tagged Box<Self> match binder, `eval(a)`) left EISENUM=1, so an `-> i64` result compared with `==` mis-routed to the R7 enum-eq path (`call <Enum>_eq` on two i64s -> deref garbage -> segfault). The rk switch + R8 fnret_enum re-set EISENUM only for enum-returning fns. R2ca: a `&[..]`/vec/&str arg leaves EISARR/EISVEC/EISSTR set; for a scalar (rk==0) result that mis-binds the value (a `&str` arg -> `let r = f("x")` bound r &str -> `println!("{}",r)` did __print_str on an i32 -> crash). The rk switch below re-sets EISSTR for a str-returning fn (rk 1/3). ALSO clear EISCLO -- a CLOSURE arg leaves EISCLO=1. LAYER4 F-CHAR-MATCH-MISCOMP (cascade-doc §IV.4 arg-tag leak): also clear EISCHAR/EISBOOL -- a `char`/bool literal ARG (`cl('#')`) left EISCHAR=1, so an `-> i64` result was mis-tagged char and Displayed as a glyph (control-char -> "empty"). The rk switch re-sets EISCHAR (rk 4) / EISBOOL (rk 7) from the RETURN kind, so the result tag comes only from the callee's return type, not the last arg. */
        if rk == 3 { EISSTR = 1; EISSTRING = 1; } else if rk == 2 { EISVEC = 1; } else if rk == 1 { EISSTR = 1; } else if rk == 4 { EISCHAR = 1; } else if rk == 5 { EIS64 = 1; } else if rk == 6 { EISARR = 1; } else if rk == 7 { EISBOOL = 1; } else if rk == 8 { EISCLO = 1; } else if rk == 9 { EISOPT = 1; EOPTRES = 0; } else if rk == 10 { EISOPT = 1; EOPTRES = 1; }   /* E2 array / A4 bool / R1 closure / (a) Option/Result */
        if rk == 9 || rk == 10 { let rst = fnret_st(no, nl); if rst >= 0 { EOPTTY = ty_intern(12, 0, rst); } let ren2 = fnret_optenum(no, nl); if ren2 >= 0 { EOPTTY = ty_intern(13, 0, ren2); } if rk == 9 { EOPTK = fnret_optk(no, nl); if fnret_optref(no, nl) == 1 { EOPTREF = 1; } } }   /* D005 P-PRESULT-BOX-ENUM-PAYLOAD-BINDER: `-> Result/Option<Box<Enum>>` -> carry the payload ENUM Ty (tag 13) so a match `Ok(e)=>e.method()` binder tags e SENUM (the enum dual of the struct-payload EOPTTY tag-12 below). LAYER5 P-PRESULT-STRUCT-PAYLOAD-BINDER: `-> Result<S,..>`/`-> Option<S>` free fn -> carry the payload struct Ty on the result so a match `Ok(v)=>v.field`/`Some(v)=>v.field` binds v struct-typed. SPAN-4 P-OPTION-PAYLOAD-DISPLAY-KIND: also set EOPTK from the DECLARED payload (str->1/char->3) so a `Some(x)=>{}` binder Displays x by the return type, not by whichever branch the body emitted last (the generic gi>=0 path below overrides for generics). */
        if rk == 10 { let est = fnret_st2(no, nl); if est >= 0 { EOPTERRTY = ty_intern(12, 0, est); } else { EOPTERRTY = 0 - 1; } }   /* LAYER6 P-PRESULT-ERR-STRUCT-PAYLOAD: `-> Result<_, E>` free fn -> carry the ERR struct Ty so a match `Err(e)=>e.field` binds e struct-typed */
        if rk == 2 { let rve = fnret_velenum(no, nl); if rve >= 0 { VELENUM = rve; EVELENUM = rve; } else { VELENUM = 0 - 1; } let rvt = fnret_velt(no, nl); if rvt >= 0 { VELT = rvt; EVELT = rvt; } else { VELT = 0 - 1; } EVELSTR_RET = fnret_velstr(no, nl); }   /* R14 P-CF-ACC: also record the fn-return Vec<String>/Vec<&str> element display-kind in the NON-leaking per-RHS channel EVELSTR_RET (read only at the Vec let-binding). R40: `-> Vec<Enum>`/`-> Vec<Struct>` call result -> thread the element type so `let x = f()` binds SVELENUM/SVELT for {:?} (else stale globals leak) */
        if fnret_ref(no, nl) == 1 && (rk == 0 || rk == 5) { EWASREF = 1; }   /* R19: `fn f(..)->&T{ &x.field }` returns the value; a caller `*f(..)` is a no-op (scalar/i64 ref return) */
        if rk == 0 && gi < 0 { let rst = fnret_st(no, nl); if rst >= 0 { EITYP = rst;
            let rgak = fnret_gak(no, nl); if rgak != 0 - 99 { EGAK = rgak; EGAT = fnret_gat(no, nl); EWASGARG = 1; }   /* std-hoist P0.3: `-> B<&str>` free fn -> hand the type-arg kind to the let-binding */
        } }   /* R2cj: `-> UserStruct` free fn -> result is struct-typed (let p = mk(..); p.field) */
        if rk == 0 { let ren = fnret_enum(no, nl); if ren >= 0 { EISENUM = 1; EIENUMTY = ren; } }   /* LAYER5 P-FN-RETURNS-ENUM: `-> UserEnum` free fn -> tag the call result an enum cell so `Box::new(f())`/`let x=f()`/a variant payload treat it as a heap enum pointer (else Box::new double-boxed it -> a fold `Add(Box::new(dbl(a)),..)` built a broken tree) */
        if rk == 0 && gi >= 0 { let rst = fnret_st(no, nl); if rst >= 0 { EITYP = rst; } }   /* std-hoist: a GENERIC fn returning a user struct BY VALUE (e.g. `fn rvec_map<F:Fn..>(..)->RVec<i32>`, `fn mk<T>()->RVec<T>`) is monomorphized as NAME_TypeName; its struct-return kind is registered in FRST just like a plain fn's -- thread it so the caller binds the result struct-typed (unblocks iterator map/filter/collect) */
        if gi >= 0 && GFRETT[gi as usize] == 1 {   /* KP0: a generic fn returning its type-param T -> result kind = the T-arg's kind (`fn id<T>(x:T)->T`; id("hi") -> &str not a pointer) */
            let ai = GFARG[gi as usize];
            let kk = if ai == 0 { k0 } else if ai == 1 { k1 } else if ai == 2 { k2 } else if ai == 3 { k3 } else if ai == 4 { k4 } else if ai == 5 { k5 } else if ai == 6 { k6 } else { k7 };
            if kk == 1 { EISSTR = 1; } else if kk == 3 { EISSTR = 1; EISSTRING = 1; } else if kk == 2 { EISVEC = 1; }
        }
        if gi >= 0 && GFRETOPT[gi as usize] > 0 {   /* KP0/K1: a generic fn returning Option<T>/Result<T> -> payload kind = the T-arg's kind (`fn wrap<T>(x:T)->Option<T>{Some(x)}`; wrap(vec).unwrap().len()) */
            let ai = GFARG[gi as usize];
            let kk = if ai == 0 { k0 } else if ai == 1 { k1 } else if ai == 2 { k2 } else if ai == 3 { k3 } else if ai == 4 { k4 } else if ai == 5 { k5 } else if ai == 6 { k6 } else { k7 };
            EISOPT = 1; if GFRETOPT[gi as usize] == 2 { EOPTRES = 1; } else { EOPTRES = 0; }
            EOPTK = kk; EOPTVELT = 0 - 1;
        }
        NC - 1
    }
}

/* R1: `Name { f: e, .. }` -> heap struct; store each field at its offset; EITYP = the type. */
/* TI3e-e: `#[derive(Default)]` -> `Type::default()` / `Default::default()` zero-fills a struct of `ty`.
   halloc does NOT zero, so store 0 across the struct's word range (matches the `..Default::default()`
   struct-update zero-fill). Fields default to 0 (int), which is correct for the integer-field structs. */
fn emit_default_struct(ty: i32) -> i32 {
    unsafe {
        let base = halloc(TSZ[ty as usize]);
        let sz = TSZ[ty as usize];
        let mut off = 0;
        while off < sz { let z = ecr(0); let ad = ebin("add", base, ec(off)); println!("  store v{} v{}", ad, z); off = off + 4; }
        base
    }
}
fn struct_literal(ty: i32) -> i32 {
    unsafe {
        let base = halloc(TSZ[ty as usize]);
        eat_ch(123);
        let mut lgak = 0; let mut lgat = 0 - 1; let mut lwas = 0;                    /* std-hoist P0.1: record this literal's concrete type-arg kind (from the value of a field whose type is T) */
        let mut lgak2 = 0; let mut lgat2 = 0 - 1; let mut lwas2 = 0;                 /* std-hoist P0.3: the nested (T-of-T) kind when the T-field value is itself a generic struct */
        let setbase = NSETF;                                                       /* R1: track explicitly-set field indices for `..base` functional update */
        loop {
            skipws();
            if POS >= LEN { break; }
            if at(POS) == 125 { POS = POS + 1; break; }
            let lp = POS;                                                           /* R2av: stall-breaker -- a field value that fails to advance POS would spin forever */
            if at(POS) == 46 && POS + 1 < LEN && at(POS + 1) == 46 {                /* R1: `..base` -- copy every field not explicitly set */
                POS = POS + 2; skipws();
                let savep = POS;                                                    /* R2ao: detect `..Default::default()` / `..Type::default()` -> zero-fill */
                let mut isdef = 0;
                if rd_ident() {
                    skipws();
                    if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {
                        POS = POS + 2; skipws();
                        if rd_ident() && ID_L == 7 && at(ID_O) == 100 && at(ID_O + 1) == 101 && at(ID_O + 2) == 102 && at(ID_O + 3) == 97 && at(ID_O + 4) == 117 && at(ID_O + 5) == 108 && at(ID_O + 6) == 116 {
                            skipws();
                            if POS < LEN && at(POS) == 40 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 41 { POS = POS + 1; isdef = 1; } }
                        }
                    }
                }
                let mut src = -1;
                if isdef == 0 { POS = savep; src = parse_expr(); }
                let mut i = 0;
                while i < NFLD {
                    if FSST[i as usize] == ty {
                        let mut wasset = 0; let mut k = setbase; while k < NSETF { if SETF[k as usize] == i { wasset = 1; } k = k + 1; }
                        if wasset == 0 {
                            let off = ec(FSB[i as usize]); let ad = ebin("add", base, off);
                            if isdef == 1 { let z = ecr(0); println!("  store v{} v{}", ad, z); }
                            else { let v = eload(ebin("add", src, off)); println!("  store v{} v{}", ad, v); }
                        }
                    }
                    i = i + 1;
                }
                skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; }
            } else {
            rd_ident();
            let fo = ID_O; let fl = ID_L;
            let fi = fld_find(ty, fo, fl);
            skipws();
            let mut e = -1;
            if POS < LEN && at(POS) == 58 { POS = POS + 1; e = parse_expr(); }   /* `f: expr` */
            else { POS = lp; e = parse_expr(); }                                 /* R2cj: `Name { x, y }` field-init shorthand -- re-parse the field ident as a local-var expr */
            let ev_typ = EITYP; if fi >= 0 && ev_typ >= 0 && FSTY[fi as usize] >= 0 && ev_typ != FSTY[fi as usize] { die_mismatch(); }   /* R2bn: struct-literal field RHS is a DIFFERENT struct type than the field's declared type -> E0308 REJECT */
            if fi >= 0 && FGEN[fi as usize] == 1 { lwas = 1; lgat = 0 - 1;                /* std-hoist P0.1: this field's type is T -> its RHS kind IS the concrete type-arg kind */
                if EISSTR == 1 { lgak = 1; } else if EISSTRING == 1 { lgak = 3; } else if EISVEC == 1 { lgak = 2; } else { lgak = 0; lgat = EITYP;
                    if EWASGARG == 1 { lwas2 = 1; lgak2 = EGAK; lgat2 = EGAT; } } }   /* std-hoist P0.3: the T-value is itself a generic struct B<..> -> carry its type-arg kind one level */
            let cb = ec(FSB[fi as usize]);
            let ad = ebin("add", base, cb);
            if fi >= 0 && FI64[fi as usize] == 1 { println!("  store8 v{} v{}", ad, e); }   /* value-width: i64/u64 field -> 64-bit store */
            else { println!("  store v{} v{}", ad, e); }
            if NSETF < 1024 { SETF[NSETF as usize] = fi; NSETF = NSETF + 1; }
            skipws();
            if POS < LEN && at(POS) == 44 { POS = POS + 1; }
            }
            if POS == lp { break; }                                                /* R2av: no progress -> stop (avoids an infinite emit loop on a malformed/untyped field value) */
        }
        NSETF = setbase;
        EISSTR = 0; EISVEC = 0; EISSTRING = 0; EISARR = 0; EIS64 = 0; EISITER = 0; EISCHAR = 0; EISOPT = 0; EISBOOL = 0; EISU = 0; EISFLT = 0; EISCLO = 0; EIS8 = 0;   /* the value is a struct, not a field's container-kind (mouth-gate: the LAST field value must not leak its kind flag onto the struct-literal binding). LAYER6 P-STRUCT-LIT-CHARS-FIELD-LEAK: a last `chars: s.chars()` field left EISITER=1 (char-cursor) -> `let c = Cur{..chars}` bound c a char-cursor (SK=13), not the struct (SST) -> c.method() mis-dispatched / crashed. Was: only EISSTR/EISVEC/EISSTRING/EISARR/EIS64 cleared; extend to the full value-kind leak set (mirrors the enum-ctor tag-leak clear). */
        EITYP = ty;
        EGAK = lgak; EGAT = lgat; EWASGARG = lwas;   /* std-hoist P0.1: hand the recorded type-arg kind to the let-binding */
        EGAK2 = lgak2; EGAT2 = lgat2; EWASGARG2 = lwas2;   /* std-hoist P0.3: hand the nested (T-of-T) kind too */
        base
    }
}
/* emit `call Type_method [self] args...` (self_vn < 0 for an associated fn). pos is just past `(`. */
fn emit_enum_mcall(en: i32, mo: i32, ml: i32, self_vn: i32) -> i32 {   /* R2: `e.method(..)` on a user enum -> call <EnumName>_<method> */
    unsafe {
        let owner = 0 - (en + 2);                     /* std-hoist GAP1: enum-owner encoding used by the method registry */
        let rs = mret_find(owner, mo, ml);
        let dsgak = DSGAK; let dsgat = DSGAT;         /* preserve across arg parsing */
        let mut a0 = -1; let mut a1 = -1; let mut a2 = -1; let mut a3 = -1;
        let mut a4 = -1; let mut a5 = -1; let mut na = 0;
        skipws();
        if peek() != 41 {
            a0 = parse_expr(); na = 1;
            if eat_ch(44) { a1 = parse_expr(); na = 2;
            if eat_ch(44) { a2 = parse_expr(); na = 3;
            if eat_ch(44) { a3 = parse_expr(); na = 4;
            if eat_ch(44) { a4 = parse_expr(); na = 5;
            if eat_ch(44) { a5 = parse_expr(); na = 6; } } } } }
        }
        eat_ch(41);
        print!("  v{}: i32 = call ", NC);
        emit_span(ENO[en as usize], ENL[en as usize]); print!("_"); emit_span(mo, ml);
        print!(" v{}", self_vn);
        if na >= 1 { print!(" v{}", a0); }
        if na >= 2 { print!(" v{}", a1); }
        if na >= 3 { print!(" v{}", a2); }
        if na >= 4 { print!(" v{}", a3); }
        if na >= 5 { print!(" v{}", a4); }
        if na >= 6 { print!(" v{}", a5); }
        println!("");
        NC = NC + 1;
        if rs == 0 - 3 {                                 /* std-hoist GAP1: method returns the generic param T -> resolve to the enum value's recorded type-arg kind */
            if dsgak == 1 { EISSTR = 1; }
            else if dsgak == 3 { EISSTRING = 1; }
            else if dsgak == 2 { EISVEC = 1; }
            else if dsgat >= 0 { EITYP = dsgat; }
        } else { if rs >= 0 { EITYP = rs; } }
        if mret_enum(ty, mo, ml) >= 0 { EISENUM = 1; EIENUMTY = mret_enum(ty, mo, ml); }   /* LAYER5 P-METHOD-RETURNS-ENUM: a method `-> UserEnum`/`-> &UserEnum` -> tag the result an enum cell so `match p.peek() { Tok::.. }` dispatches (was untagged -> a scalar disc mis-read -> crash) */
        if mret_ref(ty, mo, ml) == 1 { EWASREF = 1; }   /* LAYER5 P-METHOD-RETURNS-SLICE-ENUM-REF: a method `-> &T` result is a REFERENCE -> a match payload binder is by-ref (`*n` a no-op); else `*n` derefs the scalar payload -> segfault (peek() -> &Tok). Mirrors the free-fn fnret_ref/EWASREF path. */
        if mret_bool(owner, mo, ml) == 1 { EISBOOL = 1; }   /* LAYER6 P-SLICE-ELEM-ENUM-METHOD: an ENUM method `-> bool` result carries EISBOOL (else `v[i].is_op()` Displays 1/0 not true/false). Mirrors emit_mcall's return tagging. */
        if mret_i64(owner, mo, ml) == 1 { EIS64 = 1; }   /* enum method `-> i64/u64` */
        { let rst = mret_str(owner, mo, ml); if rst == 3 { EISSTR = 1; EISSTRING = 1; } else if rst == 1 { EISSTR = 1; } }   /* enum method `-> String`/`-> &str` */
        NC - 1
    }
}
fn emit_mcall(ty: i32, mo: i32, ml: i32, self_vn: i32) -> i32 {
    unsafe {
        let bi = blanket_find(mo, ml);
        if bi >= 0 && ty >= 0 && BIPRIM[bi as usize] == 0 { reg_blanket_inst(bi, ty); }   /* R2: TRUE blanket method on a concrete struct -> monomorphize <ty>_<method>; a prim-concrete impl (BIPRIM!=0) never applies to a struct */
        let gimidx = gim_find(ty, mo, ml); let mut gimstyp = 0 - 99;                /* S3 R1: generic-impl method -> per-T monomorph when the receiver carries a non-scalar type-arg kind */
        if gimidx >= 0 { if DSGAK == 1 { gimstyp = 0 - 3; } else if DSGAK == 3 { gimstyp = 0 - 4; } else if DSGAT >= 0 { gimstyp = DSGAT; } }
        if gimstyp != 0 - 99 { reg_gim_inst(gimidx, gimstyp); }
        let rs = mret_find(ty, mo, ml);
        let mut a0 = -1; let mut a1 = -1; let mut a2 = -1; let mut a3 = -1;
        let mut a4 = -1; let mut a5 = -1; let mut na = 0;
        let mut cagk = 0; let mut cagt = 0 - 1; let mut cahit = 0;                     /* std-hoist P0.2: capture the first arg's concrete kind (the T-valued param of new/push) */
        let mut cagk2 = 0; let mut cagt2 = 0 - 1; let mut cahit2 = 0;                   /* std-hoist GAP2: capture the second arg's concrete kind (the V-valued param) */
        skipws();
        if peek() != 41 {
            a0 = parse_expr(); na = 1;
            if EISSTR == 1 { cagk = 1; cahit = 1; } else if EISSTRING == 1 { cagk = 3; cahit = 1; } else if EISVEC == 1 { cagk = 2; cahit = 1; } else if EITYP >= 0 { cagk = 0; cagt = EITYP; cahit = 1; }
            if eat_ch(44) { a1 = parse_expr(); na = 2;
            if EISSTR == 1 { cagk2 = 1; cahit2 = 1; } else if EISSTRING == 1 { cagk2 = 3; cahit2 = 1; } else if EISVEC == 1 { cagk2 = 2; cahit2 = 1; } else if EITYP >= 0 { cagk2 = 0; cagt2 = EITYP; cahit2 = 1; }
            if eat_ch(44) { a2 = parse_expr(); na = 3;
            if eat_ch(44) { a3 = parse_expr(); na = 4;
            if eat_ch(44) { a4 = parse_expr(); na = 5;
            if eat_ch(44) { a5 = parse_expr(); na = 6; } } } } }
        }
        eat_ch(41);
        print!("  v{}: i32 = call ", NC);
        emit_span(TSO[ty as usize], TSL[ty as usize]); print!("_"); emit_span(mo, ml);
        if gimstyp != 0 - 99 { print!("_"); if gimstyp == 0 - 3 { print!("str"); } else if gimstyp == 0 - 4 { print!("string"); } else { emit_span(TSO[gimstyp as usize], TSL[gimstyp as usize]); } }   /* S3 R1: dispatch to the per-T monomorph Type_method_<styp> */
        if self_vn >= 0 { print!(" v{}", self_vn); }
        if na >= 1 { print!(" v{}", a0); }
        if na >= 2 { print!(" v{}", a1); }
        if na >= 3 { print!(" v{}", a2); }
        if na >= 4 { print!(" v{}", a3); }
        if na >= 5 { print!(" v{}", a4); }
        if na >= 6 { print!(" v{}", a5); }
        println!("");
        NC = NC + 1;
        EISARR = 0; EISVEC = 0; EISCLO = 0; EISSTR = 0; EISSTRING = 0; EISCHAR = 0; EISBOOL = 0; EISOPT = 0; EIS64 = 0; EIS8 = 0; EISU = 0; EITYP = 0 - 1; EISBYTES = 0; EISHM = 0; EISSET = 0; EISVDQ = 0; EISENUM = 0; EIENUMTY = 0 - 1;   /* LAYER5 P-ASSOC-FN-STRUCT-RETURN-FIELDKIND: a Vec/&[T]/&str/closure ARG leaves its value-kind flag set (EISVEC/EISSTR/..); clear them before the return-kind is set from the method's OWN return type below -- else `let p = Type::new(vec![..])` captured the arg's EISVEC and bound `p` as a Vec (not the struct), so `p.field` mis-dispatched (`p.t.len()` -> undefined `call len`). Mirrors emit_call's arg-leak clear (the free-fn path already did this, which is why a free fn returning the struct worked). */
        if rs == 0 - 3 {                                 /* std-hoist P0.1: method returns the generic param T -> resolve to the self value's recorded type-arg kind */
            if DSGAK == 1 { EISSTR = 1; }
            else if DSGAK == 3 { EISSTRING = 1; }
            else if DSGAK == 2 { EISVEC = 1; }
            else if DSGAT >= 0 { EITYP = DSGAT; }
        } else { if rs >= 0 { EITYP = rs; } }
        if rs == ty && ty >= 0 && TGPL[ty as usize] > 0 && (cahit == 1 || cahit2 == 1) { EGAK = cagk; EGAT = cagt; EWASGARG = 1;   /* std-hoist P0.2: a constructor/method returning the generic struct B<T> hands the T-valued arg's kind to the let-binding (GAP2: fire even if only the 2nd arg is non-scalar) */
            if TGPL2[ty as usize] > 0 && cahit2 == 1 { EGAK2 = cagk2; EGAT2 = cagt2; EWASGARG2 = 1; } }   /* std-hoist GAP2: also hand the 2nd type-param (V) kind via the *2 channel */
        if mret_bool(ty, mo, ml) == 1 { EISBOOL = 1; }   /* R2bv: `-> bool` method result prints true/false */
        if mret_i64(ty, mo, ml) == 1 { EIS64 = 1; }   /* value-width: `-> i64/u64` method result carries EIS64 */
        { let rst = mret_str(ty, mo, ml); if rst == 3 { EISSTR = 1; EISSTRING = 1; } else if rst == 1 { EISSTR = 1; } else if rst == 6 { EISCHAR = 1; EISITER = 0; } }   /* S1 L1: `-> String`/`-> &str` method result carries the string kind (else Displays as a pointer); LAYER4: `-> char` (code 6) -> EISCHAR (Displays as the char) + clear EISITER (a char VALUE, not a cursor -- mirrors R28 so `let x = c.first()` isn't bound SK=13) */
        if mret_void(ty, mo, ml) == 1 { EISUNIT = 1; }   /* Wave-2 bug 3: a void method-call as a block/fn tail must be discarded, not `ret`ed */
        if mret_enum(ty, mo, ml) >= 0 { EISENUM = 1; EIENUMTY = mret_enum(ty, mo, ml); }   /* LAYER5 P-METHOD-RETURNS-ENUM: a struct method `-> UserEnum`/`-> &UserEnum` -> tag the result an enum cell so `match p.peek() { Tok::.. }` dispatches */
        if mret_ref(ty, mo, ml) == 1 { EWASREF = 1; }   /* LAYER5 P-METHOD-RETURNS-SLICE-ENUM-REF: a struct method `-> &T` result is a REFERENCE -> a match payload binder is by-ref (`*n` a no-op); else `*n` derefs the scalar payload -> segfault (`fn peek(&self)->&Tok{&self.toks[self.pos]}`). Mirrors the free-fn fnret_ref/EWASREF path. */
        if mret_vec(ty, mo, ml) == 1 { EISVEC = 1; skipws(); if POS < LEN && (at(POS) == 46 || at(POS) == 91) { return dispatch_vec(NC - 1); } return NC - 1; }   /* LAYER7 P-METHOD-RETURNS-VEC: a struct method `-> Vec<T>`/`-> ThinVec<T>` -> tag the result EISVEC (a bound `let v = c.parse()` gets SK=2 + SVELT/SVELENUM from mret_vec's DVELT/DVELENUM side-effect); a chained `.len()/.iter()/[i]` dispatches the Vec (was untagged -> garbage / for-iter parse-drift). */
        NC - 1
    }
}
/* A: operator overloading -- if struct type `aty` has a method named by bytes (oa,ob,oc) (add/sub/mul/div/rem),
   emit `Type_method self other` on the two already-parsed operands and return the call vN (else -1). */
fn op_method(aty: i32, a: i32, b: i32, oa: i32, ob: i32, oc: i32) -> i32 {
    unsafe {
        let mut k = 0;
        while k < NMI {
            if MITY[k as usize] == aty && MIML[k as usize] == 3 && at(MIMO[k as usize]) == oa && at(MIMO[k as usize] + 1) == ob && at(MIMO[k as usize] + 2) == oc {
                let mo = MIMO[k as usize]; let ml = MIML[k as usize];
                let rs = mret_find(aty, mo, ml);
                print!("  v{}: i32 = call ", NC);
                emit_span(TSO[aty as usize], TSL[aty as usize]); print!("_"); emit_span(mo, ml);
                println!(" v{} v{}", a, b);
                NC = NC + 1;
                if rs >= 0 { EITYP = rs; } else { EITYP = aty; }
                return NC - 1;
            }
            k = k + 1;
        }
        -1
    }
}
/* R2cl: unary operator-overload dispatch -- one operand (e.g. `-x` -> Type_neg(x)). */
fn op_method1(aty: i32, a: i32, oa: i32, ob: i32, oc: i32) -> i32 {
    unsafe {
        let mut k = 0;
        while k < NMI {
            if MITY[k as usize] == aty && MIML[k as usize] == 3 && at(MIMO[k as usize]) == oa && at(MIMO[k as usize] + 1) == ob && at(MIMO[k as usize] + 2) == oc {
                let mo = MIMO[k as usize]; let ml = MIML[k as usize];
                let rs = mret_find(aty, mo, ml);
                print!("  v{}: i32 = call ", NC);
                emit_span(TSO[aty as usize], TSL[aty as usize]); print!("_"); emit_span(mo, ml);
                println!(" v{}", a);
                NC = NC + 1;
                if rs >= 0 { EITYP = rs; } else { EITYP = aty; }
                return NC - 1;
            }
            k = k + 1;
        }
        -1
    }
}
/* dispatch `.field`* then optional `.method(args)` on a struct base `bv` of type `ty`. */
fn is_iter_ty(ty: i32) -> i32 { unsafe { let mut i = 0; while i < NITER { if ITERTY[i as usize] == ty { return 1; } i = i + 1; } 0 } }
fn emit_iter_materialize(bv: i32, ty: i32) -> i32 {   /* TRAIT R5: drive a user `impl Iterator` -- loop next(&mut self) until None, collecting payloads into a Vec */
    unsafe {
        let nd = emit_vec_new();
        println!("  loop"); ckreset();
        print!("  v{}: i32 = call ", NC); emit_span(TSO[ty as usize], TSL[ty as usize]); print!("_next v{}", bv); println!("");
        let opt = NC; NC = NC + 1;
        let disc = eload(opt); println!("  brz v{}", disc); ckreset();   /* None -> exit */
        let c4 = ecr(4); let payload = eload(ebin("add", opt, c4));
        emit_vec_push(nd, payload);
        println!("  cont"); ckreset(); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        EISVEC = 1; nd
    }
}
/* LAYER4: `std::iter::from_fn(closure)` -- materialize the lazy closure-driven iterator into an eager Vec: call the
   0-arg closure repeatedly (calli code env), each result is an Option cell [disc, payload]; push each Some payload
   until None. Mirrors emit_iter_materialize but drives a closure, not a user Iterator's next(). rustc_lexer tokenize. */
fn emit_from_fn_iter(fv: i32) -> i32 {
    unsafe {
        let c4c = ecr(4); let code = eload(fv); let env = eload(ebin("add", fv, c4c));
        let nd = emit_vec_new();
        println!("  loop"); ckreset();
        println!("  v{}: i32 = calli v{} v{}", NC, code, env); let opt = NC; NC = NC + 1;
        let disc = eload(opt); println!("  brz v{}", disc); ckreset();   /* None -> exit */
        let c4 = ecr(4); let payload = eload(ebin("add", opt, c4));
        emit_vec_push(nd, payload);
        println!("  cont"); ckreset(); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        EISVEC = 1; nd
    }
}
fn dispatch_struct(bv: i32, ty: i32) -> i32 {
    unsafe {
        if is_iter_ty(ty) == 1 {                                                     /* TRAIT R5: an iterator adapter (sum/filter/map/..) on a user Iterator -> materialize into a Vec, dispatch the Vec chain */
            skipws();
            if POS < LEN && at(POS) == 46 {
                let sv = POS; POS = POS + 1; skipws(); rd_ident(); let mo = ID_O; let ml = ID_L; skipws();
                if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } skipws(); }   /* skip a `sum::<T>()` turbofish before the arg list */
                if fld_find(ty, mo, ml) < 0 && method_exists(ty, mo, ml) == 0 && POS < LEN && at(POS) == 40 { POS = sv; let nd = emit_iter_materialize(bv, ty); return dispatch_vec(nd); }   /* an adapter CALL, not a field or an inherent method */
                POS = sv;
            }
        }
        let mut cur = bv; let mut cty = ty;
        loop {
            skipws();
            if POS >= LEN || at(POS) != 46 { break; }   /* not '.' */
            let save = POS; POS = POS + 1; skipws();
            if POS < LEN && is_dig(at(POS)) {                                    /* A: tuple-struct field `.N` -> field at offset N*4 (self.0 in a method, s.1 on a local) */
                let mut n = 0; while POS < LEN && is_dig(at(POS)) { n = n * 10 + (at(POS) - 48); POS = POS + 1; }
                let fi = fld_nth(cty, n);
                let cb = ec(n * 4); let ad = ebin("add", cur, cb); let v = eload(ad);
                let mut fty = -1; let mut fk = 0;
                if fi >= 0 { fty = FSTY[fi as usize]; fk = FFK[fi as usize]; }
                if fi >= 0 && FGEN[fi as usize] == 1 && (DSGAK >= 1 || DSGAT >= 0) {   /* std-hoist P0.1: tuple field of generic type T */
                    if DSGAK == 1 || DSGAK == 3 { skipws(); let ism = POS < LEN && at(POS) == 46; let r = dispatch_string(v); if ism == false { if DSGAK == 3 { EISSTRING = 1; } else { EISSTR = 1; } } return r; }
                    if DSGAK == 2 { return dispatch_vec(v); }
                    if DSGAT >= 0 { cur = v; cty = DSGAT; DSGAK = DSGAK2; DSGAT = DSGAT2; DSGAK2 = 0 - 1; DSGAT2 = 0 - 1; continue; }   /* std-hoist P0.3: descend into the nested generic value carrying its own type-arg kind */
                }
                if fi >= 0 && FGEN[fi as usize] == 2 && (DSGAK2 >= 1 || DSGAT2 >= 0) {   /* std-hoist GAP2: tuple field of the 2nd generic param V -> resolve via the *2 channel */
                    if DSGAK2 == 1 || DSGAK2 == 3 { skipws(); let ism = POS < LEN && at(POS) == 46; let r = dispatch_string(v); if ism == false { if DSGAK2 == 3 { EISSTRING = 1; } else { EISSTR = 1; } } return r; }
                    if DSGAK2 == 2 { return dispatch_vec(v); }
                    if DSGAT2 >= 0 { cur = v; cty = DSGAT2; DSGAK = 0 - 1; DSGAT = 0 - 1; DSGAK2 = 0 - 1; DSGAT2 = 0 - 1; continue; }
                }
                if fk == 3 || fk == 1 { return dispatch_string(v); }
                if fk == 8 { skipws(); if POS < LEN && (at(POS) == 46 || at(POS) == 63) { EISOPT = 1; EOPTK = 0; EOPTREF = 0; EOPTRES = 0; return opt_postfix(v); } return v; }   /* P-STRUCT-FIELD-OPTION (tuple .N): Option<T> tuple-struct field -> route `.is_some()/.is_none()/.unwrap()/?` through opt_postfix (EISOPT set only when a method/`?` follows). */
                if fk == 2 { DVELT = 0 - 1; VELT = 0 - 1; DVELTY = 0 - 1; if fi >= 0 { let fen = enum_find(FVELO[fi as usize], FVELL[fi as usize]); DVELENUM = fen; VELENUM = fen; let fst = ty_find(FVELO[fi as usize], FVELL[fi as usize]); if fst >= 0 { DVELT = fst; VELT = fst; } else if is_string_ty(FVELO[fi as usize], FVELL[fi as usize]) { DVELTY = 3; } else if FVELL[fi as usize] == 3 && at(FVELO[fi as usize]) == 115 && at(FVELO[fi as usize] + 1) == 116 && at(FVELO[fi as usize] + 2) == 114 { DVELTY = 2; } else if FVELL[fi as usize] == 7 && at(FVELO[fi as usize]) == 72 && at(FVELO[fi as usize] + 1) == 97 && at(FVELO[fi as usize] + 2) == 115 && at(FVELO[fi as usize] + 3) == 104 && at(FVELO[fi as usize] + 4) == 77 { DVELT = 0 - 4; } } return dispatch_vec(v); }   /* D006 P-VEC-HASHMAP-ELEM (field): a `Vec<HashMap>` FIELD element -> DVELT sentinel -4 so `self.scopes[i].get/insert/len` dispatch through dispatch_hashmap */   /* LAYER8 P-VEC-STRING-ELEM-FIELD: Vec<String>(DVELTY=3,TYTAG4)/Vec<&str>(DVELTY=2,TYTAG3) field element Ty threading -> r.names[i]/.iter() bind String/&str-kind (was raw descriptor -> .as_str()/.len()/== mis-dispatch). LAYER7 P-VEC-STRUCT-ELEM (struct FIELD): a `Vec<Struct>` field -> resolve the element STRUCT-type via ty_find (mirrors the enum_find branch) so `for s in w.segs.iter() { s.field }` / `w.segs[i].field` bind the element struct-typed (was: only enum elements resolved -> a struct-element loop var stayed untagged -> `.field` read garbage). */
                if fty >= 0 { cur = v; cty = fty; } else { EITYP = -1; return v; }
                continue;
            }
            rd_ident();
            let mo = ID_O; let ml = ID_L; skipws();
            if POS < LEN && at(POS) == 40 {
                if ml == 5 && at(mo) == 99 && at(mo + 1) == 108 && at(mo + 2) == 111 && at(mo + 3) == 110 && at(mo + 4) == 101 {   /* A4: .clone() -> copy the struct cell */
                    POS = POS + 1; eat_ch(41);
                    let nc = emit_struct_clone(cur, cty);
                    EITYP = cty; return nc;
                }
                if cty >= 0 && method_exists(cty, mo, ml) == 0 { die_nomethod(mo, ml); }   /* R2bf: known struct, genuine `.method(` that no impl/default/blanket defines -> E0599 REJECT */
                let mrk = mret_str(cty, mo, ml);   /* method-? sub-arc: capture Result(4)/Option(5) return kind BEFORE cty is mutated by a chained continue */
                POS = POS + 1; let r = emit_mcall(cty, mo, ml, cur);
                if EITYP >= 0 { cur = r; cty = EITYP; continue; }   /* R2: method returned a struct (e.g. `-> &mut Self`) -> keep dispatching for a chained call */
                if EISENUM == 1 && EIENUMTY >= 0 { skipws(); if POS < LEN && at(POS) == 46 { let een = EIENUMTY; let mut q = POS + 1; let cmo = q; while q < LEN && is_idc(at(q)) { q = q + 1; } let cml = q - cmo; let mut rr = q; while rr < LEN && at(rr) == 32 { rr = rr + 1; } if cml > 0 && rr < LEN && at(rr) == 40 && method_exists(0 - (een + 2), cmo, cml) == 1 { POS = rr + 1; return emit_enum_mcall(een, cmo, cml, r); } } return r; }   /* PARSE-5 P-METHOD-RETURNS-BOX-ENUM-CHAIN: a `-> Box<Enum>`/`-> Enum` method result with a chained real enum-method (`p.parse_expr().eval()`) -> byte-scan the method name (R17-safe, mirrors the slice-elem-enum path at emit-4258) + emit_enum_mcall on the enum owner. Was: EITYP<0 for an enum -> fell to opt_postfix -> the enum cell mis-dispatched -> garbage. The bind-then-call form (`let e=p.mk(); e.eval()`) already worked via the local's SENUM tag. */
                if mrk == 4 || mrk == 5 { EISOPT = 1; if mrk == 4 { EOPTRES = 1; } else { EOPTRES = 0; } if mrk == 5 { EOPTK = mret_optk(cty, mo, ml); if mret_optref(cty, mo, ml) == 1 { EOPTREF = 1; } } let men2 = mret_optenum(cty, mo, ml); if men2 >= 0 { EOPTTY = ty_intern(13, 0, men2); } skipws(); if POS < LEN && (at(POS) == 63 || at(POS) == 46) { return opt_postfix(r); } return r; }   /* D005 P-PRESULT-BOX-ENUM-PAYLOAD-BINDER (method arm): a `-> Result/Option<Box<Enum>>` method result -> carry the payload ENUM Ty (tag 13) so `match p.parse(){Ok(e)=>e.eval()}` tags e SENUM (the method dual of fnret_optenum). */   /* SPAN-4 P-OPTION-PAYLOAD-DISPLAY-KIND (method arm): set EOPTK from the DECLARED payload so a `match p.m(){Some(x)=>..}` binder Displays x as str/char per the return type (the free-fn dual sets EOPTK via fnret_optk). */   /* method-? sub-arc: `-> Result`/`-> Option` method result -> flag EISOPT + EOPTRES; a trailing `?` OR `.unwrap()/.unwrap_or()/.map()/.is_some()` routes through opt_postfix (LAYER4: added the `.` chain -- was only `?`, so `self.bump().unwrap_or(d)` returned the raw Option CELL pointer, not the extracted payload -> garbage; rustc_lexer Cursor.bump().unwrap_or) */
                if mrk == 1 || mrk == 3 { skipws(); if POS < LEN && (at(POS) == 46 || at(POS) == 91) { return dispatch_string(r); } return r; }   /* LAYER6 P-METHOD-RETURNS-STR-CHAIN: a `-> &str` (mrk 1) / `-> String` (mrk 3) method result -> a chained `.len()/.is_empty()/[i]/.chars()/.starts_with()/..` routes through dispatch_string (the &str/String dispatcher), NOT opt_postfix (which mis-handled `.len()` on a &str -> Displayed the string itself). Mirrors the `self.chars.as_str()` str-chain routing + the free-fn `-> &str` path (which already worked). */
                skipws(); if POS < LEN && at(POS) == 46 { return opt_postfix(r); }   /* LAYER4 R25 (cascade-doc method-return MEET): a NON-struct method result (char/scalar/bool) must CHAIN a following `.method()` postfix -- mirrors the scalar-FIELD path (`.b.count_ones()`). Was: `return r` dropped the postfix, so `self.first().is_ascii_digit()` in an if-cond under-parsed to `self.first()` -> parse_block ran on the leftover `.is_ascii_digit() {..}` -> drift (rustc_lexer number). Cascade-revives ALL `self.m().is_XXX()`/`.method()` on a scalar-returning method. */
                return r;
            }  /* method */
            let fi = fld_find(cty, mo, ml);
            if fi < 0 && cty >= 0 && ml >= 1 { die_nofield(mo, ml); }   /* R2ba: known struct, genuine `.field` (not a method -- `(` handled above -- and not an empty `..` range dot), field missing -> E0609 REJECT */
            if fi < 0 { POS = save; break; }
            let cb = ec(FSB[fi as usize]); let ad = ebin("add", cur, cb);
            let fty = FSTY[fi as usize];
            let fk = FFK[fi as usize];
            let mut v = 0;
            if FI64[fi as usize] == 1 { v = eload8(ad); } else { v = eload(ad); }   /* value-width: i64/u64 field -> 64-bit load */
            if FGEN[fi as usize] == 1 && GIM_ACT == 1 && DSGAK < 1 && DSGAT < 0 { if GSUB_STYP == 0 - 3 { DSGAK = 1; } else if GSUB_STYP == 0 - 4 { DSGAK = 3; } else if GSUB_STYP >= 0 { DSGAT = GSUB_STYP; } }   /* S3 R1: inside a generic-impl monomorph, self.v (field of type T) resolves via the substituted T (GSUB_STYP) */
            if FGEN[fi as usize] == 1 && (DSGAK >= 1 || DSGAT >= 0) {              /* std-hoist P0.1: field of generic type T -> resolve to the value's recorded type-arg kind */
                if DSGAK == 1 || DSGAK == 3 { skipws(); let ism = POS < LEN && at(POS) == 46; let r = dispatch_string(v); if ism == false { if DSGAK == 3 { EISSTRING = 1; } else { EISSTR = 1; } } return r; }
                if DSGAK == 2 { return dispatch_vec(v); }
                if DSGAT >= 0 { cur = v; cty = DSGAT; DSGAK = DSGAK2; DSGAT = DSGAT2; DSGAK2 = 0 - 1; DSGAT2 = 0 - 1; continue; }   /* std-hoist P0.3: descend into the nested generic value carrying its own type-arg kind */
            }
            if FGEN[fi as usize] == 2 && (DSGAK2 >= 1 || DSGAT2 >= 0) {              /* std-hoist GAP2: field of the 2nd generic param V -> resolve via the *2 channel */
                if DSGAK2 == 1 || DSGAK2 == 3 { skipws(); let ism = POS < LEN && at(POS) == 46; let r = dispatch_string(v); if ism == false { if DSGAK2 == 3 { EISSTRING = 1; } else { EISSTR = 1; } } return r; }
                if DSGAK2 == 2 { return dispatch_vec(v); }
                if DSGAT2 >= 0 { cur = v; cty = DSGAT2; DSGAK = 0 - 1; DSGAT = 0 - 1; DSGAK2 = 0 - 1; DSGAT2 = 0 - 1; continue; }
            }
            if fk == 5 { EISCLO = 1; return v; }                                 /* R10: closure field -> the {code,env} cell; a wrapping `(s.f)` routes the trailing `(args)` to emit_closure_call */
            if fk == 13 { return dispatch_char_iter(v); }                        /* LAYER4 P-CHARS-ITER-CURSOR-FIELD: a `chars: Chars<'a>` STRUCT FIELD holds a char-cursor cell [buf,len,pos] -> route `self.chars.next()/clone()/as_str()` to dispatch_char_iter operating on the field's cell (a `&mut self` .next() advances the shared cell in place, so the position persists in the struct). Was FFK=0 -> scalar field -> `.next()` mis-dispatched -> the cursor never advanced -> infinite loop/crash. rustc_lexer Cursor{chars}. */
            if fk == 8 { skipws(); if POS < LEN && (at(POS) == 46 || at(POS) == 63) { EISOPT = 1; EOPTK = 0; EOPTREF = 0; EOPTRES = 0; return opt_postfix(v); } return v; }   /* P-STRUCT-FIELD-OPTION: an Option<T> named field -> route a trailing `.is_some()/.is_none()/.unwrap()/?` through opt_postfix. EISOPT set ONLY when a method/`?` follows (a bare read returns the cell untagged, no leak). Was FFK=0 -> die_nomethod. */
            if fk == 3 || fk == 1 { return dispatch_string(v); }                 /* R7/P: String or &str field -> .push_str/.len (prints as str) */
            if fk == 6 { return dispatch_bytes(v); }                             /* BVK N3: Vec<u8> field -> byte-slice dispatch (p.s[i]/.len()/for x in &p.s stride-1) */
            if fk == 7 { EISHM = 1; return dispatch_hashmap(v); }                 /* LAYER7 P-HASHMAP-FIELD: a `HashMap`/`BTreeMap` STRUCT FIELD -> route self.m.get/insert/contains_key/len to dispatch_hashmap (was FFK=0 scalar -> mis-dispatch). Mirrors sk==7 HashMap local. */
            if fk == 2 { DVELT = 0 - 1; VELT = 0 - 1; DVELTY = 0 - 1;   /* LAYER8 P-VEC-STRING-ELEM-FIELD-RESET: clear the element channels first -- a NESTED `r.ribs[i].names` leaves DVELT/DVELTY stale from the OUTER Vec<Rib> index, so the inner Vec<String> field mis-tagged (`.names.len()` printed the descriptor as a String). Set exactly one channel below. */ let fen = enum_find(FVELO[fi as usize], FVELL[fi as usize]); DVELENUM = fen; VELENUM = fen; let fst = ty_find(FVELO[fi as usize], FVELL[fi as usize]); if fst >= 0 { DVELT = fst; VELT = fst; } else if is_string_ty(FVELO[fi as usize], FVELL[fi as usize]) { DVELTY = 3; } else if FVELL[fi as usize] == 3 && at(FVELO[fi as usize]) == 115 && at(FVELO[fi as usize] + 1) == 116 && at(FVELO[fi as usize] + 2) == 114 { DVELTY = 2; } else if FVELL[fi as usize] == 7 && at(FVELO[fi as usize]) == 72 && at(FVELO[fi as usize] + 1) == 97 && at(FVELO[fi as usize] + 2) == 115 && at(FVELO[fi as usize] + 3) == 104 && at(FVELO[fi as usize] + 4) == 77 { DVELT = 0 - 4; } return dispatch_vec(v); }   /* D006 P-VEC-HASHMAP-ELEM (field, 2nd dispatch site): a `Vec<HashMap>` FIELD element -> DVELT sentinel -4 so `self.scopes[i].get/insert/len` dispatch through dispatch_hashmap. LAYER8 P-VEC-STRING-ELEM-FIELD (String/&str element Ty). R7: Vec field; LAYER6 P-SLICE-ELEM-ENUM: thread element enum-type; LAYER7 P-VEC-STRUCT-ELEM: also thread element STRUCT-type via ty_find so `for s in w.segs.iter() { s.field }` binds struct-typed */
            if fk == 4 {                                                         /* aggregate element-address: `.d[i]` -> index the heap array (v = the field's pointer, at base+FSB so ANY field offset works). Stride matches struct-literal storage (4-byte). */
                skipws();
                if POS < LEN && at(POS) == 91 {
                    let mut cur2 = v;
                    while POS < LEN && at(POS) == 91 { POS = POS + 1; let iv = parse_expr(); eat_ch(93); let c4 = ecr(4); let off2 = ebin("mul", iv, c4); let ad2 = ebin("add", cur2, off2); cur2 = eload(ad2); }
                    return cur2;
                }
                return v;                                                        /* bare `.d` -> array base pointer */
            }
            if fty < 0 && FSENL[fi as usize] > 0 { let fen = enum_find(FSEN[fi as usize], FSENL[fi as usize]);   /* RES-3 P-STRUCT-ENUM-FIELD-READ: a NON-struct field whose type is an enum (lazy enum_find on the stored type-name span) -> tag EISENUM so `{:?}`/a let-binding treat `s.e` as an enum (was: fell to the scalar branch -> printed the cell pointer). A chained real enum-method (`s.e.method()`) dispatches via emit_enum_mcall (byte-scan R17-safe, mirrors the slice-elem-enum + method-chain paths). */
                if fen >= 0 { EISENUM = 1; EIENUMTY = fen; skipws();
                    if POS < LEN && at(POS) == 46 { let mut q = POS + 1; let cmo = q; while q < LEN && is_idc(at(q)) { q = q + 1; } let cml = q - cmo; let mut rr = q; while rr < LEN && at(rr) == 32 { rr = rr + 1; } if cml > 0 && rr < LEN && at(rr) == 40 && method_exists(0 - (fen + 2), cmo, cml) == 1 { POS = rr + 1; return emit_enum_mcall(fen, cmo, cml, v); } }
                    return v; } }
            if fty >= 0 { cur = v; cty = fty; } else { EITYP = -1; if FI64[fi as usize] == 1 { EIS64 = 1; } if FBOOL[fi as usize] == 1 { EISBOOL = 1; } skipws(); if POS < LEN && at(POS) == 46 { return opt_postfix(v); } return v; }   /* scalar field -> done; value-width: i64/u64 field carries EIS64; LAYER5 P-STRUCT-BOOL-FIELD: bool field carries EISBOOL (Display true/false); a trailing `.method()` (e.g. s.b.count_ones()) routes through opt_postfix */
        }
        EITYP = cty; cur
    }
}
/* long-tail: dyn trait object = fat pointer [data@0, type-index@4]. b.method(args) -> runtime type-tag switch
   calling the concrete Type_method (registry MITY/MIMO/MIML built in parse_impl). */
fn dispatch_dyn(b: i32) -> i32 {
    unsafe {
        skipws();
        if POS < LEN && at(POS) == 46 {
            POS = POS + 1; rd_ident(); let mo = ID_O; let ml = ID_L;
            skipws(); eat_ch(40);
            let mut av0 = 0 - 1; let mut av1 = 0 - 1; let mut av2 = 0 - 1; let mut na = 0;
            skipws();
            if peek() != 41 { av0 = parse_expr(); na = 1; if eat_ch(44) { av1 = parse_expr(); na = 2; if eat_ch(44) { av2 = parse_expr(); na = 3; } } }
            eat_ch(41);
            let ptr = eload(b); let c4 = ecr(4); let tag = eload(ebin("add", b, c4));
            let res = NC; NC = NC + 1;
            let mut k = 0;
            while k < NMI {
                if span_eq_at(MIMO[k as usize], MIML[k as usize], mo, ml) {
                    let mty = MITY[k as usize]; let tc = ec(mty); let m = ebin("eq", tag, tc);
                    println!("  brz v{}", m); ckreset();
                    print!("  v{}: i32 = call ", NC); emit_span(TSO[mty as usize], TSL[mty as usize]); print!("_"); emit_span(mo, ml); print!(" v{}", ptr);
                    if na >= 1 { print!(" v{}", av0); } if na >= 2 { print!(" v{}", av1); } if na >= 3 { print!(" v{}", av2); }
                    println!(""); let cv = NC; NC = NC + 1; ecopy(res, cv);
                    println!("  pool"); ckreset();
                }
                k = k + 1;
            }
            return res;
        }
        b
    }
}
/* walk `.field`* from struct base bv/ty, returning the ADDRESS of the LAST field (assignment target). */
fn field_lvalue(bv: i32, ty: i32) -> i32 {
    unsafe {
        let mut cur = bv; let mut cty = ty;
        loop {
            skipws();
            if POS >= LEN || at(POS) != 46 { return cur; }
            POS = POS + 1; skipws(); rd_ident();
            let fi = fld_find(cty, ID_O, ID_L);
            let cb = ec(FSB[fi as usize]); let ad = ebin("add", cur, cb);
            skipws();
            if POS < LEN && at(POS) == 91 && (FFK[fi as usize] == 2 || FFK[fi as usize] == 4) {   /* LAYER9 P-FIELD-VEC-ELEM-LVALUE: a Vec/array FIELD element store target `self.table[i] = e` / `c.d[i] = e`. Was: field_lvalue ignored the trailing `[i]` and returned the field-SLOT address, so the store clobbered the Vec descriptor pointer (or the array-buffer pointer) -> `.len()` read 0, subsequent access segfaulted. Resolve the element address here: a Vec field slot holds the descriptor pointer (eload -> desc, eload -> buf); an array field slot holds the buffer pointer directly (one eload). */
                let mut base = eload(ad); if FFK[fi as usize] == 2 { base = eload(base); }
                POS = POS + 1; let iv = parse_expr(); eat_ch(93); skipws();
                let c4 = ecr(4); let mut ea = ebin("add", base, ebin("mul", iv, c4));
                while POS < LEN && at(POS) == 91 { let idesc = eload(ea); let ibuf = eload(idesc); POS = POS + 1; let jv = parse_expr(); eat_ch(93); skipws(); ea = ebin("add", ibuf, ebin("mul", jv, c4)); }   /* chained self.field[i][j] over a nested Vec<Vec<..>> element */
                FLV_I64 = 0; return ea;
            }
            if POS >= LEN || at(POS) != 46 { FLV_I64 = FI64[fi as usize]; return ad; }   /* value-width: expose the target field's i64/u64-ness for a 64-bit op=/store */
            cur = eload(ad); cty = FSTY[fi as usize];
        }
        cur
    }
}
/* R4: Vec<i32> = an 8-byte heap descriptor [buf-ptr@0, len@4] with a fixed 1024-elem backing buffer (no
   realloc). is_vec_ty/is_vec_macro detect the `Vec`/`vec!` spellings; emit_vec_new builds a descriptor. */
fn is_vec_ty(o: i32, l: i32) -> bool { unsafe { (l == 3 && at(o) == 86 && at(o + 1) == 101 && at(o + 2) == 99) || (TVALL > 0 && span_eq_at(TVALO, TVALL, o, l)) } }   /* Vec OR a recorded `type NAME<T>=Vec<T>` alias (ThinVec) */
fn is_option_ty(o: i32, l: i32) -> bool { unsafe { l == 6 && at(o) == 79 && at(o + 1) == 112 && at(o + 2) == 116 && at(o + 3) == 105 && at(o + 4) == 111 && at(o + 5) == 110 } }   /* P-STRUCT-FIELD-OPTION: `Option` type name -> Option-typed field (FFK=8) so `self.f.is_some()/.is_none()/.unwrap()/?` dispatch through opt_postfix (rustc newtype `S(Option<T>)` + Option fields). */
fn is_vec_macro(o: i32, l: i32) -> bool { unsafe { l == 3 && at(o) == 118 && at(o + 1) == 101 && at(o + 2) == 99 } }/* vec */
fn emit_vec_new() -> i32 {
    unsafe {
        let d = halloc(12);                      /* VEC-GROW: {buf@0,len@4,cap@8} (was 8) */
        let buf = halloc(4096);                  /* initial 1024-elem backing buffer */
        println!("  store v{} v{}", d, buf);
        let c8 = ecr(8); let cad = ebin("add", d, c8); let cap = ecr(1024);
        println!("  store v{} v{}", cad, cap);   /* cap@8 = 1024 initial element capacity */
        EISVEC = 1;
        d
    }
}
/* std-hoist P4.1: route the ALLOCATING/MUTATING Vec ops to the std RVec_* fns (present iff HAS_RVEC==1).
   RVec layout {buf@0,len@4,cap@8} -- its first 8 bytes ARE a valid built-in [buf@0,len@4] descriptor, so
   the READ ops (.len/[i]/.iter/adapters) keep working via dispatch_vec on the returned pointer. */
fn emit_rvec_new() -> i32 {
    unsafe {
        print!("  v{}: i32 = call RVec_new", NC); println!("");
        let d = NC; NC = NC + 1; EISVEC = 1;
        d
    }
}
fn emit_rvec_push(dv: i32, e: i32) {
    unsafe {
        print!("  v{}: i32 = call RVec_push v{} v{}", NC, dv, e); println!("");
        NC = NC + 1;
    }
}
/* std-hoist P4.2: route Vec iterator ADAPTERS to the std iter library (free generic fns over RVec, present
   iff HAS_RVEC==1). An RVec IS a [ptr,len] descriptor at its first 8 bytes, so `dv` (the descriptor cell) is a
   valid &RVec arg. The closure `|x| E` is parsed via parse_closure (a real lifted fat-ptr) with CLO_FORREF set
   so its param is a value-passed for-ref (`*x` a no-op) -- matching the library's `f(v.get(i))` value call. Only
   scalar-i32 element chains (dvelt0 == -1) route; struct/&str/Vec elements fall back to the built-in inline path. */
fn emit_rvec_map(dv: i32) -> i32 {
    unsafe {
        skipws(); eat_ch(40); CLO_FORREF = 1; let clo = parse_closure(); eat_ch(41);
        print!("  v{}: i32 = call rvec_map v{} v{}", NC, dv, clo); println!("");
        let d = NC; NC = NC + 1; EISCLO = 0; EISVEC = 1;
        d
    }
}
fn emit_rvec_filter(dv: i32) -> i32 {
    unsafe {
        skipws(); eat_ch(40); CLO_FORREF = 1; let clo = parse_closure(); eat_ch(41);
        print!("  v{}: i32 = call rvec_filter v{} v{}", NC, dv, clo); println!("");
        let d = NC; NC = NC + 1; EISCLO = 0; EISVEC = 1;
        d
    }
}
fn emit_rvec_sum(dv: i32) -> i32 {
    unsafe {
        print!("  v{}: i32 = call rvec_sum v{}", NC, dv); println!("");
        let d = NC; NC = NC + 1; EISVEC = 0;
        d
    }
}
fn emit_rvec_count(dv: i32) -> i32 {
    unsafe {
        print!("  v{}: i32 = call rvec_count v{}", NC, dv); println!("");
        let d = NC; NC = NC + 1; EISVEC = 0;
        d
    }
}
fn emit_rvec_fold(dv: i32) -> i32 {
    unsafe {
        skipws(); eat_ch(40); let init = parse_expr(); skipws(); eat_ch(44);
        CLO_FORREF = 1; let clo = parse_closure(); eat_ch(41);
        print!("  v{}: i32 = call rvec_fold v{} v{} v{}", NC, dv, init, clo); println!("");
        let d = NC; NC = NC + 1; EISCLO = 0; EISVEC = 0;
        d
    }
}
fn emit_rvec_anyall(dv: i32, isall: i32) -> i32 {
    unsafe {
        skipws(); eat_ch(40); CLO_FORREF = 1; let clo = parse_closure(); eat_ch(41);
        if isall == 1 { print!("  v{}: i32 = call rvec_all v{} v{}", NC, dv, clo); } else { print!("  v{}: i32 = call rvec_any v{} v{}", NC, dv, clo); }
        println!("");
        let d = NC; NC = NC + 1; EISCLO = 0; EISVEC = 0; EISBOOL = 1;
        d
    }
}
fn emit_vec_push_raw(dv: i32, e: i32) {
    unsafe {   /* non-growable append at buf[len] (no __vec_reserve). For descriptors whose @8 is NOT a cap slot -- e.g. a VecDeque [buf,len,head@8]: reserving would misread head as cap and spuriously grow, clobbering head. */
        let c4 = ecr(4); let lad = ebin("add", dv, c4); let n = eload(lad);
        let p = eload(dv); let c4b = ecr(4); let off = ebin("mul", n, c4b); let ea = ebin("add", p, off);
        println!("  store v{} v{}", ea, e);
        let one = ecr(1); let n1 = ebin("add", n, one);
        let c4c = ecr(4); let lad2 = ebin("add", dv, c4c);
        println!("  store v{} v{}", lad2, n1);
    }
}
fn emit_vec_push(dv: i32, e: i32) {
    unsafe {
        print!("  v{}: i32 = call __vec_reserve v{}", NC, dv); println!(""); NC = NC + 1;   /* VEC-GROW: ensure room for +1, may realloc buf (Vec [buf,len,cap@8]) */
        emit_vec_push_raw(dv, e);
    }
}
fn emit_desc_clone(src: i32, elsz: i32) -> i32 {
    unsafe {
        let nd = emit_vec_new();
        let c4 = ecr(4); let len = eload(ebin("add", src, c4));
        let sbuf = eload(src); let nbuf = eload(nd);
        let c4b = ecr(4); println!("  store v{} v{}", ebin("add", nd, c4b), len);
        let esz = ec(elsz); let total = ebin("mul", len, esz);
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, total); println!("  brz v{}", cnd); ckreset();
        let b = eloadb(ebin("add", sbuf, i));
        println!("  storeb v{} v{}", ebin("add", nbuf, i), b);
        println!("  cont"); ckreset(); let one = ecr(1); let ni = ebin("add", i, one); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        nd
    }
}
fn emit_struct_clone(cur: i32, cty: i32) -> i32 {
    unsafe {
        let sz = TSZ[cty as usize]; let nc = halloc(sz); let words = sz / 4; let mut i = 0;
        while i < words { let off = ec(i * 4); let v = eload(ebin("add", cur, off)); println!("  store v{} v{}", ebin("add", nc, off), v); i = i + 1; }
        let mut fi = 0;
        while fi < NFLD {
            if FSST[fi as usize] == cty {
                let fk = FFK[fi as usize];
                if fk == 2 || fk == 3 || fk == 6 { emit_field_deepcopy(cur, nc, fi, fk); }   /* BVK N3: byte-vec field (FFK=6) deep-copies at 1-byte stride (emit_field_deepcopy default elsz=1) */
            }
            fi = fi + 1;
        }
        EISVEC = 0; EISSTR = 0; EISSTRING = 0;
        nc
    }
}
fn emit_enum_clone(en: i32, cur: i32) -> i32 {   /* LAYER5 P-DERIVE-RECURSIVE: #[derive(Clone)] on an enum with no user clone impl -> copy the enum cell (disc + payload words, sized to the widest variant). Box children are shared pointers; rzrs is arena-allocated with no free/drop, so a shallow copy is observationally == rustc (which deep-clones, but the program only reads through the shared pointers). Was: `e.clone()` emitted `call <Enum>_clone` to a NON-EXISTENT fn (Clone is derived) -> garbage. */
    unsafe {
        let mut maxw = 2; let mut vi = 0;   /* min cell = 8 bytes = 2 words (disc + one payload/pad) */
        while vi < NVAR { if VEN[vi as usize] == en { let nf = VNF[vi as usize]; let mut w = 1; let mut fi = 0; while fi < nf { if VFI64[(vi * 8 + fi) as usize] == 1 { w = w + 2; } else { w = w + 1; } fi = fi + 1; } if w > maxw { maxw = w; } } vi = vi + 1; }   /* P-ENUM-I64-PAYLOAD-WIDTH: an i64 field is 2 words (8 bytes) -> the widest-variant cell size counts it as 2, so the word-copy below covers the i64's high word (was 1 word -> the high 32 bits were dropped on clone). */
        let nc = halloc(maxw * 4); let mut i = 0;
        while i < maxw { let off = ec(i * 4); let v = eload(ebin("add", cur, off)); println!("  store v{} v{}", ebin("add", nc, off), v); i = i + 1; }
        EISENUM = 1; EIENUMTY = en;
        nc
    }
}
fn enum_has_payload(en: i32) -> i32 {   /* LAYER5 P-DERIVE-RECURSIVE-EQ: 1 if any variant carries a payload (disc-only eq is then insufficient) */
    unsafe { let mut vi = 0; while vi < NVAR { if VEN[vi as usize] == en && VHAS[vi as usize] == 1 { return 1; } vi = vi + 1; } 0 }
}
fn vpay_off(vi: i32, fi: i32) -> i32 {   /* P-ENUM-I64-PAYLOAD-OFFSET: byte offset of variant vi's payload field fi within the enum cell.
    THE one place enum payload layout is decided. The cell is [disc@0, fields@4..]; an i64/isize field occupies 8
    bytes, every other field 4, so the offset of field fi is 4 + the summed WIDTHS of fields 0..fi -- NOT the fixed
    `4 + fi*4` stride that eight call sites each open-coded. Those strides disagreed with the two store paths that
    HAD been widened to a running offset (P-ENUM-I64-PAYLOAD-WIDTH), so a multi-field i64 payload stored at 4,12
    was read back at 4,8: field 0 swallowed field 1. `enum E{C{a:i64,b:i64}}` with a=10,b=20 gave 10, and the tuple
    form gave (20<<32)|10 -- the two halves visibly packed into one slot. Sixteen r2-corpus probes were wrong in
    exactly this one way. vi<0 (built-in Option/Result) keeps the 4-byte stride: those payloads are single-field. */
    unsafe { let mut o = 4; let mut k = 0; while k < fi { if vi >= 0 && VFI64[(vi * 8 + k) as usize] == 1 { o = o + 8; } else { o = o + 4; } k = k + 1; } o }
}
fn vpay_i64(vi: i32, fi: i32) -> i32 {   /* P-ENUM-I64-PAYLOAD-OFFSET: 1 if variant vi's payload field fi is 8-byte (load8/store8), guarded for vi<0. */
    unsafe { if vi >= 0 && VFI64[(vi * 8 + fi) as usize] == 1 { 1 } else { 0 } }
}
fn enum_eq_derivable(en: i32) -> i32 {   /* LAYER5 P-DERIVE-RECURSIVE-EQ: 1 if every payload field is a derivable eq kind (0/1/2, no 3) */
    unsafe { let mut vi = 0; while vi < NVAR { if VEN[vi as usize] == en && VHAS[vi as usize] == 1 { let nf = VNF[vi as usize]; let mut fi = 0; while fi < nf { if VFKIND[(vi * 8 + fi) as usize] == 3 { return 0; } fi = fi + 1; } } vi = vi + 1; } 1 }
}
fn enum_is_recursive(en: i32) -> i32 {   /* LAYER5 P-DERIVE-DEBUG: 1 if any variant field is a Box<ThisEnum> (VFKIND==1) -> {:?} must recurse via a synthetic <Enum>_dbg, not print the child pointer */
    unsafe { let mut vi = 0; while vi < NVAR { if VEN[vi as usize] == en && VHAS[vi as usize] == 1 { let nf = VNF[vi as usize]; let mut fi = 0; while fi < nf { if VFKIND[(vi * 8 + fi) as usize] == 1 { return 1; } fi = fi + 1; } } vi = vi + 1; } 0 }
}
fn enum_has_user_eq(en: i32) -> i32 {   /* LAYER5 P-DERIVE-RECURSIVE-EQ: 1 if the user defined a method `eq` on this enum (manual impl PartialEq) -> don't synthesize (avoid <Enum>_eq name collision) */
    unsafe { let owner = 0 - (en + 2); let mut i = 0; while i < NMRT { if MRTOWN[i as usize] == owner && MRTL[i as usize] == 2 && at(MRTO[i as usize]) == 101 && at(MRTO[i as usize] + 1) == 113 { return 1; } i = i + 1; } 0 }
}
fn emit_enum_eq_call(en: i32, fa: i32, fb: i32) -> i32 {   /* emit `vN = call <Enum>_eq va vb` (recursive structural eq) */
    unsafe { print!("  v{}: i32 = call ", NC); emit_span(ENO[en as usize], ENL[en as usize]); print!("_eq v{} v{}", fa, fb); println!(""); let r = NC; NC = NC + 1; r }
}
fn emit_str_desc_eq(a: i32, b: i32) -> i32 {   /* LAYER5 P-DERIVE-RECURSIVE-EQ: byte-equality of two String/&str descriptors via a CALL to __str_match (loop-free here -- __str_match owns the loop, so this is safe inside the eq fn's brz/pool variant block, unlike emit_streq_inline which emits a nested loop) */
    unsafe {
        let la = eload(ebin("add", a, ec(4))); let lb = eload(ebin("add", b, ec(4)));
        let leneq = ebin("eq", la, lb);
        let pa = eload(a); let pb = eload(b);
        print!("  v{}: i32 = call __str_match v{} v{} v{}", NC, pa, pb, lb); println!(""); let m = NC; NC = NC + 1;
        ebin("mul", leneq, m)
    }
}
fn emit_enum_eq_fn(en: i32) {   /* LAYER5 P-DERIVE-RECURSIVE-EQ: synthesize `fn <Enum>_eq(a,b)->i32` -- structural equality of two enum cells. disc must match, then AND the matching variant's fields (recurse on Box<Self>, byte-eq on String, scalar eq else). Payload-less variants: disc-eq suffices. */
    unsafe {
        NC = 0; NSYM = 0; ckreset();
        print!("fn "); emit_span(ENO[en as usize], ENL[en as usize]); print!("_eq"); println!("");
        println!("  v0: i32 = param"); println!("  v1: i32 = param");
        NC = 2;
        let a = 0; let b = 1;
        let da = eload(a); let db = eload(b);
        let de = ebin("eq", da, db);
        let rs = NC; NC = NC + 1; ecopy(rs, de);   /* rs = discs-equal (1/0); AND the field compares of the matching variant into it */
        let mut vi = 0;
        while vi < NVAR {
            if VEN[vi as usize] == en && VHAS[vi as usize] == 1 {
                let disc = VDISC[vi as usize];
                let isv = ebin("eq", da, ec(disc));
                let guard = ebin("mul", isv, de);   /* enter ONLY when da==disc AND discs equal -> b is the SAME variant, so dereferencing b's fields (esp. a String descriptor) is safe. Without the `de` factor a disc-mismatch could deref b at the wrong variant layout (a String field -> wild pointer -> segfault). */
                println!("  brz v{}", guard); ckreset();
                let nf = VNF[vi as usize]; let mut fi = 0;
                while fi < nf {
                    let fo8 = vpay_off(vi, fi); let fw8 = vpay_i64(vi, fi);   /* P-ENUM-I64-PAYLOAD-OFFSET */
                    let fa = if fw8 == 1 { eload8(ebin("add", a, ec(fo8))) } else { eload(ebin("add", a, ec(fo8))) };
                    let fb = if fw8 == 1 { eload8(ebin("add", b, ec(fo8))) } else { eload(ebin("add", b, ec(fo8))) };
                    let k = VFKIND[(vi * 8 + fi) as usize];
                    let cmp = if k == 1 { emit_enum_eq_call(en, fa, fb) } else if k == 2 { emit_str_desc_eq(fa, fb) } else { ebin("eq", fa, fb) };
                    let m = ebin("mul", rs, cmp); ecopy(rs, m);
                    fi = fi + 1;
                }
                println!("  pool"); ckreset();
            }
            vi = vi + 1;
        }
        println!("  ret v{}", rs);
    }
}
fn emit_field_deepcopy(cur: i32, nc: i32, fi: i32, fk: i32) {
    unsafe {
        let mut elsz = 1; if fk == 2 { elsz = 4; }
        let srcd = eload(ebin("add", cur, ec(FSB[fi as usize])));
        let cloned = emit_desc_clone(srcd, elsz);
        println!("  store v{} v{}", ebin("add", nc, ec(FSB[fi as usize])), cloned);
    }
}
/* S3: HashMap = [len@0, entries@4] where entries is a flat array of [key,val] pairs (8 bytes each).
   Linear scan (correctness, not an optimizing backend). String keys compared by value, else by ==. */
fn emit_opt_eq(a: i32, b: i32) -> i32 {   /* STATEFUL-ITER: structural Option== -> disc match AND (both None OR scalar payload equal) */
    unsafe {
        let c4 = ecr(4);
        let da = eload(a); let db = eload(b);
        let disceq = ebin("eq", da, db);
        let pa = eload(ebin("add", a, c4)); let pb = eload(ebin("add", b, c4));
        let payeq = ebin("eq", pa, pb);
        let one = ecr(1); let notda = ebin("sub", one, da);   /* 1 when a is None */
        let inner = ebin("add", notda, ebin("mul", da, payeq));   /* None -> 1 ; Some -> payeq */
        return ebin("mul", disceq, inner);
    }
}
fn emit_streq_inline(a: i32, b: i32) -> i32 {   /* -> vN 1 if the two String/&str descriptors are byte-equal */
    unsafe {
        let c4 = ecr(4); let la = eload(ebin("add", a, c4)); let c4b = ecr(4); let lb = eload(ebin("add", b, c4b));
        let res = NC; let eqlen = ebin("eq", la, lb); ecopy(res, eqlen); NC = NC + 1;
        let pa = eload(a); let pb = eload(b);
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, la); println!("  brz v{}", cnd); ckreset();
        let ba = eloadb(ebin("add", pa, i)); let bb = eloadb(ebin("add", pb, i));
        let beq = ebin("eq", ba, bb); let nr = ebin("mul", res, beq); ecopy(res, nr);
        println!("  cont"); ckreset(); let one = ecr(1); let ni = ebin("add", i, one); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        res
    }
}
fn emit_hm_find(map: i32, key: i32, iskeystr: i32) -> i32 {   /* -> vN index of key, or -1 */
    unsafe {
        let len = eload(map); let c4 = ecr(4); let ents = eload(ebin("add", map, c4));
        let res = NC; let m1 = ec(0 - 1); ecopy(res, m1); NC = NC + 1;
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let c8 = ecr(8); let ea = ebin("add", ents, ebin("mul", i, c8)); let ek = eload(ea);
        let mut eqv = 0; if iskeystr == 1 { eqv = emit_streq_inline(ek, key); } else { eqv = ebin("eq", ek, key); }
        let mm1 = ec(0 - 1); let isunset = ebin("eq", res, mm1); let cond = ebin("mul", isunset, eqv);
        println!("  brz v{}", cond); ckreset(); ecopy(res, i); println!("  pool"); ckreset();
        println!("  cont"); ckreset(); let one = ecr(1); let ni = ebin("add", i, one); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        res
    }
}
fn emit_hashmap_new() -> i32 {
    unsafe {
        let d = halloc(8); let ents = halloc(65536); let c4 = ecr(4); println!("  store v{} v{}", ebin("add", d, c4), ents);
        EISHM = 1; d
    }
}
/* STDCOLL N2: HashSet = a keys-only HashMap [len@0, entries@4] (pairs [key,0]). Reuses emit_hm_find /
   emit_hm_slot_addr: insert = ensure-slot (dedup automatic), contains = find>=0, len = eload. */
fn emit_hashset_new() -> i32 {
    unsafe { let d = emit_hashmap_new(); EISHM = 0; EISSET = 1; d }
}
/* STDCOLL N3: `&w[lo..hi]` on a &str/String -> a sub-slice descriptor {ptr+lo, hi-lo} (ASCII byte
   offsets; the corpus is ASCII). Open ends: `[..hi]` lo=0, `[lo..]` hi=origlen. Exclusive `..` only. */
fn emit_str_slice(d: i32) -> i32 {
    unsafe {
        POS = POS + 1; skipws();   /* consume [ */
        let mut lo = ec(0);
        if POS < LEN && at(POS) != 46 { lo = parse_expr(); }
        skipws(); eat_ch(46); eat_ch(46); skipws();
        let origlen = eload(ebin("add", d, ec(4)));
        let mut hi = origlen;
        if POS < LEN && at(POS) != 93 { hi = parse_expr(); }
        eat_ch(93);
        let nd = halloc(8);
        println!("  store v{} v{}", nd, ebin("add", eload(d), lo));
        println!("  store v{} v{}", ebin("add", nd, ec(4)), ebin("sub", hi, lo));
        EISSTR = 1;
        nd
    }
}
fn dispatch_hashset(set: i32) -> i32 {
    unsafe {
        skipws();
        if POS < LEN && at(POS) == 46 {
            let s = POS; POS = POS + 1; rd_ident();
            if id_is(108, 101, 110, 0, 0, 3) { skipws(); eat_ch(40); eat_ch(41); return eload(set); }   /* len() */
            if id_is(105, 115, 95, 101, 109, 8) { skipws(); eat_ch(40); eat_ch(41); let z = ecr(0); let r = ebin("eq", eload(set), z); EISBOOL = 1; return r; }   /* is_empty() */
            if id_is(105, 110, 115, 101, 114, 6) { skipws(); eat_ch(40); let k = parse_hm_key(); let ks = EISSTR; eat_ch(41); let z = ecr(0); emit_hm_slot_addr(set, k, ks, z); EISSET = 0; return ec(0); }   /* insert(x): ensure a slot (dedup automatic); result (was-new bool) unused by the corpus */
            if id_is(99, 111, 110, 116, 97, 8) { skipws(); eat_ch(40); let k = parse_hm_key(); let ks = EISSTR; eat_ch(41); let idx = emit_hm_find(set, k, ks); let z = ecr(0); let r = ebin("ge", idx, z); EISBOOL = 1; return r; }   /* contains(&x) -> bool */
            POS = s;
        }
        EISSET = 1;
        set
    }
}
/* STDCOLL N1: VecDeque = [buf@0, len@4, head@8]. push_back appends at buf[len] (4-byte elems, mirrors
   emit_vec_push -- the first 8 bytes ARE a valid Vec [buf,len] descriptor); pop_front reads buf[head] and
   advances head. len/head default 0 (bump-allocated over zero BSS). */
fn emit_vecdeque_new() -> i32 {
    unsafe {
        let d = halloc(12);
        let buf = halloc(4096);
        println!("  store v{} v{}", d, buf);
        EISVDQ = 1;
        d
    }
}
/* pop_front() -> Option cell [disc@0, payload@4]. disc = (head < len); payload = buf[head]; head advances
   by disc (unconditional-load guarded by disc, so an empty deque keeps head and reads an in-range slot). */
fn emit_vdq_popfront(dv: i32) -> i32 {
    unsafe {
        let c4 = ecr(4); let c8 = ecr(8);
        let len = eload(ebin("add", dv, c4));
        let head = eload(ebin("add", dv, c8));
        let hasmore = ebin("lt", head, len);
        let cell = halloc(8);
        println!("  store v{} v{}", cell, hasmore);
        let buf = eload(dv);
        let ea = ebin("add", buf, ebin("mul", head, ec(4)));
        let val = eload(ea);
        println!("  store v{} v{}", ebin("add", cell, ec(4)), val);
        let nh = ebin("add", head, hasmore);
        println!("  store v{} v{}", ebin("add", dv, ec(8)), nh);
        EISOPT = 1;
        cell
    }
}
fn dispatch_vecdeque(dv: i32) -> i32 {
    unsafe {
        skipws();
        if POS < LEN && at(POS) == 46 {
            let s = POS; POS = POS + 1; rd_ident();
            if id_is(112, 117, 115, 104, 95, 9) { skipws(); eat_ch(40); let e = parse_expr(); eat_ch(41); emit_vec_push_raw(dv, e); EISVDQ = 0; return ec(0); }   /* push_back(x): append at buf[len], len++ (RAW: VecDeque @8 is head, not cap -- no __vec_reserve) */
            if id_is(112, 111, 112, 95, 102, 9) { skipws(); eat_ch(40); eat_ch(41); return emit_vdq_popfront(dv); }   /* pop_front() -> Option */
            if id_is(108, 101, 110, 0, 0, 3) { skipws(); eat_ch(40); eat_ch(41); let c4 = ecr(4); let c8 = ecr(8); let len = eload(ebin("add", dv, c4)); let head = eload(ebin("add", dv, c8)); return ebin("sub", len, head); }   /* len() = len - head */
            if id_is(105, 115, 95, 101, 109, 8) { skipws(); eat_ch(40); eat_ch(41); let c4 = ecr(4); let c8 = ecr(8); let len = eload(ebin("add", dv, c4)); let head = eload(ebin("add", dv, c8)); let r = ebin("ge", head, len); EISBOOL = 1; return r; }   /* is_empty() -> head >= len */
            if id_is(112, 117, 115, 104, 95, 10) { skipws(); eat_ch(40); let e = parse_expr(); eat_ch(41); emit_vdq_pushfront(dv, e); EISVDQ = 0; return ec(0); }   /* push_front(x): decrement head, store buf[head] (only valid while head>0; corpus uses push_back) */
            POS = s;
        }
        EISVDQ = 1;
        dv
    }
}
fn emit_vdq_pushfront(dv: i32, e: i32) {
    unsafe {
        let c8 = ecr(8); let head = eload(ebin("add", dv, c8));
        let nh = ebin("sub", head, ec(1));
        println!("  store v{} v{}", ebin("add", dv, ec(8)), nh);
        let buf = eload(dv);
        let ea = ebin("add", buf, ebin("mul", nh, ec(4)));
        println!("  store v{} v{}", ea, e);
    }
}
/* return the ADDRESS of key's value slot; if absent, append (key, initv) and return its value address. */
fn emit_hm_slot_addr(map: i32, key: i32, iskeystr: i32, initv: i32) -> i32 {
    unsafe {
        let idx = emit_hm_find(map, key, iskeystr);
        let c4 = ecr(4); let ents = eload(ebin("add", map, c4)); let c8 = ecr(8);
        let z = ecr(0); let found = ebin("ge", idx, z);
        let addr = NC; NC = NC + 1; let lb = LBL; LBL = LBL + 1;
        println!("  brz v{}", found); ckreset();
        let ea = ebin("add", ents, ebin("mul", idx, c8)); let c4a = ecr(4); ecopy(addr, ebin("add", ea, c4a));
        println!("  goto __Lh{}", lb); ckreset(); println!("  pool"); ckreset();
        let len = eload(map); let ea2 = ebin("add", ents, ebin("mul", len, c8));
        println!("  store v{} v{}", ea2, key);
        let c4b = ecr(4); let va = ebin("add", ea2, c4b); println!("  store v{} v{}", va, initv);
        let one = ecr(1); let nl = ebin("add", len, one); println!("  store v{} v{}", map, nl);
        ecopy(addr, va);
        println!("  label __Lh{}", lb); ckreset();
        addr
    }
}
fn parse_hm_key() -> i32 { unsafe { skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; } let r = parse_expr(); if EISSTRING == 1 { EISSTR = 1; } r } }   /* map key: strip a leading & (map.get(&k)); a String key compares by value like &str */
/* Build an Option cell [disc@0, payload@4] for get/get_mut: disc = key-found; payload = value (wantaddr=0) or value SLOT ADDRESS (wantaddr=1, &mut V). Branchless: a missing key clamps to slot 0 (in-bounds of the entries buffer, never a wild load); disc guards the payload. */
fn emit_hm_getcell(map: i32, key: i32, iskeystr: i32, wantaddr: i32) -> i32 {
    unsafe {
        let idx = emit_hm_find(map, key, iskeystr);
        let z = ecr(0); let found = ebin("ge", idx, z); let sidx = ebin("mul", idx, found);
        let c4 = ecr(4); let ents = eload(ebin("add", map, c4)); let c8 = ecr(8);
        let ea = ebin("add", ents, ebin("mul", sidx, c8)); let c4b = ecr(4); let va = ebin("add", ea, c4b);
        let mut payload = va; if wantaddr == 0 { payload = eload(va); }
        let cell = halloc(8); println!("  store v{} v{}", cell, found); let c4c = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4c), payload);
        cell
    }
}
fn dispatch_hashmap(map: i32) -> i32 {
    unsafe {
        skipws();
        if POS < LEN && at(POS) == 46 {
            POS = POS + 1; rd_ident();
            if id_is(108, 101, 110, 0, 0, 3) { skipws(); eat_ch(40); eat_ch(41); return eload(map); }   /* len */
            if id_is(105, 115, 95, 101, 109, 8) { skipws(); eat_ch(40); eat_ch(41); let z = ecr(0); let r = ebin("eq", eload(map), z); EISBOOL = 1; return r; }   /* is_empty */
            if id_is(105, 110, 115, 101, 114, 6) {                                   /* insert(k, v) -> Option<V>: Some(old) on overwrite, else None */
                skipws(); eat_ch(40); let k = parse_hm_key(); let ks = EISSTR; skipws(); eat_ch(44); let v = parse_expr(); eat_ch(41);
                let idx = emit_hm_find(map, k, ks); let z = ecr(0); let found = ebin("ge", idx, z);   /* found BEFORE the (possible) append */
                let addr = emit_hm_slot_addr(map, k, ks, v); let old = eload(addr); println!("  store v{} v{}", addr, v);   /* ensure slot, capture old (=v when absent, ignored by None), overwrite */
                let cell = halloc(8); println!("  store v{} v{}", cell, found); let c4 = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4), old);
                return opt_postfix(cell);
            }
            if id_is(103, 101, 116, 0, 0, 3) {                                       /* get(k) -> Option<&V> (copy-by-value payload) */
                skipws(); eat_ch(40); let k = parse_hm_key(); let ks = EISSTR; eat_ch(41);
                let cell = emit_hm_getcell(map, k, ks, 0);
                EOPTREF = 1;   /* TI3e-l: get() yields Option<&V> -- payload models a reference, so `*get(k).unwrap()` / `if let Some(v)=get(k){ *v }` deref is a no-op (mirrors first()/find(); was an unmarked value -> `*` did a real eload -> segfault) */
                return opt_postfix(cell);
            }
            if id_is(103, 101, 116, 95, 109, 7) {                                     /* get_mut(k) -> Option<&mut V>: payload = value SLOT ADDRESS (deref-writable) */
                skipws(); eat_ch(40); let k = parse_hm_key(); let ks = EISSTR; eat_ch(41);
                return opt_postfix(emit_hm_getcell(map, k, ks, 1));
            }
            if id_is(118, 97, 108, 117, 101, 6) || id_is(107, 101, 121, 115, 0, 4) {   /* R2: values() / keys() -> Vec of that column */
                let iskeys = at(ID_O) == 107;
                skipws(); eat_ch(40); eat_ch(41);
                let c4 = ecr(4); let len = eload(map); let ents = eload(ebin("add", map, c4));
                let buf = halloc(65536);
                let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
                println!("  loop"); ckreset();
                let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
                let c8 = ecr(8); let ea = ebin("add", ents, ebin("mul", i, c8));
                let mut vv = eload(ea); if iskeys == false { let c4b = ecr(4); vv = eload(ebin("add", ea, c4b)); }
                let c4c = ecr(4); println!("  store v{} v{}", ebin("add", buf, ebin("mul", i, c4c)), vv);
                println!("  cont"); ckreset(); let one = ecr(1); ecopy(i, ebin("add", i, one));
                println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                let d = halloc(8); let c4d = ecr(4); println!("  store v{} v{}", d, buf); println!("  store v{} v{}", ebin("add", d, c4d), len);
                EISVEC = 1; return dispatch_vec(d);
            }
            if id_is(114, 101, 109, 111, 118, 6) {                                   /* remove(k) -> Option (swap-remove) */
                skipws(); eat_ch(40); let k = parse_hm_key(); let ks = EISSTR; eat_ch(41);
                let idx = emit_hm_find(map, k, ks); let cell = halloc(8);
                let z = ecr(0); let found = ebin("ge", idx, z); let lb = LBL; LBL = LBL + 1;
                println!("  brz v{}", found); ckreset();
                let c4 = ecr(4); let ents = eload(ebin("add", map, c4)); let c8 = ecr(8);
                let ea = ebin("add", ents, ebin("mul", idx, c8)); let c4b = ecr(4); let val = eload(ebin("add", ea, c4b));
                let len = eload(map); let one = ecr(1); let last = ebin("sub", len, one);
                let la = ebin("add", ents, ebin("mul", last, c8)); let lk = eload(la); let c4c = ecr(4); let lv = eload(ebin("add", la, c4c));
                println!("  store v{} v{}", ea, lk); let c4d = ecr(4); println!("  store v{} v{}", ebin("add", ea, c4d), lv);
                println!("  store v{} v{}", map, last);
                let one2 = ecr(1); println!("  store v{} v{}", cell, one2); let c4e = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4e), val);
                println!("  goto __Lh{}", lb); ckreset(); println!("  pool"); ckreset(); let z2 = ecr(0); println!("  store v{} v{}", cell, z2); println!("  label __Lh{}", lb); ckreset();
                return opt_postfix(cell);
            }
            if id_is(99, 111, 110, 116, 97, 12) {                                    /* contains_key(k) -> bool */
                skipws(); eat_ch(40); let k = parse_hm_key(); let ks = EISSTR; eat_ch(41);
                let idx = emit_hm_find(map, k, ks); let z = ecr(0); let r = ebin("ge", idx, z); EISBOOL = 1; return r;
            }
            if id_is(101, 110, 116, 114, 121, 5) {                                   /* entry(k).or_insert(v) -> value address */
                skipws(); eat_ch(40); let k = parse_hm_key(); let ks = EISSTR; eat_ch(41);
                skipws(); if POS < LEN && at(POS) == 46 { POS = POS + 1; rd_ident(); }   /* .or_insert / .or_insert_with */
                skipws(); eat_ch(40);
                skipws(); if POS < LEN && at(POS) == 124 { POS = POS + 1; while POS < LEN && at(POS) != 124 { POS = POS + 1; } POS = POS + 1; }   /* R2: or_insert_with(|| v) -- skip the closure header, eval body as the init (eager) */
                let iv = parse_expr(); let ivvec = EISVEC; eat_ch(41);
                let addr = emit_hm_slot_addr(map, k, ks, iv);
                if ivvec == 1 { let d = eload(addr); EISVEC = 1; return dispatch_vec(d); }   /* nestgen: Vec-valued entry -> yield the &mut Vec so .push(..) mutates the stored descriptor */
                return addr;
            }
        }
        if POS < LEN && at(POS) == 91 {                                          /* m[k] / m[&k] -> value for key (load value slot) */
            POS = POS + 1; let k = parse_hm_key(); let ks = EISSTR; eat_ch(93);
            let idx = emit_hm_find(map, k, ks);
            let c4 = ecr(4); let ents = eload(ebin("add", map, c4)); let c8 = ecr(8);
            let ea = ebin("add", ents, ebin("mul", idx, c8)); let c4b = ecr(4);
            let vv = eload(ebin("add", ea, c4b));
            if EHMVVEC == 1 { EISVEC = 1; return dispatch_vec(vv); }   /* nestgen: Vec-valued map -> dispatch m[k].iter()/.push/.len as a Vec */
            EISSTR = 0; EISSTRING = 0;   /* the value is an i32; a String KEY must not leave `+` in string-concat mode (m[&k] + m["y"]) */
            return vv;
        }
        map
    }
}
/* S1 (eager iterators): an iterator is just a Vec descriptor [buf,len]; adapters transform it. */
fn emit_iter_sum(dv: i32) -> i32 {
    unsafe {
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let acc = NC; let z = ecr(0); ecopy(acc, z); NC = NC + 1;
        let i = NC; let z2 = ecr(0); ecopy(i, z2); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let off = ebin("mul", i, c4); let el = eload(ebin("add", buf, off));
        let na = ebin("add", acc, el); ecopy(acc, na);
        println!("  cont"); ckreset();
        let one = ecr(1); let ni = ebin("add", i, one); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        acc
    }
}
fn emit_iter_product(dv: i32) -> i32 {   /* iterator terminal .product() -- like sum but acc starts at 1 and multiplies */
    unsafe {
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let acc = NC; let one = ecr(1); ecopy(acc, one); NC = NC + 1;
        let i = NC; let z2 = ecr(0); ecopy(i, z2); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let off = ebin("mul", i, c4); let el = eload(ebin("add", buf, off));
        let na = ebin("mul", acc, el); ecopy(acc, na);
        println!("  cont"); ckreset();
        let one2 = ecr(1); let ni = ebin("add", i, one2); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        acc
    }
}
fn emit_iter_peek(dv: i32) -> i32 {   /* R2cj: .peek() -> Option<&first-elem> cell (no consumption; our iterator model is an eager vec) */
    unsafe {
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let cell = halloc(8);
        let z = ecr(0); let hasel = ebin("gt", len, z); println!("  store v{} v{}", cell, hasel);
        let first = eload(buf);
        let c4b = ecr(4); let pad = ebin("add", cell, c4b); println!("  store v{} v{}", pad, first);
        cell
    }
}
fn emit_iter_minmax(dv: i32, ismax: i32) -> i32 {   /* -> Option<i32> cell */
    unsafe {
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let cell = halloc(8);
        let z = ecr(0); let hasel = ebin("gt", len, z); println!("  store v{} v{}", cell, hasel);   /* disc = len>0 */
        let best = NC; let first = eload(buf); ecopy(best, first); NC = NC + 1;                     /* best = buf[0] (garbage if empty; disc guards it) */
        let i = NC; let one0 = ecr(1); ecopy(i, one0); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let off = ebin("mul", i, c4); let el = eload(ebin("add", buf, off));
        let mut better = 0; if ismax == 1 { better = ebin("gt", el, best); } else { better = ebin("lt", el, best); }
        println!("  brz v{}", better); ckreset(); ecopy(best, el); println!("  pool"); ckreset();
        println!("  cont"); ckreset();
        let one = ecr(1); let ni = ebin("add", i, one); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        let c4b = ecr(4); let pad = ebin("add", cell, c4b); println!("  store v{} v{}", pad, best);
        cell
    }
}
fn emit_iter_rev(dv: i32) -> i32 {
    unsafe {
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let nd = emit_vec_new();
        let i = NC; let one0 = ecr(1); let lm1 = ebin("sub", len, one0); ecopy(i, lm1); NC = NC + 1;
        println!("  loop"); ckreset();
        let z = ecr(0); let cnd = ebin("ge", i, z); println!("  brz v{}", cnd); ckreset();
        let off = ebin("mul", i, c4); let el = eload(ebin("add", buf, off)); emit_vec_push(nd, el);
        println!("  cont"); ckreset();
        let one = ecr(1); let ni = ebin("sub", i, one); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        EISVEC = 1; nd
    }
}
/* S1: .map(|x| body) / .filter(|x| pred) -- inline the closure body into an eager build loop. The param is
   marked for-ref (R19) so both `|x| *x` and `|&x| x` see the element value. */
fn emit_iter_map(dv: i32) -> i32 {
    unsafe {
        eat_ch(40); skipws(); eat_ch(124); skipws(); while POS < LEN && at(POS) == 38 { POS = POS + 1; }
        let mut istup = 0; let mut patpos = 0;
        if POS < LEN && at(POS) == 40 { istup = 1; patpos = POS; let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } POS = POS + 1; } }   /* ALPHABET: `|&(a,b)|` tuple-pattern param -- accept the same alphabet the for-loop does */
        else { rd_ident(); }
        let po = ID_O; let pl = ID_L;
        skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
        eat_ch(124);
        skipws(); if POS + 1 < LEN && at(POS) == 45 && at(POS + 1) == 62 { POS = POS + 2; skipws(); while POS < LEN && at(POS) != 123 { POS = POS + 1; } }   /* LAYER3: `.map(|x| -> T { .. })` -- skip the closure return type up to the block body (mirrors R1 in parse_closure) */
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let nd = emit_vec_new();
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        let xslot = NC; NC = NC + 1;
        let selt = DVELT;                                                            /* R2bx: element type of the iterated Vec, so `|r| r.field`/`r.len()` dispatch correctly */
        let selty = DVELTY;                                                          /* &str/String element carried as the interned Ty (TYTAG 3/4), not via DVELT==-2 */
        let mark = NSYM;
        if istup == 0 { sym_add(po, pl, xslot); mark_forref();
            if selt >= 0 { SST[(NSYM - 1) as usize] = selt; SFORREF[(NSYM - 1) as usize] = 0; }   /* R2bx: struct element -> r is struct-typed (r.field) */
            if selt == 0 - 3 { SK[(NSYM - 1) as usize] = 2; SFORREF[(NSYM - 1) as usize] = 0; }  /* R2bx: Vec element -> r is a Vec descriptor (r.len()) */
            if selt == 0 - 2 || (selty >= 0 && TYTAG[selty as usize] == 3) { SK[(NSYM - 1) as usize] = 1; SFORREF[(NSYM - 1) as usize] = 0; }   /* &str element -> r is a &str descriptor so `|s| s.len()`/`.chars()`/… dispatch (was a bare scalar -> s.len() misfired / segfaulted) */
            if selty >= 0 && TYTAG[selty as usize] == 4 { SK[(NSYM - 1) as usize] = 3; SFORREF[(NSYM - 1) as usize] = 0; } }   /* String element */
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let off = ebin("mul", i, c4); let el = eload(ebin("add", buf, off)); ecopy(xslot, el);
        if istup == 1 { let sv = POS; POS = patpos; bind_tuple_pat(el, 0, 0 - 1); POS = sv; }   /* ALPHABET: bind a=el.0, b=el.1 per element */
        skipws();
        let mut r = 0; let mut bvec = 0; let mut bety = 0 - 1; let mut betup = 0; let mut betupty = 0 - 1;
        if POS < LEN && at(POS) == 123 {                                             /* R39: `.map(|x| { let ..; tail })` block body -- parse statements, the tail expr is the mapped value (via TAILDST, same as the if-expr branch capture) */
            let slot = NC; NC = NC + 1; parse_branch_into(slot);
            r = slot; bvec = EISVEC; bety = EITYP; betup = if TYTAG[EITY as usize] == 11 { 1 } else { 0 }; betupty = ETUPTY;
        } else {
            r = parse_expr(); bvec = EISVEC; bety = EITYP; betup = if TYTAG[EITY as usize] == 11 { 1 } else { 0 }; betupty = ETUPTY;
        }
        emit_vec_push(nd, r);
        println!("  cont"); ckreset();
        let one = ecr(1); let ni = ebin("add", i, one); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        NSYM = mark; eat_ch(41);
        DVELT = 0 - 1; DVELTY = 0 - 1;   /* P-VECVEC: the map RESULT's element kind = the body's, NOT g's (was left -3 -> a chained `.sum()` summed pointers). */
        if bvec == 1 { DVELT = 0 - 3; } else if bety >= 0 { DVELT = bety; }
        if betup == 1 { VELTY = ty_intern(11, 0, betupty); DVELTY = VELTY; }   /* `map(|x| (a,b)).collect()` -> the result Vec's element is a tuple; record its Ty so {:?}/for-destructure see a tuple, not a pointer (mirrors enumerate.map at emit_iter_enum_map) */
        EISVEC = 1; nd
    }
}
fn emit_iter_filter(dv: i32) -> i32 {
    unsafe {
        eat_ch(40); skipws(); eat_ch(124); skipws(); while POS < LEN && at(POS) == 38 { POS = POS + 1; }
        rd_ident(); let po = ID_O; let pl = ID_L;
        skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
        eat_ch(124);
        skipws(); if POS + 1 < LEN && at(POS) == 45 && at(POS + 1) == 62 { POS = POS + 2; skipws(); while POS < LEN && at(POS) != 123 { POS = POS + 1; } }   /* LAYER3: `.filter(|x| -> bool { .. })` -- skip the closure return type up to the block body */
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let nd = emit_vec_new();
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        let xslot = NC; NC = NC + 1;
        let selt = DVELT; let selty = DVELTY;                                        /* R2bx: element type of the iterated Vec, so `|r| r.field`/`r.len()` dispatch correctly */
        let mark = NSYM; sym_add(po, pl, xslot); mark_forref();
        if selt >= 0 { SST[(NSYM - 1) as usize] = selt; SFORREF[(NSYM - 1) as usize] = 0; }   /* R2bx: struct element -> r is struct-typed (r.field) */
        if selt == 0 - 3 { SK[(NSYM - 1) as usize] = 2; SFORREF[(NSYM - 1) as usize] = 0; }   /* R2bx: Vec element -> r is a Vec descriptor (r.len()) */
        if selt == 0 - 2 || (selty >= 0 && TYTAG[selty as usize] == 3) { SK[(NSYM - 1) as usize] = 1; SFORREF[(NSYM - 1) as usize] = 0; }   /* R19b: &str element -> `|s| s.len()` predicate dispatches */
        if selty >= 0 && TYTAG[selty as usize] == 4 { SK[(NSYM - 1) as usize] = 3; SFORREF[(NSYM - 1) as usize] = 0; }   /* String element */
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let off = ebin("mul", i, c4); let el = eload(ebin("add", buf, off)); ecopy(xslot, el);
        skipws();
        let pred = if POS < LEN && at(POS) == 123 { let slot = NC; NC = NC + 1; parse_branch_into(slot); slot } else { parse_expr() };   /* LAYER3: `.filter(|x| -> bool { .. })` block body -- tail expr is the predicate (mirrors map's block path) */
        println!("  brz v{}", pred); ckreset(); emit_vec_push(nd, el); println!("  pool"); ckreset();
        println!("  cont"); ckreset();
        let one = ecr(1); let ni = ebin("add", i, one); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        NSYM = mark; eat_ch(41);
        EISVEC = 1; nd
    }
}
/* S1: .filter_map(|x| body) -- body yields an Option/Result cell [disc,payload]; push payload where disc!=0, drop otherwise.
   Mirrors emit_iter_filter but the pushed value is the Some/Ok payload, not the element. The param is for-ref (R19) so
   `|x| *x` (Vec<Option<T>>) sees the element cell; a scalar param also reaches opt_postfix so `|r| r.ok()` dispatches. */
fn emit_iter_filtermap(dv: i32) -> i32 {
    unsafe {
        eat_ch(40); skipws(); eat_ch(124); skipws(); while POS < LEN && at(POS) == 38 { POS = POS + 1; }
        rd_ident(); let po = ID_O; let pl = ID_L;
        skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
        eat_ch(124);
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let nd = emit_vec_new();
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        let xslot = NC; NC = NC + 1;
        let selt = DVELT;                                                            /* R2cd: element type of the iterated Vec (struct / Vec-descriptor) */
        let mark = NSYM; sym_add(po, pl, xslot); mark_forref();
        if selt >= 0 { SST[(NSYM - 1) as usize] = selt; SFORREF[(NSYM - 1) as usize] = 0; }   /* struct element -> x is struct-typed */
        if selt == 0 - 3 { SK[(NSYM - 1) as usize] = 2; SFORREF[(NSYM - 1) as usize] = 0; }   /* Vec element -> x is a Vec descriptor */
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let off = ebin("mul", i, c4); let el = eload(ebin("add", buf, off)); ecopy(xslot, el);
        let opt = parse_expr();
        let disc = eload(opt);
        println!("  brz v{}", disc); ckreset();
        let c4b = ecr(4); let payload = eload(ebin("add", opt, c4b)); emit_vec_push(nd, payload); println!("  pool"); ckreset();
        println!("  cont"); ckreset();
        let one = ecr(1); let ni = ebin("add", i, one); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        NSYM = mark; eat_ch(41);
        EISVEC = 1; nd
    }
}
fn emit_iter_flatmap(dv: i32) -> i32 {   /* flat_map(|x| iter) -> concat of each mapped iterable */
    unsafe {
        eat_ch(40); skipws(); eat_ch(124); skipws(); while POS < LEN && at(POS) == 38 { POS = POS + 1; }
        rd_ident(); let po = ID_O; let pl = ID_L;
        skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
        eat_ch(124);
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let nd = emit_vec_new();
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        let xslot = NC; NC = NC + 1;
        let mark = NSYM; sym_add(po, pl, xslot); mark_forref();
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let off = ebin("mul", i, c4); let el = eload(ebin("add", buf, off)); ecopy(xslot, el);
        let inner = parse_expr();
        let ilen = eload(ebin("add", inner, c4)); let ibuf = eload(inner);
        let j = NC; let z2 = ecr(0); ecopy(j, z2); NC = NC + 1;
        println!("  loop"); ckreset();
        let icnd = ebin("lt", j, ilen); println!("  brz v{}", icnd); ckreset();
        let joff = ebin("mul", j, c4); let ie = eload(ebin("add", ibuf, joff)); emit_vec_push(nd, ie);
        println!("  cont"); ckreset(); let one2 = ecr(1); let nj = ebin("add", j, one2); ecopy(j, nj);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        println!("  cont"); ckreset(); let one = ecr(1); let ni = ebin("add", i, one); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        NSYM = mark; eat_ch(41); EISVEC = 1; nd
    }
}
/* S1 follow-ons. Each parses `( |x| .. )` (or `(init, |acc, x| ..)` for fold, `(n)` for take/skip). */
fn iter_open_closure() -> i32 {   /* eat `( |` and a single param (optional & / : type); returns the param NAME slot via ID -- caller sym_adds */
    unsafe {
        eat_ch(40); skipws(); eat_ch(124); skipws(); while POS < LEN && at(POS) == 38 { POS = POS + 1; }
        rd_ident(); let po = ID_O;
        skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
        eat_ch(124); po
    }
}
fn emit_iter_anyall(dv: i32, isall: i32) -> i32 {
    unsafe {
        let po = iter_open_closure(); let pl = ID_L;
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let mut ival = 0; if isall == 1 { ival = 1; } let acc = NC; let init = ec(ival); ecopy(acc, init); NC = NC + 1;
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        let xslot = NC; NC = NC + 1;
        let mark = NSYM; sym_add(po, pl, xslot); mark_forref();
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let off = ebin("mul", i, c4); let el = eload(ebin("add", buf, off)); ecopy(xslot, el);
        let pred = parse_expr();
        let mut na = 0; if isall == 1 { na = ebin("mul", acc, pred); } else { na = ebin("add", acc, pred); } ecopy(acc, na);
        println!("  cont"); ckreset(); let one = ecr(1); let ni = ebin("add", i, one); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        NSYM = mark; eat_ch(41); EISBOOL = 1;
        if isall == 1 { return acc; }
        let z2 = ecr(0); ebin("ne", acc, z2)
    }
}
fn emit_iter_findpos(dv: i32, ispos: i32) -> i32 {   /* -> Option (index if ispos else element) */
    unsafe {
        let fdvelt = DVELT; let fdvelty = DVELTY;   /* DBG: capture the element kind before the predicate clobbers it, for the {:?} payload kind */
        let po = iter_open_closure(); let pl = ID_L;
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let cell = halloc(8); let z0 = ecr(0); println!("  store v{} v{}", cell, z0);
        let found = NC; let z1 = ecr(0); ecopy(found, z1); NC = NC + 1;
        let i = NC; let z2 = ecr(0); ecopy(i, z2); NC = NC + 1;
        let xslot = NC; NC = NC + 1;
        let mark = NSYM; sym_add(po, pl, xslot); mark_forref();
        if fdvelt == 0 - 2 || (fdvelty >= 0 && TYTAG[fdvelty as usize] == 3) { SK[(NSYM - 1) as usize] = 1; SFORREF[(NSYM - 1) as usize] = 0; }   /* R19b: &str element -> predicate param `s` is a &str descriptor (`|s| s.len()`/`.starts_with()`); was a bare scalar -> mis-evaluated -> wrong match */
        else if fdvelty >= 0 && TYTAG[fdvelty as usize] == 4 { SK[(NSYM - 1) as usize] = 3; SFORREF[(NSYM - 1) as usize] = 0; }
        else if fdvelt >= 0 { SST[(NSYM - 1) as usize] = fdvelt; SFORREF[(NSYM - 1) as usize] = 0; }
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let off = ebin("mul", i, c4); let el = eload(ebin("add", buf, off)); ecopy(xslot, el);
        let pred = parse_expr();
        let zf = ecr(0); let nf = ebin("eq", found, zf); let cond = ebin("mul", pred, nf);
        println!("  brz v{}", cond); ckreset();
        let one1 = ecr(1); println!("  store v{} v{}", cell, one1);
        let c4b = ecr(4); let pad = ebin("add", cell, c4b); let mut pv = i; if ispos == 0 { pv = el; } println!("  store v{} v{}", pad, pv);
        let one2 = ecr(1); ecopy(found, one2);
        println!("  pool"); ckreset();
        println!("  cont"); ckreset(); let one = ecr(1); let ni = ebin("add", i, one); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        NSYM = mark; eat_ch(41); EISBOOL = 0;   /* result is an index/element Option, not the predicate's bool */
        if ispos == 0 { EOPTREF = 1; }           /* R2: find yields Option<&T> -- payload models a reference, `*x` is a no-op */
        EISOPT = 1;                              /* DBG: find/position return an Option -> tag it so a bare `{:?}` prints Some(x)/None (was unset -> AOPT=0 -> printed the cell pointer) */
        if fdvelty >= 0 && TYTAG[fdvelty as usize] == 3 { EOPTK = 1; } else if fdvelty >= 0 && TYTAG[fdvelty as usize] == 4 { EOPTK = 3; } else if fdvelt == 0 - 2 { EOPTK = 1; }   /* DBG: &str/String find payload -> quote in {:?} */
        opt_postfix(cell)
    }
}
fn emit_iter_flatten(dv: i32) -> i32 {   /* R2: flatten() on an iterator of Vecs -> concat all inner elements into one Vec */
    unsafe {
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let nd = emit_vec_new();
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let inner = eload(ebin("add", buf, ebin("mul", i, c4)));   /* inner Vec descriptor */
        let ilen = eload(ebin("add", inner, c4)); let ibuf = eload(inner);
        let j = NC; let z2 = ecr(0); ecopy(j, z2); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd2 = ebin("lt", j, ilen); println!("  brz v{}", cnd2); ckreset();
        let el = eload(ebin("add", ibuf, ebin("mul", j, c4))); emit_vec_push(nd, el);
        println!("  cont"); ckreset(); let one2 = ecr(1); ecopy(j, ebin("add", j, one2));
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        println!("  cont"); ckreset(); let one = ecr(1); ecopy(i, ebin("add", i, one));
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        DVELT = 0 - 1; DVELTY = 0 - 1; EISVEC = 1; nd
    }
}
fn emit_iter_findmap(dv: i32) -> i32 {   /* R2as: find_map(|x| -> Option) -- first Some. Closure is LIFTED (parse_closure) so an early `return` inside the body rets from the closure fn, not the enclosing frame. */
    unsafe {
        eat_ch(40); let clo = parse_closure(); eat_ch(41);
        let c4c = ecr(4); let code = eload(clo); let env = eload(ebin("add", clo, c4c));
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let cell = halloc(8); let z0 = ecr(0); println!("  store v{} v{}", cell, z0);
        let found = NC; let zf0 = ecr(0); ecopy(found, zf0); NC = NC + 1;
        let i = NC; let zi = ecr(0); ecopy(i, zi); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let off = ebin("mul", i, c4); let ea = eload(ebin("add", buf, off));   /* R2as: pass the elem VALUE -- the `|&x|` param strips the ref and binds x to the value */
        print!("  v{}: i32 = calli v{} v{} v{}", NC, code, env, ea); println!(""); let opt = NC; NC = NC + 1;
        let disc = eload(opt);
        let zf = ecr(0); let nf = ebin("eq", found, zf); let some = ebin("ne", disc, zf); let cond = ebin("mul", some, nf);
        println!("  brz v{}", cond); ckreset();
        println!("  store v{} v{}", cell, disc);
        let c4b = ecr(4); let pad = ebin("add", cell, c4b); let optpad = eload(ebin("add", opt, c4b));
        println!("  store v{} v{}", pad, optpad);
        let one2 = ecr(1); ecopy(found, one2);
        println!("  pool"); ckreset();
        println!("  cont"); ckreset(); let one = ecr(1); ecopy(i, ebin("add", i, one));
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        EISCLO = 0; EISVEC = 0; EISSTR = 0;   /* R2as: parse_closure left EISCLO=1 -- clear so `let r = ..find_map()` binds r as a scalar Option cell, not a closure */
        opt_postfix(cell)
    }
}
fn emit_iter_fold(dv: i32) -> i32 {
    unsafe {
        eat_ch(40); let init = parse_expr(); let ity = EITYP; let atup = if TYTAG[EITY as usize] == 11 { 1 } else { 0 }; let atupty = ETUPTY; skipws(); eat_ch(44);
        skipws(); eat_ch(124); skipws();
        if atup == 1 && POS < LEN && at(POS) == 40 {   /* R10/tuple: fold with a TUPLE accumulator `fold((0,1),|(s,p),&x| (s+x,p*x))` -- destructure acc each iter, body yields a new tuple, copy back */
            let patpos = POS;
            let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } POS = POS + 1; }   /* skip `(s,p)` */
            skipws(); eat_ch(44); skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; } rd_ident(); let xo = ID_O; let xl = ID_L;
            skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
            eat_ch(124);
            let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
            let acc = NC; ecopy(acc, init); NC = NC + 1;
            let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
            let xslot = NC; NC = NC + 1;
            println!("  loop"); ckreset();
            let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
            let off = ebin("mul", i, c4); let el = eload(ebin("add", buf, off)); ecopy(xslot, el);
            let mark = NSYM;
            let savep = POS; POS = patpos; bind_tuple_pat(acc, 0, atupty); POS = savep;   /* re-bind s,p from acc (loads sit inside the loop -> re-read each iter) */
            sym_add(xo, xl, xslot); SFORREF[(NSYM - 1) as usize] = 1;   /* `&x` element param -> `*x`/`x` is the value */
            let r = parse_expr(); ecopy(acc, r);
            println!("  cont"); ckreset(); let one = ecr(1); let ni = ebin("add", i, one); ecopy(i, ni);
            println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
            NSYM = mark; eat_ch(41); ETUPN = 2; ETUPTY = atupty; EITY_PIN = ty_intern(11, 0, atupty); return acc;
        }
        rd_ident(); let ao = ID_O; let al = ID_L;
        skipws(); eat_ch(44); skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; } rd_ident(); let xo = ID_O; let xl = ID_L;
        skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
        eat_ch(124);
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let acc = NC; ecopy(acc, init); NC = NC + 1;
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        let xslot = NC; NC = NC + 1;
        let mark = NSYM; sym_add(ao, al, acc); sym_add(xo, xl, xslot); mark_forref();
        if ity >= 0 { SST[(NSYM - 2) as usize] = ity; }   /* R2av: struct accumulator -> acc.field dispatches (was a frontend HANG) */
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let off = ebin("mul", i, c4); let el = eload(ebin("add", buf, off)); ecopy(xslot, el);
        let r = parse_expr(); ecopy(acc, r);
        println!("  cont"); ckreset(); let one = ecr(1); let ni = ebin("add", i, one); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        NSYM = mark; eat_ch(41); acc
    }
}
fn emit_iter_stepby(dv: i32) -> i32 {   /* step_by(n) -> every nth element */
    unsafe {
        eat_ch(40); let n = parse_expr(); eat_ch(41);
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let nd = emit_vec_new();
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let off = ebin("mul", i, c4); let el = eload(ebin("add", buf, off)); emit_vec_push(nd, el);
        println!("  cont"); ckreset(); let ni = ebin("add", i, n); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        EISVEC = 1; nd
    }
}
fn emit_iter_takeskip(dv: i32, istake: i32) -> i32 {
    unsafe {
        eat_ch(40); let n = parse_expr(); eat_ch(41);
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let nd = emit_vec_new();
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let off = ebin("mul", i, c4); let el = eload(ebin("add", buf, off));
        if istake == 1 { let lt = ebin("lt", i, n); println!("  brz v{}", lt); ckreset(); emit_vec_push(nd, el); println!("  pool"); ckreset(); }
        else { let ge = ebin("ge", i, n); println!("  brz v{}", ge); ckreset(); emit_vec_push(nd, el); println!("  pool"); ckreset(); }
        println!("  cont"); ckreset(); let one = ecr(1); let ni = ebin("add", i, one); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        EISVEC = 1; nd
    }
}
/* S1 tail: .enumerate().map(|(i,x)| body) and .zip(other).map(|(x,y)| body) fused into an eager build loop. */
fn emit_iter_enum_map(dv: i32) -> i32 {
    unsafe {
        skipws(); if POS < LEN && at(POS) == 46 { POS = POS + 1; rd_ident(); }        /* .map */
        eat_ch(40); skipws(); eat_ch(124); skipws(); eat_ch(40);
        rd_ident(); let io = ID_O; let il = ID_L; skipws(); eat_ch(44); skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; } rd_ident(); let xo = ID_O; let xl = ID_L; skipws(); eat_ch(41); skipws(); eat_ch(124);
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let nd = emit_vec_new();
        let islot = NC; let z = ecr(0); ecopy(islot, z); NC = NC + 1;
        let xslot = NC; NC = NC + 1;
        let mark = NSYM; sym_add(io, il, islot); sym_add(xo, xl, xslot); mark_forref();
        println!("  loop"); ckreset();
        let cnd = ebin("lt", islot, len); println!("  brz v{}", cnd); ckreset();
        let off = ebin("mul", islot, c4); let el = eload(ebin("add", buf, off)); ecopy(xslot, el);
        let r = parse_expr(); let rtup = if TYTAG[EITY as usize] == 11 { 1 } else { 0 }; let rtupty = ETUPTY; emit_vec_push(nd, r);
        println!("  cont"); ckreset(); let one = ecr(1); let ni = ebin("add", islot, one); ecopy(islot, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        NSYM = mark; eat_ch(41); EISVEC = 1;
        if rtup == 1 { VELTY = ty_intern(11, 0, rtupty); }   /* R10/tuple: `.enumerate().map(|..| (a,b)).collect()` -> record the result Vec's element tuple type so `for (i,x) in &pairs` destructures (dvelty TYTAG==11) instead of reading a pointer */
        nd
    }
}
fn emit_iter_zip_map(dv: i32) -> i32 {
    unsafe {
        eat_ch(40); let other = parse_expr(); eat_ch(41);
        skipws(); if POS < LEN && at(POS) == 46 { POS = POS + 1; rd_ident(); }        /* .map */
        eat_ch(40); skipws(); eat_ch(124); skipws(); eat_ch(40);
        skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; } rd_ident(); let xo = ID_O; let xl = ID_L; skipws(); eat_ch(44); skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; } rd_ident(); let yo = ID_O; let yl = ID_L; skipws(); eat_ch(41); skipws(); eat_ch(124);
        let c4 = ecr(4); let lena = eload(ebin("add", dv, c4)); let lenb = eload(ebin("add", other, c4));
        let bufa = eload(dv); let bufb = eload(other);
        let nd = emit_vec_new();
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        let xslot = NC; NC = NC + 1; let yslot = NC; NC = NC + 1;
        let mark = NSYM; sym_add(xo, xl, xslot); mark_forref(); sym_add(yo, yl, yslot); mark_forref();
        println!("  loop"); ckreset();
        let ca = ebin("lt", i, lena); let cb = ebin("lt", i, lenb); let cnd = ebin("mul", ca, cb); println!("  brz v{}", cnd); ckreset();
        let off = ebin("mul", i, c4); let ex = eload(ebin("add", bufa, off)); ecopy(xslot, ex); let ey = eload(ebin("add", bufb, off)); ecopy(yslot, ey);
        let r = parse_expr(); emit_vec_push(nd, r);
        println!("  cont"); ckreset(); let one = ecr(1); let ni = ebin("add", i, one); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        NSYM = mark; eat_ch(41); EISVEC = 1; nd
    }
}
/* R2 (MEET-1 adapters): take_while / skip_while / chain / min_by_key / max_by_key -- eager iterator builders. */
fn emit_iter_takewhile(dv: i32) -> i32 {
    unsafe {
        let twv = DVELT; let twty = DVELTY;
        let po = iter_open_closure(); let pl = ID_L;
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let nd = emit_vec_new();
        let still = NC; let one0 = ecr(1); ecopy(still, one0); NC = NC + 1;
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        let xslot = NC; NC = NC + 1;
        let mark = NSYM; sym_add(po, pl, xslot); mark_forref();
        if twv == 0 - 2 || (twty >= 0 && TYTAG[twty as usize] == 3) { SK[(NSYM - 1) as usize] = 1; SFORREF[(NSYM - 1) as usize] = 0; } else if twty >= 0 && TYTAG[twty as usize] == 4 { SK[(NSYM - 1) as usize] = 3; SFORREF[(NSYM - 1) as usize] = 0; } else if twv >= 0 { SST[(NSYM - 1) as usize] = twv; SFORREF[(NSYM - 1) as usize] = 0; }   /* R20b: &str/String/struct element -> predicate param dispatches */
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let el = eload(ebin("add", buf, ebin("mul", i, c4))); ecopy(xslot, el);
        let pred = parse_expr(); ecopy(still, ebin("mul", still, pred));
        println!("  brz v{}", still); ckreset(); emit_vec_push(nd, el); println!("  pool"); ckreset();
        println!("  cont"); ckreset(); let one = ecr(1); ecopy(i, ebin("add", i, one));
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        NSYM = mark; eat_ch(41); EISBOOL = 0; EISVEC = 1; nd
    }
}
fn emit_iter_skipwhile(dv: i32) -> i32 {
    unsafe {
        let twv = DVELT; let twty = DVELTY;
        let po = iter_open_closure(); let pl = ID_L;
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let nd = emit_vec_new();
        let skp = NC; let one0 = ecr(1); ecopy(skp, one0); NC = NC + 1;
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        let xslot = NC; NC = NC + 1;
        let mark = NSYM; sym_add(po, pl, xslot); mark_forref();
        if twv == 0 - 2 || (twty >= 0 && TYTAG[twty as usize] == 3) { SK[(NSYM - 1) as usize] = 1; SFORREF[(NSYM - 1) as usize] = 0; } else if twty >= 0 && TYTAG[twty as usize] == 4 { SK[(NSYM - 1) as usize] = 3; SFORREF[(NSYM - 1) as usize] = 0; } else if twv >= 0 { SST[(NSYM - 1) as usize] = twv; SFORREF[(NSYM - 1) as usize] = 0; }   /* R20b: &str/String/struct element -> predicate param dispatches */
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let el = eload(ebin("add", buf, ebin("mul", i, c4))); ecopy(xslot, el);
        let pred = parse_expr(); ecopy(skp, ebin("mul", skp, pred));
        let z2 = ecr(0); let keep = ebin("eq", skp, z2);
        println!("  brz v{}", keep); ckreset(); emit_vec_push(nd, el); println!("  pool"); ckreset();
        println!("  cont"); ckreset(); let one = ecr(1); ecopy(i, ebin("add", i, one));
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        NSYM = mark; eat_ch(41); EISBOOL = 0; EISVEC = 1; nd
    }
}
fn emit_iter_chain(dv: i32) -> i32 {
    unsafe {
        eat_ch(40); let other = parse_expr(); eat_ch(41);
        let c4 = ecr(4); let lena = eload(ebin("add", dv, c4)); let bufa = eload(dv);
        let lenb = eload(ebin("add", other, c4)); let bufb = eload(other);
        let nd = emit_vec_new();
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        println!("  loop"); ckreset(); let c1 = ebin("lt", i, lena); println!("  brz v{}", c1); ckreset();
        emit_vec_push(nd, eload(ebin("add", bufa, ebin("mul", i, c4))));
        println!("  cont"); ckreset(); let o1 = ecr(1); ecopy(i, ebin("add", i, o1));
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        let j = NC; let z2 = ecr(0); ecopy(j, z2); NC = NC + 1;
        println!("  loop"); ckreset(); let c2 = ebin("lt", j, lenb); println!("  brz v{}", c2); ckreset();
        emit_vec_push(nd, eload(ebin("add", bufb, ebin("mul", j, c4))));
        println!("  cont"); ckreset(); let o2 = ecr(1); ecopy(j, ebin("add", j, o2));
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        EISVEC = 1; nd
    }
}
fn emit_iter_minmaxkey(dv: i32, ismax: i32) -> i32 {   /* -> Option cell */
    unsafe {
        eat_ch(40); skipws(); eat_ch(124); skipws(); while POS < LEN && at(POS) == 38 { POS = POS + 1; }   /* ALPHABET: accept `|&&(k,_)| k` tuple-pattern key (was iter_open_closure: single-ident only -> tuple returned the whole element) */
        let mut istup = 0; let mut patpos = 0;
        if POS < LEN && at(POS) == 40 { istup = 1; patpos = POS; let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } POS = POS + 1; } }
        else { rd_ident(); }
        let po = ID_O; let pl = ID_L;
        skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
        eat_ch(124);
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let cell = halloc(8); let z = ecr(0); let hasel = ebin("gt", len, z); println!("  store v{} v{}", cell, hasel);
        let best = NC; let z1 = ecr(0); ecopy(best, z1); NC = NC + 1;
        let bestkey = NC; let z2 = ecr(0); ecopy(bestkey, z2); NC = NC + 1;
        let seen = NC; let z3 = ecr(0); ecopy(seen, z3); NC = NC + 1;
        let i = NC; let z4 = ecr(0); ecopy(i, z4); NC = NC + 1;
        let xslot = NC; NC = NC + 1;
        let mark = NSYM; if istup == 0 { sym_add(po, pl, xslot); mark_forref(); }
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let el = eload(ebin("add", buf, ebin("mul", i, c4)));
        if istup == 0 { ecopy(xslot, el); } else { let sv = POS; POS = patpos; bind_tuple_pat(el, 0, 0 - 1); POS = sv; }
        let key = parse_expr();
        let mut cmp = 0; if ismax == 1 { cmp = ebin("gt", key, bestkey); } else { cmp = ebin("lt", key, bestkey); }
        let zs = ecr(0); let notseen = ebin("eq", seen, zs); let better = ebin("add", notseen, cmp);
        println!("  brz v{}", better); ckreset();
        ecopy(best, el); ecopy(bestkey, key); let o1 = ecr(1); ecopy(seen, o1);
        println!("  pool"); ckreset();
        println!("  cont"); ckreset(); let o2 = ecr(1); ecopy(i, ebin("add", i, o2));
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        NSYM = mark; eat_ch(41);
        let c4b = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4b), best);
        cell
    }
}
/* R2 (MEET-1, in-place slice mutators): reverse / dedup / retain(closure). Operate on the [ptr,len] descriptor;
   arrays reach these through the R2b slice route, so one implementation serves Vec + array + slice. */
fn emit_vec_reverse(dv: i32) -> i32 {
    unsafe {
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let lo = NC; let z = ecr(0); ecopy(lo, z); NC = NC + 1;
        let one = ecr(1); let hh = ebin("sub", len, one); let hi = NC; ecopy(hi, hh); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", lo, hi); println!("  brz v{}", cnd); ckreset();
        let alo = ebin("add", buf, ebin("mul", lo, c4)); let ahi = ebin("add", buf, ebin("mul", hi, c4));
        let a = eload(alo); let b = eload(ahi);
        println!("  store v{} v{}", alo, b); println!("  store v{} v{}", ahi, a);
        println!("  cont"); ckreset();
        let o2 = ecr(1); ecopy(lo, ebin("add", lo, o2)); ecopy(hi, ebin("sub", hi, o2));
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        ec(0)
    }
}
fn emit_vec_rotate(dv: i32, isleft: i32) -> i32 {   /* v.rotate_left(n)/rotate_right(n) in place: new[i] = old[(i+n) mod len] (right -> n' = len-n) */
    unsafe {
        skipws(); eat_ch(40); let n0 = parse_expr(); eat_ch(41);
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let nm = ebin("mod", n0, len);
        let mut n = nm; if isleft == 0 { n = ebin("mod", ebin("sub", len, nm), len); }
        let nd = emit_vec_new();
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
        let idx = ebin("mod", ebin("add", i, n), len);
        let el = eload(ebin("add", buf, ebin("mul", idx, c4)));
        emit_vec_push(nd, el);
        println!("  cont"); ckreset(); let o1 = ecr(1); ecopy(i, ebin("add", i, o1));
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        println!("  store v{} v{}", dv, eload(nd));                                 /* point the descriptor at the rotated buffer (in-place mutation) */
        let c4b = ecr(4); println!("  store v{} v{}", ebin("add", dv, c4b), eload(ebin("add", nd, c4b)));
        ec(0)
    }
}
fn emit_vec_dedup(dv: i32) -> i32 {
    unsafe {
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let w = NC; let one = ecr(1); ecopy(w, one); NC = NC + 1;
        let r = NC; let one2 = ecr(1); ecopy(r, one2); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", r, len); println!("  brz v{}", cnd); ckreset();
        let cur = eload(ebin("add", buf, ebin("mul", r, c4)));
        let o1 = ecr(1); let prev = eload(ebin("add", buf, ebin("mul", ebin("sub", w, o1), c4)));
        let differ = ebin("ne", cur, prev); println!("  brz v{}", differ); ckreset();
        println!("  store v{} v{}", ebin("add", buf, ebin("mul", w, c4)), cur);
        let o2 = ecr(1); ecopy(w, ebin("add", w, o2));
        println!("  pool"); ckreset();
        println!("  cont"); ckreset();
        let o3 = ecr(1); ecopy(r, ebin("add", r, o3));
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        let z = ecr(0); let hasel = ebin("gt", len, z); let nl = ebin("mul", w, hasel);
        println!("  store v{} v{}", ebin("add", dv, c4), nl);
        ec(0)
    }
}
fn emit_vec_retain(dv: i32) -> i32 {
    unsafe {
        skipws(); eat_ch(40); skipws(); eat_ch(124); skipws();
        if POS < LEN && at(POS) == 38 { POS = POS + 1; }
        rd_ident(); let xo = ID_O; let xl = ID_L;
        skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
        eat_ch(124);
        let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
        let w = NC; let z = ecr(0); ecopy(w, z); NC = NC + 1;
        let r = NC; let z2 = ecr(0); ecopy(r, z2); NC = NC + 1;
        let mark = NSYM; let xslot = NC; NC = NC + 1; sym_add(xo, xl, xslot); SFORREF[(NSYM - 1) as usize] = 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", r, len); println!("  brz v{}", cnd); ckreset();
        let el = eload(ebin("add", buf, ebin("mul", r, c4))); ecopy(xslot, el);
        let pred = parse_expr(); println!("  brz v{}", pred); ckreset();
        println!("  store v{} v{}", ebin("add", buf, ebin("mul", w, c4)), el);
        let o1 = ecr(1); ecopy(w, ebin("add", w, o1));
        println!("  pool"); ckreset();
        println!("  cont"); ckreset();
        let o2 = ecr(1); ecopy(r, ebin("add", r, o2));
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        NSYM = mark;
        println!("  store v{} v{}", ebin("add", dv, c4), w);
        eat_ch(41);
        ec(0)
    }
}
/* STATEFUL-ITER: a bound `let mut it = v.iter()` is a cursor cell [buf@0, len@4, pos@8].
   .next() -> Some(elem)+advance / None ; .peek() -> Some(&next) / None (no advance). */
fn dispatch_iter(cur: i32) -> i32 {
    unsafe {
        skipws();
        if POS < LEN && at(POS) == 46 {
            let msave = POS; POS = POS + 1; rd_ident();
            if id_is(110, 101, 120, 116, 0, 4) {   /* next() */
                skipws(); eat_ch(40); eat_ch(41);
                let c4 = ecr(4); let c8 = ecr(8);
                let buf = eload(cur); let len = eload(ebin("add", cur, c4)); let pos = eload(ebin("add", cur, c8));
                let more = ebin("lt", pos, len);
                let ea = ebin("add", buf, ebin("mul", pos, c4)); let elem = eload(ea);
                let np = ebin("add", pos, more);   /* advance by 1 only when more (more is 0/1) */
                println!("  store v{} v{}", ebin("add", cur, c8), np);
                let opt = halloc(8); println!("  store v{} v{}", opt, more); println!("  store v{} v{}", ebin("add", opt, c4), elem);
                EISOPT = 1; EOPTRES = 0; EOPTREF = 1; return opt_postfix(opt);   /* iter yields &T -> payload is by-ref (`*x` no-op) */
            }
            if id_is(112, 101, 101, 107, 0, 4) {   /* peek() -> Some(&next)/None, no advance */
                skipws(); eat_ch(40); eat_ch(41);
                let c4 = ecr(4); let c8 = ecr(8);
                let buf = eload(cur); let len = eload(ebin("add", cur, c4)); let pos = eload(ebin("add", cur, c8));
                let more = ebin("lt", pos, len);
                let ea = ebin("add", buf, ebin("mul", pos, c4)); let elem = eload(ea);
                let opt = halloc(8); println!("  store v{} v{}", opt, more); println!("  store v{} v{}", ebin("add", opt, c4), elem);
                EISOPT = 1; EOPTRES = 0; EOPTREF = 1; return opt_postfix(opt);
            }
            POS = msave;   /* not next/peek -- fall through to the eager Vec dispatch (cursor buf@0/len@4 match a Vec descriptor; sum/len/count/map/.. work) */
        }
        return dispatch_vec(cur);
    }
}
/* P-CHARS-ITER: a bound `let mut it = s.chars()` char-cursor [buf@0, len@4, pos@8] over a &str/String's
   bytes (ASCII, 1 byte/char). .next() -> Some(char)+advance / None ; .peek() -> Some(char)/None (no advance).
   Mirrors dispatch_iter but the element load is a BYTE (eloadb) and the payload is tagged EISCHAR so Display
   prints the glyph and `c as u32` works. This is the lexer cursor primitive (rustc_lexer's Cursor wraps Chars). */
fn dispatch_char_iter(cur: i32) -> i32 {
    unsafe {
        skipws();
        if POS < LEN && at(POS) == 46 {
            let msave = POS; POS = POS + 1; rd_ident();
            if id_is(110, 101, 120, 116, 0, 4) {   /* next() */
                skipws(); eat_ch(40); eat_ch(41);
                let c4 = ecr(4); let c8 = ecr(8);
                let buf = eload(cur); let len = eload(ebin("add", cur, c4)); let pos = eload(ebin("add", cur, c8));
                let more = ebin("lt", pos, len);
                let byte = eloadb(ebin("add", buf, pos));
                let np = ebin("add", pos, more);   /* advance by 1 only when more (more is 0/1) */
                println!("  store v{} v{}", ebin("add", cur, c8), np);
                let opt = halloc(8); println!("  store v{} v{}", opt, more); println!("  store v{} v{}", ebin("add", opt, c4), byte);
                EISOPT = 1; EOPTRES = 0; EISCHAR = 1; EISITER = 0;   /* payload is a char (not a &T); LAYER4 R28: clear EISITER -- the .next() RESULT is an Option<char> whose unwrapped payload is a char VALUE, NOT a cursor. Was: EISITER stayed 1 (from the cursor) -> `let b = it.next().unwrap()` matched the `eiter==1 && echar==1 -> SK=13` let-binding rule -> b bound as a char-CURSOR -> `println!("{}",b)` dispatched b as a cursor cell -> loaded the char value as [buf,len,pos] -> crash. */
                let r = opt_postfix(opt); skipws(); if POS < LEN && at(POS) == 46 { EISCHAR = 1; return opt_postfix(r); } return r;
            }
            if id_is(112, 101, 101, 107, 0, 4) {   /* peek() -> Some(char)/None, no advance */
                skipws(); eat_ch(40); eat_ch(41);
                let c4 = ecr(4); let c8 = ecr(8);
                let buf = eload(cur); let len = eload(ebin("add", cur, c4)); let pos = eload(ebin("add", cur, c8));
                let more = ebin("lt", pos, len);
                let byte = eloadb(ebin("add", buf, pos));
                let opt = halloc(8); println!("  store v{} v{}", opt, more); println!("  store v{} v{}", ebin("add", opt, c4), byte);
                EISOPT = 1; EOPTRES = 0; EISCHAR = 1;
                return opt_postfix(opt);
            }
            if id_is(99, 108, 111, 110, 101, 5) {   /* clone() -> a fresh cursor cell copying [buf,len,pos]; advancing the clone leaves the original's pos untouched (rustc_lexer's Cursor.first()/second() peek by cloning Chars) */
                skipws(); eat_ch(40); eat_ch(41);
                let c4 = ecr(4); let c8 = ecr(8);
                let nc = halloc(12);
                println!("  store v{} v{}", nc, eload(cur));
                println!("  store v{} v{}", ebin("add", nc, c4), eload(ebin("add", cur, c4)));
                println!("  store v{} v{}", ebin("add", nc, c8), eload(ebin("add", cur, c8)));
                EISITER = 1; EISCHAR = 1; EISVEC = 0; EISSTR = 0; EISSTRING = 0;   /* re-signal a char-cursor so `let mut c2 = it.clone()` binds SK=13; or chain another cursor method inline */
                skipws(); if POS < LEN && at(POS) == 46 { return dispatch_char_iter(nc); }
                return nc;
            }
            if id_is(97, 115, 95, 115, 116, 6) {   /* as_str() -> the remaining &str from the cursor pos: [buf+pos, len-pos] */
                skipws(); eat_ch(40); eat_ch(41);
                let c4 = ecr(4); let c8 = ecr(8);
                let buf = eload(cur); let len = eload(ebin("add", cur, c4)); let pos = eload(ebin("add", cur, c8));
                let desc = halloc(8);
                println!("  store v{} v{}", desc, ebin("add", buf, pos));
                println!("  store v{} v{}", ebin("add", desc, c4), ebin("sub", len, pos));
                EISSTR = 1; skipws(); if POS < LEN && (at(POS) == 46 || at(POS) == 91) { return dispatch_string(desc); }   /* LAYER4 P-CHARS-ITER: `self.chars.as_str()` -> a &str [ptr,len] descriptor; a chained `.len()/.is_empty()/[i]/...` must route through dispatch_string (the &str dispatcher), NOT opt_postfix (which mis-handled `.len()` on a &str -> crash; rustc_lexer pos_within_token = len_remaining - self.chars.as_str().len()) */
                return desc;
            }
            POS = msave;   /* not next/peek/clone/as_str -- fall through */
        }
        EISSTR = 1; return cur;
    }
}
/* STATEFUL-ITER: a `splitn(n,sep)` str-cursor [buf@0, len@4, pos@8, remaining@12, sep@16].
   .next() -> Some(&str piece)/None. remaining>1: piece up to the next sep (advance past it, remaining--);
   remaining==1 (last): the whole rest; None once pos>len. */
fn dispatch_split_iter(cur: i32) -> i32 {
    unsafe {
        skipws();
        if POS < LEN && at(POS) == 46 {
            let msave = POS; POS = POS + 1; rd_ident();
            if id_is(110, 101, 120, 116, 0, 4) {   /* next() */
                skipws(); eat_ch(40); eat_ch(41);
                let c4 = ecr(4); let c8 = ecr(8); let c12 = ecr(12); let c16 = ecr(16); let one = ecr(1);
                let buf = eload(cur); let len = eload(ebin("add", cur, c4)); let pos = eload(ebin("add", cur, c8));
                let rem = eload(ebin("add", cur, c12)); let sep = eload(ebin("add", cur, c16));
                let more = ebin("le", pos, len);   /* a piece is available while pos <= len */
                let rest = ebin("sub", len, pos);  /* bytes remaining */
                let pp = ebin("add", buf, pos);    /* piece start ptr */
                /* seg = (rem>1) ? __str_seglen(pp, rest, sep) : rest  (last piece keeps the whole rest) */
                print!("  v{}: i32 = call __str_seglen v{} v{} v{}", NC, pp, rest, sep); println!(""); let seg0 = NC; NC = NC + 1;
                let lastp = ebin("le", rem, one);   /* remaining<=1 -> take the rest */
                let seg = ebin("add", ebin("mul", lastp, rest), ebin("mul", ebin("sub", one, lastp), seg0));   /* lastp ? rest : seg0 */
                /* advance: newpos = pos + seg + 1 (skip the sep; when last, jumps past len -> None next) */
                let np = ebin("add", ebin("add", pos, seg), one);
                println!("  store v{} v{}", ebin("add", cur, c8), np);
                println!("  store v{} v{}", ebin("add", cur, c12), ebin("sub", rem, one));   /* remaining-- */
                let piece = halloc(8); println!("  store v{} v{}", piece, pp); println!("  store v{} v{}", ebin("add", piece, c4), seg);
                let opt = halloc(8); println!("  store v{} v{}", opt, more); println!("  store v{} v{}", ebin("add", opt, c4), piece);
                EISOPT = 1; EOPTRES = 0; EOPTK = 1;   /* payload is a &str descriptor */
                return opt_postfix(opt);
            }
            POS = msave;
        }
        return cur;
    }
}
fn dispatch_vec(dv0: i32) -> i32 {
    unsafe {
        let mut dv = dv0;
        let dvelt0 = DVELT;   /* R2ax: capture element kind before any parse_expr resets the global */
        let dvelty0 = DVELTY;   /* BVK N4: capture the element interned Ty before parse_expr(iv) resets it, so a scalar v[i] read can restore a bool/i64/char element kind */
        let dvelenum0 = DVELENUM;   /* R24b: capture the element enum-type for a `v[i]` read */
        loop {
        skipws();
        if POS < LEN && at(POS) == 46 {
            let msave = POS; POS = POS + 1; rd_ident();
            EISITER = 0;   /* STATEFUL-ITER: reset per method; only a terminal iter/peekable (below) re-sets it */
            if id_is(99, 108, 111, 110, 101, 5) {   /* .clone() on a Vec -> DEEP copy (new buffer + element copy); preserve element kind/Ty. Was unmatched (only `cloned` (6) matched) -> fell to `return dv` untagged, so `let s=v.clone()` bound a non-Vec. */
                skipws(); eat_ch(40); eat_ch(41);
                let c4 = ecr(4); let oldbuf = eload(dv); let oldlen = eload(ebin("add", dv, c4));
                let nd = emit_vec_new(); let newbuf = eload(nd);
                println!("  store v{} v{}", ebin("add", nd, c4), oldlen);
                let ci = NC; let z = ecr(0); ecopy(ci, z); NC = NC + 1;
                println!("  loop"); ckreset(); let cnd = ebin("lt", ci, oldlen); println!("  brz v{}", cnd); ckreset();
                let off = ebin("mul", ci, c4); let val = eload(ebin("add", oldbuf, off));
                println!("  store v{} v{}", ebin("add", newbuf, off), val);
                println!("  cont"); ckreset(); let one = ecr(1); ecopy(ci, ebin("add", ci, one)); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                dv = nd; EISVEC = 1; DVELT = dvelt0; DVELTY = dvelty0; continue;
            }
            if id_is(108, 101, 110, 0, 0, 3) { skipws(); eat_ch(40); eat_ch(41); let c4 = ecr(4); let ad = ebin("add", dv, c4); return eload(ad); }   /* len */
            if id_is(99, 111, 117, 110, 116, 5) { skipws(); eat_ch(40); eat_ch(41); if HAS_RVEC == 1 && (dvelt0 == 0 || dvelt0 == 0 - 1) { return emit_rvec_count(dv); } let c4 = ecr(4); let ad = ebin("add", dv, c4); return eload(ad); }   /* count (std-hoist P4.2: route to rvec_count when std present) */
            if id_is(105, 116, 101, 114, 0, 4) || id_is(99, 108, 111, 110, 101, 6) || id_is(99, 111, 112, 105, 101, 6) || id_is(105, 110, 116, 111, 95, 9) || id_is(112, 101, 101, 107, 97, 8) { skipws(); eat_ch(40); eat_ch(41); EISITER = 1; if dvelt0 >= 0 { EVELT = dvelt0; } if dvelenum0 >= 0 { EVELENUM = dvelenum0; } }   /* iter/cloned/copied/into_iter/peekable -> no-op, chain on; STATEFUL-ITER: mark a possible cursor terminus. LAYER7 P-VEC-STRUCT-ELEM: thread the element struct-type/enum-type onto EVELT/EVELENUM so `for a in v.iter() { a.field }` binds the loop var struct/enum-typed (like `for a in &v`; was: only DVELT/VELT set at 6364, not EVELT -> the loop var untagged -> `.field` dropped). */
            else if id_is(112, 101, 101, 107, 0, 4) { skipws(); eat_ch(40); eat_ch(41); let pc = emit_iter_peek(dv); EISOPT = 1; EOPTREF = 1; return opt_postfix(pc); }   /* R2cj: peek -> Option<&first> (`*x` no-op) */
            else if id_is(115, 117, 109, 0, 0, 3) { skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }   /* P-VECVEC: sum::<T>() turbofish -- erase ::<..> before the arg list (was left -> `.sum::<i32>()` fell to a generic `call sum`) */
                skipws(); eat_ch(40); eat_ch(41); if HAS_RVEC == 1 && (dvelt0 == 0 || dvelt0 == 0 - 1) { return emit_rvec_sum(dv); } return emit_iter_sum(dv); }   /* sum (std-hoist P4.2: route to rvec_sum) */
            else if id_is(112, 114, 111, 100, 117, 7) { skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }   /* NAMERES: product::<T>() turbofish -- erase ::<..> like sum (was left -> `.product::<i32>()` leaked to the free-call site) */
                skipws(); eat_ch(40); eat_ch(41); return emit_iter_product(dv); }   /* product() */
            else if id_is(109, 97, 120, 0, 0, 3) { skipws(); eat_ch(40); eat_ch(41); let ety = DVELTY; let mc = emit_iter_minmax(dv, 1); EOPTTY = ety; EISOPT = 1; EOPTREF = 1; return opt_postfix(mc); }   /* max -> Option<&T> (`*x` no-op) */
            else if id_is(109, 105, 110, 0, 0, 3) { skipws(); eat_ch(40); eat_ch(41); let ety = DVELTY; let mc = emit_iter_minmax(dv, 0); EOPTTY = ety; EISOPT = 1; EOPTREF = 1; return opt_postfix(mc); }   /* min -> Option<&T> (`*x` no-op) */
            else if id_is(114, 101, 118, 0, 0, 3) { skipws(); eat_ch(40); eat_ch(41); dv = emit_iter_rev(dv); }   /* rev -> chain on */
            else if id_is(99, 111, 108, 108, 101, 7) { skipws(); let mut tostr = 0; if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); if POS < LEN && at(POS) == 60 { let gs = POS + 1; let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } let mut q = gs; while q + 6 <= POS { if is_string_ty(q, 6) { tostr = 1; } q = q + 1; } } } eat_ch(40); eat_ch(41); if tostr == 1 || ANNSTR == 1 { return emit_charvec_to_string(dv); } EISVEC = 1; return dv; }   /* collect -> the vec; TI3e-i: `::<String>` or a `let s: String =` annotation -> pack the char-Vec into a String */
            else if id_is(115, 112, 108, 105, 116, 8) { skipws(); eat_ch(40); let nn = parse_expr(); eat_ch(41);   /* ADAPT: split_at(n) -> (left,right) tuple of [ptr,len] slice descriptors sharing the buffer */
                let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
                let ld = halloc(8); println!("  store v{} v{}", ld, buf); println!("  store v{} v{}", ebin("add", ld, c4), nn);
                let rbuf = ebin("add", buf, ebin("mul", nn, c4));
                let rd = halloc(8); println!("  store v{} v{}", rd, rbuf); println!("  store v{} v{}", ebin("add", rd, c4), ebin("sub", len, nn));
                let tup = halloc(8); println!("  store v{} v{}", tup, ld); println!("  store v{} v{}", ebin("add", tup, c4), rd);
                ETUPN = 2; ETUPSLICE = 1; EITY_PIN = ty_intern(11, 0, ETUPTY); return tup; }
            else if id_is(115, 99, 97, 110, 0, 4) {   /* ADAPT: scan(init, |acc, &x| { .. Some(v) }) -> Vec of yielded payloads; acc is a &mut cell so `*acc` reads/writes it */
                skipws(); eat_ch(40); let init = parse_expr(); skipws(); eat_ch(44); skipws();
                eat_ch(124); skipws();
                if POS < LEN && at(POS) == 38 { POS = POS + 1; } rd_ident(); let ao = ID_O; let al = ID_L;
                skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 44 && at(POS) != 124 { POS = POS + 1; } }
                eat_ch(44); skipws();
                if POS < LEN && at(POS) == 38 { POS = POS + 1; } rd_ident(); let xo = ID_O; let xl = ID_L;
                skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
                eat_ch(124); skipws();
                let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
                let acell = halloc(4); println!("  store v{} v{}", acell, init);
                let accslot = NC; ecopy(accslot, acell); NC = NC + 1;
                let nd = emit_vec_new();
                let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
                let xslot = NC; NC = NC + 1;
                let mark = NSYM; sym_add(ao, al, accslot); sym_add(xo, xl, xslot);
                println!("  loop"); ckreset(); let cnd = ebin("lt", i, len); println!("  brz v{}", cnd); ckreset();
                let el = eload(ebin("add", buf, ebin("mul", i, c4))); ecopy(xslot, el);
                let rslot = NC; NC = NC + 1;
                if POS < LEN && at(POS) == 123 { parse_branch_into(rslot); } else { let rv = parse_expr(); ecopy(rslot, rv); }
                let payload = eload(ebin("add", rslot, c4)); emit_vec_push(nd, payload);
                println!("  cont"); ckreset(); let one = ecr(1); ecopy(i, ebin("add", i, one)); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                NSYM = mark; eat_ch(41);
                EISVEC = 1; DVELT = 0 - 1; return dispatch_vec(nd); }
            else if id_is(109, 97, 112, 0, 0, 3) { if HAS_RVEC == 1 && (dvelt0 == 0 || dvelt0 == 0 - 1) { dv = emit_rvec_map(dv); } else { dv = emit_iter_map(dv); } }        /* map(closure) -> chain on (std-hoist P4.2: route to rvec_map) */
            else if id_is(102, 105, 108, 116, 101, 10) { dv = emit_iter_filtermap(dv); }   /* filter_map(|x| Option) -> keep Some payload, drop None; chain on */
            else if id_is(102, 105, 108, 116, 101, 6) { if HAS_RVEC == 1 && (dvelt0 == 0 || dvelt0 == 0 - 1) { dv = emit_rvec_filter(dv); } else { dv = emit_iter_filter(dv); } }   /* filter(closure) -> chain on (std-hoist P4.2: route to rvec_filter) */
            else if id_is(97, 110, 121, 0, 0, 3) { if HAS_RVEC == 1 && (dvelt0 == 0 || dvelt0 == 0 - 1) { return emit_rvec_anyall(dv, 0); } return emit_iter_anyall(dv, 0); }   /* any -> bool (std-hoist P4.2: route to rvec_any) */
            else if id_is(97, 108, 108, 0, 0, 3) { if HAS_RVEC == 1 && (dvelt0 == 0 || dvelt0 == 0 - 1) { return emit_rvec_anyall(dv, 1); } return emit_iter_anyall(dv, 1); }   /* all -> bool (std-hoist P4.2: route to rvec_all) */
            else if id_is(102, 111, 108, 100, 0, 4) { if HAS_RVEC == 1 && (dvelt0 == 0 || dvelt0 == 0 - 1) { return emit_rvec_fold(dv); } return emit_iter_fold(dv); }     /* fold(init, |acc,x| ..) (std-hoist P4.2: route to rvec_fold) */
            else if id_is(102, 105, 110, 100, 95, 8) { return emit_iter_findmap(dv); }   /* R2as: find_map(|x| Option) -> first Some */
            else if id_is(102, 105, 110, 100, 0, 4) { return emit_iter_findpos(dv, 0); }   /* find -> Option<elem> */
            else if id_is(112, 111, 115, 105, 116, 8) { return emit_iter_findpos(dv, 1); }   /* position -> Option<idx> */
            else if id_is(116, 97, 107, 101, 0, 4) { dv = emit_iter_takeskip(dv, 1); }   /* take(n) -> chain on */
            else if id_is(115, 107, 105, 112, 0, 4) { dv = emit_iter_takeskip(dv, 0); }   /* skip(n) -> chain on */
            else if id_is(101, 110, 117, 109, 101, 9) { skipws(); eat_ch(40); eat_ch(41); dv = emit_iter_enum_map(dv); }   /* enumerate().map(|(i,x)|..) fused */
            else if id_is(122, 105, 112, 0, 0, 3) { dv = emit_iter_zip_map(dv); }        /* zip(other).map(|(x,y)|..) fused */
            else if id_is(102, 108, 97, 116, 95, 8) { dv = emit_iter_flatmap(dv); }      /* flat_map(|x| iter) */
            else if id_is(102, 108, 97, 116, 116, 7) { skipws(); eat_ch(40); eat_ch(41); dv = emit_iter_flatten(dv); }   /* R2: flatten() on Vec<Vec> iter -> concat inner elements */
            else if id_is(106, 111, 105, 110, 0, 4) {   /* R26b: Vec<&str>/Vec<String>.join(SEP) -> a String (elements concatenated with SEP between) */
                skipws(); eat_ch(40); let sep = parse_expr(); eat_ch(41);
                let c4 = ecr(4); let buf = eload(dv); let ln = eload(ebin("add", dv, c4));
                let dst = emit_str_new();
                let ji = NC; let z = ecr(0); ecopy(ji, z); NC = NC + 1;
                println!("  loop"); ckreset(); let cnd = ebin("lt", ji, ln); println!("  brz v{}", cnd); ckreset();
                let z2 = ecr(0); let gt = ebin("gt", ji, z2); println!("  brz v{}", gt); ckreset();
                emit_str_push_str(dst, sep);   /* separator before every element except the first */
                println!("  pool"); ckreset();
                let elem = eload(ebin("add", buf, ebin("mul", ji, c4)));
                emit_str_push_str(dst, elem);
                println!("  cont"); ckreset(); let one = ecr(1); ecopy(ji, ebin("add", ji, one)); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                EISSTR = 1; EISSTRING = 1; return dst;
            }
            else if id_is(115, 116, 101, 112, 95, 7) { dv = emit_iter_stepby(dv); }      /* step_by(n) */
            else if id_is(116, 97, 107, 101, 95, 10) { dv = emit_iter_takewhile(dv); }    /* take_while(|x| pred) */
            else if id_is(115, 107, 105, 112, 95, 10) { dv = emit_iter_skipwhile(dv); }   /* skip_while(|x| pred) */
            else if id_is(99, 104, 97, 105, 110, 5) { dv = emit_iter_chain(dv); }         /* chain(other) */
            else if id_is(109, 105, 110, 95, 98, 10) { let ety = DVELTY; let mc = emit_iter_minmaxkey(dv, 0); EOPTTY = ety; EISOPT = 1; EOPTREF = 1; return opt_postfix(mc); }   /* min_by_key(|x| k) -> Option<&T> */
            else if id_is(109, 97, 120, 95, 98, 10) { let ety = DVELTY; let mc = emit_iter_minmaxkey(dv, 1); EOPTTY = ety; EISOPT = 1; EOPTREF = 1; return opt_postfix(mc); }    /* max_by_key(|x| k) -> Option<&T> */
            else {
            POS = msave; POS = POS + 1; rd_ident();
            if id_is(112, 117, 115, 104, 0, 4) { skipws(); eat_ch(40); let e = parse_expr(); eat_ch(41); if HAS_RVEC == 1 { emit_rvec_push(dv, e); } else { emit_vec_push(dv, e); } return ec(0); }        /* push (std-hoist P4.1: route to RVec_push when std present) */
            if id_is(115, 111, 114, 116, 0, 4) { skipws(); eat_ch(40); eat_ch(41);   /* R6: sort() -- ascending bubble sort in place */
                let p = eload(dv); let c4 = ecr(4); let n = eload(ebin("add", dv, c4));
                print!("  v{}: i32 = call __vec_sort v{} v{}", NC, p, n); println!(""); NC = NC + 1;
                return ec(0); }
            if id_is(115, 111, 114, 116, 95, 13) { skipws(); eat_ch(40); eat_ch(41);   /* R2: sort_unstable() == sort() */
                let p = eload(dv); let c4 = ecr(4); let n = eload(ebin("add", dv, c4));
                print!("  v{}: i32 = call __vec_sort v{} v{}", NC, p, n); println!(""); NC = NC + 1;
                return ec(0); }
            if id_is(115, 111, 114, 116, 95, 7) {                                    /* R2: sort_by(|a,b| ord) -- comparator bubble sort (swap when cmp>0) */
                skipws(); eat_ch(40); skipws(); eat_ch(124); skipws();
                if POS < LEN && at(POS) == 38 { POS = POS + 1; } rd_ident(); let ao = ID_O; let al = ID_L; skipws(); eat_ch(44); skipws();
                if POS < LEN && at(POS) == 38 { POS = POS + 1; } rd_ident(); let bo = ID_O; let bl = ID_L;
                skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
                eat_ch(124);
                let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
                let aslot = NC; NC = NC + 1; let bslot = NC; NC = NC + 1;
                let mark = NSYM; sym_add(ao, al, aslot); mark_forref(); sym_add(bo, bl, bslot); mark_forref();
                let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
                let one = ecr(1); let lm1 = ebin("sub", len, one);
                println!("  loop"); ckreset(); let oc = ebin("lt", i, len); println!("  brz v{}", oc); ckreset();
                let k = NC; let z2 = ecr(0); ecopy(k, z2); NC = NC + 1;
                println!("  loop"); ckreset(); let ic = ebin("lt", k, lm1); println!("  brz v{}", ic); ckreset();
                let c4b = ecr(4); let ak = ebin("add", buf, ebin("mul", k, c4b)); let o1 = ecr(1); let ak1 = ebin("add", buf, ebin("mul", ebin("add", k, o1), c4b));
                let av = eload(ak); let bv = eload(ak1); ecopy(aslot, av); ecopy(bslot, bv);
                let cmp = parse_expr();
                let z3 = ecr(0); let sw = ebin("gt", cmp, z3); println!("  brz v{}", sw); ckreset();
                println!("  store v{} v{}", ak, bv); println!("  store v{} v{}", ak1, av);
                println!("  pool"); ckreset();
                println!("  cont"); ckreset(); let o2 = ecr(1); ecopy(k, ebin("add", k, o2)); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                println!("  cont"); ckreset(); let o3 = ecr(1); ecopy(i, ebin("add", i, o3)); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                NSYM = mark; eat_ch(41); return ec(0);
            }
            if id_is(115, 111, 114, 116, 95, 11) {                                   /* R2: sort_by_key(|x| key) -- key bubble sort (re-parse key for both elems) */
                let selt = DVELT;                                                    /* R2: element struct-type so `|p| p.field` key dispatches */
                skipws(); eat_ch(40); skipws(); eat_ch(124); skipws();
                if POS < LEN && at(POS) == 38 { POS = POS + 1; } rd_ident(); let xo = ID_O; let xl = ID_L;
                skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
                eat_ch(124);
                let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
                let xslot = NC; NC = NC + 1;
                let mark = NSYM; sym_add(xo, xl, xslot); mark_forref();
                if selt >= 0 { SST[(NSYM - 1) as usize] = selt; }                    /* R2: struct-typed key var -> p.field reads the field */
                let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
                let one = ecr(1); let lm1 = ebin("sub", len, one);
                println!("  loop"); ckreset(); let oc = ebin("lt", i, len); println!("  brz v{}", oc); ckreset();
                let k = NC; let z2 = ecr(0); ecopy(k, z2); NC = NC + 1;
                println!("  loop"); ckreset(); let ic = ebin("lt", k, lm1); println!("  brz v{}", ic); ckreset();
                let c4b = ecr(4); let ak = ebin("add", buf, ebin("mul", k, c4b)); let o1 = ecr(1); let ak1 = ebin("add", buf, ebin("mul", ebin("add", k, o1), c4b));
                let av = eload(ak); let bv = eload(ak1);
                let bodypos = POS; ecopy(xslot, av); let kav = parse_expr(); let endpos = POS;
                let ka = NC; ecopy(ka, kav); NC = NC + 1;   /* stabilize: the identity key `|&x| x` returns xslot itself, which the next ecopy(xslot,bv) would redefine -> ka==kb, never swaps (312) */
                ecopy(xslot, bv); POS = bodypos; let kb = parse_expr(); POS = endpos;
                let gt = ebin("gt", ka, kb); println!("  brz v{}", gt); ckreset();
                println!("  store v{} v{}", ak, bv); println!("  store v{} v{}", ak1, av);
                println!("  pool"); ckreset();
                println!("  cont"); ckreset(); let o2 = ecr(1); ecopy(k, ebin("add", k, o2)); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                println!("  cont"); ckreset(); let o3 = ecr(1); ecopy(i, ebin("add", i, o3)); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                NSYM = mark; eat_ch(41); return ec(0);
            }
            if id_is(101, 120, 116, 101, 110, 6) { skipws(); eat_ch(40); skipws();   /* R2: extend(other) -- append other's elems */
                if POS < LEN && at(POS) == 38 { POS = POS + 1; } let other = parse_expr(); eat_ch(41);
                let c4 = ecr(4); let olen = eload(ebin("add", other, c4)); let obuf = eload(other);
                let j = NC; let z = ecr(0); ecopy(j, z); NC = NC + 1;
                println!("  loop"); ckreset(); let cnd = ebin("lt", j, olen); println!("  brz v{}", cnd); ckreset();
                let el = eload(ebin("add", obuf, ebin("mul", j, c4))); emit_vec_push(dv, el);
                println!("  cont"); ckreset(); let o1 = ecr(1); ecopy(j, ebin("add", j, o1)); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                return ec(0); }
            if id_is(112, 111, 112, 0, 0, 3) {                                                                                                        /* pop -> Option<i32> */
                skipws(); eat_ch(40); eat_ch(41);
                let c4 = ecr(4); let lad = ebin("add", dv, c4); let len = eload(lad);
                let cell = halloc(8);
                let lb = LBL; LBL = LBL + 1;
                let z = ecr(0); let gt = ebin("gt", len, z);
                println!("  brz v{}", gt); ckreset();
                let one = ecr(1); let n = ebin("sub", len, one);
                let c4b = ecr(4); let lad2 = ebin("add", dv, c4b); println!("  store v{} v{}", lad2, n);
                let p = eload(dv); let c4c = ecr(4); let off = ebin("mul", n, c4c); let ea = ebin("add", p, off); let val = eload(ea);
                let s1 = ecr(1); println!("  store v{} v{}", cell, s1);
                let c4d = ecr(4); let cad = ebin("add", cell, c4d); println!("  store v{} v{}", cad, val);
                println!("  goto __Lp{}", lb); ckreset();
                println!("  pool"); ckreset();
                let z2 = ecr(0); println!("  store v{} v{}", cell, z2);
                println!("  label __Lp{}", lb); ckreset();
                EITYP = DVELT;                                                                                                                             /* R2an: pop() over Vec<Struct> -> Option payload carries the elem struct type */
                return opt_postfix(cell);   /* R2ck: chain a directly-applied `.unwrap()`/`.unwrap_or(..)` etc. on pop()'s Option */
            }
            if id_is(114, 101, 118, 101, 114, 7) { skipws(); eat_ch(40); eat_ch(41); return emit_vec_reverse(dv); }   /* R2: reverse() in place */
            if ID_L == 11 && at(ID_O) == 114 && at(ID_O + 7) == 108 { return emit_vec_rotate(dv, 1); }   /* rotate_left(n) */
            if ID_L == 12 && at(ID_O) == 114 && at(ID_O + 7) == 114 { return emit_vec_rotate(dv, 0); }   /* rotate_right(n) */
            if id_is(100, 101, 100, 117, 112, 5) { skipws(); eat_ch(40); eat_ch(41); return emit_vec_dedup(dv); }     /* R2: dedup() consecutive */
            if id_is(98, 105, 110, 97, 114, 13) {                               /* R2bh: binary_search(&x) -> Result<usize,usize> [disc,payload]; disc=1 Ok(idx), disc=0 Err(insert) */
                skipws(); eat_ch(40); skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; } let needle = parse_expr(); eat_ch(41);
                let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let buf = eload(dv);
                let lo = NC; let z = ecr(0); ecopy(lo, z); NC = NC + 1;
                let hi = NC; ecopy(hi, len); NC = NC + 1;
                println!("  loop"); ckreset();
                let cnd = ebin("lt", lo, hi); println!("  brz v{}", cnd); ckreset();        /* the SOLE bare brz: lower-bound loop, branchless body (cf single-brz) */
                let two = ecr(2); let mid = ebin("div", ebin("add", lo, hi), two);
                let midv = eload(ebin("add", buf, ebin("mul", mid, c4)));
                let one = ecr(1); let cond = ebin("lt", midv, needle); let notc = ebin("sub", one, cond);
                let nlo = ebin("add", ebin("mul", ebin("add", mid, one), cond), ebin("mul", lo, notc));
                let nhi = ebin("add", ebin("mul", hi, cond), ebin("mul", mid, notc));
                println!("  cont"); ckreset();
                ecopy(lo, nlo); ecopy(hi, nhi);
                println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                let inb = ebin("lt", lo, len); let si = ebin("mul", lo, inb);    /* clamp read to buf[0] when lo==len */
                let sv = eload(ebin("add", buf, ebin("mul", si, c4)));
                let eqv = ebin("eq", sv, needle); let found = ebin("mul", inb, eqv);
                let cell = halloc(8); println!("  store v{} v{}", cell, found);
                let c4b = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4b), lo);
                return opt_postfix(cell);
            }
            if id_is(114, 101, 116, 97, 105, 6) { return emit_vec_retain(dv); }                                      /* R2: retain(|x| pred) */
            if id_is(99, 111, 110, 116, 97, 8) {                                /* R2: contains(&x) -> bool */
                skipws(); eat_ch(40); skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; } let needle = parse_expr(); eat_ch(41);
                let p = eload(dv); let c4 = ecr(4); let n = eload(ebin("add", dv, c4));
                print!("  v{}: i32 = call __vec_idxof v{} v{} v{}", NC, p, n, needle); println!(""); let idx = NC; NC = NC + 1;
                let z = ecr(0); let r = ebin("ge", idx, z); EISBOOL = 1; return r;
            }
            if id_is(103, 101, 116, 0, 0, 3) {                                  /* RUSTC-FE LEX: get(i) -> Option<&T> (bounds-checked; mirrors first()/last()) -- was UNHANDLED for a Vec (fell through -> `*v.get(i).unwrap()` segfaulted) */
                skipws(); eat_ch(40); let idx = parse_expr(); eat_ch(41);
                let p = eload(dv); let c4 = ecr(4); let n = eload(ebin("add", dv, c4));
                let cell = halloc(8);
                let z = ecr(0); let inb = ebin("mul", ebin("ge", idx, z), ebin("lt", idx, n));   /* 0 <= i < len -> Some */
                println!("  store v{} v{}", cell, inb);
                let c4e = ecr(4); let ea = ebin("add", p, ebin("mul", idx, c4e)); let vl = eload(ea);   /* the element value (payload; ignored when None) */
                let c4b = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4b), vl);
                EISOPT = 1; EOPTREF = 1; if dvelt0 == 0 - 2 || (dvelty0 >= 0 && TYTAG[dvelty0 as usize] == 3) { EOPTK = 1; } else if dvelty0 >= 0 && TYTAG[dvelty0 as usize] == 4 { EOPTK = 3; } else { EOPTK = 0; }
                return opt_postfix(cell);   /* get(i) yields Option<&T> -- `*x` is a no-op */
            }
            if id_is(102, 105, 114, 115, 116, 5) {                              /* R2: first() -> Option<&T> */
                skipws(); eat_ch(40); eat_ch(41);
                let p = eload(dv); let c4 = ecr(4); let n = eload(ebin("add", dv, c4));
                let cell = halloc(8); let z = ecr(0); let gt = ebin("gt", n, z); println!("  store v{} v{}", cell, gt);
                let v0 = eload(p); let c4b = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4b), v0);
                EISOPT = 1; EOPTREF = 1; if dvelt0 == 0 - 2 || (dvelty0 >= 0 && TYTAG[dvelty0 as usize] == 3) { EOPTK = 1; } else if dvelty0 >= 0 && TYTAG[dvelty0 as usize] == 4 { EOPTK = 3; } else { EOPTK = 0; }   /* DBG: &str/String element -> Option payload kind so `{:?}` quotes it */
                return opt_postfix(cell);   /* R2aj: first() yields Option<&T> -- `*x` is a no-op */
            }
            if id_is(108, 97, 115, 116, 0, 4) {                                 /* R2: last() -> Option<&T> */
                skipws(); eat_ch(40); eat_ch(41);
                let p = eload(dv); let c4 = ecr(4); let n = eload(ebin("add", dv, c4));
                let cell = halloc(8); let z = ecr(0); let gt = ebin("gt", n, z); println!("  store v{} v{}", cell, gt);
                let one = ecr(1); let idx = ebin("sub", n, one); let c4b = ecr(4); let ea = ebin("add", p, ebin("mul", idx, c4b)); let vl = eload(ea);
                let c4c = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4c), vl);
                EISOPT = 1; EOPTREF = 1; if dvelt0 == 0 - 2 || (dvelty0 >= 0 && TYTAG[dvelty0 as usize] == 3) { EOPTK = 1; } else if dvelty0 >= 0 && TYTAG[dvelty0 as usize] == 4 { EOPTK = 3; } else { EOPTK = 0; }
                return opt_postfix(cell);   /* R2aj: last() yields Option<&T> -- `*x` is a no-op */
            }
            if id_is(99, 108, 101, 97, 114, 5) { skipws(); eat_ch(40); eat_ch(41);   /* R2: clear() */
                let c4 = ecr(4); let z = ecr(0); println!("  store v{} v{}", ebin("add", dv, c4), z); return ec(0); }
            if id_is(105, 115, 95, 101, 109, 8) { skipws(); eat_ch(40); eat_ch(41);   /* R2: is_empty() -> bool */
                let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let z = ecr(0); let r = ebin("eq", len, z); EISBOOL = 1; return r; }
            if id_is(116, 114, 117, 110, 99, 8) { skipws(); eat_ch(40); let n = parse_expr(); eat_ch(41);   /* R2: truncate(n) -> len=min(n,len) */
                let c4 = ecr(4); let lad = ebin("add", dv, c4); let len = eload(lad);
                let res = NC; ecopy(res, len); NC = NC + 1;
                let lt = ebin("lt", n, len); println!("  brz v{}", lt); ckreset(); ecopy(res, n); println!("  pool"); ckreset();
                println!("  store v{} v{}", lad, res); return ec(0); }
            if id_is(115, 119, 97, 112, 0, 4) { skipws(); eat_ch(40); let i = parse_expr(); skipws(); eat_ch(44); let j = parse_expr(); eat_ch(41);   /* R2: swap(i,j) */
                let buf = eload(dv); let c4 = ecr(4);
                let ai = ebin("add", buf, ebin("mul", i, c4)); let aj = ebin("add", buf, ebin("mul", j, c4));
                let a = eload(ai); let b = eload(aj);
                println!("  store v{} v{}", ai, b); println!("  store v{} v{}", aj, a); return ec(0); }
            if id_is(100, 114, 97, 105, 110, 5) { skipws(); eat_ch(40);   /* PTR-ROOT: drain(a..b) -> Vec of removed [a,b); shift the tail left, shrink v; chainable (.sum()/.collect()) */
                let a = parse_expr(); skipws();
                if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 { POS = POS + 2; }
                let b = parse_expr(); eat_ch(41);
                let c4 = ecr(4); let lad = ebin("add", dv, c4); let len = eload(lad); let buf = eload(dv);
                let nd = emit_vec_new();
                let i = NC; ecopy(i, a); NC = NC + 1;                              /* collect drained [a,b) */
                println!("  loop"); ckreset(); let cnd = ebin("lt", i, b); println!("  brz v{}", cnd); ckreset();
                let el = eload(ebin("add", buf, ebin("mul", i, c4))); emit_vec_push(nd, el);
                println!("  cont"); ckreset(); let o1 = ecr(1); ecopy(i, ebin("add", i, o1)); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                let k = NC; ecopy(k, a); NC = NC + 1;                              /* shift tail: buf[k] = buf[k+(b-a)] for k=a.. len-(b-a) */
                let gap = ebin("sub", b, a); let nlen = ebin("sub", len, gap);
                println!("  loop"); ckreset(); let cnd2 = ebin("lt", k, nlen); println!("  brz v{}", cnd2); ckreset();
                let src = eload(ebin("add", buf, ebin("mul", ebin("add", k, gap), c4)));
                println!("  store v{} v{}", ebin("add", buf, ebin("mul", k, c4)), src);
                println!("  cont"); ckreset(); let o2 = ecr(1); ecopy(k, ebin("add", k, o2)); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                println!("  store v{} v{}", lad, nlen);                            /* shrink v */
                DVELT = 0 - 1; EISVEC = 1; return dispatch_vec(nd); }
            if id_is(115, 119, 97, 112, 95, 11) { skipws(); eat_ch(40); let i = parse_expr(); eat_ch(41);   /* PTR-ROOT: swap_remove(i) -> buf[i], replace buf[i] with last, len-=1 */
                let c4 = ecr(4); let lad = ebin("add", dv, c4); let len = eload(lad); let buf = eload(dv);
                let ai = ebin("add", buf, ebin("mul", i, c4));
                let x = NC; ecopy(x, eload(ai)); NC = NC + 1;
                let one = ecr(1); let lm1 = ebin("sub", len, one);
                let lastv = eload(ebin("add", buf, ebin("mul", lm1, c4)));
                println!("  store v{} v{}", ai, lastv); println!("  store v{} v{}", lad, lm1);
                EITYP = DVELT; return x; }
            if id_is(114, 101, 109, 111, 118, 6) { skipws(); eat_ch(40); let i = parse_expr(); eat_ch(41);   /* R2: remove(i) -> removed elem; shift left */
                let c4 = ecr(4); let lad = ebin("add", dv, c4); let len = eload(lad); let buf = eload(dv);
                let saved = NC; ecopy(saved, eload(ebin("add", buf, ebin("mul", i, c4)))); NC = NC + 1;
                let k = NC; ecopy(k, i); NC = NC + 1;
                let one = ecr(1); let lm1 = ebin("sub", len, one);
                println!("  loop"); ckreset();
                let cnd = ebin("lt", k, lm1); println!("  brz v{}", cnd); ckreset();
                let c4b = ecr(4); let dst = ebin("add", buf, ebin("mul", k, c4b));
                let o1 = ecr(1); let src = ebin("add", buf, ebin("mul", ebin("add", k, o1), c4b));
                println!("  store v{} v{}", dst, eload(src));
                println!("  cont"); ckreset();
                let o2 = ecr(1); ecopy(k, ebin("add", k, o2));
                println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                println!("  store v{} v{}", lad, lm1); return saved; }
            if id_is(105, 110, 115, 101, 114, 6) { skipws(); eat_ch(40); let i = parse_expr(); skipws(); eat_ch(44); let val = parse_expr(); eat_ch(41);   /* R2: insert(i,v) -> shift right, store at i */
                let c4 = ecr(4); let lad = ebin("add", dv, c4); let len = eload(lad); let buf = eload(dv);
                let k = NC; ecopy(k, len); NC = NC + 1;
                println!("  loop"); ckreset();
                let cnd = ebin("gt", k, i); println!("  brz v{}", cnd); ckreset();
                let c4b = ecr(4); let dst = ebin("add", buf, ebin("mul", k, c4b));
                let o1 = ecr(1); let src = ebin("add", buf, ebin("mul", ebin("sub", k, o1), c4b));
                println!("  store v{} v{}", dst, eload(src));
                println!("  cont"); ckreset();
                let o2 = ecr(1); ecopy(k, ebin("sub", k, o2));
                println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                let ai = ebin("add", buf, ebin("mul", i, c4)); println!("  store v{} v{}", ai, val);
                let one = ecr(1); println!("  store v{} v{}", lad, ebin("add", len, one)); return ec(0); }
            return dv;                                                          /* unknown method -> stop the chain */
            }                                                                   /* close else (non-adapter methods) */
            continue;                                                           /* an adapter set dv -> chain on to the next method */
        }
        if POS < LEN && at(POS) == 91 {                                                                                                              /* v[i] read (chained [i][j].. for Vec<Vec<..>>) */
            POS = POS + 1; let iv = parse_expr(); eat_ch(93);
            let p = eload(dv); let c4 = ecr(4); let off = ebin("mul", iv, c4); let ea = ebin("add", p, off);
            let mut cur = eload(ea);
            skipws();
            while POS < LEN && at(POS) == 91 {                                                                                                        /* cur is an inner Vec descriptor */
                POS = POS + 1; let iv2 = parse_expr(); eat_ch(93);
                let p2 = eload(cur); let c4b = ecr(4); let off2 = ebin("mul", iv2, c4b); let ea2 = ebin("add", p2, off2); cur = eload(ea2);
                skipws();
            }
            if dvelt0 == 0 - 2 { skipws(); if POS < LEN && at(POS) == 46 { EISSTR = 1; return dispatch_string(cur); } EISSTR = 1; }   /* R2ax: Vec<&str> element -> &str descriptor pointer */
            if dvelt0 == 0 - 3 { EISVEC = 1; skipws(); if POS < LEN && (at(POS) == 46 || at(POS) == 91) { DVELT = 0 - 1; return dispatch_vec(cur); } }   /* container-kind K2: Vec<Vec> element -> inner Vec (.iter()/.len()/[j] dispatch) */
            if dvelt0 == 0 - 5 { skipws(); if POS < LEN && at(POS) == 46 { return dispatch_dyn(cur); } return cur; }   /* D003 P-VEC-DYN-ELEM: a `Vec<Box<dyn Trait>>`/`Vec<&dyn Trait>` element (DVELT sentinel -5) -> route a chained `v[i].method()` through dispatch_dyn (per-element vtable-tag dispatch; was: mis-dispatched to a fixed concrete method). Mirrors the -4 HashMap / -3 Vec<Vec> element sentinels. */
            if dvelt0 == 0 - 4 { EISHM = 1; skipws(); if POS < LEN && at(POS) == 46 { return dispatch_hashmap(cur); } return cur; }   /* D006 P-VEC-HASHMAP-ELEM: a `Vec<HashMap>` element (SVELT sentinel -4) -> route a chained `v[i].get/insert/len/contains_key/..` through dispatch_hashmap (the scopes-stack idiom `self.scopes[i].get(&k)`; was: untagged -> HashMap method mis-dispatched/parse-drift). Mirrors the -2 &str / -3 Vec<Vec> element sentinels. */
            if dvelty0 >= 0 && TYTAG[dvelty0 as usize] == 6 { EISBOOL = 1; }   /* BVK N4: Vec<bool> element read -> tag EISBOOL so `!v[i]` takes the bool path (x==0, not int bitwise-NOT 0-x-1) and Display shows true/false (was: !true -> -2 -> truthy, dijkstra never relaxed) */
            if dvelenum0 >= 0 { EISENUM = 1; EIENUMTY = dvelenum0;   /* R24b: Vec<Enum> element -> tag so `{:?}(v[i])` / `match v[i]` dispatch */
                if POS + 7 < LEN && at(POS) == 46 && at(POS + 1) == 99 && at(POS + 2) == 108 && at(POS + 3) == 111 && at(POS + 4) == 110 && at(POS + 5) == 101 && at(POS + 6) == 40 && at(POS + 7) == 41 { POS = POS + 8; return emit_enum_clone(dvelenum0, cur); }   /* LAYER6 P-SLICE-ELEM-ENUM-CLONE: `v[i].clone()` / `p.toks[i].clone()` chained directly on the indexed enum element -> emit_enum_clone (was: generic undefined `call clone` -> crash on a struct-field slice). Matched as the EXACT byte sequence `.clone()` (no ws, no rd_ident) to leave the non-clone postfix path untouched. */
                if POS < LEN && at(POS) == 46 { let mut q = POS + 1; let mo = q; while q < LEN && is_idc(at(q)) { q = q + 1; } let ml = q - mo; let mut r = q; while r < LEN && at(r) == 32 { r = r + 1; } if ml > 0 && r < LEN && at(r) == 40 && method_exists(0 - (dvelenum0 + 2), mo, ml) == 1 { POS = r + 1; return emit_enum_mcall(dvelenum0, mo, ml, cur); } } }   /* LAYER6 P-SLICE-ELEM-ENUM-METHOD: `v[i].method()` / `p.toks[i].kind()` -- a real inherent method on the indexed enum element -> emit_enum_mcall (was: mis-dispatched -> a pointer/garbage). Byte-scanned the method-name span (no rd_ident, no rewind -- R17-safe); only fires when method_exists confirms a real enum method followed by `(`, so a `.field`/`{:?}` postfix is untouched. */
            if dvelty0 >= 0 && TYTAG[dvelty0 as usize] == 3 { skipws(); if POS < LEN && at(POS) == 46 { EISSTR = 1; return dispatch_string(cur); } EISSTR = 1; }   /* R27b: bound Vec<&str> element (carried via dvelty0 TYTAG==3, not dvelt0==-2) -> Display/method as &str (was a pointer) */
            if dvelty0 >= 0 && TYTAG[dvelty0 as usize] == 4 { skipws(); if POS < LEN && at(POS) == 46 { EISSTR = 1; EISSTRING = 1; return dispatch_string(cur); } EISSTR = 1; EISSTRING = 1; }   /* R27b: bound Vec<String> element */
            if dvelt0 >= 0 { EITYP = dvelt0; skipws(); if POS < LEN && at(POS) == 46 { return dispatch_struct(cur, dvelt0); } }   /* R32b: Vec<Struct> element -> route a chained `v[i].field`/`v[i].method()` through dispatch_struct (was: returned the raw struct pointer -> a trailing method/field was dropped) */
            skipws(); if POS < LEN && at(POS) == 46 { EISVEC = 0; return opt_postfix(cur); }   /* LAYER9 P-SLICE-SCALAR-ELEM-POSTFIX: a plain scalar/bool/i64 `v[i]` element followed by a `.method()` (`.clone()`, `.abs()`, ...) -> route through opt_postfix (scalar dispatch), clearing the Vec channel first. Was: the scalar `[i]` branch returned `cur` with the trailing postfix UNCONSUMED, so a bare `v[i].clone()`/`v[i].method()` in a fn tail leaked back to a Vec re-dispatch (dispatch_vec's deep-clone) -> a heap pointer instead of the element. Mirrors the typed-element branches above (4217-4219) which already route a trailing `.` through their element dispatcher. A `let x = v[i].clone()` masked it by consuming the postfix at the let seam. */
            return cur;
        }
        EISVEC = 1;                                                             /* DBG: bare Vec descriptor (no method/index consumed) IS a Vec -> so `println!("{:?}", v)` on a bound Vec sets AVEC (was 0 -> printed a pointer) */
        return dv;
        }
    }
}

/* R5: postfix Option/Result ops on a cell value v -- `?`, .unwrap(), .unwrap_or(d), .is_some/none/ok/err().
   disc==1 = success (Some/Ok), disc==0 = fail (None/Err); payload at v+4. Rewinds a non-matching `.method`. */
fn opt_postfix(v: i32) -> i32 {
    unsafe {
        let wasopt = EISOPT; let wasres = EOPTRES; EISOPT = 0;   /* container-kind (a): extracting methods clear is-Option; bare-cell returns restore */
        skipws();
        if POS < LEN && at(POS) == 63 {                                       /* v?  -> early-return on fail */
            if CUR_RET_RES == 1 && wasres == 0 { die_qmark(); }               /* CHECK N4: E0277 -- `?` on an Option (wasres==0) in a `-> Result` fn -> REJECT (conservative: only this exact rustc-rejected shape) */
            POS = POS + 1;
            let disc = eload(v); let z = ecr(0); let isfail = ebin("eq", disc, z);
            println!("  brz v{}", isfail); ckreset();
            println!("  ret v{}", v);
            println!("  pool"); ckreset();
            let c4 = ecr(4); let pa = ebin("add", v, c4); if EOPTREF == 1 { EWASREF = 1; } return eload(pa);   /* R2: `?` on Option<&T> (find/get/first) -- the extracted payload is a ref, so a bound `let x = ..?` gets `*x` no-op (mirrors unwrap) */
        }
        if POS < LEN && at(POS) == 46 {
            EITY_PIN = 0 - 1;   /* FLAG-RETIRE pin-hardening: a `.` postfix (field/method/unwrap) TRANSFORMS the receiver -- the receiver's pinned Ty is invalid for the result, so clear it (the result's own flags/pin set the Ty). Prevents a sub-expression producer's pin leaking to the enclosing expr (the (3,4).0 class). */
            let s = POS;
            POS = POS + 1;
            if POS < LEN && is_dig(at(POS)) {                                      /* R1: t.N tuple field -> load(v + N*4) */
                let mut fi = 0;
                while POS < LEN && is_dig(at(POS)) { fi = fi * 10 + (at(POS) - 48); POS = POS + 1; }
                let fk = tupty_fk(ETUPTY, fi);                                  /* tuple-field-kind: restore field kind (&str/String/Vec/char/i64) so Display isn't a pointer */
                let off = ec(fi * 4); let ad = ebin("add", v, off); let r = opt_postfix(eload(ad)); apply_kind(fk); return r;
            }
            rd_ident();
            if id_is(116, 97, 107, 101, 0, 4) { skipws(); if POS < LEN && at(POS) == 40 { let tk = POS; POS = POS + 1; skipws(); if POS < LEN && at(POS) == 41 { POS = POS + 1;
                /* P-STRUCT-FIELD-OPTION: `o.take()` on an Option CELL value v -- return the OLD Option (a copy of the [disc,payload] cell) + set the ORIGINAL cell's disc to 0 (None) IN PLACE (`v` is the cell pointer, shared with the field slot / local, so the None persists). Empty-parens guarded (Iterator::take(n) has an arg). Makes field.take() CORRECT (leave-None), not a silent miscompile. */
                let nc = halloc(8); let c4 = ecr(4);
                let d0 = eload(v); println!("  store v{} v{}", nc, d0);
                let p0 = eload(ebin("add", v, c4)); println!("  store v{} v{}", ebin("add", nc, c4), p0);
                let z = ecr(0); println!("  store v{} v{}", v, z);
                EISOPT = 1; return opt_postfix(nc);
            } POS = tk; } }
            if id_is(105, 110, 116, 111, 0, 4) && INTO_TARGET >= 0 {                  /* R2cc: `expr.into()` with a known target type T -> call T_from(expr) */
                skipws(); eat_ch(40); eat_ch(41);
                let tgt = INTO_TARGET; INTO_TARGET = 0 - 1;
                print!("  v{}: i32 = call ", NC); emit_span(TSO[tgt as usize], TSL[tgt as usize]); print!("_from"); println!(" v{}", v);
                NC = NC + 1; EITYP = tgt; return opt_postfix(NC - 1);
            }
            if id_is(99, 108, 111, 110, 101, 5) { skipws(); eat_ch(40); eat_ch(41); return v; }   /* A4: .clone() on a scalar/descriptor -> identity (shallow) */
            if id_is(105, 115, 95, 97, 108, 13) {                                /* char.is_alphabetic() -> a-z || A-Z */
                skipws(); eat_ch(40); eat_ch(41);
                let az = ebin("mul", ebin("ge", v, ec(97)), ebin("le", v, ec(122)));
                let auz = ebin("mul", ebin("ge", v, ec(65)), ebin("le", v, ec(90)));
                let r = ebin("add", az, auz); EISCHAR = 0; EISBOOL = 1; return opt_postfix(r);
            }
            if id_is(105, 115, 95, 97, 108, 15) {                                /* char.is_alphanumeric() -> a-z || A-Z || 0-9 */
                skipws(); eat_ch(40); eat_ch(41);
                let az = ebin("mul", ebin("ge", v, ec(97)), ebin("le", v, ec(122)));
                let auz = ebin("mul", ebin("ge", v, ec(65)), ebin("le", v, ec(90)));
                let d09 = ebin("mul", ebin("ge", v, ec(48)), ebin("le", v, ec(57)));
                let r = ebin("add", ebin("add", az, auz), d09); EISCHAR = 0; EISBOOL = 1; return opt_postfix(r);
            }
            if id_is(105, 115, 95, 110, 117, 10) {                               /* LAYER2: char.is_numeric() -> 0-9 (ASCII) */
                skipws(); eat_ch(40); eat_ch(41);
                let r = ebin("mul", ebin("ge", v, ec(48)), ebin("le", v, ec(57))); EISCHAR = 0; EISBOOL = 1; return opt_postfix(r);
            }
            if id_is(105, 115, 95, 97, 115, 14) && at(ID_O + 9) == 100 {          /* LAYER2: char.is_ascii_digit() -> 0-9 */
                skipws(); eat_ch(40); eat_ch(41);
                let r = ebin("mul", ebin("ge", v, ec(48)), ebin("le", v, ec(57))); EISCHAR = 0; EISBOOL = 1; return opt_postfix(r);
            }
            if id_is(105, 115, 95, 97, 115, 8) {                                 /* LAYER4: char/u8.is_ascii() -> v <= 127 (bool). Was unrecognized -> parser mis-consumed the following `{` in an if-EXPRESSION condition (rustc_lexer unescape MixedUnit::from). */
                skipws(); eat_ch(40); eat_ch(41);
                let r = ebin("le", v, ec(127)); EISCHAR = 0; EISBOOL = 1; return opt_postfix(r);
            }
            if id_is(105, 115, 95, 97, 115, 19) {                                /* LAYER6 P-CHAR-IS-ASCII-ALPHA: char.is_ascii_alphabetic() (len 19) -> a-z || A-Z (bool). Was UNRECOGNIZED -> the char-method dispatch fell through returning the RECEIVER char (identity), so `is_id_continue(c)=c.is_ascii_alphanumeric()||c=='_'` saw a truthy char -> always true -> rustc_lexer eat_while(is_id_continue) never terminated -> the whole input lexed as ONE token (the single-token-collapse). */
                skipws(); eat_ch(40); eat_ch(41);
                let az = ebin("mul", ebin("ge", v, ec(97)), ebin("le", v, ec(122)));
                let auz = ebin("mul", ebin("ge", v, ec(65)), ebin("le", v, ec(90)));
                let r = ebin("add", az, auz); EISCHAR = 0; EISBOOL = 1; return opt_postfix(r);
            }
            if id_is(105, 115, 95, 97, 115, 21) {                                /* LAYER6 P-CHAR-IS-ASCII-ALPHA: char.is_ascii_alphanumeric() (len 21) -> a-z || A-Z || 0-9 (bool). Same identity-fallthrough defect as is_ascii_alphabetic; this is the exact predicate rustc_lexer's is_id_continue uses. */
                skipws(); eat_ch(40); eat_ch(41);
                let az = ebin("mul", ebin("ge", v, ec(97)), ebin("le", v, ec(122)));
                let auz = ebin("mul", ebin("ge", v, ec(65)), ebin("le", v, ec(90)));
                let d09 = ebin("mul", ebin("ge", v, ec(48)), ebin("le", v, ec(57)));
                let r = ebin("add", ebin("add", az, auz), d09); EISCHAR = 0; EISBOOL = 1; return opt_postfix(r);
            }
            if id_is(105, 115, 95, 101, 109, 13) {                               /* LAYER4: char.is_emoji_char() -> false (keeper-stubbed unicode; non-ASCII emoji classification is out of scope). Was UNRECOGNIZED -> the char-method dispatch fell through WITHOUT consuming `.is_emoji_char()`, so POS stuck at `.is_emoji_char` -> in the emoji match arm `c if !c.is_ascii() && c.is_emoji_char() => ..` the guard parse_expr returned early, eat_ch(=>) desynced, the arm made no progress, R24 broke the whole `match first_char` early, and the arm loop's NSYM=mark dropped self's binding -> every later `self.method()` (line_comment's scrutinee) lost its struct styp -> parse drift. */
                skipws(); eat_ch(40); eat_ch(41);
                let r = ec(0); EISCHAR = 0; EISBOOL = 1; return opt_postfix(r);
            }
            if id_is(105, 115, 95, 119, 104, 13) {                               /* LAYER2: char.is_whitespace() -> space/tab/nl/cr */
                skipws(); eat_ch(40); eat_ch(41);
                let sp = ebin("eq", v, ec(32)); let tb = ebin("eq", v, ec(9)); let nl = ebin("eq", v, ec(10)); let cr = ebin("eq", v, ec(13));
                let r = ebin("add", ebin("add", sp, tb), ebin("add", nl, cr)); EISCHAR = 0; EISBOOL = 1; return opt_postfix(r);
            }
            if id_is(116, 111, 95, 97, 115, 18) && at(ID_O + 9) == 117 {          /* char.to_ascii_uppercase() -> a-z minus 32 */
                skipws(); eat_ch(40); eat_ch(41);
                let islow = ebin("mul", ebin("ge", v, ec(97)), ebin("le", v, ec(122)));
                let r = ebin("sub", v, ebin("mul", islow, ec(32))); EISCHAR = 1; return opt_postfix(r);
            }
            if id_is(116, 111, 95, 97, 115, 18) && at(ID_O + 9) == 108 {          /* char.to_ascii_lowercase() -> A-Z plus 32 */
                skipws(); eat_ch(40); eat_ch(41);
                let isup = ebin("mul", ebin("ge", v, ec(65)), ebin("le", v, ec(90)));
                let r = ebin("add", v, ebin("mul", isup, ec(32))); EISCHAR = 1; return opt_postfix(r);
            }
            if id_is(116, 111, 95, 100, 105, 8) {                                /* char.to_digit(radix) -> Option<u32> (Some(d) if d<radix) */
                skipws(); eat_ch(40); let radix = parse_expr(); eat_ch(41);
                let ok09 = ebin("mul", ebin("ge", v, ec(48)), ebin("le", v, ec(57)));
                let okaz = ebin("mul", ebin("ge", v, ec(97)), ebin("le", v, ec(122)));
                let okup = ebin("mul", ebin("ge", v, ec(65)), ebin("le", v, ec(90)));
                let d09 = ebin("sub", v, ec(48)); let daz = ebin("sub", v, ec(87)); let dup = ebin("sub", v, ec(55));   /* a->10 (97-87), A->10 (65-55) */
                let anyok = ebin("add", ebin("add", ok09, okaz), okup); let noneok = ebin("sub", ec(1), anyok);
                let d = ebin("add", ebin("add", ebin("add", ebin("mul", d09, ok09), ebin("mul", daz, okaz)), ebin("mul", dup, okup)), ebin("mul", ec(99), noneok));
                let valid = ebin("lt", d, radix);
                let cell = halloc(8); println!("  store v{} v{}", cell, valid); let c4 = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4), d);
                EISOPT = 1; EOPTRES = 0; EOPTK = 0; EISCHAR = 0; return opt_postfix(cell);
            }
            if id_is(116, 111, 95, 115, 116, 9) || id_is(116, 111, 95, 111, 119, 8) {   /* R2ck: int.to_string()/.to_owned() -> format into a new arena String (str receiver = identity) */
                skipws(); eat_ch(40); eat_ch(41);
                if EISSTR == 1 || EISSTRING == 1 { return v; }
                let is64 = EIS64;
                let desc = emit_str_new();
                if is64 == 1 { println!("  v{}: i32 = call __push_int64 v{} v{}", NC, desc, v); NC = NC + 1; }
                else { println!("  v{}: i32 = call __push_int v{} v{}", NC, desc, v); NC = NC + 1; }
                EISSTR = 1; EISSTRING = 1; EIS64 = 0; EIS8 = 0;
                return desc;
            }
            if id_is(99, 111, 112, 105, 101, 6) || id_is(99, 108, 111, 110, 101, 6) { skipws(); eat_ch(40); eat_ch(41); return opt_postfix(v); }   /* Option::copied()/cloned() -> no-op (payload already by-value), chain on */
            if EISFLT == 1 && (id_is(102, 108, 111, 111, 114, 5) || id_is(99, 101, 105, 108, 0, 4) || id_is(114, 111, 117, 110, 100, 5)) {   /* LAYER2: f64 floor()/ceil()/round() on the fixed-point (scale 1e9) value v */
                let isfloor = id_is(102, 108, 111, 111, 114, 5); let isceil = id_is(99, 101, 105, 108, 0, 4);
                skipws(); eat_ch(40); eat_ch(41);
                let sc = emit_i64_lit(1000000000); let z0 = emit_i64_lit(0);
                let q = ebin8(3, v, sc); let base = ebin8(2, q, sc); let rem = ebin8(1, v, base);   /* base = trunc-toward-zero; rem = fractional part (sign of v) */
                let res = NC; NC = NC + 1; ecopy64(res, base);
                if isfloor == true {                                                   /* floor: if rem<0 (v<0 w/ frac) -> base - 1 */
                    let remneg = emit_binop("lt", rem, z0, 1, 0); let baseminus = ebin8(1, base, sc);
                    println!("  brz v{}", remneg); ckreset(); ecopy64(res, baseminus); println!("  pool"); ckreset();
                } else if isceil == true {                                             /* ceil: if rem>0 (v>0 w/ frac) -> base + 1 */
                    let rempos = emit_binop("gt", rem, z0, 1, 0); let baseplus = ebin8(0, base, sc);
                    println!("  brz v{}", rempos); ckreset(); ecopy64(res, baseplus); println!("  pool"); ckreset();
                } else {                                                               /* round: half away from zero -- if |rem| >= 5e8 add sign(v)*1 */
                    let half = emit_i64_lit(500000000);
                    let remneg = emit_binop("lt", rem, z0, 1, 0); let negrem = ebin8(1, z0, rem);
                    let absrem = NC; NC = NC + 1; ecopy64(absrem, rem); println!("  brz v{}", remneg); ckreset(); ecopy64(absrem, negrem); println!("  pool"); ckreset();
                    let adj = NC; NC = NC + 1; ecopy64(adj, sc); let negsc = ebin8(1, z0, sc); println!("  brz v{}", remneg); ckreset(); ecopy64(adj, negsc); println!("  pool"); ckreset();   /* adj = +1 (v>=0) / -1 (v<0) */
                    let gehalf = emit_binop("ge", absrem, half, 1, 0); let baseadj = ebin8(0, base, adj);
                    println!("  brz v{}", gehalf); ckreset(); ecopy64(res, baseadj); println!("  pool"); ckreset();
                }
                EISFLT = 1; EIS64 = 1; return opt_postfix(res);
            }
            if ID_L == 12 && at(ID_O) == 119 && at(ID_O + 1) == 114 && at(ID_O + 2) == 97 && at(ID_O + 8) == 95 {   /* E4: wrapping_add/sub/mul -> plain op (32-bit is natural u32 wraparound); u8 receiver masks 0xFF; chains */
                let op9 = at(ID_O + 9);
                let recv8 = EIS8;                                                  /* R1: capture receiver width before the arg clobbers EIS8 */
                let recv64 = EIS64;                                                /* TI3e-p: capture receiver 64-bit-ness before the arg clobbers EIS64 */
                let recvu = EISU;                                                  /* value-width: capture receiver unsignedness (u32) so the wrap result Displays unsigned */
                skipws(); eat_ch(40); let b = parse_expr(); let b64 = EIS64; eat_ch(41);
                if recv64 == 1 {   /* TI3e-p: i64 receiver -> 64-bit wrapping op (was a plain 32-bit ebin -> truncated to 32 bits, e.g. a checksum hash) */
                    let w = w64(recv64, b64);
                    let mut wr64 = v;
                    if op9 == 97 { wr64 = i64op("add", v, recv64, b, b64, w); }
                    else if op9 == 115 { wr64 = i64op("sub", v, recv64, b, b64, w); }
                    else if op9 == 109 { wr64 = i64op("mul", v, recv64, b, b64, w); }
                    EIS64 = 1; EISU = recvu; return opt_postfix(wr64);
                }
                let mut wr = v;
                if op9 == 97 { wr = ebin("add", v, b); }
                if op9 == 115 { wr = ebin("sub", v, b); }
                if op9 == 109 { wr = ebin("mul", v, b); }
                if recv8 == 1 { let lo = ebin("and", wr, ec(255)); EIS8 = 1; return opt_postfix(lo); }   /* u8 wraparound = low 8 bits */
                EISU = recvu; return opt_postfix(wr);   /* u32 wrap: natural 32-bit; carry unsigned so a negative-bit result prints unsigned */
            }
            if id_is(99, 111, 117, 110, 116, 10) {                               /* R2: int .count_ones() -> popcount (u64/i64 -> 64-bit __popcount64; else 32-bit __popcount) */
                let r64 = EIS64;
                skipws(); eat_ch(40); eat_ch(41);
                if r64 == 1 {
                    let cn = ecr(64); print!("  v{}: i64 = call __popcount64 v{} v{}", NC, v, cn); println!(""); let r = NC; NC = NC + 1; EIS64 = 1; return opt_postfix(r);
                }
                let cn = ecr(32); print!("  v{}: i32 = call __popcount v{} v{}", NC, v, cn); println!(""); let r = NC; NC = NC + 1; EIS64 = 0; return opt_postfix(r);
            }
            if id_is(97, 98, 115, 0, 0, 3) {                                      /* R2: i32/i64 .abs() -- value-width: 64-bit ops + EIS64 when receiver is i64 */
                let r64 = EIS64;
                skipws(); eat_ch(40); eat_ch(41);
                let z = ecr(0); let isneg = emit_binop("lt", v, z, r64, 0); let neg = emit_binop("sub", z, v, r64, r64);
                let res = NC; NC = NC + 1; if r64 == 1 { ecopy64(res, v); } else { ecopy(res, v); }
                println!("  brz v{}", isneg); ckreset(); if r64 == 1 { ecopy64(res, neg); } else { ecopy(res, neg); } println!("  pool"); ckreset();
                EIS64 = r64; return opt_postfix(res);
            }
            if id_is(115, 105, 103, 110, 117, 6) {                                /* LAYER2: i32/i64 .signum() -> -1/0/1 */
                let r64 = EIS64;
                skipws(); eat_ch(40); eat_ch(41);
                let z = ecr(0); let pos = emit_binop("gt", v, z, r64, 0); let neg = emit_binop("lt", v, z, r64, 0);
                let r = ebin("sub", pos, neg); EIS64 = 0; return opt_postfix(r);
            }
            if id_is(99, 108, 97, 109, 112, 5) {                                  /* LAYER2: i32/i64 .clamp(lo, hi) -> max(v,lo) then min(_,hi) */
                let r64 = EIS64;
                skipws(); eat_ch(40); let lo = parse_expr(); skipws(); eat_ch(44); let hi = parse_expr(); skipws(); eat_ch(41);
                let gtlo = emit_binop("gt", v, lo, r64, 0); let t = NC; NC = NC + 1; if r64 == 1 { ecopy64(t, lo); } else { ecopy(t, lo); }
                println!("  brz v{}", gtlo); ckreset(); if r64 == 1 { ecopy64(t, v); } else { ecopy(t, v); } println!("  pool"); ckreset();
                let lthi = emit_binop("lt", t, hi, r64, 0); let r = NC; NC = NC + 1; if r64 == 1 { ecopy64(r, hi); } else { ecopy(r, hi); }
                println!("  brz v{}", lthi); ckreset(); if r64 == 1 { ecopy64(r, t); } else { ecopy(r, t); } println!("  pool"); ckreset();
                EIS64 = r64; return opt_postfix(r);
            }
            if id_is(109, 105, 110, 0, 0, 3) {                                    /* R2: i32/i64 .min(b) */
                let r64 = EIS64;
                skipws(); eat_ch(40); let b = parse_expr(); eat_ch(41);
                let lt = emit_binop("lt", v, b, r64, 0); let res = NC; NC = NC + 1; if r64 == 1 { ecopy64(res, b); } else { ecopy(res, b); }
                println!("  brz v{}", lt); ckreset(); if r64 == 1 { ecopy64(res, v); } else { ecopy(res, v); } println!("  pool"); ckreset();
                EIS64 = r64; return opt_postfix(res);
            }
            if id_is(109, 97, 120, 0, 0, 3) {                                     /* R2: i32/i64 .max(b) */
                let r64 = EIS64;
                skipws(); eat_ch(40); let b = parse_expr(); eat_ch(41);
                let gt = emit_binop("gt", v, b, r64, 0); let res = NC; NC = NC + 1; if r64 == 1 { ecopy64(res, b); } else { ecopy(res, b); }
                println!("  brz v{}", gt); ckreset(); if r64 == 1 { ecopy64(res, v); } else { ecopy(res, v); } println!("  pool"); ckreset();
                EIS64 = r64; return opt_postfix(res);
            }
            if id_is(99, 109, 112, 0, 0, 3) {                                     /* R2: i32/i64 .cmp(&b) -> Ordering as i32 (1/0/-1); 64-bit compares when receiver is i64 */
                let r64 = EIS64;
                skipws(); eat_ch(40); skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; } let b = parse_expr(); eat_ch(41);
                let g = emit_binop("gt", v, b, r64, 0); let l = emit_binop("lt", v, b, r64, 0); let r = ebin("sub", g, l);
                EIS64 = 0; return opt_postfix(r);
            }
            if id_is(112, 111, 119, 0, 0, 3) {                                    /* R2: i32/i64 .pow(e) -- repeated multiply; 64-bit mul when receiver is i64 */
                let r64 = EIS64;
                skipws(); eat_ch(40); let e = parse_expr(); eat_ch(41);
                let res = NC; let one = ecr(1); if r64 == 1 { ecopy64(res, one); } else { ecopy(res, one); } NC = NC + 1;
                let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
                println!("  loop"); ckreset(); let cnd = ebin("lt", i, e); println!("  brz v{}", cnd); ckreset();
                let nr = emit_binop("mul", res, v, r64, r64); if r64 == 1 { ecopy64(res, nr); } else { ecopy(res, nr); }
                let o2 = ecr(1); let ni = ebin("add", i, o2); ecopy(i, ni);
                println!("  cont"); ckreset(); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                EIS64 = r64; return opt_postfix(res);
            }
            if id_is(114, 101, 109, 95, 101, 10) {                                /* R2: rem_euclid(b) -> ((a%b)+b)%b */
                skipws(); eat_ch(40); let b = parse_expr(); eat_ch(41);
                let m = ebin("mod", v, b); let s = ebin("add", m, b); let u = ebin("mod", s, b);
                return opt_postfix(u);
            }
            if id_is(105, 115, 95, 112, 111, 15) {                                /* R2: is_power_of_two() -> x>0 && (x&(x-1))==0 */
                skipws(); eat_ch(40); eat_ch(41);
                let z = ecr(0); let one = ecr(1); let xm1 = ebin("sub", v, one); let band = ebin("and", v, xm1);
                let isz = ebin("eq", band, z); let pos = ebin("gt", v, z); let r = ebin("and", pos, isz);
                EISBOOL = 1; return r;
            }
            if ID_L == 11 && at(ID_O) == 99 && at(ID_O + 1) == 104 && at(ID_O + 7) == 95 {   /* R2: checked_add/sub/mul(b) -> Option<i32> (i64 overflow detect) */
                let op8 = at(ID_O + 8);
                let recv8 = EIS8;
                skipws(); eat_ch(40); let b = parse_expr(); eat_ch(41);
                if recv8 == 1 {   /* value-width: u8 checked_add/sub -> None when the result leaves 0..=255 */
                    let mut op = "add"; if op8 == 115 { op = "sub"; }
                    let sum = ebin(op, v, b); let lo = ebin("and", sum, ec(255)); let ov = ebin("ne", sum, lo);
                    let out = halloc(8); let one = ecr(1); println!("  store v{} v{}", out, one);
                    let c4 = ecr(4); println!("  store v{} v{}", ebin("add", out, c4), lo);
                    println!("  brz v{}", ov); ckreset(); let z = ecr(0); println!("  store v{} v{}", out, z); println!("  pool"); ckreset();
                    EIS8 = 0; return opt_postfix(out);
                }
                let mut op = "add"; if op8 == 115 { op = "sub"; } if op8 == 109 { op = "mul"; }
                let a64 = emit_convs(v); let b64 = emit_convs(b); let r64 = ebin8raw(op, a64, b64);
                let r32 = ebin(op, v, b); let r3264 = emit_convs(r32);
                let ov = ebin8raw("ne", r64, r3264);
                let out = halloc(8); let one = ecr(1); println!("  store v{} v{}", out, one);
                let c4 = ecr(4); println!("  store v{} v{}", ebin("add", out, c4), r32);
                println!("  brz v{}", ov); ckreset(); let z = ecr(0); println!("  store v{} v{}", out, z); println!("  pool"); ckreset();
                EIS64 = 0; EIS8 = 0; return opt_postfix(out);
            }
            if ID_L == 14 && at(ID_O) == 115 && at(ID_O + 1) == 97 && at(ID_O + 9) == 103 {   /* R2: saturating_add/sub(b) -> clamp to i32::MIN/MAX (index fix: 'g' is at 9, not 10) */
                let op11 = at(ID_O + 11);
                let recv8s = EIS8;
                skipws(); eat_ch(40); let b = parse_expr(); eat_ch(41);
                if recv8s == 1 {   /* value-width: u8 saturating_add/sub -> clamp to 0..=255 (brz skips the block when the cond is 0) */
                    let mut op = "add"; if op11 == 115 { op = "sub"; }
                    let sum = ebin(op, v, b);
                    let res = NC; ecopy(res, sum); NC = NC + 1;
                    let hi = ebin("gt", sum, ec(255)); println!("  brz v{}", hi); ckreset(); ecopy(res, ec(255)); println!("  pool"); ckreset();   /* sum>255 -> 255 */
                    let lo = ebin("lt", sum, ec(0)); println!("  brz v{}", lo); ckreset(); let z = ecr(0); ecopy(res, z); println!("  pool"); ckreset();   /* sum<0 (sub) -> 0 */
                    EIS8 = 1; return opt_postfix(res);
                }
                let mut op = "add"; if op11 == 115 { op = "sub"; }
                let a64 = emit_convs(v); let b64 = emit_convs(b); let r64 = ebin8raw(op, a64, b64);
                let r32 = ebin(op, v, b); let r3264 = emit_convs(r32);
                let ov = ebin8raw("ne", r64, r3264);
                let res = NC; ecopy(res, r32); NC = NC + 1;
                println!("  brz v{}", ov); ckreset();
                let z0 = ecr(0); let z64 = emit_convs(z0); let pos = ebin8raw("gt", r64, z64);
                let mx = ecr(2147483647); let mn = ebin("sub", ebin("sub", ec(0), mx), ec(1));
                ecopy(res, mn); println!("  brz v{}", pos); ckreset(); ecopy(res, mx); println!("  pool"); ckreset();
                println!("  pool"); ckreset();
                EIS64 = 0; return opt_postfix(res);
            }
            if id_is(109, 97, 112, 0, 0, 3) && POS < LEN && at(POS) == 40 && (POS + 1 >= LEN || at(POS + 1) != 124) {   /* R-either: .map(FN) -- a function/ctor VALUE (not a `|x| body` closure). Some/Ok(x) -> Some/Ok(FN(x)); None/Err unchanged. */
                skipws(); eat_ch(40); skipws();
                let out = halloc(8); let disc = eload(v); println!("  store v{} v{}", out, disc);
                println!("  brz v{}", disc); ckreset();
                let c4 = ecr(4); let payload = eload(ebin("add", v, c4));
                let bv = emit_fnval_apply(payload);
                let c4b = ecr(4); println!("  store v{} v{}", ebin("add", out, c4b), bv);
                println!("  pool"); ckreset();
                eat_ch(41);
                EISOPT = 1; EOPTRES = wasres; return opt_postfix(out);
            }
            if id_is(109, 97, 112, 0, 0, 3) {                                     /* R1: .map(|x| body) on Option/Result -> Some(body)/None (or Ok/Err) */
                skipws(); eat_ch(40); skipws(); eat_ch(124); skipws();
                if POS < LEN && at(POS) == 38 { POS = POS + 1; }
                rd_ident(); let xo = ID_O; let xl = ID_L;
                skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
                eat_ch(124);
                let out = halloc(8); let disc = eload(v); println!("  store v{} v{}", out, disc);
                println!("  brz v{}", disc); ckreset();
                let c4 = ecr(4); let payload = eload(ebin("add", v, c4));
                let mark = NSYM; let xslot = NC; ecopy(xslot, payload); NC = NC + 1; sym_add(xo, xl, xslot);
                if EOPTK == 2 { SK[(NSYM - 1) as usize] = 2; if EOPTVELT >= 0 { SVELT[(NSYM - 1) as usize] = EOPTVELT; } }   /* container-kind K1: Option<Vec>.map(|v| ..) -> v is a Vec (.iter()/.len() dispatch) */
                let bv = parse_expr();
                let c4b = ecr(4); let oad = ebin("add", out, c4b); println!("  store v{} v{}", oad, bv);
                NSYM = mark;
                println!("  pool"); ckreset();
                eat_ch(41);
                return opt_postfix(out);
            }
            if id_is(97, 110, 100, 95, 116, 8) && POS < LEN && at(POS) == 40 && (POS + 1 >= LEN || at(POS + 1) != 124) {   /* R2ca: .and_then(func) -- bare fn value; call func(payload) which yields an Option/Result cell, keep Err/None as-is */
                skipws(); eat_ch(40); skipws();
                rd_ident(); let fo = ID_O; let fl = ID_L; skipws(); eat_ch(41);
                let disc = eload(v); let out = NC; NC = NC + 1; ecopy(out, v);
                println!("  brz v{}", disc); ckreset();
                let c4 = ecr(4); let payload = eload(ebin("add", v, c4));
                print!("  v{}: i32 = call ", NC); emit_span(fo, fl); println!(" v{}", payload); let r = NC; NC = NC + 1;
                ecopy(out, r);
                println!("  pool"); ckreset();
                EISOPT = 1; EOPTRES = wasres; return opt_postfix(out);
            }
            if id_is(97, 110, 100, 95, 116, 8) {                                  /* R2: .and_then(|x| body) -- body returns an Option; flatten */
                skipws(); eat_ch(40); skipws(); eat_ch(124); skipws();
                if POS < LEN && at(POS) == 38 { POS = POS + 1; }
                rd_ident(); let xo = ID_O; let xl = ID_L;
                skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
                eat_ch(124);
                let out = halloc(8); let z = ecr(0); println!("  store v{} v{}", out, z);
                let disc = eload(v); println!("  brz v{}", disc); ckreset();
                let c4 = ecr(4); let payload = eload(ebin("add", v, c4));
                let mark = NSYM; let xslot = NC; ecopy(xslot, payload); NC = NC + 1; sym_add(xo, xl, xslot);
                let inner = parse_expr();
                let idc = eload(inner); println!("  store v{} v{}", out, idc);
                let c4b = ecr(4); let ip = eload(ebin("add", inner, c4b)); println!("  store v{} v{}", ebin("add", out, c4b), ip);
                NSYM = mark; println!("  pool"); ckreset(); eat_ch(41);
                EISOPT = 1; EOPTRES = wasres; return opt_postfix(out);
            }
            if id_is(102, 105, 108, 116, 101, 6) {                                /* R2: .filter(|x| pred) -- Some(x) if pred else None */
                let ik = EOPTK; let ity = EOPTTY;                                  /* LAYER2: preserve the input Option's payload kind for the result */
                skipws(); eat_ch(40); skipws(); eat_ch(124); skipws();
                if POS < LEN && at(POS) == 38 { POS = POS + 1; }
                rd_ident(); let xo = ID_O; let xl = ID_L;
                skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
                eat_ch(124);
                let out = halloc(8); let z = ecr(0); println!("  store v{} v{}", out, z);
                let disc = eload(v); println!("  brz v{}", disc); ckreset();
                let c4 = ecr(4); let payload = eload(ebin("add", v, c4));
                let mark = NSYM; let xslot = NC; ecopy(xslot, payload); NC = NC + 1; sym_add(xo, xl, xslot); SFORREF[(NSYM - 1) as usize] = 1;   /* filter's closure param is &T -> `*x` is a no-op */
                let pred = parse_expr();
                println!("  brz v{}", pred); ckreset();
                let one = ecr(1); println!("  store v{} v{}", out, one);
                let c4b = ecr(4); println!("  store v{} v{}", ebin("add", out, c4b), payload);
                println!("  pool"); ckreset();
                NSYM = mark; println!("  pool"); ckreset(); eat_ch(41);
                EISOPT = 1; EOPTRES = 0; EOPTK = ik; EOPTTY = ity;                 /* LAYER2: the filter result is an Option<same payload> -- flag it so {:?}/bind treat it as Some/None, not the raw disc */
                return opt_postfix(out);
            }
            if id_is(117, 110, 119, 114, 97, 14) {                                /* R2: .unwrap_or_else(|| body) -- body is the None default */
                skipws(); eat_ch(40); skipws(); eat_ch(124); while POS < LEN && at(POS) != 124 { POS = POS + 1; } eat_ch(124);
                let disc = eload(v); let res = NC; NC = NC + 1; let lb = LBL; LBL = LBL + 1;
                let c4 = ecr(4); let payload = eload(ebin("add", v, c4)); ecopy(res, payload);
                println!("  brz v{}", disc); ckreset(); println!("  goto __Lu{}", lb); ckreset(); println!("  pool"); ckreset();
                let d = parse_expr(); ecopy(res, d);
                println!("  label __Lu{}", lb); ckreset(); eat_ch(41);
                return opt_postfix(res);
            }
            if id_is(111, 107, 95, 111, 114, 5) {                                 /* R2: .ok_or(err) -- Some(x)->Ok(x), None->Err(err) (same [disc,payload] repr) */
                skipws(); eat_ch(40); let err = parse_expr(); eat_ch(41);
                let out = halloc(8); let disc = eload(v); println!("  store v{} v{}", out, disc);
                let c4 = ecr(4); let payload = eload(ebin("add", v, c4));
                let sel = NC; NC = NC + 1; ecopy(sel, err);
                println!("  brz v{}", disc); ckreset(); ecopy(sel, payload); println!("  pool"); ckreset();
                let c4b = ecr(4); println!("  store v{} v{}", ebin("add", out, c4b), sel);
                EISOPT = 1; EOPTRES = 1;   /* LAYER4: ok_or(err) yields a RESULT (Ok/Err) -> mark EOPTRES so a chained `?` in a `-> Result` fn is accepted (rustc_lexer unescape scan_escape: `chars.next().ok_or(..)?`) */
                return opt_postfix(out);
            }
            if id_is(109, 97, 112, 95, 111, 6) {                                  /* R2: map_or(default, |x| body) -> scalar */
                skipws(); eat_ch(40); let d = parse_expr(); skipws(); eat_ch(44); skipws(); eat_ch(124); skipws();
                if POS < LEN && at(POS) == 38 { POS = POS + 1; }
                rd_ident(); let xo = ID_O; let xl = ID_L;
                skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
                eat_ch(124);
                let disc = eload(v); let res = NC; NC = NC + 1; ecopy(res, d);
                println!("  brz v{}", disc); ckreset();
                let c4 = ecr(4); let payload = eload(ebin("add", v, c4));
                let mark = NSYM; let xslot = NC; ecopy(xslot, payload); NC = NC + 1; sym_add(xo, xl, xslot);
                let bv = parse_expr(); ecopy(res, bv);
                NSYM = mark; println!("  pool"); ckreset(); eat_ch(41);
                return opt_postfix(res);
            }
            if id_is(111, 114, 0, 0, 0, 2) {                                      /* R2: or(other) -> cell */
                skipws(); eat_ch(40); let other = parse_expr(); eat_ch(41);
                let disc = eload(v); let res = NC; NC = NC + 1; ecopy(res, other);
                println!("  brz v{}", disc); ckreset(); ecopy(res, v); println!("  pool"); ckreset();
                return opt_postfix(res);
            }
            if id_is(111, 114, 95, 101, 108, 7) {                                 /* R2: or_else(|| other) -> cell (lazy) */
                skipws(); eat_ch(40); skipws(); eat_ch(124); while POS < LEN && at(POS) != 124 { POS = POS + 1; } eat_ch(124);
                let disc = eload(v); let res = NC; NC = NC + 1; let lb = LBL; LBL = LBL + 1; ecopy(res, v);
                println!("  brz v{}", disc); ckreset(); println!("  goto __Lu{}", lb); ckreset(); println!("  pool"); ckreset();
                let d = parse_expr(); ecopy(res, d);
                println!("  label __Lu{}", lb); ckreset(); eat_ch(41);
                return opt_postfix(res);
            }
            if id_is(117, 110, 119, 114, 97, 17) {                                /* R2: unwrap_or_default() -> payload or 0 */
                skipws(); eat_ch(40); eat_ch(41);
                let disc = eload(v); let result = NC; NC = NC + 1; let z = ecr(0); ecopy(result, z);
                println!("  brz v{}", disc); ckreset();
                let c4 = ecr(4); let pv = eload(ebin("add", v, c4)); ecopy(result, pv);
                println!("  pool"); ckreset();
                return opt_postfix(result);
            }
            if id_is(105, 115, 95, 115, 111, 11) {                                /* R2: is_some_and(|x| pred) -> bool */
                skipws(); eat_ch(40); skipws(); eat_ch(124); skipws();
                if POS < LEN && at(POS) == 38 { POS = POS + 1; }
                rd_ident(); let xo = ID_O; let xl = ID_L;
                skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
                eat_ch(124);
                let disc = eload(v); let res = NC; NC = NC + 1; let z = ecr(0); ecopy(res, z);
                println!("  brz v{}", disc); ckreset();
                let c4 = ecr(4); let payload = eload(ebin("add", v, c4));
                let mark = NSYM; let xslot = NC; ecopy(xslot, payload); NC = NC + 1; sym_add(xo, xl, xslot); SFORREF[(NSYM - 1) as usize] = 1;
                let pred = parse_expr(); ecopy(res, pred);
                NSYM = mark; println!("  pool"); ckreset(); eat_ch(41);
                EISBOOL = 1; return res;
            }
            if id_is(109, 97, 112, 95, 101, 7) && POS < LEN && at(POS) == 40 && (POS + 1 >= LEN || at(POS + 1) != 124) {   /* R-either: .map_err(FN) -- a function/ctor VALUE (e.g. `Either::Left`), not a `|e| body` closure. Ok(x) unchanged; Err(e) -> Err(FN(e)). */
                skipws(); eat_ch(40); skipws();
                let out = halloc(8); let disc = eload(v); println!("  store v{} v{}", out, disc);
                let c4 = ecr(4); let payload = eload(ebin("add", v, c4));
                let sel = NC; NC = NC + 1; ecopy(sel, payload);                   /* Ok -> keep payload */
                println!("  brz v{}", disc); ckreset(); let lb = LBL; LBL = LBL + 1; println!("  goto __Le{}", lb); ckreset();
                println!("  pool"); ckreset();
                let ev = emit_fnval_apply(payload);                              /* Err -> FN(payload) */
                ecopy(sel, ev);
                println!("  label __Le{}", lb); ckreset();
                let c4b = ecr(4); println!("  store v{} v{}", ebin("add", out, c4b), sel);
                eat_ch(41);
                return opt_postfix(out);
            }
            if id_is(109, 97, 112, 95, 101, 7) {                                  /* R2ca: map_err(|e| body) -- Ok(x) unchanged; Err(e) -> Err(body) */
                skipws(); eat_ch(40); skipws(); eat_ch(124); skipws();
                if POS < LEN && at(POS) == 38 { POS = POS + 1; }
                rd_ident(); let xo = ID_O; let xl = ID_L;
                skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } }
                eat_ch(124);
                let out = halloc(8); let disc = eload(v); println!("  store v{} v{}", out, disc);
                let c4 = ecr(4); let payload = eload(ebin("add", v, c4));
                let sel = NC; NC = NC + 1; ecopy(sel, payload);
                println!("  brz v{}", disc); ckreset();
                let lb = LBL; LBL = LBL + 1; println!("  goto __Le{}", lb); ckreset();   /* Ok -> keep payload */
                println!("  pool"); ckreset();
                let mark = NSYM; let xslot = NC; ecopy(xslot, payload); NC = NC + 1; sym_add(xo, xl, xslot);
                let bv = parse_expr(); ecopy(sel, bv); NSYM = mark;
                println!("  label __Le{}", lb); ckreset();
                let c4b = ecr(4); println!("  store v{} v{}", ebin("add", out, c4b), sel);
                eat_ch(41);
                return opt_postfix(out);
            }
            if id_is(111, 107, 95, 111, 114, 10) {                                /* R2ca: ok_or_else(|| body) -- Some(x)->Ok(x); None->Err(body) (lazy) */
                skipws(); eat_ch(40); skipws(); eat_ch(124); while POS < LEN && at(POS) != 124 { POS = POS + 1; } eat_ch(124);
                let out = halloc(8); let disc = eload(v); println!("  store v{} v{}", out, disc);
                let c4 = ecr(4); let payload = eload(ebin("add", v, c4));
                let sel = NC; NC = NC + 1; ecopy(sel, payload);
                println!("  brz v{}", disc); ckreset();
                let lb = LBL; LBL = LBL + 1; println!("  goto __Le{}", lb); ckreset();   /* Some -> keep payload */
                println!("  pool"); ckreset();
                let d = parse_expr(); ecopy(sel, d);
                println!("  label __Le{}", lb); ckreset();
                let c4b = ecr(4); println!("  store v{} v{}", ebin("add", out, c4b), sel);
                eat_ch(41);
                EISOPT = 1; EOPTRES = 1;   /* LAYER4: ok_or_else(||..) yields a RESULT -> mark EOPTRES so a chained `?` is accepted */
                return opt_postfix(out);
            }
            if id_is(117, 110, 119, 114, 97, 10) {                                /* R2ca: unwrap_err() -- panic on Ok/Some (disc==1), else return the Err/None payload */
                skipws(); eat_ch(40); eat_ch(41);
                let disc = eload(v); let z = ecr(0); let isok = ebin("ne", disc, z);
                println!("  brz v{}", isok); ckreset();
                emit_flush_call(); let c60 = ecr(60); let c101 = ecr(101); println!("  v{}: i32 = syscall v{} v{}", NC, c60, c101); NC = NC + 1;
                println!("  pool"); ckreset();
                let c4 = ecr(4); let pa = ebin("add", v, c4); return eload(pa);
            }
            if id_is(102, 108, 97, 116, 116, 7) {                                 /* R2ca: flatten() on Option<Option<T>> -- None if outer None, else the inner Option cell */
                skipws(); eat_ch(40); eat_ch(41);
                let out = halloc(8); let z = ecr(0); println!("  store v{} v{}", out, z);
                let disc = eload(v); println!("  brz v{}", disc); ckreset();
                let c4 = ecr(4); let inner = eload(ebin("add", v, c4));
                let idc = eload(inner); println!("  store v{} v{}", out, idc);
                let c4b = ecr(4); let ip = eload(ebin("add", inner, c4b)); println!("  store v{} v{}", ebin("add", out, c4b), ip);
                println!("  pool"); ckreset();
                return opt_postfix(out);
            }
            if id_is(111, 107, 0, 0, 0, 2) { skipws(); eat_ch(40); eat_ch(41); return opt_postfix(v); }   /* R2ca: Result::ok() -> Option (same [disc,payload] repr) */
            if id_is(103, 101, 116, 95, 111, 13) {                                /* get_or_insert(v) -> &mut payload; if None, set the Option to Some(v) in place */
                skipws(); eat_ch(40); let nv = parse_expr(); eat_ch(41);
                let c4 = ecr(4); let pa = ebin("add", v, c4);
                let disc = eload(v); let z = ecr(0); let isnone = ebin("eq", disc, z);
                let existing = eload(pa);
                let newpay = ebin("add", ebin("mul", isnone, nv), ebin("mul", ebin("sub", ec(1), isnone), existing));
                println!("  store v{} v{}", pa, newpay);   /* payload = isnone ? nv : existing */
                let one = ecr(1); println!("  store v{} v{}", v, one);   /* disc = Some */
                EISOPT = 0; return pa;   /* &mut payload address (a following `*v` eloads it) */
            }
            if id_is(117, 110, 119, 114, 97, 6) {                                 /* unwrap -- panic (exit 101) on None/Err */
                skipws(); eat_ch(40); eat_ch(41);
                let disc = eload(v); let z = ecr(0); let isnone = ebin("eq", disc, z);
                println!("  brz v{}", isnone); ckreset();
                emit_flush_call(); let c60 = ecr(60); let c101 = ecr(101); println!("  v{}: i32 = syscall v{} v{}", NC, c60, c101); NC = NC + 1;
                println!("  pool"); ckreset();
                let c4 = ecr(4); let pa = ebin("add", v, c4); let pv = eload(pa); if EOPTREF == 1 { EWASREF = 1; }
                if EOPTK == 2 { EISVEC = 1; skipws(); if POS < LEN && (at(POS) == 46 || at(POS) == 91) { DVELT = EOPTVELT; return dispatch_vec(pv); } return pv; }   /* container-kind K1: Vec payload -> chain .len()/.iter()/[i] through dispatch_vec */
                if EOPTK == 3 { EISSTR = 1; EISSTRING = 1; } else if EOPTK == 1 { EISSTR = 1; }
                if EOPTTY >= 0 { apply_ty(EOPTTY); let et = EOPTTY; EOPTTY = 0 - 1; if TYTAG[et as usize] == 12 { EITYP = TYAUX[et as usize]; } }   /* TI3: restore the Option payload's Ty (tuple -> EISTUP/ETUPTY; struct -> EITYP) so `.N`/`.field` on min_by_key().unwrap() works */
                return pv;   /* R2aj: unwrap of Option<&T> (first/last) -> payload is a reference, following `*` is a no-op */
            }
            if id_is(117, 110, 119, 114, 97, 9) {                             /* unwrap_or(d) */
                skipws(); eat_ch(40); let d = parse_expr(); eat_ch(41);
                let disc = eload(v); let result = NC; NC = NC + 1; let lb = LBL; LBL = LBL + 1;
                println!("  brz v{}", disc); ckreset();
                let c4 = ecr(4); let pa = ebin("add", v, c4); let pv = eload(pa); ecopy(result, pv);
                println!("  goto __Lu{}", lb); ckreset();
                println!("  pool"); ckreset();
                ecopy(result, d);
                println!("  label __Lu{}", lb); ckreset();
                return result;
            }
            if id_is(105, 115, 95, 115, 111, 7) { skipws(); eat_ch(40); eat_ch(41); let disc = eload(v); let one = ecr(1); let r = ebin("eq", disc, one); EISBOOL = 1; return r; }   /* is_some */
            if id_is(105, 115, 95, 111, 107, 5) { skipws(); eat_ch(40); eat_ch(41); let disc = eload(v); let one = ecr(1); let r = ebin("eq", disc, one); EISBOOL = 1; return r; }    /* is_ok */
            if id_is(105, 115, 95, 110, 111, 7) { skipws(); eat_ch(40); eat_ch(41); let disc = eload(v); let z = ecr(0); let r = ebin("eq", disc, z); EISBOOL = 1; return r; }        /* is_none */
            if id_is(105, 115, 95, 101, 114, 6) { skipws(); eat_ch(40); eat_ch(41); let disc = eload(v); let z = ecr(0); let r = ebin("eq", disc, z); EISBOOL = 1; return r; }        /* is_err */
            let pmo = ID_O; let pml = ID_L; let pbi = blanket_find(pmo, pml);   /* R1 keystone: a trait method on a scalar receiver -> a primitive impl `impl Tr for i32` */
            skipws();
            if pbi >= 0 && POS < LEN && at(POS) == 40 {
                POS = POS + 1;
                let mut a0 = -1; let mut a1 = -1; let mut a2 = -1; let mut na = 0;
                skipws();
                if peek() != 41 {
                    a0 = parse_expr(); na = 1;
                    if eat_ch(44) { a1 = parse_expr(); na = 2;
                    if eat_ch(44) { a2 = parse_expr(); na = 3; } }
                }
                eat_ch(41);
                reg_blanket_inst(pbi, 0 - 2);
                print!("  v{}: i32 = call i32_", NC); emit_span(pmo, pml); print!(" v{}", v);
                if na >= 1 { print!(" v{}", a0); }
                if na >= 2 { print!(" v{}", a1); }
                if na >= 3 { print!(" v{}", a2); }
                println!("");
                NC = NC + 1; EISOPT = 0; EOPTRES = 0; let rv = NC - 1;
                if BIRET[pbi as usize] == 3 { EISSTR = 1; EISSTRING = 1; return rv; }   /* R16b: `-> String` prim method -> tag the result (was a bare pointer via opt_postfix) */
                if BIRET[pbi as usize] == 1 { EISSTR = 1; return rv; }                  /* `-> &str` */
                return opt_postfix(rv);
            }
            POS = s; EISOPT = wasopt; EOPTRES = wasres; return v;
        }
        EISOPT = wasopt; EOPTRES = wasres;
        v
    }
}

/* R6: String = the same [buf@0, len@4] descriptor as &str (so it prints via the str path) but OWNED, with a
   fixed 4096-byte backing buffer that push_str/push mutate. Locals tracked via SK=3. */
fn is_string_ty(o: i32, l: i32) -> bool { unsafe { l == 6 && at(o) == 83 && at(o + 1) == 116 && at(o + 2) == 114 && at(o + 3) == 105 && at(o + 4) == 110 && at(o + 5) == 103 } }   /* String */
fn is_i64_ty(o: i32, l: i32) -> bool { unsafe { (l == 3 && at(o) == 105 && at(o + 1) == 54 && at(o + 2) == 52) || (l == 5 && at(o) == 105 && at(o + 1) == 115 && at(o + 2) == 105 && at(o + 3) == 122 && at(o + 4) == 101) } }   /* LAYER6 P-ENUM-I64-PAYLOAD-WIDTH: signed 64-bit type name (i64 / isize) -- for tagging an enum i64-payload match binder SI64. u64/usize left out (unsigned Display differs). CELL-WIDENING RESOLVED (stage1 hoist 2026-08-15): an i64 payload field now takes an 8-byte cell slot (VFI64-driven sz + store8 at construction; eload8 + ecopy64 at the match_pat_payload binder) -- large STORED payloads no longer truncate. */
/* TI3e-i: pack a Vec of char values (4-byte elements) into a contiguous arena String {buf@0,len@4}.
   Emits a runtime byte-copy loop (storeb the low byte of each char), then a descriptor + cursor bump --
   the char-Vec -> String half of `.collect::<String>()` / `let s: String = chars.collect()`. */
fn emit_charvec_to_string(dv: i32) -> i32 {
    unsafe {
        let c4 = ecr(4);
        let count = eload(ebin("add", dv, c4));
        let vbuf = eload(dv);
        let cur = eload(egref_str_cursor());
        let sbuf = ebin("add", egref_str_arena(), cur);
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, count); println!("  brz v{}", cnd); ckreset();
        let c4i = ecr(4); let cv = eload(ebin("add", vbuf, ebin("mul", i, c4i)));
        let dst = ebin("add", sbuf, i); println!("  storeb v{} v{}", dst, cv);
        println!("  cont"); ckreset();
        let one = ecr(1); let ni = ebin("add", i, one); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        let c3 = ecr(3); let c4d = ecr(4); let doff = ebin("mul", ebin("div", ebin("add", count, c3), c4d), c4d);
        let desc = ebin("add", sbuf, doff); println!("  store v{} v{}", desc, sbuf);
        let c4e = ecr(4); println!("  store v{} v{}", ebin("add", desc, c4e), count);
        let c8 = ecr(8); let newcur = ebin("add", cur, ebin("add", doff, c8)); println!("  store v{} v{}", egref_str_cursor(), newcur);
        EISSTR = 1; EISSTRING = 1; EISVEC = 0;
        desc
    }
}
/* TI3e-j: split_whitespace().count() -- count runs of non-whitespace (ASCII: byte > 32) in the str
   {buf@0,len@4}. Emits a branchless runtime loop: newword = notws & !prev_inword; cnt += newword. */
fn emit_ws_word_count(dv: i32) -> i32 {
    unsafe {
        let c4 = ecr(4);
        let p = eload(dv);
        let n = eload(ebin("add", dv, c4));
        let cnt = NC; let z0 = ecr(0); ecopy(cnt, z0); NC = NC + 1;
        let i = NC; let z1 = ecr(0); ecopy(i, z1); NC = NC + 1;
        let inw = NC; let z2 = ecr(0); ecopy(inw, z2); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, n); println!("  brz v{}", cnd); ckreset();
        let c = eloadb(ebin("add", p, i));
        let c32 = ecr(32); let notws = ebin("gt", c, c32);
        let one = ecr(1); let prevnot = ebin("sub", one, inw);
        let newword = ebin("mul", notws, prevnot);
        let nc = ebin("add", cnt, newword); ecopy(cnt, nc);
        ecopy(inw, notws);
        let one2 = ecr(1); let ni = ebin("add", i, one2); ecopy(i, ni);
        println!("  cont"); ckreset(); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        cnt
    }
}
fn emit_str_new() -> i32 {
    unsafe {
        let d = halloc(12);                      /* STR-GROW: {buf@0,len@4,cap@8} (was 8) */
        let buf = halloc(64);
        println!("  store v{} v{}", d, buf);
        let c8 = ecr(8); let cad = ebin("add", d, c8); let cap = ecr(64);
        println!("  store v{} v{}", cad, cap);   /* cap@8 = 4096 initial byte capacity */
        EISSTR = 1; EISSTRING = 1;
        d
    }
}
fn emit_str_push_byte(dv: i32, ch: i32) {
    unsafe {
        let one0 = ecr(1); print!("  v{}: i32 = call __str_reserve v{} v{}", NC, dv, one0); println!(""); NC = NC + 1;   /* STR-GROW: room for +1, may realloc buf */
        let c4 = ecr(4); let dlad = ebin("add", dv, c4); let dl = eload(dlad);
        let dp = eload(dv); let da = ebin("add", dp, dl);
        println!("  storeb v{} v{}", da, ch);
        let one = ecr(1); let dl1 = ebin("add", dl, one);
        let c4b = ecr(4); let dlad2 = ebin("add", dv, c4b);
        println!("  store v{} v{}", dlad2, dl1);
    }
}
fn emit_str_push_str(dv: i32, src: i32) {
    unsafe {
        let c4b0 = ecr(4); let slad0 = ebin("add", src, c4b0); let sl0 = eload(slad0);   /* STR-GROW: reserve room for src len BEFORE loading dst buf (grow may move it) */
        print!("  v{}: i32 = call __str_reserve v{} v{}", NC, dv, sl0); println!(""); NC = NC + 1;
        let c4a = ecr(4); let dlad = ebin("add", dv, c4a); let dl0 = eload(dlad);
        let dp = eload(dv); let dbase = ebin("add", dp, dl0);
        let sp = eload(src);
        let c4b = ecr(4); let slad = ebin("add", src, c4b); let sl = eload(slad);
        let islot = NC; let z = ecr(0); ecopy(islot, z); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", islot, sl);
        println!("  brz v{}", cnd); ckreset();
        let sa = ebin("add", sp, islot); let byte = eloadb(sa);
        let da = ebin("add", dbase, islot);
        println!("  storeb v{} v{}", da, byte);
        let one = ecr(1); let i1 = ebin("add", islot, one); ecopy(islot, i1);
        println!("  cont"); ckreset(); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        let dl2 = ebin("add", dl0, sl);
        let c4c = ecr(4); let dlad2 = ebin("add", dv, c4c);
        println!("  store v{} v{}", dlad2, dl2);
    }
}
/* std-hoist P4.3: route the ALLOCATING/MUTATING built-in String ops to the std RString_* fns (present iff
   HAS_RSTRING==1). RString layout {buf@0,len@4,cap@8} -- its first 8 bytes ARE a valid built-in [ptr@0,len@4]
   descriptor (buf packs the contiguous UTF-8 bytes, len in bytes), so the READ ops (.len/Display %s) keep
   working via dispatch_string on the returned pointer. Only new/push_str/push are routed. */
fn emit_rstring_new() -> i32 {
    unsafe {
        print!("  v{}: i32 = call RString_new", NC); println!("");
        let d = NC; NC = NC + 1; EISSTR = 1; EISSTRING = 1;
        d
    }
}
/* src is a &str [ptr@0,len@4] descriptor -> pass its ptr + byte-len to RString_push_str_bytes(&mut self, ptr, n) */
fn emit_rstring_push_str(dv: i32, src: i32) {
    unsafe {
        let sp = eload(src);
        let c4 = ecr(4); let slad = ebin("add", src, c4); let sl = eload(slad);
        print!("  v{}: i32 = call RString_push_str_bytes v{} v{} v{}", NC, dv, sp, sl); println!("");
        NC = NC + 1;
    }
}
/* ch is a 1-byte (ASCII) char -> RString_push_byte(&mut self, b); multibyte is a follow-up */
fn emit_rstring_push_byte(dv: i32, ch: i32) {
    unsafe {
        print!("  v{}: i32 = call RString_push_byte v{} v{}", NC, dv, ch); println!("");
        NC = NC + 1;
    }
}
/* R25: &[u8] byte slice ops on a [ptr,len] descriptor -- .len() and slice[i] (byte load). */
fn dispatch_bytes(dv: i32) -> i32 {
    unsafe {
        skipws();
        if POS < LEN && at(POS) == 46 {
            let s = POS; POS = POS + 1; rd_ident();
            if id_is(108, 101, 110, 0, 0, 3) { skipws(); eat_ch(40); eat_ch(41); let c4 = ecr(4); let ad = ebin("add", dv, c4); return eload(ad); }   /* len */
            if id_is(105, 116, 101, 114, 0, 4) { skipws(); eat_ch(40); eat_ch(41); EISBYTES = 1; EISVEC = 0; skipws(); if POS < LEN && at(POS) == 46 { return dispatch_bytes(dv); } return dv; }   /* RUSTC-FE byte-vec-repr LIFT: iter() on a Vec<u8> -> the byte descriptor (a `for x in v.iter()` reads sbytes=1 -> stride-1 byte loads; a chained adapter re-routes through dispatch_bytes). Was UNHANDLED for SK=6 -> `.iter()` crashed once a Vec<u8> LOCAL became a byte-vec. */
            if id_is(112, 117, 115, 104, 0, 4) { skipws(); eat_ch(40); let e = parse_expr(); eat_ch(41);   /* RUSTC-FE: push(b) on an owned Vec<u8> (byte descriptor [buf@0,len@4]) -- storeb at buf[len], len++ (1-byte stride, consistent with the [i] eloadb reads). Closes P-STRUCT-FIELD-VEC-U8-ELEM-READ: a `Vec<u8>` STRUCT FIELD grown via push had no push handler here (dispatch_bytes was read-only) -> len stayed 0 -> reads gave 0. */
                let c4 = ecr(4); let lad = ebin("add", dv, c4); let ln = eload(lad); let buf = eload(dv);
                let ea = ebin("add", buf, ln); println!("  storeb v{} v{}", ea, e);
                let one = ecr(1); let nl = ebin("add", ln, one); println!("  store v{} v{}", lad, nl);
                return ec(0);
            }
            if id_is(116, 111, 95, 118, 101, 6) { skipws(); eat_ch(40); eat_ch(41); EISBYTES = 1; return dv; }   /* BVK N2: to_vec() on a &[u8] -> the {ptr,len} descriptor IS a Vec<u8> (identity, keep byte tag). Handled HERE so it parses in a struct-field-init too; safe now that a Vec<u8> field is FFK=6 (byte dispatch). */
            POS = s;
        }
        if POS < LEN && at(POS) == 91 {                                                                                                              /* slice[i] -> byte */
            POS = POS + 1; let iv = parse_expr(); eat_ch(93);
            let p = eload(dv); let ea = ebin("add", p, iv); return eloadb(ea);
        }
        EISBYTES = 1;
        dv
    }
}
fn dispatch_string(dv: i32) -> i32 {
    unsafe {
        skipws();
        if POS < LEN && at(POS) == 46 {
            let s = POS;
            POS = POS + 1; rd_ident();
            if id_is(99, 108, 111, 110, 101, 5) { skipws(); eat_ch(40); eat_ch(41); EISSTR = 1; return dv; }   /* clone() on a &str/String -> identity descriptor (was `call clone` -> undefined; breaks .clone() on a &str generic param) */
            if id_is(116, 111, 95, 115, 116, 9) || id_is(116, 111, 95, 111, 119, 8) { skipws(); eat_ch(40); eat_ch(41); EISSTR = 1; return dv; }   /* TI3e: to_string()/to_owned() on a &str/String -> identity descriptor (mirrors clone(); was `call to_string` via the generic emit_call fallback -> undefined link, dropped the 2nd+ element in vec!["a".to_string(), ..]). */
            if id_is(108, 101, 110, 0, 0, 3) { skipws(); eat_ch(40); eat_ch(41); let c4 = ecr(4); let ad = ebin("add", dv, c4); return eload(ad); }   /* len */
            if id_is(105, 115, 95, 101, 109, 8) { skipws(); eat_ch(40); eat_ch(41); let c4 = ecr(4); let ln = eload(ebin("add", dv, c4)); let z = ecr(0); let r = ebin("eq", ln, z); EISSTR = 0; EISSTRING = 0; EISBOOL = 1; return r; }   /* is_empty() -> bool (clear str tag) */
            if id_is(112, 117, 115, 104, 95, 8) { skipws(); eat_ch(40); let src = parse_expr(); eat_ch(41); if HAS_RSTRING == 1 { emit_rstring_push_str(dv, src); } else { emit_str_push_str(dv, src); } return ec(0); }  /* push_str (std-hoist P4.3: route to RString_push_str_bytes when std present) */
            if id_is(112, 117, 115, 104, 0, 4) { skipws(); eat_ch(40); let ch = parse_expr(); eat_ch(41); if HAS_RSTRING == 1 { emit_rstring_push_byte(dv, ch); } else { emit_str_push_byte(dv, ch); } return ec(0); }    /* push (std-hoist P4.3: route to RString_push_byte when std present) */
            if id_is(97, 115, 95, 98, 121, 8) { skipws(); eat_ch(40); eat_ch(41); EISSTR = 0; EISSTRING = 0; return dispatch_bytes(dv); }   /* K2: as_bytes() -> reinterpret the {ptr@0,len@4} str descriptor as a &[u8] byte slice (.len()/[i]) */
            if id_is(97, 115, 95, 115, 116, 6) { skipws(); eat_ch(40); eat_ch(41); EISSTR = 1; EISSTRING = 0; skipws(); if POS < LEN && (at(POS) == 46 || at(POS) == 91) { return dispatch_string(dv); } return dv; }   /* LAYER8 P-STRING-AS-STR: String::as_str() -> &str IDENTITY (the same {ptr,len} descriptor). Tag EISSTR so a following `==`/comparison byte-compares; a chained `.len()/.starts_with()/[i]` re-routes through dispatch_string. Was: unrecognized -> `.as_str()` not consumed -> `s.as_str() == x` parse-drifted (the resolver's `rib.names[i].as_str() == name`). */
            if id_is(112, 97, 114, 115, 101, 5) {                                 /* R2: parse::<i32>() -> Result<i32> (disc always Ok in this model) */
                skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }
                skipws(); eat_ch(40); eat_ch(41);
                let p = eload(dv); let c4 = ecr(4); let ln = eload(ebin("add", dv, c4));
                print!("  v{}: i32 = call __str_atoi v{} v{}", NC, p, ln); println!(""); let val = NC; NC = NC + 1;
                print!("  v{}: i32 = call __str_valid_int v{} v{}", NC, p, ln); println!(""); let ok = NC; NC = NC + 1;   /* P-STRING: parse validates -> Err on a non-integer string (disc = validity, not always Ok) */
                let cell = halloc(8); println!("  store v{} v{}", cell, ok);
                let c4b = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4b), val);
                EISOPT = 1; EOPTRES = 1; return opt_postfix(cell);
            }
            if id_is(115, 112, 108, 105, 116, 16) {   /* TI3e-j: split_whitespace().count() -- count whitespace-delimited words */
                skipws(); eat_ch(40); eat_ch(41); skipws();
                if POS < LEN && at(POS) == 46 { POS = POS + 1; rd_ident();
                    if id_is(99, 111, 117, 110, 116, 5) { skipws(); eat_ch(40); eat_ch(41); return emit_ws_word_count(dv); }   /* .count() */
                }
                return dv;
            }
            if id_is(115, 112, 108, 105, 116, 6) {   /* STATEFUL-ITER: splitn(n, sep) -> a str-cursor [buf,len,pos,remaining,sep]; it.next() yields sep-delimited pieces, the nth keeps the rest */
                skipws(); eat_ch(40); let nn = parse_expr(); skipws(); eat_ch(44); skipws();
                let mut sep = 0;
                if POS < LEN && at(POS) == 34 { POS = POS + 1; sep = at(POS); while POS < LEN && at(POS) != 34 { if at(POS) == 92 { POS = POS + 1; } POS = POS + 1; } POS = POS + 1; }
                else if POS < LEN && at(POS) == 39 { POS = POS + 1; sep = at(POS); while POS < LEN && at(POS) != 39 { POS = POS + 1; } POS = POS + 1; }
                eat_ch(41);
                let buf = eload(dv); let c4 = ecr(4); let ln = eload(ebin("add", dv, c4));
                let cur = halloc(20);
                println!("  store v{} v{}", cur, buf);
                println!("  store v{} v{}", ebin("add", cur, ec(4)), ln);
                println!("  store v{} v{}", ebin("add", cur, ec(8)), ec(0));
                println!("  store v{} v{}", ebin("add", cur, ec(12)), nn);
                println!("  store v{} v{}", ebin("add", cur, ec(16)), ec(sep));
                ESPLIT = 1; return cur;
            }
            if id_is(109, 97, 116, 99, 104, 7) {   /* LAYER2: matches(PAT).count() -- occurrences of a single-char pattern (= split count minus 1) */
                skipws(); eat_ch(40); skipws();
                let mut sep = 0;
                if POS < LEN && at(POS) == 34 { POS = POS + 1; sep = at(POS); while POS < LEN && at(POS) != 34 { if at(POS) == 92 { POS = POS + 1; } POS = POS + 1; } POS = POS + 1; }
                else if POS < LEN && at(POS) == 39 { POS = POS + 1; sep = at(POS); while POS < LEN && at(POS) != 39 { POS = POS + 1; } POS = POS + 1; }
                eat_ch(41);
                skipws();
                if POS < LEN && at(POS) == 46 { POS = POS + 1; rd_ident();
                    if id_is(99, 111, 117, 110, 116, 5) { skipws(); eat_ch(40); eat_ch(41);
                        let p = eload(dv); let c4 = ecr(4); let ln = eload(ebin("add", dv, c4)); let cs = ec(sep);
                        print!("  v{}: i32 = call __str_count_sep v{} v{} v{}", NC, p, ln, cs); println!(""); NC = NC + 1;
                        EISSTR = 0; EISSTRING = 0; return NC - 1;
                    }
                }
            }
            if id_is(115, 112, 108, 105, 116, 5) {   /* R6: split(SEP).count() -- fused; single-byte separator -> occurrences+1 */
                skipws(); eat_ch(40); skipws();
                let mut sep = 0;
                if POS < LEN && at(POS) == 34 { POS = POS + 1; sep = at(POS); while POS < LEN && at(POS) != 34 { if at(POS) == 92 { POS = POS + 1; } POS = POS + 1; } POS = POS + 1; }   /* "x" */
                else if POS < LEN && at(POS) == 39 { POS = POS + 1; sep = at(POS); while POS < LEN && at(POS) != 39 { POS = POS + 1; } POS = POS + 1; }   /* 'x' */
                eat_ch(41);
                skipws();
                if POS < LEN && at(POS) == 46 { let msave = POS; POS = POS + 1; rd_ident();
                    if id_is(99, 111, 117, 110, 116, 5) { skipws(); eat_ch(40); eat_ch(41);   /* .count() */
                        let p = eload(dv); let c4 = ecr(4); let ln = eload(ebin("add", dv, c4)); let cs = ec(sep);
                        print!("  v{}: i32 = call __str_count_sep v{} v{} v{}", NC, p, ln, cs); println!(""); let cnt = NC; NC = NC + 1;
                        let one = ecr(1); return ebin("add", cnt, one);
                    }
                    /* R22c: split(SEP).<map/filter/sum/...> -- materialize the &str Vec and route the chained adapter through dispatch_vec (was: unsupported `return dv`). collect handled just below. */
                    if !id_is(99, 111, 108, 108, 101, 7) {
                        POS = msave;
                        let p = eload(dv); let c4 = ecr(4); let ln = eload(ebin("add", dv, c4)); let cs = ec(sep);
                        let vd = emit_vec_new(); let vbuf = eload(vd); let dbuf = halloc(4096);
                        print!("  v{}: i32 = call __str_split_into v{} v{} v{} v{} v{}", NC, vbuf, dbuf, p, ln, cs); println!(""); let cnt = NC; NC = NC + 1;
                        let c4b = ecr(4); println!("  store v{} v{}", ebin("add", vd, c4b), cnt);
                        EISVEC = 1; EVELSTR = 1; DVELT = 0 - 2;   /* element is &str -> a chained map/filter/for closure param binds as &str */
                        return dispatch_vec(vd);
                    }
                    if id_is(99, 111, 108, 108, 101, 7) {   /* R2ax: .collect::<Vec<&str>>() -> Vec of &str slice descriptors */
                        skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }
                        skipws(); eat_ch(40); eat_ch(41);
                        let p = eload(dv); let c4 = ecr(4); let ln = eload(ebin("add", dv, c4)); let cs = ec(sep);
                        let vd = emit_vec_new(); let vbuf = eload(vd); let dbuf = halloc(4096);
                        print!("  v{}: i32 = call __str_split_into v{} v{} v{} v{} v{}", NC, vbuf, dbuf, p, ln, cs); println!(""); let cnt = NC; NC = NC + 1;
                        let c4b = ecr(4); println!("  store v{} v{}", ebin("add", vd, c4b), cnt);
                        EISVEC = 1; EVELSTR = 1; return vd;
                    }
                }
                return dv;   /* split without count/collect -- unsupported, best-effort */
            }
            if id_is(102, 105, 110, 100, 0, 4) {                                  /* R2: find(needle) -> Option<usize>; needle is a char/byte or a &str */
                skipws(); eat_ch(40); let arg = parse_expr(); let ach = EISCHAR; eat_ch(41);
                let hp = eload(dv); let c4 = ecr(4); let hl = eload(ebin("add", dv, c4));
                let mut np = arg; let mut nl = ecr(1);
                if ach == 0 { np = eload(arg); let c4b = ecr(4); nl = eload(ebin("add", arg, c4b)); }
                else { let nb = halloc(4); println!("  storeb v{} v{}", nb, arg); np = nb; }   /* R2ax: char needle -> 1-byte scratch buffer */
                print!("  v{}: i32 = call __str_find v{} v{} v{} v{}", NC, hp, hl, np, nl); println!(""); let idx = NC; NC = NC + 1;
                let cell = halloc(8); let z = ecr(0); let found = ebin("ge", idx, z);
                println!("  store v{} v{}", cell, found); let c4c = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4c), idx);
                return opt_postfix(cell);
            }
            if id_is(99, 111, 110, 116, 97, 8) {                                  /* R2: contains(needle) -> bool */
                skipws(); eat_ch(40); let arg = parse_expr(); eat_ch(41);
                let hp = eload(dv); let c4 = ecr(4); let hl = eload(ebin("add", dv, c4));
                let np = eload(arg); let c4b = ecr(4); let nl = eload(ebin("add", arg, c4b));
                print!("  v{}: i32 = call __str_find v{} v{} v{} v{}", NC, hp, hl, np, nl); println!(""); let idx = NC; NC = NC + 1;
                let z = ecr(0); let r = ebin("ge", idx, z); EISBOOL = 1; return r;
            }
            if id_is(115, 112, 108, 105, 116, 10) {                               /* LAYER2: split_once(sep) -> Option<(&str,&str)>: find sep, split into [left, right] &str slices */
                skipws(); eat_ch(40); let arg = parse_expr(); let ach = EISCHAR; eat_ch(41);
                let hp = eload(dv); let c4 = ecr(4); let hl = eload(ebin("add", dv, c4));
                let mut np = arg; let mut nl = ecr(1);
                if ach == 0 { np = eload(arg); nl = eload(ebin("add", arg, c4)); }
                else { let nb = halloc(4); println!("  storeb v{} v{}", nb, arg); np = nb; }   /* char sep -> 1-byte scratch buffer */
                print!("  v{}: i32 = call __str_find v{} v{} v{} v{}", NC, hp, hl, np, nl); println!(""); let idx = NC; NC = NC + 1;
                let z = ecr(0); let found = ebin("ge", idx, z);
                let ld = halloc(8); println!("  store v{} v{}", ld, hp); println!("  store v{} v{}", ebin("add", ld, c4), idx);   /* left = [hp, idx] */
                let roff = ebin("add", idx, nl); let rp = ebin("add", hp, roff); let rl = ebin("sub", hl, roff);
                let rd = halloc(8); println!("  store v{} v{}", rd, rp); println!("  store v{} v{}", ebin("add", rd, c4), rl);   /* right = [hp+idx+seplen, hl-idx-seplen] */
                let tup = halloc(8); println!("  store v{} v{}", tup, ld); println!("  store v{} v{}", ebin("add", tup, c4), rd);   /* tuple cell [ld, rd] */
                let cell = halloc(8); println!("  store v{} v{}", cell, found); println!("  store v{} v{}", ebin("add", cell, c4), tup);   /* Option cell [disc=found, tuple] */
                let tyid = NTUPTY; let base = TFKCUR; TFK[TFKCUR as usize] = 1; TFKCUR = TFKCUR + 1; TFK[TFKCUR as usize] = 1; TFKCUR = TFKCUR + 1; TUPTY_BASE[tyid as usize] = base; TUPTY_N[tyid as usize] = 2; NTUPTY = NTUPTY + 1;   /* register the (&str,&str) tuple type */
                EISOPT = 1; EOPTRES = 0; ETUPN = 2; EOPTTY = ty_intern(11, 0, tyid);
                return opt_postfix(cell);
            }
            if id_is(115, 116, 97, 114, 116, 11) {                                /* R2: starts_with(prefix) -> bool */
                skipws(); eat_ch(40); let arg = parse_expr(); eat_ch(41);
                let hp = eload(dv); let np = eload(arg); let c4b = ecr(4); let nl = eload(ebin("add", arg, c4b));
                print!("  v{}: i32 = call __str_match v{} v{} v{}", NC, hp, np, nl); println!(""); let r = NC; NC = NC + 1; EISBOOL = 1; return r;
            }
            if id_is(115, 116, 114, 105, 112, 12) && at(ID_O + 6) == 112 {        /* LAYER4: strip_prefix(pref) -> Option<&str>: Some([hp+preflen, hl-preflen]) if starts with pref, else None */
                skipws(); eat_ch(40); let arg = parse_expr(); eat_ch(41);
                let hp = eload(dv); let c4 = ecr(4); let hl = eload(ebin("add", dv, c4));
                let np = eload(arg); let nl = eload(ebin("add", arg, c4));
                print!("  v{}: i32 = call __str_match v{} v{} v{}", NC, hp, np, nl); println!(""); let m = NC; NC = NC + 1;
                let rp = ebin("add", hp, nl); let rl = ebin("sub", hl, nl);
                let sd = halloc(8); println!("  store v{} v{}", sd, rp); println!("  store v{} v{}", ebin("add", sd, c4), rl);
                let cell = halloc(8); println!("  store v{} v{}", cell, m); println!("  store v{} v{}", ebin("add", cell, c4), sd);
                EISOPT = 1; EOPTRES = 0; EOPTK = 1; return opt_postfix(cell);   /* payload is a &str */
            }
            if id_is(115, 116, 114, 105, 112, 12) && at(ID_O + 6) == 115 {        /* LAYER4: strip_suffix(suf) -> Option<&str>: Some([hp, hl-suflen]) if ends with suf, else None */
                skipws(); eat_ch(40); let arg = parse_expr(); eat_ch(41);
                let hp = eload(dv); let c4 = ecr(4); let hl = eload(ebin("add", dv, c4));
                let np = eload(arg); let nl = eload(ebin("add", arg, c4));
                let start = ebin("add", hp, ebin("sub", hl, nl));
                print!("  v{}: i32 = call __str_match v{} v{} v{}", NC, start, np, nl); println!(""); let m = NC; NC = NC + 1;
                let rl = ebin("sub", hl, nl);
                let sd = halloc(8); println!("  store v{} v{}", sd, hp); println!("  store v{} v{}", ebin("add", sd, c4), rl);
                let cell = halloc(8); println!("  store v{} v{}", cell, m); println!("  store v{} v{}", ebin("add", cell, c4), sd);
                EISOPT = 1; EOPTRES = 0; EOPTK = 1; return opt_postfix(cell);
            }
            if id_is(101, 110, 100, 115, 95, 9) {                                 /* R2: ends_with(suffix) -> bool */
                skipws(); eat_ch(40); let arg = parse_expr(); eat_ch(41);
                let hp = eload(dv); let c4 = ecr(4); let hl = eload(ebin("add", dv, c4));
                let np = eload(arg); let c4b = ecr(4); let nl = eload(ebin("add", arg, c4b));
                let start = ebin("add", hp, ebin("sub", hl, nl));
                print!("  v{}: i32 = call __str_match v{} v{} v{}", NC, start, np, nl); println!(""); let r = NC; NC = NC + 1; EISBOOL = 1; return r;
            }
            if id_is(116, 114, 105, 109, 0, 4) {                                  /* R2: trim() -> &str sub-descriptor (no alloc) */
                skipws(); eat_ch(40); eat_ch(41);
                let p = eload(dv); let c4 = ecr(4); let ln = eload(ebin("add", dv, c4));
                print!("  v{}: i32 = call __str_lead_ws v{} v{}", NC, p, ln); println!(""); let lead = NC; NC = NC + 1;
                let np = ebin("add", p, lead); let nn = ebin("sub", ln, lead);
                print!("  v{}: i32 = call __str_trail_ws v{} v{}", NC, np, nn); println!(""); let trail = NC; NC = NC + 1;
                let fl = ebin("sub", nn, trail);
                let cell = halloc(8); println!("  store v{} v{}", cell, np);
                let c4b = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4b), fl);
                EISSTR = 1; skipws(); if POS < LEN && at(POS) == 46 { return dispatch_string(cell); }   /* P-STRING: `s.trim().parse()`/`.len()` etc -- re-dispatch a chained method on the TRIMMED descriptor (was: returned cell, so `.parse()` fell to a generic `call parse`) */
                return cell;
            }
            if id_is(116, 114, 105, 109, 95, 10) {                                /* STR: trim_start() -> strip leading ws only */
                skipws(); eat_ch(40); eat_ch(41);
                let p = eload(dv); let c4 = ecr(4); let ln = eload(ebin("add", dv, c4));
                print!("  v{}: i32 = call __str_lead_ws v{} v{}", NC, p, ln); println!(""); let lead = NC; NC = NC + 1;
                let cell = halloc(8); println!("  store v{} v{}", cell, ebin("add", p, lead));
                let c4b = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4b), ebin("sub", ln, lead));
                EISSTR = 1; return cell;
            }
            if id_is(116, 114, 105, 109, 95, 8) {                                 /* STR: trim_end() -> strip trailing ws only */
                skipws(); eat_ch(40); eat_ch(41);
                let p = eload(dv); let c4 = ecr(4); let ln = eload(ebin("add", dv, c4));
                print!("  v{}: i32 = call __str_trail_ws v{} v{}", NC, p, ln); println!(""); let trail = NC; NC = NC + 1;
                let cell = halloc(8); println!("  store v{} v{}", cell, p);
                let c4b = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4b), ebin("sub", ln, trail));
                EISSTR = 1; return cell;
            }
            if id_is(116, 111, 95, 117, 112, 12) || id_is(116, 111, 95, 108, 111, 12) {   /* R2: to_uppercase()/to_lowercase() -> new arena String */
                let isup = at(ID_O + 3) == 117;                                    /* to_Up vs to_Lo */
                skipws(); eat_ch(40); eat_ch(41);
                let p = eload(dv); let c4 = ecr(4); let ln = eload(ebin("add", dv, c4));
                let cur = eload(egref_str_cursor()); let base = ebin("add", egref_str_arena(), cur);
                let mut lo = ec(65); let mut delta = ecr(32); if isup { lo = ec(97); delta = ebin("sub", ec(0), ec(32)); }
                print!("  v{}: i32 = call __str_casemap v{} v{} v{} v{} v{}", NC, base, p, ln, lo, delta); println!(""); NC = NC + 1;
                let c3 = ecr(3); let c4b = ecr(4); let doff = ebin("mul", ebin("div", ebin("add", ln, c3), c4b), c4b);
                let desc = ebin("add", base, doff); println!("  store v{} v{}", desc, base);
                let c4c = ecr(4); println!("  store v{} v{}", ebin("add", desc, c4c), ln);
                let c8 = ecr(8); let newcur = ebin("add", cur, ebin("add", doff, c8)); println!("  store v{} v{}", egref_str_cursor(), newcur);
                EISSTR = 1; EISSTRING = 1; return desc;
            }
            if id_is(114, 101, 112, 101, 97, 6) {                                 /* R2: repeat(n) -> new arena String */
                skipws(); eat_ch(40); let cnt = parse_expr(); eat_ch(41);
                let p = eload(dv); let c4 = ecr(4); let ln = eload(ebin("add", dv, c4));
                let total = ebin("mul", ln, cnt);
                let cur = eload(egref_str_cursor()); let base = ebin("add", egref_str_arena(), cur);
                print!("  v{}: i32 = call __str_repeat_into v{} v{} v{} v{}", NC, base, p, ln, cnt); println!(""); NC = NC + 1;
                let c3 = ecr(3); let c4b = ecr(4); let doff = ebin("mul", ebin("div", ebin("add", total, c3), c4b), c4b);
                let desc = ebin("add", base, doff); println!("  store v{} v{}", desc, base);
                let c4c = ecr(4); println!("  store v{} v{}", ebin("add", desc, c4c), total);
                let c8 = ecr(8); let newcur = ebin("add", cur, ebin("add", doff, c8)); println!("  store v{} v{}", egref_str_cursor(), newcur);
                EISSTR = 1; EISSTRING = 1; return desc;
            }
            if id_is(114, 101, 112, 108, 97, 7) {                                 /* R2: replace(from,to) -> new arena String */
                skipws(); eat_ch(40); let fromd = parse_expr(); eat_ch(44); let tod = parse_expr(); eat_ch(41);
                let hp = eload(dv); let c4 = ecr(4); let hl = eload(ebin("add", dv, c4));
                let cur = eload(egref_str_cursor()); let base = ebin("add", egref_str_arena(), cur);
                print!("  v{}: i32 = call __str_replace_into v{} v{} v{} v{} v{}", NC, base, hp, hl, fromd, tod); println!(""); let total = NC; NC = NC + 1;
                let c3 = ecr(3); let c4b = ecr(4); let doff = ebin("mul", ebin("div", ebin("add", total, c3), c4b), c4b);
                let desc = ebin("add", base, doff); println!("  store v{} v{}", desc, base);
                let c4c = ecr(4); println!("  store v{} v{}", ebin("add", desc, c4c), total);
                let c8 = ecr(8); let newcur = ebin("add", cur, ebin("add", doff, c8)); println!("  store v{} v{}", egref_str_cursor(), newcur);
                EISSTR = 1; EISSTRING = 1; return desc;
            }
            if id_is(99, 104, 97, 114, 115, 5) || id_is(98, 121, 116, 101, 115, 5) {   /* R2: chars()/bytes() -- support the `.count()` terminal -> byte length (ASCII) */
                let ischar = id_is(99, 104, 97, 114, 115, 5);   /* R36: chars() (vs bytes()) -> .next()/.nth() unwrap to a char (Display prints the glyph), not a u8 */
                skipws(); eat_ch(40); eat_ch(41); skipws();
                if POS < LEN && at(POS) == 46 { let s2 = POS; POS = POS + 1; rd_ident();
                    if id_is(99, 111, 117, 110, 116, 5) { skipws(); eat_ch(40); eat_ch(41); let c4 = ecr(4); return eload(ebin("add", dv, c4)); }
                    if id_is(110, 101, 120, 116, 0, 4) {                             /* R36: chars()/bytes().next() -> Option<char/u8> of the first element (== nth(0)) */
                        skipws(); eat_ch(40); eat_ch(41);
                        let p = eload(dv); let c4 = ecr(4); let ln = eload(ebin("add", dv, c4));
                        let k = ec(0); let valid = ebin("lt", k, ln); let byte = eloadb(ebin("add", p, k));
                        let cell = halloc(8); println!("  store v{} v{}", cell, valid);
                        let c4b = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4b), byte);
                        let r = opt_postfix(cell); if ischar == 1 { EISCHAR = 1; skipws(); if POS < LEN && at(POS) == 46 { return opt_postfix(r); } }   /* R36: chain a trailing char method (.to_ascii_uppercase()) on the unwrapped char */
                        return r;
                    }
                    if id_is(114, 101, 118, 0, 0, 3) {                               /* R2am: chars().rev().collect::<String>() -> reversed arena String */
                        skipws(); eat_ch(40); eat_ch(41); skipws();
                        if POS < LEN && at(POS) == 46 { POS = POS + 1; rd_ident();
                            if id_is(99, 111, 108, 108, 101, 7) {                     /* collect */
                                skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }
                                skipws(); eat_ch(40); eat_ch(41);
                                let p = eload(dv); let c4 = ecr(4); let ln = eload(ebin("add", dv, c4));
                                let cur = eload(egref_str_cursor()); let base = ebin("add", egref_str_arena(), cur);
                                print!("  v{}: i32 = call __str_rev v{} v{} v{}", NC, base, p, ln); println!(""); NC = NC + 1;
                                let c3 = ecr(3); let c4b = ecr(4); let doff = ebin("mul", ebin("div", ebin("add", ln, c3), c4b), c4b);
                                let desc = ebin("add", base, doff); println!("  store v{} v{}", desc, base);
                                let c4c = ecr(4); println!("  store v{} v{}", ebin("add", desc, c4c), ln);
                                let c8 = ecr(8); let newcur = ebin("add", cur, ebin("add", doff, c8)); println!("  store v{} v{}", egref_str_cursor(), newcur);
                                EISSTR = 1; EISSTRING = 1; return desc;
                            }
                        }
                    }
                    let mut fcollect = 0;   /* R29b: lookahead -- does `filter(|..|..)` terminate in `.collect` (my String branch) vs `.count` (fall through)? */
                    if id_is(102, 105, 108, 116, 101, 6) && POS < LEN && at(POS) == 40 && POS + 1 < LEN && at(POS + 1) == 124 {
                        let mut q = POS + 1; let mut d = 1; while q < LEN && d > 0 { if at(q) == 40 { d = d + 1; } if at(q) == 41 { d = d - 1; } q = q + 1; }   /* skip the closure `( .. )` */
                        while q < LEN && (at(q) == 32 || at(q) == 9) { q = q + 1; }
                        if q < LEN && at(q) == 46 { q = q + 1; if q + 6 < LEN && at(q) == 99 && at(q + 1) == 111 && at(q + 2) == 108 && at(q + 3) == 108 && at(q + 4) == 101 && at(q + 5) == 99 && at(q + 6) == 116 { fcollect = 1; } }
                    }
                    if id_is(102, 105, 108, 116, 101, 6) && fcollect == 1 && POS < LEN && at(POS) == 40 && POS + 1 < LEN && at(POS + 1) == 124 {   /* R29b: chars().filter(|c| pred).collect::<String>() -- keep chars where pred holds */
                        skipws(); eat_ch(40); skipws(); eat_ch(124); skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; } rd_ident(); let co = ID_O; let cl = ID_L; skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } } eat_ch(124);
                        let bodypos = POS;
                        let dst = emit_str_new();
                        let p = eload(dv); let c4 = ecr(4); let ln = eload(ebin("add", dv, c4));
                        let fi2 = NC; let z = ecr(0); ecopy(fi2, z); NC = NC + 1;
                        let cslot = NC; NC = NC + 1;
                        let mark = NSYM; sym_add(co, cl, cslot); SCHAR[(NSYM - 1) as usize] = 1;
                        println!("  loop"); ckreset(); let cnd = ebin("lt", fi2, ln); println!("  brz v{}", cnd); ckreset();
                        let byte = eloadb(ebin("add", p, fi2)); ecopy(cslot, byte);
                        POS = bodypos; let pred = parse_expr();   /* filter predicate */
                        println!("  brz v{}", pred); ckreset();   /* pred==0 -> skip the push */
                        emit_str_push_byte(dst, cslot);
                        println!("  pool"); ckreset();
                        println!("  cont"); ckreset(); let one = ecr(1); ecopy(fi2, ebin("add", fi2, one)); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                        NSYM = mark; skipws(); eat_ch(41);
                        let c4d = ecr(4);
                        skipws(); if POS < LEN && at(POS) == 46 { POS = POS + 1; rd_ident();
                            if id_is(99, 111, 117, 110, 116, 5) { skipws(); eat_ch(40); eat_ch(41); return eload(ebin("add", dst, c4d)); }   /* .count() -> the number of kept chars (the String's len) */
                            if id_is(99, 111, 108, 108, 101, 7) { skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } } skipws(); eat_ch(40); eat_ch(41); } }
                        EISSTR = 1; EISSTRING = 1; return dst;
                    }
                    if id_is(109, 97, 112, 0, 0, 3) {   /* R28b: chars().map(|c| body).collect::<String>() -- transform each char, build a String */
                        skipws(); eat_ch(40); skipws(); eat_ch(124); skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; } rd_ident(); let co = ID_O; let cl = ID_L; skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } } eat_ch(124);
                        let bodypos = POS;
                        let dst = emit_str_new();
                        let p = eload(dv); let c4 = ecr(4); let ln = eload(ebin("add", dv, c4));
                        let mi = NC; let z = ecr(0); ecopy(mi, z); NC = NC + 1;
                        let cslot = NC; NC = NC + 1;
                        let mark = NSYM; sym_add(co, cl, cslot); SCHAR[(NSYM - 1) as usize] = 1;
                        println!("  loop"); ckreset(); let cnd = ebin("lt", mi, ln); println!("  brz v{}", cnd); ckreset();
                        let byte = eloadb(ebin("add", p, mi)); ecopy(cslot, byte);
                        POS = bodypos; let rv = parse_expr();   /* map body -> a char */
                        emit_str_push_byte(dst, rv);
                        println!("  cont"); ckreset(); let one = ecr(1); ecopy(mi, ebin("add", mi, one)); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                        NSYM = mark; skipws(); eat_ch(41);   /* close the closure arg */
                        skipws(); if POS < LEN && at(POS) == 46 { POS = POS + 1; rd_ident();   /* .collect() */
                            if id_is(99, 111, 108, 108, 101, 7) { skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } } skipws(); eat_ch(40); eat_ch(41); } }
                        EISSTR = 1; EISSTRING = 1; return dst;
                    }
                    if id_is(110, 116, 104, 0, 0, 3) {                               /* R2: chars()/bytes().nth(k) -> Option<char/u8> */
                        skipws(); eat_ch(40); let k = parse_expr(); eat_ch(41);
                        let p = eload(dv); let c4 = ecr(4); let ln = eload(ebin("add", dv, c4));
                        let valid = ebin("lt", k, ln); let byte = eloadb(ebin("add", p, k));
                        let cell = halloc(8); println!("  store v{} v{}", cell, valid);
                        let c4b = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4b), byte);
                        let r = opt_postfix(cell); if ischar == 1 { EISCHAR = 1; skipws(); if POS < LEN && at(POS) == 46 { return opt_postfix(r); } } return r;   /* R36: chars().nth(k).unwrap() -> a char (+chain a trailing char method), not a u8 */
                    }
                    if id_is(102, 105, 108, 116, 101, 6) {   /* R2bt: bytes().filter(|&b| pred).count() */
                        eat_ch(40); skipws(); eat_ch(124); skipws(); while POS < LEN && at(POS) == 38 { POS = POS + 1; }
                        rd_ident(); let po = ID_O; let pl = ID_L; skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 124 { POS = POS + 1; } } eat_ch(124);
                        let p = eload(dv); let c4 = ecr(4); let len = eload(ebin("add", dv, c4));
                        let cnt = NC; let z0 = ecr(0); ecopy(cnt, z0); NC = NC + 1; let i = NC; ecopy(i, ec(0)); NC = NC + 1; let bslot = NC; NC = NC + 1;
                        let mark = NSYM; sym_add(po, pl, bslot); mark_forref(); println!("  loop"); ckreset();
                        println!("  brz v{}", ebin("lt", i, len)); ckreset();
                        ecopy(bslot, eloadb(ebin("add", p, i))); let pred = parse_expr();
                        println!("  brz v{}", pred); ckreset(); ecopy(cnt, ebin("add", cnt, ec(1))); println!("  pool"); ckreset();
                        println!("  cont"); ckreset(); ecopy(i, ebin("add", i, ec(1))); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                        NSYM = mark; eat_ch(41); skipws();
                        if POS < LEN && at(POS) == 46 { POS = POS + 1; rd_ident(); if id_is(99, 111, 117, 110, 116, 5) { skipws(); eat_ch(40); eat_ch(41); } }
                        EISBOOL = 0; EISSTR = 0; EISSTRING = 0; return cnt;
                    }
                    POS = s2;
                }
                if ischar == 1 {   /* P-CHARS-ITER: bare `s.chars()` (no recognized terminal) stored as a cursor: `let mut it = s.chars()`. Build [buf, bytelen, pos=0] and signal a char-cursor local (SK=13). Was: returned the length (an int) -> it.next() NOELF. */
                    let c4 = ecr(4); let cur = halloc(12);
                    println!("  store v{} v{}", cur, eload(dv));
                    println!("  store v{} v{}", ebin("add", cur, c4), eload(ebin("add", dv, c4)));
                    println!("  store v{} v{}", ebin("add", cur, ec(8)), ec(0));
                    EISITER = 1; EISCHAR = 1; EISVEC = 0; EISSTR = 0; EISSTRING = 0;
                    return cur;
                }
                let c4 = ecr(4); return eload(ebin("add", dv, c4));
            }
            POS = s;
        }
        EISSTR = 1;
        dv
    }
}

/* R2bg: does the body span [bs,be) assign to NAME (o,l)? -- `NAME =` (not ==) or `NAME op=`. */
fn body_mutates(o: i32, l: i32, bs: i32, be: i32) -> i32 {
    unsafe {
        let mut q = bs;
        loop {
            if q >= be { return 0; }
            if is_idst(at(q)) {
                let s = q;
                while q < be && is_idc(at(q)) { q = q + 1; }
                if span_eq_at(o, l, s, q - s) {
                    let mut p = q;
                    while p < be && (at(p) == 32 || at(p) == 9) { p = p + 1; }
                    if p < be && (at(p) == 43 || at(p) == 45 || at(p) == 42 || at(p) == 47) { if p + 1 < be && at(p + 1) == 61 { return 1; } }   /* op= */
                    if p < be && at(p) == 61 && (p + 1 >= be || at(p + 1) != 61) { return 1; }   /* = (not ==) */
                }
            } else { q = q + 1; }
        }
    }
}
/* R10: parse `|params| body` -> register + build a heap {code, env} fat pointer from captured locals. */
/* MOVEBORROW m3: scan a `move ||` closure body (POS at the leading `|`) and mark every captured
   non-Copy String(sk=3)/Vec(sk=2) outer local as moved. Call AFTER parse_closure so the closure's own
   capture-reads don't trip the use-check. Skips the `|params|` header; body ends at a depth-0 `;`/`,`. */
fn mark_move_captures() {
    unsafe {
        if POS < LEN && at(POS) == 124 { POS = POS + 1; if POS < LEN && at(POS) == 124 { POS = POS + 1; } else { while POS < LEN && at(POS) != 124 { POS = POS + 1; } if POS < LEN { POS = POS + 1; } } }   /* skip |params| (or ||) */
        let mut depth = 0;
        loop {
            skipws();
            if POS >= LEN { break; }
            let c = at(POS);
            if c == 123 || c == 40 || c == 91 { depth = depth + 1; POS = POS + 1; }
            else if c == 125 || c == 41 || c == 93 { if depth == 0 { break; } depth = depth - 1; POS = POS + 1; }
            else if depth == 0 && (c == 59 || c == 44) { break; }
            else if is_idst(c) { rd_ident(); let io = ID_O; let il = ID_L; let sk = sym_kind(io, il); if sk == 2 || sk == 3 { mark_moved(io, il); } }
            else { POS = POS + 1; }
        }
    }
}
fn parse_closure() -> i32 {
    unsafe {
        let cid = NCLO; NCLO = NCLO + 1;
        CFORREF[cid as usize] = CLO_FORREF; CLO_FORREF = 0;   /* std-hoist P4.2: routed iter-adapter closure -> its params are element for-refs (`*x` no-op) */
        POS = POS + 1;                                        /* first `|` */
        skipws();
        let mut np = 0;
        if POS < LEN && at(POS) == 124 { POS = POS + 1; }     /* `||` no params */
        else {
            loop {
                skipws();
                if POS < LEN && at(POS) == 124 { POS = POS + 1; break; }
                while POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); }   /* R2as: `|&x|`/`|&&x|` param -- strip the ref pattern; x binds the value (was: rd_ident on `&` read nothing -> x unbound = v-1) */
                if is_kw_mut_next() { }
                rd_ident(); CPO[(cid * 4 + np) as usize] = ID_O; CPL[(cid * 4 + np) as usize] = ID_L; np = np + 1;
                skipws();
                if POS < LEN && at(POS) == 58 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; if is_kw_mut_next() { } } rd_ident(); skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }
                skipws();
                if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { if POS < LEN && at(POS) == 124 { POS = POS + 1; } break; }
            }
        }
        CNP[cid as usize] = np;
        skipws();
        if POS + 1 < LEN && at(POS) == 45 && at(POS + 1) == 62 { POS = POS + 2; skipws(); while POS < LEN && at(POS) != 123 { POS = POS + 1; } }   /* LAYER3 M2: closure explicit return type `|x| -> T { body }` -- skip `-> T` (a return type has no `{`, and such a closure always has a block body) so body detection lands on the `{` */
        skipws();
        let mut isblock = 0; let bstart = POS;
        if POS < LEN && at(POS) == 123 {
            isblock = 1; let mut d = 0;
            loop { if POS >= LEN { break; } if at(POS) == 123 { d = d + 1; POS = POS + 1; } else if at(POS) == 125 { d = d - 1; POS = POS + 1; if d == 0 { break; } } else { POS = POS + 1; } }
        } else {
            let mut d = 0;
            loop { if POS >= LEN { break; } let c = at(POS);
                if c == 40 || c == 91 || c == 123 { d = d + 1; POS = POS + 1; }
                else if c == 41 || c == 93 || c == 125 { if d == 0 { break; } d = d - 1; POS = POS + 1; }
                else if d == 0 && (c == 44 || c == 59) { break; } else { POS = POS + 1; } }
        }
        let bend = POS;
        CBO[cid as usize] = bstart; CBL[cid as usize] = bend - bstart; CBLK[cid as usize] = isblock;
        let mut crbp = bstart; while crbp < bend && (at(crbp) == 32 || at(crbp) == 9) { crbp = crbp + 1; }
        let mut crisc = 0;
        if crbp < bend && at(crbp) == 124 { crisc = 1; }
        else if crbp + 4 < bend && at(crbp) == 109 && at(crbp + 1) == 111 && at(crbp + 2) == 118 && at(crbp + 3) == 101 { let mut crq = crbp + 4; while crq < bend && (at(crq) == 32 || at(crq) == 9) { crq = crq + 1; } if crq < bend && at(crq) == 124 { crisc = 1; } }
        CRETCLO[cid as usize] = crisc;
        /* capture scan: idents in the body resolving to an enclosing local (not a param, not after `.`) */
        let mut nc = 0; let mut q = bstart;
        loop {
            if q >= bend { break; }
            if is_idst(at(q)) {
                let s = q;
                while q < bend && is_idc(at(q)) { q = q + 1; }
                let len = q - s;
                let mut isp = 0; let mut i = 0;
                while i < np { if span_eq_at(CPO[(cid * 4 + i) as usize], CPL[(cid * 4 + i) as usize], s, len) { isp = 1; } i = i + 1; }
                let mut pp = s - 1;
                while pp >= bstart && (at(pp) == 32 || at(pp) == 9) { pp = pp - 1; }
                let isdot = pp >= bstart && at(pp) == 46;
                let si = sym_find(s, len);
                if isp == 0 && !isdot && si >= 0 {
                    let mut dup = 0; let mut j = 0;
                    while j < nc { if span_eq_at(CAO[(cid * 8 + j) as usize], CAL[(cid * 8 + j) as usize], s, len) { dup = 1; } j = j + 1; }
                    if dup == 0 && nc < 8 {
                        CAO[(cid * 8 + nc) as usize] = s; CAL[(cid * 8 + nc) as usize] = len;
                        let ck = sym_kind(s, len);
                        CAK[(cid * 8 + nc) as usize] = ck; CAS[(cid * 8 + nc) as usize] = sym_styp(s, len);
                        let mut br = 0;
                        if ck == 0 && body_mutates(s, len, bstart, bend) == 1 { br = 1; }   /* R2bg: mutated scalar capture -> capture by address (FnMut writeback) */
                        CABR[(cid * 8 + nc) as usize] = br;
                        nc = nc + 1;
                    }
                }
            } else { q = q + 1; }
        }
        CNC[cid as usize] = nc;
        let mut envsz = nc * 4; if envsz < 4 { envsz = 4; }
        let env = halloc(envsz);
        let mut i = 0;
        while i < nc {
            let cv0 = sym_find(CAO[(cid * 8 + i) as usize], CAL[(cid * 8 + i) as usize]);
            let mut cv = cv0;
            if CABR[(cid * 8 + i) as usize] == 1 { cv = eref(cv0); }   /* R2bg: store &outer_cell so mutations write back */
            let off = ec(i * 4); let ad = ebin("add", env, off);
            println!("  store v{} v{}", ad, cv);
            i = i + 1;
        }
        let clo = halloc(8);
        println!("  v{}: i32 = faddr __closure_{}", NC, cid); let fa = NC; NC = NC + 1;
        println!("  store v{} v{}", clo, fa);
        let c4 = ecr(4); let ad2 = ebin("add", clo, c4);
        println!("  store v{} v{}", ad2, env);
        EISCLO = 1; ECRETCLO = CRETCLO[cid as usize];
        clo
    }
}
/* R10: call a closure fat pointer fv -> calli code env args... */
fn emit_closure_call(fv: i32) -> i32 {
    unsafe {
        POS = POS + 1;                                       /* `(` */
        let mut a0 = -1; let mut a1 = -1; let mut a2 = -1; let mut a3 = -1; let mut na = 0;
        skipws();
        if peek() != 41 {
            a0 = parse_expr(); na = 1;
            if eat_ch(44) { a1 = parse_expr(); na = 2;
            if eat_ch(44) { a2 = parse_expr(); na = 3;
            if eat_ch(44) { a3 = parse_expr(); na = 4; } } }
        }
        eat_ch(41);
        let code = eload(fv);
        let c4 = ecr(4); let envad = ebin("add", fv, c4); let env = eload(envad);
        print!("  v{}: i32 = calli v{} v{}", NC, code, env);
        if na >= 1 { print!(" v{}", a0); }
        if na >= 2 { print!(" v{}", a1); }
        if na >= 3 { print!(" v{}", a2); }
        if na >= 4 { print!(" v{}", a3); }
        println!("");
        NC = NC + 1;
        NC - 1
    }
}
fn emit_closure_body(cid: i32) {
    unsafe {
        NC = 0; NSYM = 0; ckreset();
        println!("fn __closure_{}", cid);
        let envvn = NC; println!("  v{}: i32 = param", envvn); NC = NC + 1;
        let np = CNP[cid as usize];
        let mut i = 0;
        while i < np { println!("  v{}: i32 = param", NC); NC = NC + 1; i = i + 1; }
        let nc = CNC[cid as usize];
        i = 0;
        while i < nc {
            let off = ec(i * 4); let ad = ebin("add", envvn, off); let lv = eload(ad);
            sym_add(CAO[(cid * 8 + i) as usize], CAL[(cid * 8 + i) as usize], lv);
            SK[(NSYM - 1) as usize] = CAK[(cid * 8 + i) as usize];
            SST[(NSYM - 1) as usize] = CAS[(cid * 8 + i) as usize];
            if CABR[(cid * 8 + i) as usize] == 1 { SBYREF[(NSYM - 1) as usize] = 1; }   /* R2bg: lv is the address of the outer cell -- reads deref, writes store through it */
            i = i + 1;
        }
        i = 0;
        while i < np { sym_add(CPO[(cid * 4 + i) as usize], CPL[(cid * 4 + i) as usize], envvn + 1 + i); if CFORREF[cid as usize] == 1 { SFORREF[(NSYM - 1) as usize] = 1; } i = i + 1; }   /* std-hoist P4.2: routed iter closure -> params are value-passed element for-refs (`*x` a no-op) */
        let czero = ecr(0);
        POS = CBO[cid as usize];
        if CBLK[cid as usize] == 1 { parse_block(); println!("  ret v{}", czero); }
        else { let r = parse_expr(); println!("  ret v{}", r); }
    }
}

/* W1b: env-dropping thunk for an address-taken fn -> `fn __fp_NAME(env, a0..) { ret NAME(a0..) }`.
   calli passes env as the implicit first arg; the thunk ignores it and forwards the real args. */
fn emit_fnptr_thunk(k: i32) {
    unsafe {
        NC = 0; NSYM = 0; ckreset();
        let ac = FPTAC[k as usize];
        print!("fn __fp_"); emit_span(FPTO[k as usize], FPTL[k as usize]); println!("");
        println!("  v0: i32 = param");   /* env (ignored) */
        NC = 1;
        let mut i = 0;
        while i < ac { println!("  v{}: i32 = param", NC); NC = NC + 1; i = i + 1; }
        print!("  v{}: i32 = call ", NC); emit_span(FPTO[k as usize], FPTL[k as usize]);
        i = 0;
        while i < ac { print!(" v{}", i + 1); i = i + 1; }
        println!("");
        let r = NC; NC = NC + 1;
        println!("  ret v{}", r);
    }
}
fn mac_find(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NMAC { if span_eq_at(MACO[i as usize], MACL[i as usize], o, l) { return i; } i = i + 1; } -1 }
}
/* A: prepass -- register `macro_rules! NAME { ( $p:expr, .. ) => { BODY } }` (single rule, expr params). */
fn prepass_macros() {
    unsafe {
        POS = 0;
        loop {
            skipws();
            if POS >= LEN { break; }
            let save = POS;
            if rd_ident() && id_is(109, 97, 99, 114, 111, 11) {                     /* macro_rules */
                skipws(); if POS < LEN && at(POS) == 33 { POS = POS + 1; }           /* ! */
                skipws(); rd_ident(); let mo = ID_O; let ml = ID_L;
                skipws(); eat_ch(123);                                              /* outer { */
                skipws(); eat_ch(40);                                              /* ( params ) */
                let mut np = 0; let mut mdepth = 0; let mut isrep = 0;   /* LAYER3 M1: isrep=1 for a repetition matcher `$( $x:frag )sep*` */
                loop {
                    skipws();
                    if POS >= LEN { break; }
                    let c = at(POS);
                    if c == 41 && mdepth == 0 { break; }                            /* the matcher's closing `)` at depth 0 */
                    if c == 36 && POS + 1 < LEN && at(POS + 1) == 40 {              /* LAYER3 M1: `$( $x:frag )sep*` repetition matcher */
                        isrep = 1; POS = POS + 2; skipws();
                        if POS < LEN && at(POS) == 36 { POS = POS + 1; rd_ident();
                            if np < 4 { MACPO[(NMAC * 4 + np) as usize] = ID_O; MACPL[(NMAC * 4 + np) as usize] = ID_L; np = np + 1; }
                            skipws(); if POS < LEN && at(POS) == 58 { POS = POS + 1; skipws(); rd_ident(); } }   /* $x:expr */
                        while POS < LEN && at(POS) != 41 { POS = POS + 1; } if POS < LEN { POS = POS + 1; }   /* to the group's ) */
                        skipws(); if POS < LEN && at(POS) != 41 && at(POS) != 42 && at(POS) != 43 { POS = POS + 1; }   /* optional separator (, or ;) */
                        if POS < LEN && (at(POS) == 42 || at(POS) == 43) { POS = POS + 1; }   /* * or + */
                    }
                    else if c == 40 || c == 91 || c == 123 { mdepth = mdepth + 1; POS = POS + 1; }   /* ( [ { -- nested (e.g. `#[cfg(..)]`) */
                    else if c == 41 || c == 93 || c == 125 { if mdepth > 0 { mdepth = mdepth - 1; } POS = POS + 1; }
                    else if c == 36 {                                               /* $name:frag param */
                        POS = POS + 1; rd_ident();
                        if np < 4 { MACPO[(NMAC * 4 + np) as usize] = ID_O; MACPL[(NMAC * 4 + np) as usize] = ID_L; np = np + 1; }
                        skipws(); if POS < LEN && at(POS) == 58 { POS = POS + 1; skipws(); rd_ident(); }   /* : fragment specifier */
                    }
                    else { POS = POS + 1; }                                          /* literal matcher token (`,` `as` `#` etc.) -- skip; GUARANTEES progress (was: spun on `#[..]`) */
                }
                if POS < LEN && at(POS) == 41 { POS = POS + 1; }
                skipws(); if POS < LEN && at(POS) == 61 { POS = POS + 1; } if POS < LEN && at(POS) == 62 { POS = POS + 1; }   /* => */
                skipws(); eat_ch(123);                                              /* body { */
                let bo = POS; let mut d = 1;
                while POS < LEN && d > 0 { if at(POS) == 123 { d = d + 1; } if at(POS) == 125 { d = d - 1; } POS = POS + 1; }
                let be = POS - 1;                                                   /* before the body's } */
                MACO[NMAC as usize] = mo; MACL[NMAC as usize] = ml; MACBO[NMAC as usize] = bo; MACBL[NMAC as usize] = be - bo; MACNP[NMAC as usize] = np;
                if isrep == 1 { MACNP[NMAC as usize] = 99; }   /* LAYER3 M1: repeated macro -> MACNP=99 sentinel; MACBO/MACBL stay the WHOLE body (the `$(...)*` group is found + expanded per-arg at use, embedded or whole-body) */
                NMAC = NMAC + 1;
                skipws(); if POS < LEN && at(POS) == 125 { POS = POS + 1; }         /* outer } */
            } else { POS = save; skip_tok(); }
        }
    }
}
/* A: expand `NAME!(args)` -- POS is at the `!`. Bind each arg span, then parse the body with `$p` substitution. */
fn expand_macro(mi: i32) -> i32 {
    unsafe {
        if MACNP[mi as usize] == 99 {   /* LAYER3 M1: repetition macro -- structural init/group/tail split; emit the body's `$( <inner> )*` group once per call arg, `$x`=arg k. Handles fan-out (whole body IS the group) AND embedded `{ init; $(acc)* tail }` at expr position. */
            POS = POS + 1; skipws(); eat_ch(40);
            let argstart = POS; let mut d0 = 0;
            loop { if POS >= LEN { break; } let c = at(POS);
                if c == 40 || c == 91 || c == 60 { d0 = d0 + 1; POS = POS + 1; }
                else if c == 41 { if d0 == 0 { break; } d0 = d0 - 1; POS = POS + 1; }
                else if c == 93 || c == 62 { if d0 > 0 { d0 = d0 - 1; } POS = POS + 1; }
                else { POS = POS + 1; } }
            let argend = POS; let retpos = if POS < LEN { POS + 1 } else { POS };
            let saved_np = CURNP; CURNP = 1; CURPO[0] = MACPO[(mi * 4) as usize]; CURPL[0] = MACPL[(mi * 4) as usize];
            let ws = MACBO[mi as usize]; let we = ws + MACBL[mi as usize];
            let mut gp = ws; while gp + 1 < we && !(at(gp) == 36 && at(gp + 1) == 40) { gp = gp + 1; }   /* find the `$(` group */
            let inner = gp + 2; let mut ge = inner; let mut dd = 1;
            while ge < we && dd > 0 { if at(ge) == 40 { dd = dd + 1; } else if at(ge) == 41 { dd = dd - 1; if dd == 0 { break; } } ge = ge + 1; }   /* group inner [inner, ge) */
            let mut ag = ge; if ag < we { ag = ag + 1; }   /* past ) */
            while ag < we && (at(ag) == 32 || at(ag) == 9 || at(ag) == 10 || at(ag) == 13) { ag = ag + 1; }
            if ag < we && at(ag) != 42 && at(ag) != 43 && at(ag) != 125 { ag = ag + 1; }   /* optional separator */
            if ag < we && (at(ag) == 42 || at(ag) == 43) { ag = ag + 1; }   /* * or + */
            POS = ws; skipws();
            let isblock = POS < we && at(POS) == 123;
            let mut v = ec(0);
            if isblock { POS = POS + 1; loop { skipws(); if POS >= gp { break; } parse_stmt(); } }   /* init statements before the group */
            let mut sp = argstart;   /* emit the group inner once per call arg */
            loop {
                while sp < argend && (at(sp) == 32 || at(sp) == 9 || at(sp) == 10 || at(sp) == 13) { sp = sp + 1; }
                if sp >= argend { break; }
                let ao = sp; let mut d1 = 0;
                loop { if sp >= argend { break; } let c = at(sp);
                    if c == 40 || c == 91 || c == 60 { d1 = d1 + 1; sp = sp + 1; }
                    else if c == 41 || c == 93 || c == 62 { if d1 == 0 { break; } d1 = d1 - 1; sp = sp + 1; }
                    else if d1 == 0 && (c == 44 || c == 59) { break; }
                    else { sp = sp + 1; } }
                CURAO[0] = ao; CURAL[0] = sp - ao;
                POS = inner; loop { skipws(); if POS >= ge { break; } parse_stmt(); }
                while sp < argend && (at(sp) == 32 || at(sp) == 9 || at(sp) == 10 || at(sp) == 13) { sp = sp + 1; }
                if sp < argend && (at(sp) == 44 || at(sp) == 59) { sp = sp + 1; }
            }
            if isblock { POS = ag; skipws(); if POS < we && at(POS) != 125 { v = parse_expr(); } }   /* tail expr = the block value */
            CURNP = saved_np; POS = retpos; return v;
        }
        POS = POS + 1; skipws(); eat_ch(40);
        let np = MACNP[mi as usize];
        let saved_np = CURNP;
        let mut k = 0;
        loop {
            skipws();
            if POS >= LEN || at(POS) == 41 { break; }
            let ao = POS; let mut d = 0;
            loop {
                if POS >= LEN { break; }
                let c = at(POS);
                if c == 40 || c == 91 || c == 60 { d = d + 1; POS = POS + 1; }
                else if d == 0 && c == 61 && POS + 1 < LEN && at(POS + 1) == 62 { break; }   /* R-either: `=>` separates matcher metavars (`$p:pat => $r:expr`) -- split the arg here too, else `pattern => result` is captured as ONE arg, the trailing metavar reads a stale span -> mis-expand/hang */
                else if c == 41 || c == 93 || c == 62 { if d == 0 { break; } d = d - 1; POS = POS + 1; }
                else if d == 0 && c == 44 { break; }
                else { POS = POS + 1; }
            }
            if k < 4 { CURAO[k as usize] = ao; CURAL[k as usize] = POS - ao; }
            k = k + 1;
            skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else if POS + 1 < LEN && at(POS) == 61 && at(POS + 1) == 62 { POS = POS + 2; }   /* consume the `=>` separator too */
        }
        if POS < LEN && at(POS) == 41 { POS = POS + 1; }
        let retpos = POS;
        CURNP = np;
        let mut j = 0; while j < np { CURPO[j as usize] = MACPO[(mi * 4 + j) as usize]; CURPL[j as usize] = MACPL[(mi * 4 + j) as usize]; j = j + 1; }
        POS = MACBO[mi as usize];
        let v = parse_expr();
        POS = retpos;
        CURNP = saved_np;
        v
    }
}
/* R2ci: soft-float is not implemented in this chain (the backend has no float ops), so a runtime f64
   value cannot be represented. The one float use we CAN honor is a constant `<floatlit> as <int-type>`,
   whose value is known at compile time (truncation toward zero == the integer part `n`). Everything else
   (float arithmetic, printing an f64, `int as f64`) is a clean REJECT instead of the old SIGSEGV. */
fn cast_floatlit(n: i64) -> i32 {
    unsafe {
        let fo = POS;
        skipws();
        if POS + 1 < LEN && at(POS) == 97 && at(POS + 1) == 115 && (POS + 2 >= LEN || !is_idc(at(POS + 2))) {
            POS = POS + 2; skipws(); rd_ident();                     /* consume the `as <type>` target */
            let t0 = at(ID_O); let t1 = at(ID_O + 1);
            if t0 == 102 { die_unsupported(fo, 1); }                 /* `<floatlit> as f32/f64` -> still a float, unsupported */
            if (t0 == 105 || t0 == 117) {                            /* i8/i16/i32/i64/isize/u8/.../usize -> truncated int */
                if ID_L == 3 && t1 == 54 && at(ID_O + 2) == 52 { return opt_postfix(emit_i64_lit(n)); }   /* i64/u64 */
                let mut m = n; if m >= 2147483648 { m = m % 2147483648; } if m < 0 - 2147483648 { m = 0; }
                return opt_postfix(ec64raw(m));
            }
            die_unsupported(fo, 1);                                  /* `as` some non-numeric type */
        }
        die_unsupported(fo, 1);                                      /* bare f64 value: soft-float unsupported */
        0
    }
}
/* NAMERES: build a bare enum-variant constructor cell A(..)/A{..} (variant index vi already resolved; POS at `(` or `{`). Mirrors the qualified E::Variant path. */
fn emit_bare_variant(vi: i32) -> i32 {
    unsafe {
        let en = VEN[vi as usize];
        let disc = VDISC[vi as usize]; let has = VHAS[vi as usize]; let nf = VNF[vi as usize];
        let mut sz = 4 + nf * 4; if sz < 8 { sz = 8; }
        let cell = halloc(sz);
        let cd = ec(disc); println!("  store v{} v{}", cell, cd);
        let mut pgak = 0; let mut pgat = 0 - 1; let mut phas = 0;
        if has == 1 {
            if at(POS) == 123 {                                                  /* struct-variant A{ f: e, .. } */
                POS = POS + 1; let mut fi = 0;
                loop {
                    skipws(); if POS < LEN && at(POS) == 125 { break; }
                    rd_ident(); skipws(); eat_ch(58);
                    let mut arg = parse_expr(); skipws(); if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 { arg = emit_range_vec(arg); }   /* P-RANGE-VALUE-IN-VARIANT-ARG: a bare `lo..hi` payload */
                    if fi == 0 { if EISSTR == 1 { pgak = 1; phas = 1; } else if EISSTRING == 1 { pgak = 3; phas = 1; } else if EISVEC == 1 { pgak = 2; phas = 1; } else if EITYP >= 0 { pgak = 0; pgat = EITYP; phas = 1; } }
                    let off = ec(vpay_off(vi, fi)); let ad = ebin("add", cell, off); if vpay_i64(vi, fi) == 1 { println!("  store8 v{} v{}", ad, arg); } else { println!("  store v{} v{}", ad, arg); }   /* P-ENUM-I64-PAYLOAD-OFFSET */
                    fi = fi + 1;
                    skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; }
                }
                eat_ch(125);
            } else {                                                             /* tuple-variant A(e, ..) */
                POS = POS + 1; let mut fi = 0;
                loop {
                    skipws(); if POS < LEN && at(POS) == 41 { break; }
                    let mut arg = parse_expr(); skipws(); if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 { arg = emit_range_vec(arg); }   /* P-RANGE-VALUE-IN-VARIANT-ARG: a bare `lo..hi` payload */
                    if fi == 0 { if EISSTR == 1 { pgak = 1; phas = 1; } else if EISSTRING == 1 { pgak = 3; phas = 1; } else if EISVEC == 1 { pgak = 2; phas = 1; } else if EITYP >= 0 { pgak = 0; pgat = EITYP; phas = 1; } }
                    let off = ec(vpay_off(vi, fi)); let ad = ebin("add", cell, off); if vpay_i64(vi, fi) == 1 { println!("  store8 v{} v{}", ad, arg); } else { println!("  store v{} v{}", ad, arg); }   /* P-ENUM-I64-PAYLOAD-OFFSET */
                    fi = fi + 1;
                    skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; }
                }
                eat_ch(41);
            }
        }
        EISENUM = 1; EIENUMTY = en; if phas == 1 { EGAK = pgak; EGAT = pgat; EWASGARG = 1; }
        cell
    }
}
/* R-either (P-RANGE-VALUE-IN-VARIANT-ARG): materialize a bare `lo..[=]hi` range value into an eager Vec.
   POS is at the `..`; `v0` is the already-parsed low bound. Mirrors the parenthesized `(a..b)` path at
   parse_atom (5042) but does NOT consume a closing `)` -- the caller owns its delimiter. Lets a range
   appear as a variant-constructor arg (`Left(0..10)`) without the `..hi` tail leaking into the enclosing
   context (which desynced a non-last match arm -> false non-exhaustive). */
fn emit_range_vec(v0: i32) -> i32 {
    unsafe {
        POS = POS + 2; let mut incl = 0; if POS < LEN && at(POS) == 61 { POS = POS + 1; incl = 1; }
        skipws();
        if POS >= LEN || at(POS) == 41 || at(POS) == 44 || at(POS) == 93 || at(POS) == 125 || at(POS) == 59 {   /* P-RANGE-VALUE-IN-VARIANT-ARG: open-ended `lo..` (RangeFrom) -- no end token. An infinite range can't be materialized; emit an empty Vec so the parse does NOT desync (was: parse_expr on the `)` ran POS past the delimiter, drifting the enclosing parse ~100 lines downstream -> a spurious `cannot find fn FnOnce`). Iterating it is out of scope. */
            let nd = emit_vec_new(); EISVEC = 1; return nd;
        }
        let ev = parse_expr();
        let nd = emit_vec_new();
        let islot = NC; ecopy(islot, v0); NC = NC + 1;
        println!("  loop"); ckreset();
        let mut cnd = 0; if incl == 1 { cnd = ebin("le", islot, ev); } else { cnd = ebin("lt", islot, ev); }
        println!("  brz v{}", cnd); ckreset();
        emit_vec_push(nd, islot);
        println!("  cont"); ckreset(); let one = ecr(1); let ni = ebin("add", islot, one); ecopy(islot, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
        EISVEC = 1; nd
    }
}
/* R-either (P-HIGHER-ORDER-METHOD-FUNCTION-VALUE-ARG): apply a function-VALUE arg (NOT a `|x| body` closure)
   to a single argument `av`, returning the result SSA. POS is at the value's first token; consumes the value.
   - a variant constructor `E::Variant` / bare `Variant` (payload arity 1) -> the cell [disc, av];
   - a path `mod::fn` or a bare fn name `f` -> `call f av`.
   Used by the Option/Result HOF methods (map/map_err/...) when the arg is a fn value like `Either::Left`. */
fn emit_fnval_apply(av: i32) -> i32 {
    unsafe {
        skipws(); rd_ident(); let o = ID_O; let l = ID_L; skipws();
        if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {                      /* `E::Variant` or `path::fn` */
            let en = enum_find(o, l);
            POS = POS + 2; skipws(); rd_ident(); let no = ID_O; let nl = ID_L;
            if en >= 0 {
                let vi = var_find(no, nl);
                if vi >= 0 {
                    let cell = halloc(8); let cd = ec(VDISC[vi as usize]); println!("  store v{} v{}", cell, cd);
                    let c4 = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4), av);
                    return cell;
                }
            }
            print!("  v{}: i32 = call ", NC); emit_span(no, nl); println!(" v{}", av); let r = NC; NC = NC + 1;
            return r;
        }
        let vi = var_find(o, l);
        if vi >= 0 { if fn_is(o, l) == 0 {                                            /* bare variant constructor used as a fn value */
            let cell = halloc(8); let cd = ec(VDISC[vi as usize]); println!("  store v{} v{}", cell, cd);
            let c4 = ecr(4); println!("  store v{} v{}", ebin("add", cell, c4), av);
            return cell;
        } }
        print!("  v{}: i32 = call ", NC); emit_span(o, l); println!(" v{}", av); let r = NC; NC = NC + 1;   /* bare fn name */
        r
    }
}
fn parse_atom() -> i32 {
    unsafe {
        skipws();
        EISSTR = 0; EITYP = -1; EISVEC = 0; EISSTRING = 0; EISCLO = 0; EISARR = 0; EISCHAR = 0; EIS64 = 0; EWASREF = 0; EISBYTES = 0; EISBOOL = 0; EISENUM = 0; EISHM = 0; EIENUMTY = 0 - 1; EIS8 = 0; EMUTBOR = 0; EISDYNVEC = 0; EWASBOXREF = 0; EVELT = 0 - 1; DVELT = 0 - 1; EISU = 0; EOPTREF = 0; EVELSTR = 0; EITY_PIN = 0 - 1; EWASGARG = 0; EWASGARG2 = 0; EISUNIT = 0; ECRETCLO = 0; EISOPT = 0; EISVDQ = 0; EISSET = 0; EISFLT = 0;
        /* STDIN-READ: `[std::][io::]stdin().read_to_string(&mut s)` / `.read_to_end(&mut v)` -- lower to a syscall(0)
           read into the target String/Vec {ptr@0,len@4} descriptor. rzrs has no std stdin; the stdin handle is ignored
           (fd 0). Recognized as a whole idiom here (not via method dispatch, which routes read_to_string to a bare call).
           Keeps the S1 parser portable std Rust that rustc ALSO compiles, so the AST differential's BOOTSTRAP holds. */
        { let svp = POS; let mut ok = 0;
            if rd_ident() {
                if id_is(115, 116, 100, 0, 0, 3) { skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); rd_ident(); } }   /* std:: */
                if id_is(105, 111, 0, 0, 0, 2) { skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); rd_ident(); } }       /* io:: */
                if id_is(115, 116, 100, 105, 110, 5) {   /* stdin */
                    skipws();
                    if POS < LEN && at(POS) == 40 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 41 { POS = POS + 1; ok = 1; } }   /* () */
                }
            }
            if ok == 1 {
                skipws();
                if POS < LEN && at(POS) == 46 { POS = POS + 1; rd_ident();
                    if id_is(114, 101, 97, 100, 95, 14) || id_is(114, 101, 97, 100, 95, 11) {   /* read_to_string / read_to_end */
                        skipws(); eat_ch(40); skipws();
                        if POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); }               /* & */
                        if POS + 2 < LEN && at(POS) == 109 && at(POS + 1) == 117 && at(POS + 2) == 116 { POS = POS + 3; skipws(); }   /* mut */
                        rd_ident(); let bo = ID_O; let bl = ID_L;
                        let d = sym_find(bo, bl);                                                 /* target String/Vec descriptor {ptr@0,len@4} */
                        eat_ch(41);
                        let buf = halloc(1500000);
                        let c0 = ecr(0); let c0b = ecr(0); let cap = ecr(1500000);
                        println!("  v{}: i32 = syscall v{} v{} v{} v{}", NC, c0, c0b, buf, cap); let nread = NC; NC = NC + 1;   /* read(fd=0, buf, 1.5MB) -> nread */
                        let c4 = ecr(4);
                        println!("  store v{} v{}", d, buf);                                      /* descriptor.ptr = buf */
                        println!("  store v{} v{}", ebin("add", d, c4), nread);                   /* descriptor.len = nread */
                        let cell = halloc(8); let c1 = ecr(1);                                     /* return Ok(nread): disc=1 so a following .unwrap() yields nread, no panic */
                        println!("  store v{} v{}", cell, c1);
                        println!("  store v{} v{}", ebin("add", cell, c4), nread);
                        EISOPT = 1; return opt_postfix(cell);
                    }
                }
            }
            POS = svp;
        }
        /* FS-READ: `std::fs::read_to_string(<path>)` -> open(path, O_RDONLY) + read + close, materialize Ok(String).
           NUL-terminates the path IN PLACE (one storeb at ptr[len] -- the path is s1_parser's own fresh arena String,
           so ptr[len] is inside its block), avoiding a copy loop. Portable std Rust that rustc ALSO runs, so the
           S-stage dual-compile BOOTSTRAP (rz==rc) holds. Mirrors STDIN-READ + the P-MOD-FILE-SPLICE open/read. */
        { let svp = POS; let mut ok = 0;
            if rd_ident() {
                if id_is(115, 116, 100, 0, 0, 3) { skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); rd_ident(); } }   /* std:: */
                if id_is(102, 115, 0, 0, 0, 2) { skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); rd_ident();   /* fs:: */
                    if id_is(114, 101, 97, 100, 95, 14) { ok = 1; }   /* read_to_string */
                } }
            }
            if ok == 1 {
                skipws(); eat_ch(40); skipws();
                if POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); }                        /* optional & */
                let pd = parse_expr();                                                            /* path String/&str descriptor {ptr@0,len@4} */
                skipws(); eat_ch(41);
                let c0 = ecr(0); let c4 = ecr(4);
                let pptr = eload(pd);
                let plen = eload(ebin("add", pd, c4));
                println!("  storeb v{} v{}", ebin("add", pptr, plen), c0);                        /* NUL-terminate the path in place */
                let c2 = ecr(2);
                println!("  v{}: i32 = syscall v{} v{} v{} v{}", NC, c2, pptr, c0, c0); let fd = NC; NC = NC + 1;   /* open(path, O_RDONLY, 0) */
                let cap = ecr(1500000); let cbuf = halloc(1500000);
                println!("  v{}: i32 = syscall v{} v{} v{} v{}", NC, c0, fd, cbuf, cap); let nread = NC; NC = NC + 1;   /* read(fd, cbuf, cap) */
                let c3 = ecr(3);
                println!("  v{}: i32 = syscall v{} v{} v{} v{}", NC, c3, fd, c0, c0); NC = NC + 1;   /* close(fd) */
                let sdesc = halloc(8);                                                            /* String {ptr@0=cbuf, len@4=nread} */
                println!("  store v{} v{}", sdesc, cbuf);
                println!("  store v{} v{}", ebin("add", sdesc, c4), nread);
                let cell = halloc(8); let c1 = ecr(1);                                             /* Ok(String): disc@0=1, payload@4=sdesc */
                println!("  store v{} v{}", cell, c1);
                println!("  store v{} v{}", ebin("add", cell, c4), sdesc);
                EISOPT = 1; EOPTRES = 1; EOPTK = 3; return opt_postfix(cell);                     /* Result, String payload -> unwrap restores String flags */
            }
            POS = svp;
        }
        if POS < LEN && at(POS) == 98 && POS + 5 <= LEN && at(POS + 1) == 114 && at(POS + 2) == 101 && at(POS + 3) == 97 && at(POS + 4) == 107 && (POS + 5 >= LEN || is_idc(at(POS + 5)) == false) {   /* LAYER4 P-MATCH-ARM-BREAK: `break` in EXPRESSION position (`_ => break`) -- break/continue are `!`-typed exprs; emit brk here so the match-arm fallback parse_expr handles them (was a bare-ident expr -> `or v-1 v-1` + goto match-end -> the loop never broke -> infinite loop; rustc_lexer eat_decimal_digits `_ => break`). In parse_atom (not emit_match's arm chain, which miscompiles when enlarged). */
            POS = POS + 5; skipws();
            if POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); }
            else if POS < LEN && at(POS) != 44 && at(POS) != 125 && at(POS) != 59 && at(POS) != 41 { let bv = parse_expr(); if NLRES > 0 { lvf_capture(NLRES - 1); ecopy(LRES[(NLRES - 1) as usize], bv); } }
            println!("  brk"); EISUNIT = 1; let z = ecr(0); return z;
        }
        if POS < LEN && at(POS) == 99 && POS + 8 <= LEN && at(POS + 1) == 111 && at(POS + 2) == 110 && at(POS + 3) == 116 && at(POS + 4) == 105 && at(POS + 5) == 110 && at(POS + 6) == 117 && at(POS + 7) == 101 && (POS + 8 >= LEN || is_idc(at(POS + 8)) == false) {   /* LAYER4 P-MATCH-ARM-BREAK: `continue` in expression position -> emit cnt */
            POS = POS + 8; skipws();
            if POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); }
            println!("  cnt"); EISUNIT = 1; let z = ecr(0); return z;
        }
        if POS + 1 < LEN && at(POS) == 39 && is_idst(at(POS + 1)) {                 /* R2ch: `'label: loop {..}` in expression position (value-loop) -- register the label so parse_loop_body propagates the break out (guard vs a `'c'` char literal via the `:`) */
            let lsv = POS; POS = POS + 1; rd_ident(); let lo = ID_O; let ll = ID_L;
            skipws();
            if POS < LEN && at(POS) == 58 && (POS + 1 >= LEN || at(POS + 1) != 58) {
                POS = POS + 1; skipws(); rd_ident();
                if is_kw_loop() {
                    let f = NC; let z = ecr(0); ecopy(f, z); NC = NC + 1;
                    let cf2 = NC; let z3 = ecr(0); ecopy(cf2, z3); NC = NC + 1;
                    LBLO[NLBL as usize] = lo; LBLL[NLBL as usize] = ll; LBLFLAG[NLBL as usize] = f;
                    LBLCFLAG[NLBL as usize] = cf2; LBLLVL[NLBL as usize] = LDEPTH + 1;
                    LBLRES[NLBL as usize] = NLRES; NLBL = NLBL + 1;
                    let rv = emit_loop_val();
                    NLBL = NLBL - 1;
                    return rv;
                }
                POS = lsv;
            } else { POS = lsv; }
        }
        if POS + 1 < LEN && at(POS) == 98 && at(POS + 1) == 34 {                    /* R25: b"..." byte-string literal -> [ptr,len] byte slice */
            POS = POS + 2; let so = POS;
            while POS < LEN && at(POS) != 34 { if at(POS) == 92 { POS = POS + 1; } POS = POS + 1; }
            let sl = POS - so; POS = POS + 1;
            let d = emit_str_literal(so, sl); EISSTR = 0; EISBYTES = 1;
            skipws();
            if POS < LEN && (at(POS) == 91 || at(POS) == 46) { return dispatch_bytes(d); }   /* R2: b"..."[i] / b"..".method() -- route the postfix through dispatch_bytes (was returned raw -> the stray [i] derailed the enclosing parse) */
            return d;
        }
        if POS < LEN && at(POS) == 124 { return parse_closure(); }                 /* |params| body */
        { let ps = POS; if rd_ident() && id_is(109, 111, 118, 101, 0, 4) { skipws(); if POS < LEN && at(POS) == 124 { let msave = POS; let r = parse_closure(); let cend = POS; POS = msave; mark_move_captures(); POS = cend; return r; } } POS = ps; }   /* R1: `move |params| body`; MOVEBORROW m3: a `move ||` moves its captured non-Copy String/Vec locals (scan the body AFTER parse_closure so the capture-read itself doesn't trip the use-check) */
        if POS < LEN && at(POS) == 34 {                                            /* "string literal" */
            POS = POS + 1; let so = POS;
            while POS < LEN && at(POS) != 34 { if at(POS) == 92 { POS = POS + 1; } POS = POS + 1; }
            let sl = POS - so; POS = POS + 1;
            return dispatch_string(emit_str_literal(so, sl));   /* R2: route `"lit".method()` through dispatch_string (was returned raw -> literal-receiver methods silently no-op'd) */
        }
        if eat_ch(40) {
            skipws();
            if POS < LEN && at(POS) == 41 { POS = POS + 1; return ec(0); }             /* R24: () unit value -> 0 (e.g. Ok(())) */
            let v0 = parse_expr(); let v0ty = EITYP; let k0 = cur_kind(); skipws();
            if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 {               /* exotica: (a..b) range expr -> materialize a Vec (enables (a..b).sum()/.step_by()/..) */
                POS = POS + 2; let mut incl = 0; if POS < LEN && at(POS) == 61 { POS = POS + 1; incl = 1; }
                let ev = parse_expr(); eat_ch(41);
                let nd = emit_vec_new();
                let islot = NC; ecopy(islot, v0); NC = NC + 1;
                println!("  loop"); ckreset();
                let mut cnd = 0; if incl == 1 { cnd = ebin("le", islot, ev); } else { cnd = ebin("lt", islot, ev); }
                println!("  brz v{}", cnd); ckreset();
                emit_vec_push(nd, islot);
                println!("  cont"); ckreset(); let one = ecr(1); let ni = ebin("add", islot, one); ecopy(islot, ni);
                println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                skipws(); if POS < LEN && (at(POS) == 46 || at(POS) == 91) { return dispatch_vec(nd); }   /* (a..b).method()/[i] */
                EISVEC = 1; return nd;
            }
            if POS < LEN && at(POS) == 44 {                                        /* R23: (a, b, ..) tuple -> heap cell [e0,e1,..] */
                let cell = halloc(64);                                             /* PAT: store each element IMMEDIATELY (was: collect into global TUPV, which a NESTED tuple parse clobbered -> ((1,2),(3,4)) built outer[0]=3) */
                let z0 = ecr(0); println!("  store v{} v{}", ebin("add", cell, z0), v0);
                let tyid = NTUPTY; let base = TFKCUR;                              /* tuple-field-kind: register a type recording each field's kind */
                TFK[TFKCUR as usize] = k0; TFKCUR = TFKCUR + 1;
                let mut n = 1;
                while POS < LEN && at(POS) == 44 {
                    POS = POS + 1; skipws();
                    if POS < LEN && at(POS) == 41 { break; }                       /* trailing comma */
                    let ev = parse_expr(); let ek = cur_kind(); println!("  store v{} v{}", ebin("add", cell, ec(n * 4)), ev); TFK[TFKCUR as usize] = ek; TFKCUR = TFKCUR + 1; n = n + 1; skipws();
                }
                eat_ch(41);
                TUPTY_BASE[tyid as usize] = base; TUPTY_N[tyid as usize] = n; NTUPTY = NTUPTY + 1;
                ETUPN = n; ETUPTY = tyid; EITY_PIN = ty_intern(11, 0, tyid);   /* R2cg: mark a tuple cell + arity for {:?} Debug; tuple-field-kind: hand the type id to the binding */
                return cell;
            }
            eat_ch(41);
            if EISCLO == 1 { skipws(); if POS < LEN && at(POS) == 40 { let r = emit_closure_call(v0); return opt_postfix(r); } }   /* R10: `(s.f)(args)` / `(closure_expr)(args)` -- a parenthesized closure value called through */
            skipws();
            if v0ty >= 0 && POS < LEN && at(POS) == 46 && (POS + 1 >= LEN || at(POS + 1) != 46) { EITYP = v0ty; return dispatch_struct(v0, v0ty); }   /* trait-core: `(a+b).field`/`.method()` where the paren result is a struct (e.g. an operator-overload Output) -- route the postfix through dispatch_struct (opt_postfix does not handle a struct .field, so the raw pointer leaked) */
            return opt_postfix(v0);   /* R2: `(expr).method()` -- route a parenthesized-expr receiver through opt_postfix too */
        }
        if eat_ch(33) { let v = parse_atom(); let vb = EISBOOL; if vb == 1 { let z = ecr(0); let r = ebin("eq", v, z); EISBOOL = 1; return r; } let r = ebin("sub", ebin("sub", ec(0), v), ec(1)); EISBOOL = 0; return r; }   /* !x: bool -> x==0 ; int -> bitwise NOT = 0-x-1 */
        if eat_ch(45) { let v = parse_atom(); let vty = EITYP; let v64 = EIS64;
            if vty >= 0 { let r = op_method1(vty, v, 110, 101, 103); if r >= 0 { return r; } }   /* R2cl: -x on a struct with impl Neg -> Type_neg(x) */
            let z = ecr(0); EITYP = -1; return emit_binop("sub", z, v, v64, v64); }  /* -x numeric; value-width: 64-bit sub + EIS64 when operand is i64 (else identical to ebin) */
        if eat_ch(38) { return parse_addr(); }                                            /* &place */
        if eat_ch(42) { let p = parse_atom(); if EWASBOXREF == 1 { EWASBOXREF = 0; EWASREF = 0; return p; } if EWASREF == 1 { return p; } if EISSTR == 1 || EISSTRING == 1 { return p; } return eload(p); }   /* *p deref read (no-op on a for-ref loop var; TI3e-m: also a no-op on a &str/String descriptor -- `*w` on a &str from `for w in strs.iter()` must not eload the buf-ptr). D004 IC-3: a by-ref match/iflet payload (EWASBOXREF) skips exactly ONE `*` (clears both flags) so a further `*` does a real eload -- `**e` on Option<Box<T>>/Option<&T> = one ergonomic skip + one Box/ref load. */
        if POS < LEN && is_dig(at(POS)) {
            if at(POS) == 48 && POS + 1 < LEN && at(POS + 1) == 120 {                   /* E4: 0x hex literal */
                POS = POS + 2; let mut hv: i64 = 0;
                while POS < LEN && (chexval(at(POS)) >= 0 || at(POS) == 95) { if at(POS) != 95 { hv = hv * 16 + (chexval(at(POS)) as i64); } POS = POS + 1; }   /* `_` digit separators skipped */
                while POS < LEN && is_idc(at(POS)) { POS = POS + 1; }                   /* type suffix (u8/u32/..) */
                if hv >= 2147483648 { return opt_postfix(emit_i64_lit(hv)); }
                return opt_postfix(ec64raw(hv));   /* R2: numeric-literal receiver -> route through opt_postfix so `.method()`/`.N` dispatch */
            }
            if at(POS) == 48 && POS + 1 < LEN && at(POS + 1) == 98 {                     /* R2: 0b binary literal */
                POS = POS + 2; let mut bv: i64 = 0;
                while POS < LEN && ((at(POS) >= 48 && at(POS) <= 49) || at(POS) == 95) { if at(POS) != 95 { bv = bv * 2 + ((at(POS) - 48) as i64); } POS = POS + 1; }
                while POS < LEN && is_idc(at(POS)) { POS = POS + 1; }
                if bv >= 2147483648 { return opt_postfix(emit_i64_lit(bv)); }
                return opt_postfix(ec64raw(bv));
            }
            if at(POS) == 48 && POS + 1 < LEN && at(POS + 1) == 111 {                    /* R2: 0o octal literal */
                POS = POS + 2; let mut ov: i64 = 0;
                while POS < LEN && ((at(POS) >= 48 && at(POS) <= 55) || at(POS) == 95) { if at(POS) != 95 { ov = ov * 8 + ((at(POS) - 48) as i64); } POS = POS + 1; }
                while POS < LEN && is_idc(at(POS)) { POS = POS + 1; }
                if ov >= 2147483648 { return opt_postfix(emit_i64_lit(ov)); }
                return opt_postfix(ec64raw(ov));
            }
            let mut n: i64 = 0;
            while POS < LEN && (is_dig(at(POS)) || at(POS) == 95) { if at(POS) != 95 { n = n * 10 + ((at(POS) - 48) as i64); } POS = POS + 1; }   /* `_` digit separators (5_000_000) skipped */
            if POS < LEN && at(POS) == 46 && POS + 1 < LEN && is_dig(at(POS + 1)) {     /* SOFTFLOAT: float literal `D.D` -> fixed-point scaled i64 (value * 1e9); `<floatlit> as int` still truncates to the integer part */
                POS = POS + 1;                                                          /* consume `.` */
                let mut f: i64 = 0; let mut fc = 0;
                while POS < LEN && (is_dig(at(POS)) || at(POS) == 95) { if at(POS) != 95 { f = f * 10 + ((at(POS) - 48) as i64); fc = fc + 1; } POS = POS + 1; }   /* fractional digits */
                while POS < LEN && is_idc(at(POS)) { POS = POS + 1; }                   /* f64/f32/_f64 suffix */
                let asave = POS; skipws();
                let has_as = POS + 1 < LEN && at(POS) == 97 && at(POS + 1) == 115 && (POS + 2 >= LEN || !is_idc(at(POS + 2)));
                POS = asave;
                if has_as { return cast_floatlit(n); }                                  /* `<floatlit> as int` -> integer part (truncate toward zero) */
                let mut fs = f; let mut k = fc; while k < 9 { fs = fs * 10; k = k + 1; } while k > 9 { fs = fs / 10; k = k - 1; }   /* scale the fraction to 9 places */
                let scaled = n * 1000000000 + fs;                                        /* SOFTFLOAT: fixed-point value = round(V * 1e9) */
                let r = emit_i64_lit(scaled); EIS64 = 1; EISFLT = 1; return r;
            }
            let sfo = POS; while POS < LEN && is_idc(at(POS)) { POS = POS + 1; }         /* E4: integer type suffix (5u32/33u8) */
            let sfl = POS - sfo;
            if sfl == 2 && (at(sfo) == 117 || at(sfo) == 105) && at(sfo + 1) == 56 { let r = ec64raw(n); EIS8 = 1; if at(sfo) == 117 { EISU = 1; } return opt_postfix(r); }   /* value-width: `NNu8`/`NNi8` suffix -> tag u8/i8 (wrapping/saturating/... mask 0xFF) */
            if sfl == 3 && at(sfo) == 105 && at(sfo + 1) == 54 && at(sfo + 2) == 52 { if n >= 2147483648 || n < 0 { return opt_postfix(emit_i64_lit(n)); } let r = ec64raw(n); EIS64 = 1; return opt_postfix(r); }   /* `NNi64` suffix */
            if sfl == 3 && at(sfo) == 117 && at(sfo + 1) == 54 && at(sfo + 2) == 52 { let r = emit_i64_lit(n); EISU = 1; return opt_postfix(r); }   /* value-width: `NNu64` suffix -> 64-bit unsigned (Display unsigned) */
            if n < 0 { let r = emit_i64_lit(n); EISU = 1; return opt_postfix(r); }   /* value-width: decimal literal overflowed i64 (> i64::MAX) -> a u64 value; emit its two's-complement bits + unsigned Display (e.g. 18446744073709551615) */
            if n >= 2147483648 { return opt_postfix(emit_i64_lit(n)); }
            return opt_postfix(ec64raw(n));
        }
        if POS + 1 < LEN && at(POS) == 98 && at(POS + 1) == 39 {                          /* R2: byte-char literal b'c' -- a u8 numeric (NOT a char) */
            POS = POS + 2; let mut cc = at(POS);
            if cc == 92 { POS = POS + 1; let e = at(POS); if e == 110 { cc = 10; } else if e == 116 { cc = 9; } else if e == 48 { cc = 0; } else { cc = e; } }
            POS = POS + 1; if POS < LEN && at(POS) == 39 { POS = POS + 1; }
            return ec(cc);
        }
        if POS < LEN && at(POS) == 39 {                                                   /* char literal 'c' */
            POS = POS + 1; let mut cc = at(POS);
            if cc == 92 { POS = POS + 1; let e = at(POS); if e == 110 { cc = 10; } else if e == 116 { cc = 9; } else if e == 114 { cc = 13; } else if e == 48 { cc = 0; } else if e == 117 && POS + 1 < LEN && at(POS + 1) == 123 { POS = POS + 2; cc = 0; while POS < LEN && at(POS) != 125 { let h = at(POS); let hd = if h >= 48 && h <= 57 { h - 48 } else if h >= 97 && h <= 102 { h - 97 + 10 } else if h >= 65 && h <= 70 { h - 65 + 10 } else { 0 }; cc = cc * 16 + hd; POS = POS + 1; } } else { cc = e; } }   /* LAYER6 P-CHAR-UNICODE-ESCAPE (expression): the char-literal EXPRESSION emitter only decoded `\n` -- extend to \t \r \0 and `'\u{XXXX}'` hex (mirrors read_pat_scalar/ceval_atom). Was: `'\u{0009}'` -> cc='u' + POS stuck at `{XXXX}'` -> crash/garbage; `'\t'`/`'\r'`/`'\0'` -> the escaped char, wrong. */
            POS = POS + 1; if POS < LEN && at(POS) == 39 { POS = POS + 1; }
            let r = ec(cc); EISCHAR = 1; return r;
        }
        if POS < LEN && at(POS) == 91 {                                            /* R11: [e; n] or [e0, e1, ...] local array literal */
            POS = POS + 1; skipws();
            if POS < LEN && at(POS) == 93 { POS = POS + 1; EISARR = 1; EARRLEN = 0; EARRELT = 0 - 1; EARRVTY = 0 - 1; return halloc(4); }   /* C-S0I64-LOCALARR: empty array literal `[]` (e.g. `let b: [i64;0] = []`) -- no element to parse; return a minimal buffer. Was reachable only after the [T;N] annotation began being consumed (else `[]` never parsed). */
            let epos = POS;                                                        /* R2ag: save element POS for re-parse */
            let e0 = parse_expr(); let e0arr = EISARR; let e0elt = EITYP; let e0vty = EITY; skipws();
            if POS < LEN && at(POS) == 59 {                                        /* [e; n] repeat */
                POS = POS + 1; skipws();
                let mut n = 0;
                if POS < LEN && is_dig(at(POS)) { while POS < LEN && is_dig(at(POS)) { n = n * 10 + (at(POS) - 48); POS = POS + 1; } }
                else if POS < LEN && is_idst(at(POS)) { rd_ident(); let ci = cfind(ID_O, ID_L); if ci >= 0 { n = CVAL[ci as usize]; } }   /* CONST-ORDER: `[e; N]` repeat count from a const */
                eat_ch(93);
                let resz = if EANNARR64 == 1 { 8 } else { 4 };
                let mut sz = n * resz; if sz < resz { sz = resz; }
                let buf = halloc(sz);
                let mut i = 0;
                while i < n {
                    let off = ec(i * resz); let ad = ebin("add", buf, off);
                    if e0arr == 1 {                                               /* R2ag: aggregate element -> re-eval per slot for a distinct inner buffer */
                        let sv = POS; POS = epos; let ev = parse_expr(); POS = sv;
                        println!("  store v{} v{}", ad, ev);
                    } else { if EANNARR64 == 1 { println!("  store8 v{} v{}", ad, e0); } else { println!("  store v{} v{}", ad, e0); } }
                    i = i + 1;
                }
                EISARR = 1; EARRLEN = n; EARRELT = e0elt; EARRVTY = e0vty; return buf;
            }
            let buf = halloc(4096);                                                /* [e0, e1, ...] list (fixed cap) */
            let lesz = if EANNARR64 == 1 { 8 } else { 4 };
            let z0 = ecr(0); let ad0 = ebin("add", buf, z0); if EANNARR64 == 1 { println!("  store8 v{} v{}", ad0, e0); } else { println!("  store v{} v{}", ad0, e0); }
            let mut i = 1;
            loop {
                skipws();
                if POS < LEN && at(POS) == 44 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 93 { break; }
                    let e = parse_expr(); let off = ec(i * lesz); let ad = ebin("add", buf, off); if EANNARR64 == 1 { println!("  store8 v{} v{}", ad, e); } else { println!("  store v{} v{}", ad, e); } i = i + 1;
                } else { break; }
            }
            eat_ch(93);
            EISARR = 1; EARRLEN = i; EARRELT = e0elt; EARRVTY = e0vty; return buf;
        }
        if POS < LEN && at(POS) == 36 {                                             /* A: `$param` inside a macro body -> splice-parse the bound argument */
            POS = POS + 1; rd_ident();
            let mut k = 0;
            while k < CURNP {
                if span_eq_at(CURPO[k as usize], CURPL[k as usize], ID_O, ID_L) {
                    let save = POS; POS = CURAO[k as usize]; let v = parse_expr(); POS = save; return v;
                }
                k = k + 1;
            }
            return ec(0);
        }
        rd_ident();
        let o = ID_O; let l = ID_L;
        if POS + 2 < LEN && at(POS) == 58 && at(POS + 1) == 58 && at(POS + 2) == 60 {   /* R2ah: expression-position turbofish PATH::<TYPES> -- erase the ::<...> (untyped model), so Ok::<i32,i32>(8)->Ok(8), Vec::<i32>::new()->Vec::new() */
            POS = POS + 2; let mut d = 1; POS = POS + 1; skipws();
            let tso = POS; while POS < LEN && is_idc(at(POS)) { POS = POS + 1; } let tsl = POS - tso;   /* TRAIT R3: capture the first type-arg (`m::<A>()` -> A) as the monomorph key */
            let tty = ty_find(tso, tsl); if tty >= 0 { TURBO_STYP = tty; }
            while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; }
        }
        if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {                        /* value-width: `Type::MAX` / `Type::MIN` primitive assoc const (route through opt_postfix so `i32::MAX.wrapping_add(1)` chains) */
            let tk = prim_int_ty(o, l);
            if tk > 0 {
                let sp = POS; POS = POS + 2; skipws(); let gotc = rd_ident();
                let ismax = gotc && ID_L == 3 && at(ID_O) == 77 && at(ID_O + 1) == 65 && at(ID_O + 2) == 88;
                let ismin = gotc && ID_L == 3 && at(ID_O) == 77 && at(ID_O + 1) == 73 && at(ID_O + 2) == 78;
                if ismax == true {
                    if tk == 1 { return opt_postfix(ec(127)); }
                    if tk == 2 { return opt_postfix(ec(32767)); }
                    if tk == 3 { return opt_postfix(ec(2147483647)); }
                    if tk == 5 { return opt_postfix(ec(255)); }
                    if tk == 6 { return opt_postfix(ec(65535)); }
                    if tk == 7 { return opt_postfix(emit_i64_lit(4294967295)); }                    /* u32::MAX -> positive i64 (Displays unsigned-correct) */
                    if tk == 4 || tk == 10 { return opt_postfix(emit_i64_lit(9223372036854775807)); }  /* i64/isize::MAX */
                    if tk == 8 || tk == 9 { let c = ec64raw(0 - 1); let r = emit_convs(c); EIS64 = 1; EISU = 1; return opt_postfix(r); }   /* u64/usize::MAX = all-ones 64-bit (sign-extend -1); unsigned Display */
                }
                if ismin == true {
                    if tk == 1 { return opt_postfix(ec(0 - 128)); }
                    if tk == 2 { return opt_postfix(ec(0 - 32768)); }
                    if tk == 3 { let m = ebin("sub", ec(0), ec(2147483647)); return opt_postfix(ebin("sub", m, ec(1))); }   /* i32::MIN = -2147483648 (avoid an out-of-i32 literal) */
                    if tk == 5 || tk == 6 || tk == 7 || tk == 8 || tk == 9 { return opt_postfix(ec(0)); }   /* unsigned MIN = 0 */
                }
                POS = sp;                                                              /* not MAX/MIN (e.g. i32::from) -> restore, fall through */
            }
            let rty = if GSUB_ACT == 1 && span_eq_at(o, l, GSUB_TPO, GSUB_TPL) { GSUB_STYP } else { ty_find(o, l) };   /* TRAIT R3: T::CONST (T=GSUB concrete type) / Type::CONST via the trait-const registry */
            if rty >= 0 { let sp = POS; POS = POS + 2; skipws(); rd_ident(); let ci = tconst_find(rty, ID_O, ID_L); if ci >= 0 { EIS64 = 1; return opt_postfix(emit_i64_lit(TCVAL[ci as usize])); } POS = sp; }
        }
        if (l == 3 && at(o) == 115 && at(o + 1) == 116 && at(o + 2) == 100 || l == 3 && at(o) == 109 && at(o + 1) == 101 && at(o + 2) == 109) && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {
            let sp = POS; let mut ok = 0;                                          /* MEM: std::mem::FN / mem::FN */
            if at(o) == 109 { POS = POS + 2; ok = 1; }                             /* mem:: */
            else { POS = POS + 2; skipws(); rd_ident(); if ID_L == 3 && at(ID_O) == 109 && at(ID_O + 1) == 101 && at(ID_O + 2) == 109 && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; ok = 1; } }   /* std::mem:: */
            if ok == 1 {
                skipws(); rd_ident(); let fo = ID_O; let fl = ID_L; skipws(); eat_ch(40);
                if id_is(115, 119, 97, 112, 0, 4) {                                /* swap(&mut a, &mut b) -- a/b a scalar slot or a p.field address */
                    let sa = mref_slot(); let aa = MREFADDR; skipws(); eat_ch(44); let sb = mref_slot(); let ba = MREFADDR; eat_ch(41);
                    let va = NC; if aa == 1 { ecopy(va, eload(sa)); } else { ecopy(va, sa); } NC = NC + 1;
                    let vb = NC; if ba == 1 { ecopy(vb, eload(sb)); } else { ecopy(vb, sb); } NC = NC + 1;
                    if aa == 1 { println!("  store v{} v{}", sa, vb); } else { ecopy(sa, vb); }
                    if ba == 1 { println!("  store v{} v{}", sb, va); } else { ecopy(sb, va); }
                    EISUNIT = 1; return ec(0);
                }
                if id_is(114, 101, 112, 108, 97, 7) {                              /* replace(&mut a, v) -> old */
                    let sa = mref_slot(); let sty = MREFSTY; skipws(); eat_ch(44);
                    let old = NC; ecopy(old, sa); NC = NC + 1;
                    let v = parse_expr(); eat_ch(41); ecopy(sa, v);
                    if sty >= 0 { EITYP = sty; } return old;
                }
                if id_is(116, 97, 107, 101, 0, 4) {                                /* take(&mut a) -> old; a := default (0 / empty Vec) */
                    let sa = mref_slot(); let k = MREFK; eat_ch(41);
                    let old = NC; ecopy(old, sa); NC = NC + 1;
                    if k == 2 { let nv = emit_vec_new(); ecopy(sa, nv); EISVEC = 1; } else { let z = ecr(0); ecopy(sa, z); }
                    return old;
                }
            }
            POS = sp;                                                              /* not a handled mem fn -> restore, fall through */
        }
        if (l == 3 && at(o) == 99 && at(o + 1) == 109 && at(o + 2) == 112 || l == 3 && at(o) == 115 && at(o + 1) == 116 && at(o + 2) == 100) && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {
            let sp = POS; let mut ok = 0;                                          /* W-C (authoring-unlock proof): std::cmp::max/min / cmp::max/min -- the result width is decided by READING the arg's interned Ty (EITY), not by threading a flag */
            if at(o) == 99 { POS = POS + 2; ok = 1; }                              /* cmp:: */
            else { POS = POS + 2; skipws(); rd_ident(); if ID_L == 3 && at(ID_O) == 99 && at(ID_O + 1) == 109 && at(ID_O + 2) == 112 && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; ok = 1; } }   /* std::cmp:: */
            if ok == 1 {
                skipws(); rd_ident(); let fo = ID_O; let fl = ID_L;
                let ismax = fl == 3 && at(fo) == 109 && at(fo + 1) == 97 && at(fo + 2) == 120;   /* max */
                let ismin = fl == 3 && at(fo) == 109 && at(fo + 1) == 105 && at(fo + 2) == 110;  /* min */
                if ismax == true || ismin == true {
                    skipws(); eat_ch(40); let a = parse_expr(); let aty = EITY;     /* AUTHOR VIA Ty: read the first arg's interned Ty (not EIS64) */
                    let is64 = if TYTAG[aty as usize] == 2 { 1 } else { 0 };        /* Ty-dispatch: tag 2 == i64 -> 64-bit compare/copy */
                    skipws(); eat_ch(44); let b = parse_expr(); skipws(); eat_ch(41);
                    let cmp = if ismax == true { emit_binop("gt", a, b, is64, 0) } else { emit_binop("lt", a, b, is64, 0) };
                    let res = NC; NC = NC + 1; if is64 == 1 { ecopy64(res, b); } else { ecopy(res, b); }
                    println!("  brz v{}", cmp); ckreset(); if is64 == 1 { ecopy64(res, a); } else { ecopy(res, a); } println!("  pool"); ckreset();
                    EIS64 = is64; EITY_PIN = aty; return opt_postfix(res);          /* result carries the arg's Ty (pinned), not a fresh flag */
                }
            }
            POS = sp;                                                              /* not a handled cmp fn -> restore, fall through */
        }
        if POS < LEN && at(POS) == 33 { let mi = mac_find(o, l); if mi >= 0 { return expand_macro(mi); } }   /* A: user macro NAME!(args) */
        if is_kw_true() { EISBOOL = 1; return ec(1); }
        if is_kw_false() { EISBOOL = 1; return ec(0); }
        if id_is(102, 111, 114, 109, 97, 6) && POS < LEN && at(POS) == 33 { return emit_format(); }   /* S2: format!(..) -> String */
        if id_is(109, 97, 116, 99, 104, 7) && POS < LEN && at(POS) == 33 {         /* R24: matches!(scrut, p | p | ..) -> OR of equality tests */
            POS = POS + 1; eat_ch(40);
            let scrut = parse_expr(); eat_ch(44);
            let mut sum = ecr(0);
            loop {
                skipws();
                if POS >= LEN { break; }
                if at(POS) == 41 { break; }
                let mut this = 0;
                if at(POS) == 95 { POS = POS + 1; this = ec(1); }                  /* _ wildcard -> always */
                else if is_dig(at(POS)) || (at(POS) == 45 && POS + 1 < LEN && is_dig(at(POS + 1))) || at(POS) == 39 || (at(POS) == 98 && POS + 1 < LEN && at(POS + 1) == 39) {   /* P-MATCH-PATTERN: int / char `'x'` / byte `b'x'` literal or `lo..[=]hi` range */
                    if at(POS) == 98 && POS + 1 < LEN && at(POS + 1) == 39 { POS = POS + 1; }   /* skip the `b` of a byte literal; read_pat_scalar reads the `'x'` */
                    let lo = read_pat_scalar(); skipws();
                    if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 { POS = POS + 2; if POS < LEN && at(POS) == 61 { POS = POS + 1; } skipws(); if POS < LEN && at(POS) == 98 && POS + 1 < LEN && at(POS + 1) == 39 { POS = POS + 1; } let hi = read_pat_scalar(); this = ebin("mul", ebin("ge", scrut, ec(lo)), ebin("le", scrut, ec(hi))); }
                    else { this = ebin("eq", scrut, ec(lo)); }
                } else {
                    rd_ident(); let bo = ID_O; let bl = ID_L; skipws();
                    let mut qual = 0;
                    if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; rd_ident(); qual = 1; }
                    let vi = var_find(ID_O, ID_L); let od = opt_disc(ID_O, ID_L);
                    if qual == 1 || vi >= 0 || od >= 0 {                            /* E::V or bare variant -> disc compare (enum heap value) */
                        let mut disc = 0; if vi >= 0 { disc = VDISC[vi as usize]; } else { disc = od; }
                        skipws(); if POS < LEN && at(POS) == 40 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } POS = POS + 1; } }
                        let dv = eload(scrut); this = ebin("eq", dv, ec(disc));
                    } else {                                                       /* P-MATCH-PATTERN: binding `n` -> bind n=scrut, always matches structurally (a guard, if any, is the real test) */
                        let slot = NC; ecopy(slot, scrut); NC = NC + 1; sym_add(bo, bl, slot);
                        this = ec(1);
                    }
                }
                sum = ebin("add", sum, this);
                skipws();
                if POS < LEN && at(POS) == 124 && (POS + 1 >= LEN || at(POS + 1) != 124) { POS = POS + 1; } else { break; }
            }
            let z = ecr(0); let mut result = ebin("ne", sum, z);
            skipws();
            let gs = POS; let ggot = rd_ident();
            if ggot && id_is(105, 102, 0, 0, 0, 2) { let g = parse_expr(); let z2 = ecr(0); result = ebin("mul", result, ebin("ne", g, z2)); }   /* P-MATCH-PATTERN: `matches!(x, n if GUARD)` -- AND the guard */
            else { POS = gs; }
            eat_ch(41);
            EISBOOL = 1;   /* P-MATCH-PATTERN: matches! yields a bool (was printed as 0/1) */
            return result;
        }
        if is_kw_match() { return parse_match(); }
        if is_kw_if() { return parse_if_expr(); }
        if is_kw_loop() { return emit_loop_val(); }                                /* R1: `loop { break v; }` as a value */
        if cfind(o, l) >= 0 { let ci = cfind(o, l); return ec(CVAL[ci as usize]); }  /* E1: folded integer const -> literal */
        if l == 4 && at(o) == 83 && at(o + 1) == 101 && at(o + 2) == 108 && at(o + 3) == 102 && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {   /* R2bz: `Self::CONST` in a (default) method body -> the impl's associated const (registered by name in prepass_consts) */
            let sp = POS; POS = POS + 2; skipws(); rd_ident(); let ci = cfind(ID_O, ID_L);
            if ci >= 0 { skipws(); if POS >= LEN || at(POS) != 40 { return ec(CVAL[ci as usize]); } }
            POS = sp;
        }
        if is_std(o, l) {                                                          /* std::process::exit(e) -> exit syscall */
            eat_ch(58); eat_ch(58); rd_ident();
            eat_ch(58); eat_ch(58); rd_ident();
            if id_is(102, 114, 111, 109, 95, 7) {                                  /* LAYER4: std::iter::from_fn(closure) -> materialize the closure-driven iterator into a Vec */
                skipws(); eat_ch(40); let fv = parse_expr(); eat_ch(41);
                return emit_from_fn_iter(fv);
            }
            skipws(); eat_ch(40); let e = parse_expr(); eat_ch(41);
            emit_flush_call(); let c60 = ecr(60);
            println!("  v{}: i32 = syscall v{} v{}", NC, c60, e); let r = NC; NC = NC + 1;
            return r;
        }
        let mut nameclash_struct = 0;   /* LAYER5 P-PRESULT-NAMECLASH: a USER `struct Err {..}` coexists with the prelude `Err(..)` variant in different namespaces (rustc name-res). rustc disambiguates by FORM: braces `{` -> the struct literal, parens `(` -> the variant ctor. So defer to the type path ONLY when a user struct of this name exists AND the next token is `{` (else `Err { pos: 7 }` hit the ctor, eat_ch(40) missed the `{`, parse-drift -> segfault; while `Err(n)` must still reach the ctor). */
        if opt_disc(o, l) >= 0 && ty_find(o, l) >= 0 { let sv = POS; skipws(); if POS < LEN && at(POS) == 123 { nameclash_struct = 1; } POS = sv; }
        if opt_disc(o, l) >= 0 && nameclash_struct == 0 {                                                  /* R5: bare Some/None/Ok/Err constructor */
            let disc = opt_disc(o, l); let has = opt_has(o, l);
            let optres2 = if nm_ok(o, l) || nm_err(o, l) { 1 } else { 0 };
            let cell = halloc(8);
            let cd = ec(disc); println!("  store v{} v{}", cell, cd);
            EOPTK = 0; EOPTVELT = 0 - 1;
            if has {
                skipws(); eat_ch(40); let arg = parse_expr(); eat_ch(41);
                if EISVEC == 1 { EOPTK = 2; EOPTVELT = EVELT; } else if EISSTRING == 1 { EOPTK = 3; } else if EISSTR == 1 { EOPTK = 1; }   /* container-kind K1: remember a container payload */
                EISVEC = 0; EISSTR = 0; EISSTRING = 0; EISARR = 0;   /* container-kind K1: the Option cell itself is scalar-kind */
                let c4 = ecr(4); let ad = ebin("add", cell, c4); println!("  store v{} v{}", ad, arg);
            }
            EISOPT = 1; EOPTRES = optres2;   /* container-kind (a): set AFTER payload parse (parse_expr resets EISOPT) */
            return opt_postfix(cell);
        }
        skipws();
        let en = enum_find(o, l);
        if en >= 0 && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {        /* E::Variant [ (arg) ] constructor */
            POS = POS + 2; skipws(); rd_ident();
            let mut vi = var_find_in(en, ID_O, ID_L); if vi < 0 { vi = var_find(ID_O, ID_L); }   /* LAYER6 P-QUALIFIED-VARIANT-RESOLUTION (ctor arm): `E::V` construction resolves V WITHIN enum E, not by the global first-name-match -- else `Expr::Num(5)` built a cell with `Tok::Num`'s disc (an earlier enum sharing the name) -> a wrong discriminant -> the value read back as the wrong variant. Mirrors the match-arm var_find_in fix (L6-13c). */
            let disc = VDISC[vi as usize]; let has = VHAS[vi as usize]; let nf = VNF[vi as usize];
            let mut sz = 4; { let mut fw = 0; while fw < nf { if VFI64[(vi * 8 + fw) as usize] == 1 { sz = sz + 8; } else { sz = sz + 4; } fw = fw + 1; } } if sz < 8 { sz = 8; }   /* P-ENUM-I64-PAYLOAD-WIDTH: an i64/isize payload field takes an 8-byte cell slot (was a fixed 4 -> the high 32 bits were dropped) */
            let cell = halloc(sz);
            let cd = ec(disc); println!("  store v{} v{}", cell, cd);
            let mut pgak = 0; let mut pgat = 0 - 1; let mut phas = 0;   /* std-hoist GAP1: capture the first payload's concrete type-arg kind */
            if has == 1 {
                skipws();
                if POS < LEN && at(POS) == 123 {                                    /* R1: struct-variant `V{ f: e, .. }` -- store positionally in decl order */
                    POS = POS + 1;
                    let mut fi = 0; let mut po = 4;
                    loop {
                        skipws(); if POS < LEN && at(POS) == 125 { break; }
                        rd_ident(); skipws(); eat_ch(58);
                        let mut arg = parse_expr(); skipws(); if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 { arg = emit_range_vec(arg); }   /* P-RANGE-VALUE-IN-VARIANT-ARG */
                        if fi == 0 { if EISSTR == 1 { pgak = 1; phas = 1; } else if EISSTRING == 1 { pgak = 3; phas = 1; } else if EISVEC == 1 { pgak = 2; phas = 1; } else if EITYP >= 0 { pgak = 0; pgat = EITYP; phas = 1; } }
                        let off = ec(vpay_off(vi, fi)); let ad = ebin("add", cell, off); if vpay_i64(vi, fi) == 1 { println!("  store8 v{} v{}", ad, arg); } else { println!("  store v{} v{}", ad, arg); }   /* P-ENUM-I64-PAYLOAD-WIDTH: 8-byte store for an i64 field, running offset */
                        fi = fi + 1;
                        skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; }
                    }
                    eat_ch(125);
                } else {
                    eat_ch(40);
                    let mut fi = 0; let mut po = 4;
                    loop {
                        skipws(); if POS < LEN && at(POS) == 41 { break; }
                        let mut arg = parse_expr(); skipws(); if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 { arg = emit_range_vec(arg); }   /* P-RANGE-VALUE-IN-VARIANT-ARG */
                        if fi == 0 { if EISSTR == 1 { pgak = 1; phas = 1; } else if EISSTRING == 1 { pgak = 3; phas = 1; } else if EISVEC == 1 { pgak = 2; phas = 1; } else if EITYP >= 0 { pgak = 0; pgat = EITYP; phas = 1; } }
                        let off = ec(vpay_off(vi, fi)); let ad = ebin("add", cell, off); if vpay_i64(vi, fi) == 1 { println!("  store8 v{} v{}", ad, arg); } else { println!("  store v{} v{}", ad, arg); }   /* P-ENUM-I64-PAYLOAD-WIDTH: 8-byte store for an i64 field, running offset */
                        fi = fi + 1;
                        skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; }
                    }
                    eat_ch(41);
                }
            }
            EISENUM = 1; EIENUMTY = en; if phas == 1 { EGAK = pgak; EGAT = pgat; EWASGARG = 1; }   /* std-hoist GAP1: hand the payload's type-arg kind to the let-binding */
            return cell;
        }
        if is_vec_ty(o, l) && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {  /* Vec::new() */
            POS = POS + 2; rd_ident(); skipws(); eat_ch(40); eat_ch(41);
            if HAS_RVEC == 1 { return emit_rvec_new(); }   /* std-hoist P4.1: route to std RVec_new */
            return emit_vec_new();
        }
        if l == 7 && at(o) == 72 && at(o + 1) == 97 && at(o + 2) == 115 && at(o + 3) == 104 && at(o + 4) == 77 && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {   /* S3: HashMap::new() */
            POS = POS + 2; rd_ident(); skipws(); eat_ch(40); eat_ch(41);
            return emit_hashmap_new();
        }
        if l == 8 && at(o) == 86 && at(o + 1) == 101 && at(o + 2) == 99 && at(o + 3) == 68 && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {   /* STDCOLL N1: VecDeque::new() (V-e-c-D, len 8) */
            POS = POS + 2; rd_ident(); skipws(); eat_ch(40); eat_ch(41);
            return emit_vecdeque_new();
        }
        if l == 7 && at(o) == 72 && at(o + 1) == 97 && at(o + 2) == 115 && at(o + 3) == 104 && at(o + 4) == 83 && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {   /* STDCOLL N2: HashSet::new() (Hash-S vs Hash-M) */
            POS = POS + 2; rd_ident(); skipws(); eat_ch(40); eat_ch(41);
            return emit_hashset_new();
        }
        if l == 8 && at(o) == 66 && at(o + 1) == 84 && at(o + 2) == 114 && at(o + 3) == 101 && at(o + 4) == 101 && at(o + 5) == 77 && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {   /* LAYER2 P6: BTreeMap::new() -- reuse HashMap storage/dispatch; the btree Ty (tag 18) marks it for SORTED iteration */
            POS = POS + 2; rd_ident(); skipws(); eat_ch(40); eat_ch(41);
            let d = emit_hashmap_new(); EITY_PIN = ty_intern(18, 0, 0); return d;
        }
        if l == 8 && at(o) == 66 && at(o + 1) == 84 && at(o + 2) == 114 && at(o + 3) == 101 && at(o + 4) == 101 && at(o + 5) == 83 && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {   /* LAYER2 P6: BTreeSet::new() -- reuse HashSet storage/dispatch; btree Ty (tag 19) */
            POS = POS + 2; rd_ident(); skipws(); eat_ch(40); eat_ch(41);
            let d = emit_hashset_new(); EITY_PIN = ty_intern(19, 0, 0); return d;
        }
        if is_vec_macro(o, l) && POS < LEN && at(POS) == 33 {                         /* vec![ e, ... ] */
            POS = POS + 1; skipws(); eat_ch(91);
            let dv = if HAS_RVEC == 1 { emit_rvec_new() } else { emit_vec_new() };   /* std-hoist P4.1 */
            VELT = 0 - 1; VELENUM = 0 - 1;
            VELVEC = 0;
            skipws();
            if POS < LEN && at(POS) != 93 {
                loop {
                    let epos = POS;   /* TI3e-o: element source start -- re-eval per row for `vec![vec![..]; n]` so rows don't alias */
                    let e = parse_expr();
                    VELT = EITYP;                                                     /* R1: remember the element struct-type (homogeneous) for `for p in &v { p.field }` */
                    if EISENUM == 1 { VELENUM = EIENUMTY; }                            /* R23b: enum element -> remember the enum-type for {:?}/for/index */
                    VELTY = EITY;                                                     /* TI3: remember the element's interned Ty (incl. tuple/struct) */
                    VELVEC = EISVEC;                                                  /* R2bx: element is itself a Vec (Vec<Vec<..>>) */
                    skipws();
                    if POS < LEN && at(POS) == 59 {                                   /* R2: vec![e; n] repeat form -- push e n times (was ignored -> length-1 Vec) */
                        POS = POS + 1; skipws(); let n = parse_expr();
                        let islot = NC; let z = ecr(0); ecopy(islot, z); NC = NC + 1;
                        println!("  loop"); ckreset();
                        let cnd = ebin("lt", islot, n);
                        println!("  brz v{}", cnd); ckreset();
                        if VELVEC == 1 { let save = POS; POS = epos; let ev = parse_expr(); POS = save; if HAS_RVEC == 1 { emit_rvec_push(dv, ev); } else { emit_vec_push(dv, ev); } }   /* TI3e-o: Vec element -> re-emit its construction INSIDE the runtime loop so each row is a fresh inner Vec (was: push the same descriptor n times -> all rows aliased) */
                        else if HAS_RVEC == 1 { emit_rvec_push(dv, e); } else { emit_vec_push(dv, e); }
                        println!("  cont"); ckreset();
                        let one = ecr(1); let ni = ebin("add", islot, one); ecopy(islot, ni);
                        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                        eat_ch(93); EISVEC = 1;
                        skipws(); if POS < LEN && (at(POS) == 46 || at(POS) == 91) { DVELT = VELT; return dispatch_vec(dv); }   /* PTR-ROOT: vec![e;n] temporary followed by .method()/[i] -- dispatch the chain (was returned raw -> the trailing chain read a pointer) */
                        return dv;
                    }
                    if WANTDYN == 1 {                                                 /* A: coerce Box<dyn> element to a [data, type-index] fat pointer */
                        let ety2 = EITYP; let fat = halloc(8); println!("  store v{} v{}", fat, e);
                        let c4d = ecr(4); let etyc = ec(ety2); println!("  store v{} v{}", ebin("add", fat, c4d), etyc);
                        if HAS_RVEC == 1 { emit_rvec_push(dv, fat); } else { emit_vec_push(dv, fat); }
                    } else { if HAS_RVEC == 1 { emit_rvec_push(dv, e); } else { emit_vec_push(dv, e); } }
                    skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 93 { break; } } else { POS = POS; break; }   /* R35: a trailing comma `vec![a,b,]` -> after the comma, `]` ends the list (was: parse_expr on `]` pushed a spurious 0 element) */
                }
            }
            eat_ch(93); EISVEC = 1; DVELENUM = VELENUM;   /* R23b: an inline vec! carries its element enum-type (-1 for non-enum) so {:?} is fresh, not a stale global */
            skipws(); if POS < LEN && (at(POS) == 46 || at(POS) == 91) { DVELT = VELT; return dispatch_vec(dv); }   /* PTR-ROOT: vec![..] temporary followed by .iter()/.method()/[i] -- dispatch the chain (was returned raw -> fold/sum/scan read a pointer 0x1700008) */
            return dv;
        }
        if is_string_ty(o, l) && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {  /* String::new() / String::from("..") */
            POS = POS + 2; rd_ident();
            let isfrom = id_is(102, 114, 111, 109, 0, 4);   /* from */
            skipws(); eat_ch(40);
            let dv = if HAS_RSTRING == 1 { emit_rstring_new() } else { emit_str_new() };   /* std-hoist P4.3: route String::new/from to std RString */
            if isfrom { skipws(); let src = parse_expr(); if HAS_RSTRING == 1 { emit_rstring_push_str(dv, src); } else { emit_str_push_str(dv, src); } EISSTR = 1; EISSTRING = 1; }
            eat_ch(41);
            return dv;
        }
        let ty = ty_find(o, l);
        if ty >= 0 {
            if POS < LEN && at(POS) == 123 { let sv = struct_literal(ty); skipws(); if POS < LEN && at(POS) == 46 { if EWASGARG == 1 { DSGAK = EGAK; DSGAT = EGAT; DSGAK2 = EGAK2; DSGAT2 = EGAT2; } else { DSGAK = 0 - 1; DSGAT = 0 - 1; DSGAK2 = 0 - 1; DSGAT2 = 0 - 1; } return dispatch_struct(sv, ty); } return sv; }           /* Name { .. } literal (+ chained `.method()` -> dispatch) */
            if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {                 /* Name::assoc(args) or Name::CONST */
                POS = POS + 2; skipws(); rd_ident();
                let mo = ID_O; let ml = ID_L; skipws();
                if id_is(100, 101, 102, 97, 117, 7) && method_exists(ty, mo, ml) == 0 && POS < LEN && at(POS) == 40 { POS = POS + 1; skipws(); eat_ch(41); EITYP = ty; return emit_default_struct(ty); }   /* TI3e-e: `Type::default()` with no user impl -> derive(Default) zero-fill */
                if POS < LEN && at(POS) == 40 { POS = POS + 1; let r = emit_mcall(ty, mo, ml, -1); EITYP = ty;
                    skipws(); if POS < LEN && at(POS) == 46 { DSGAK = 0 - 1; DSGAT = 0 - 1; DSGAK2 = 0 - 1; DSGAT2 = 0 - 1; return dispatch_struct(r, ty); }   /* R1: `Type::new().method()` -- chain the postfix on the Self result (was: returned r, so `.add(..)` fell to a stray closure/call) */
                    return r; }   /* assoc fn -- assume it returns Self */
                let ci = cfind(mo, ml);                                              /* R1: associated const `Type::CONST` (registered by prepass_consts) */
                if ci >= 0 { return ec(CVAL[ci as usize]); }
                return ec(0);
            }
            if TSZ[ty as usize] == 0 && (POS >= LEN || at(POS) != 40) {              /* unit-struct value `S` (zero-size; not `S{..}`/`S::`/`S(..)`) -- give it a valid &self address + its type so `let s=S; s.f()` dispatches */
                let base = halloc(4);
                EITYP = ty;
                if POS < LEN && at(POS) == 46 { DSGAK = 0 - 1; DSGAT = 0 - 1; DSGAK2 = 0 - 1; DSGAT2 = 0 - 1; return dispatch_struct(base, ty); }   /* R2bz: direct `S.method()` on a unit-struct value must dispatch the postfix (else the `.m()` was left unparsed -> stray call) */
                return base;
            }
        }
        if l == 3 && at(o) == 66 && at(o + 1) == 111 && at(o + 2) == 120 && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {   /* R2: Box::new(EXPR) -> heap-alloc a cell, store EXPR, return the pointer (Box = a heap pointer) */
            POS = POS + 2; skipws(); rd_ident();                                     /* new */
            skipws(); eat_ch(40);
            let e = parse_expr();
            let ety = EITYP; let een = EISENUM; let eenty = EIENUMTY; let evec = EISVEC; let estr = EISSTRING; let eclo = EISCLO;
            skipws(); eat_ch(41);
            if ety >= 0 || een == 1 || evec == 1 || estr == 1 || eclo == 1 {          /* the inner is ALREADY a heap value (struct/enum/Vec/String/closure fat-ptr) -- Box is transparent (a newtype pointer). A closure cell is {code,env}: Box<dyn Fn> over it is identity. */
                EITYP = ety; EISENUM = een; EIENUMTY = eenty; EISVEC = evec; EISSTRING = estr; EISCLO = eclo;
                return e;
            }
            let b = halloc(4);                                                       /* scalar -- box it: alloc a cell + store */
            println!("  store v{} v{}", b, e);
            return b;
        }
        if ((l == 2 && at(o) == 82 && at(o + 1) == 99) || (l == 3 && at(o) == 65 && at(o + 1) == 114 && at(o + 2) == 99)) && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {   /* S8 L4: `Rc::`/`Arc::` -- TRANSPARENT value semantics (correct for immutable Rc; shared-mutation Rc<RefCell> is a separate pipeline) */
            POS = POS + 2; skipws(); rd_ident(); let rmo = ID_O; let rml = ID_L;    /* new / clone / from */
            skipws(); eat_ch(40);
            if rml == 5 && at(rmo) == 99 && at(rmo + 1) == 108 && at(rmo + 2) == 111 { skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; } }   /* clone(&a) -> strip the leading & */
            let e = parse_expr(); skipws(); eat_ch(41); return opt_postfix(e);       /* Rc::new(x)/Rc::clone(&x)/Rc::from(x) -> the inner value, kind preserved */
        }
        if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {                     /* P3: MODULE::func(..) / crate:: / self:: -> flat call (rzrs has one namespace) */
            POS = POS + 2; skipws(); rd_ident();
            loop { skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); rd_ident(); } else { break; } }   /* a::b::func -> last segment */
            let fo = ID_O; let fl = ID_L; skipws();
            let qsty = ty_find(fo, fl);   /* LAYER4: qualified path struct/tuple literal `m::S{..}` / `a::b::S(..)` -- the last segment is a struct type; resolve it (rzrs has one namespace) */
            if qsty >= 0 && POS < LEN && at(POS) == 123 { let qcell = struct_literal(qsty); EITYP = qsty; skipws(); if POS < LEN && at(POS) == 46 { DSGAK = 0 - 1; DSGAT = 0 - 1; DSGAK2 = 0 - 1; DSGAT2 = 0 - 1; return dispatch_struct(qcell, qsty); } return opt_postfix(qcell); }
            if qsty >= 0 && POS < LEN && at(POS) == 40 {
                POS = POS + 1; let cell = halloc(TSZ[qsty as usize]); let mut fi = 0;
                loop { skipws(); if POS < LEN && at(POS) == 41 { break; } let mut arg = parse_expr(); skipws(); if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 { arg = emit_range_vec(arg); } let off = ec(fi * 4); println!("  store v{} v{}", ebin("add", cell, off), arg); fi = fi + 1; skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; } }
                eat_ch(41); EITYP = qsty; skipws(); if POS < LEN && at(POS) == 46 { DSGAK = 0 - 1; DSGAT = 0 - 1; DSGAK2 = 0 - 1; DSGAT2 = 0 - 1; return dispatch_struct(cell, qsty); }
                return opt_postfix(cell);
            }
            let qvi = var_find(fo, fl);   /* LAYER4: qualified path enum variant `m::E::A` / `a::b::E::A(..)` on a module-nested enum -- last segment is a variant */
            if qvi >= 0 && fn_is(fo, fl) == 0 {
                if POS < LEN && at(POS) == 40 { return emit_bare_variant(qvi); }
                if VHAS[qvi as usize] == 0 && (POS >= LEN || (at(POS) != 40 && at(POS) != 123)) { return opt_postfix(emit_bare_variant(qvi)); }
            }
            if id_is(100, 101, 102, 97, 117, 7) && INTO_TARGET >= 0 && POS < LEN && at(POS) == 40 { POS = POS + 1; skipws(); eat_ch(41); let dty = INTO_TARGET; EITYP = dty; return emit_default_struct(dty); }   /* TI3e-e: `Default::default()` (bare) -> zero-fill the `let x: T` annotation's struct type T */
            if id_is(102, 114, 111, 109, 95, 14) && POS < LEN && at(POS) == 40 {     /* R2ap: <int>::from_str_radix(s, base) -> Ok(parsed) */
                POS = POS + 1; let s = parse_expr(); let c4 = ecr(4); let sbuf = eload(s); let slen = eload(ebin("add", s, c4));
                skipws(); eat_ch(44); let base = parse_expr(); skipws(); eat_ch(41);
                print!("  v{}: i32 = call __from_radix v{} v{} v{}", NC, sbuf, slen, base); println!(""); let val = NC; NC = NC + 1;
                let cell = halloc(8); let one = ecr(1); println!("  store v{} v{}", cell, one); println!("  store v{} v{}", ebin("add", cell, c4), val);
                return opt_postfix(cell);
            }
            if id_is(116, 114, 121, 95, 102, 8) && POS < LEN && at(POS) == 40 {     /* TYPE::try_from(EXPR) -> Ok(v)/Err by the target int type's range */
                POS = POS + 1; let v = parse_expr(); skipws(); eat_ch(41);
                let mut lo = 0; let mut hi = 255; let mut known = 1;
                if l == 2 && at(o) == 117 && at(o + 1) == 56 { lo = 0; hi = 255; }                            /* u8 */
                else if l == 2 && at(o) == 105 && at(o + 1) == 56 { lo = 0 - 128; hi = 127; }                /* i8 */
                else if l == 3 && at(o) == 117 && at(o + 1) == 49 && at(o + 2) == 54 { lo = 0; hi = 65535; } /* u16 */
                else if l == 3 && at(o) == 105 && at(o + 1) == 49 && at(o + 2) == 54 { lo = 0 - 32768; hi = 32767; } /* i16 */
                else { known = 0; }
                if known == 1 {
                    let valid = ebin("mul", ebin("ge", v, ec(lo)), ebin("le", v, ec(hi)));
                    let c4 = ecr(4); let cell = halloc(8); println!("  store v{} v{}", cell, valid); println!("  store v{} v{}", ebin("add", cell, c4), v);
                    EISOPT = 1; EOPTRES = 1; return opt_postfix(cell);
                }
            }
            if id_is(102, 114, 111, 109, 0, 4) && POS < LEN && at(POS) == 40 && ty_find(o, l) < 0 {   /* TRAIT R4: <prim-int or generic-T>::from(x) -- numeric From is identity/widen (i64::from(8), T::from(8)) */
                POS = POS + 1; let v = parse_expr(); let v64 = EIS64; skipws(); eat_ch(41);
                let ptk = prim_int_ty(o, l);
                if (ptk == 4 || ptk == 8 || ptk == 9 || ptk == 10) && v64 == 0 { EIS64 = 1; return opt_postfix(emit_convs(v)); }   /* -> i64/u64/isize/usize: sign-extend a 32-bit source */
                EIS64 = v64; return opt_postfix(v);
            }
            if POS < LEN && at(POS) == 40 { POS = POS + 1; let r = emit_call(fo, fl); return opt_postfix(r); }
            return ec(0);
        }
        if sym_kind(o, l) == 4 && POS < LEN && at(POS) == 40 { let rb = sym_bool(o, l); let scr = SCRETCLO[sym_idx(o, l) as usize]; let r = emit_closure_call(sym_find(o, l)); if rb == 1 { EISBOOL = 1; } if scr == 1 { EISCLO = 1; ECRETCLO = 0; } return r; }   /* R10: closure call; std-hoist: a bool-returning Fn param tags the calli result bool so `!f(x)` uses ==0 */
        if sym_find(o, l) >= 0 && POS < LEN && at(POS) == 40 { return opt_postfix(emit_closure_call(sym_find(o, l))); }   /* LAYER4: a bound LOCAL called as `local(args)` is a fn-pointer/closure invoke even when SK!=4 -- e.g. a `Some(mk_kind)` match-payload binding from `Option<fn(bool)->LiteralKind>` (the pattern binder does not set SK=4). rustc rejects a non-callable local call, so a bound-local immediately followed by `(` is always a closure/fn-ptr call (rustc_lexer c_or_byte_string). */
        if var_find(o, l) >= 0 {   /* NAMERES: bare enum-variant constructor A(..)/A{..} via `use E::*` (qualified E::A handled above) */
            let vfi = var_find(o, l);
            if fn_is(o, l) == 0 { if sym_find(o, l) < 0 {
                if POS < LEN && at(POS) == 40 { return emit_bare_variant(vfi); }   /* tuple-variant A(..) */
                if POS < LEN && at(POS) == 123 { return emit_bare_variant(vfi); }   /* LAYER4: bare STRUCT-variant `V { f: e, .. }` (via `use E::*`) -- emit_bare_variant parses the `{ .. }` fields (positional, decl order) exactly like the qualified `E::V { .. }` path; was DEFERRED here (only tuple-variants called it) so `Int { base, empty_int: false }` in rustc_lexer number() built a disc-only cell and left the `{` unconsumed -> downstream parse drift + miscompile */
                if VHAS[vfi as usize] == 0 && (POS >= LEN || (at(POS) != 40 && at(POS) != 123)) { return opt_postfix(emit_bare_variant(vfi)); }   /* LAYER4 L4-1: bare UNIT variant `A` as a value (via `use E::*`) -> the disc-only enum cell */
            } }
        }
        if POS < LEN && at(POS) == 40 {                                              /* A: tuple struct constructor S(a, b, ..) -> heap cell [f0, f1, ..] (distinguish from a fn call by the type table) */
            let tsty = ty_find(o, l);
            if tsty >= 0 {
                POS = POS + 1;
                let cell = halloc(TSZ[tsty as usize]);
                let mut fi = 0;
                loop {
                    skipws(); if POS < LEN && at(POS) == 41 { break; }
                    let mut arg = parse_expr(); skipws(); if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 { arg = emit_range_vec(arg); }   /* P-RANGE-VALUE-IN-VARIANT-ARG: tuple-struct `S(lo..hi)` */
                    let off = ec(fi * 4); let ad = ebin("add", cell, off); println!("  store v{} v{}", ad, arg);
                    fi = fi + 1;
                    skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; }
                }
                eat_ch(41);
                EITYP = tsty;
                skipws(); if POS < LEN && at(POS) == 46 { DSGAK = 0 - 1; DSGAT = 0 - 1; DSGAK2 = 0 - 1; DSGAT2 = 0 - 1; return dispatch_struct(cell, tsty); }   /* R2bz: direct `S(a,..).method()` on a tuple-struct value must dispatch the postfix */
                return opt_postfix(cell);
            }
        }
        if POS < LEN && at(POS) == 40 {                                              /* NAMERES: E0425 -- a bare `NAME(` free call (Some/Ok/enum-variants/`::`/closures/tuple-structs already excluded above). Reject iff not a user fn (prepass knows all), not an engine `_`-helper, not `drop`, and NOT a leaked method (preceded by `.`, the Pin-Art discriminator). */
            let is_drop = l == 4 && at(o) == 100 && at(o + 1) == 114 && at(o + 2) == 111 && at(o + 3) == 112;
            if fn_is(o, l) == 0 && at(o) != 95 && is_drop == false && preceded_by_dot(o) == 0 { die_undefined(o, l); }
            POS = POS + 1; let r = emit_call(o, l);   /* call [+ postfix] */
            let es = EISSTR; let est = EISSTRING; let ev = EISVEC; let eb = EISBYTES; let een = if EISENUM == 1 { EIENUMTY } else { 0 - 1 };   /* PARSE-5: capture enum-ness of a `-> Box<Enum>`/`-> Enum` free-fn result before the postfix check clobbers the flags */
            skipws();
            if POS < LEN && (at(POS) == 46 || at(POS) == 91) {                     /* P: method/index on a call result of known kind */
                if ev == 1 { return dispatch_vec(r); }
                if eb == 1 { return dispatch_bytes(r); }
                if es == 1 || est == 1 { return dispatch_string(r); }
                if een >= 0 && at(POS) == 46 { let mut q = POS + 1; let cmo = q; while q < LEN && is_idc(at(q)) { q = q + 1; } let cml = q - cmo; let mut rr = q; while rr < LEN && at(rr) == 32 { rr = rr + 1; } if cml > 0 && rr < LEN && at(rr) == 40 && method_exists(0 - (een + 2), cmo, cml) == 1 { POS = rr + 1; return emit_enum_mcall(een, cmo, cml, r); } }   /* PARSE-5 P-FN-RETURNS-BOX-ENUM-CHAIN (free-fn dual of the method-chain fix): a `-> Box<Enum>` free fn with a chained real enum-method (`mk(5).eval()`) -> emit_enum_mcall on the enum owner (byte-scan R17-safe, mirrors emit-4258 + the method-chain arm). Was: fell to opt_postfix -> the enum cell mis-dispatched -> garbage. */
            }
            return opt_postfix(r);
        }
        let styp = sym_styp(o, l);
        let sv = sym_find(o, l);
        if sv < 0 && fn_is(o, l) == 1 && !(POS < LEN && at(POS) == 40) {              /* W1b: bare fn name used as a value -> fat pointer {faddr(__fp_fn), 0}; __fp_ thunk drops the env arg */
            reg_fnptr(o, l, fn_argc(o, l));
            let clo = halloc(8);
            print!("  v{}: i32 = faddr __fp_", NC); emit_span(o, l); println!(""); let fa = NC; NC = NC + 1;
            println!("  store v{} v{}", clo, fa);
            let c4 = ecr(4); let z = ecr(0); println!("  store v{} v{}", ebin("add", clo, c4), z);
            EISCLO = 1;
            return clo;
        }
        if sv >= 0 && sym_kind(o, l) == 4 { EISCLO = 1; return sv; }                 /* R10: closure value (fat pointer) */
        if sv >= 0 && sym_kind(o, l) == 5 {                                          /* R11: local array -> a[i] read / a.len() / R2: a.<method>() via a slice descriptor */
            skipws();
            if POS < LEN && at(POS) == 46 {
                let s = POS; POS = POS + 1; rd_ident();
                if id_is(108, 101, 110, 0, 0, 3) { skipws(); eat_ch(40); eat_ch(41); return ec(sym_arrlen(o, l)); }   /* len -> compile-time count */
                POS = s;                                                             /* R2: any other method (iter/contains/first/last/sum-chains) -> present as a [ptr,len] slice and dispatch_vec */
                let d = halloc(8); let z = ecr(0); println!("  store v{} v{}", ebin("add", d, z), sv);
                let c4 = ecr(4); let cw = ec(sym_arrlen(o, l)); println!("  store v{} v{}", ebin("add", d, c4), cw);
                EISVEC = 1; let r = dispatch_vec(d); let aelt = SARRELT[sym_idx(o, l) as usize]; if aelt >= 0 { EVELT = aelt; }   /* R2ar: .iter() over array-of-structs -> loop var struct-typed */
                let avty = SARRVTY[sym_idx(o, l) as usize]; if avty >= 0 { DVELTY = avty; }   /* TI3e-m: `.iter()` over a &str/String array -> thread the element interned Ty so the loop var is tagged (&str eq / *w no-op) */
                return r;
            }
            if POS < LEN && at(POS) == 91 {                                          /* R2: `a[i]` (scalar) or chained `a[i][j]` (nested array -- each element is an inner-array pointer) */
                let arrlen0 = sym_arrlen(o, l);                                       /* CONSTEVAL R3: static OOB -- if the index folds to a const >= the fixed-array length -> E0080 */
                if arrlen0 > 0 { let isave = POS; POS = POS + 1; CVALID = 1; let ci = ceval_expr(); skipws(); let cok = CVALID == 1 && POS < LEN && at(POS) == 93; POS = isave; if cok && ci >= arrlen0 { die_const_oob(); } }
                let mut cur = sv; let re64 = SARR64[sym_idx(o, l) as usize]; let mut rd64 = 0;
                while POS < LEN && at(POS) == 91 { POS = POS + 1; let iv = parse_expr(); eat_ch(93); skipws(); let rmore = POS < LEN && at(POS) == 91; let rlast64 = if re64 == 1 && rmore == false { 1 } else { 0 }; let cwr = if rlast64 == 1 { ecr(8) } else { ecr(4) }; let off = ebin("mul", iv, cwr); let ad = ebin("add", cur, off); if rlast64 == 1 { cur = eload8(ad); rd64 = 1; } else { cur = eload(ad); } }
                if rd64 == 1 { EIS64 = 1; }
                let aelt = SARRELT[sym_idx(o, l) as usize];                       /* R2ar: array-of-structs -> element is a struct-cell pointer; a[i].field dispatches */
                if aelt >= 0 { EITYP = aelt; skipws(); if POS < LEN && at(POS) == 46 { DSGAK = 0 - 1; DSGAT = 0 - 1; DSGAK2 = 0 - 1; DSGAT2 = 0 - 1; return dispatch_struct(cur, aelt); } }
                skipws(); if POS < LEN && at(POS) == 46 { EISVEC = 0; return opt_postfix(cur); }   /* LAYER9 P-ARRAY-SCALAR-ELEM-POSTFIX: the LOCAL-ARRAY [i] dual of R2's P-SLICE-SCALAR-ELEM-POSTFIX -- a scalar `a[i].method()` (`.clone()`, `.abs()`, ...) on a local `[T;N]` array followed by a `.` must route through opt_postfix (scalar dispatch), else the unconsumed postfix leaks to a Vec/deep-clone re-dispatch -> garbage. The array-of-structs branch above already routes a trailing `.` through dispatch_struct; the scalar fall-through did not. */
                return cur;
            }
            EISARR = 1; EARRLEN = sym_arrlen(o, l); EARRELT = SARRELT[sym_idx(o, l) as usize]; EARRVTY = SARRVTY[sym_idx(o, l) as usize];   /* TI3e-m: thread the array element interned Ty to the for-loop */   /* R2cf: bare array read flags array-ness (for `for x in a` -> build a [ptr,len] descriptor) */
            return sv;
        }
        if sv >= 0 && sym_kind(o, l) == 6 { return dispatch_bytes(sv); }             /* R25: &[u8] byte slice -> .len()/[i] */
        if sv >= 0 && sym_kind(o, l) == 7 { let hvi = sym_idx(o, l); EHMVVEC = 0; if hvi >= 0 && SARRELT[hvi as usize] == 1 { EHMVVEC = 1; } return dispatch_hashmap(sv); }           /* S3: HashMap -> insert/get/contains_key/len/entry */
        if sv >= 0 && sym_kind(o, l) == 8 { return dispatch_dyn(sv); }               /* long-tail: &dyn trait object -> type-tag dispatch */
        if sv >= 0 && sym_kind(o, l) == 9 { let vix = sym_idx(o, l); if vix >= 0 { DVELTY = SVELTY[vix as usize]; } return dispatch_iter(sv); }   /* STATEFUL-ITER: cursor local -> it.next()/.peek() */
        if sv >= 0 && sym_kind(o, l) == 10 { return dispatch_split_iter(sv); }   /* STATEFUL-ITER: splitn str-cursor -> it.next() yields &str pieces */
        if sv >= 0 && sym_kind(o, l) == 11 { return dispatch_vecdeque(sv); }   /* STDCOLL N1: VecDeque local -> push_back/pop_front/len/is_empty */
        if sv >= 0 && sym_kind(o, l) == 12 { return dispatch_hashset(sv); }   /* STDCOLL N2: HashSet local -> insert/contains/len/is_empty */
        if sv >= 0 && sym_kind(o, l) == 13 { return dispatch_char_iter(sv); }   /* P-CHARS-ITER: bound `let mut it = s.chars()` char-cursor -> it.next() yields Some(char)/None (ASCII bytewise) */
        if sv >= 0 && sym_kind(o, l) == 2 { if sym_moved(o, l) == 1 { die_moved(); } let vix = sym_idx(o, l); if vix >= 0 { DVELT = SVELT[vix as usize]; VELT = SVELT[vix as usize]; DVELTY = SVELTY[vix as usize]; DVELENUM = SVELENUM[vix as usize]; if SVECDYN[vix as usize] == 1 { DVELT = 0 - 5; EISDYNVEC = 1; } } skipws(); if POS < LEN && at(POS) == 59 { mark_moved(o, l); } return dispatch_vec(sv); }               /* R4: Vec local -> .push/.len/.pop/[i]; R23b: thread the element enum-type; R25b: VELT so a bound Vec<Struct> {:?} sees the element struct-type; D003: a Vec<dyn> element -> DVELT sentinel -5 (dispatch_dyn per-element) */
        if sv >= 0 && (sym_kind(o, l) == 3 || sym_kind(o, l) == 1) { let k = sym_kind(o, l); if k == 3 && sym_moved(o, l) == 1 { die_moved(); } skipws(); let ismethod = POS < LEN && at(POS) == 46; if k == 3 && POS < LEN && at(POS) == 59 { mark_moved(o, l); } let r = dispatch_string(sv); if ismethod == false { if k == 3 { EISSTRING = 1; } else { EISSTR = 1; } } return r; }   /* R6: String/&str local -> .len; MOVEBORROW m1 (String only, k==3): use-after-move check + `let NEW=OLD;` bare-terminal move (&str k==1 is a borrow, never moved) */
        if sv >= 0 && styp >= 0 {                                                    /* struct local/param -> field/method (R18: move-checked unless Copy) */
            skipws();
            if POS < LEN && at(POS) == 91 {                                          /* operator-trait P.index-op: struct `a[i]` -> impl Index; call Type_index(&self, i). The method's `&self.d[i]` returns the element VALUE (& transparent through dispatch_struct), so use the result directly. */
                POS = POS + 1; let idx = parse_expr(); eat_ch(93);
                print!("  v{}: i32 = call ", NC); emit_span(TSO[styp as usize], TSL[styp as usize]); print!("_index"); println!(" v{} v{}", sv, idx); NC = NC + 1;
                return opt_postfix(NC - 1);
            }
            let iscopy = TCOPY[styp as usize];
            let isref = sym_forref(o, l);   /* LAYER11 P-MUT-REBORROW-MOVE: a `&Struct`/`&mut Struct` param is a reference -- a bare read REBORROWS, never moves */
            let iselem = sym_elemref(o, l);   /* S1-AST: a `for p in &vec_of_struct`/`.iter()` struct loop var is a `&Struct` element -- never moved (you cannot move out of a reference); move-exempt like a &Struct param */
            if iscopy == 0 && isref == 0 && iselem == 0 && sym_moved(o, l) == 1 { die_moved(); }                  /* use of a moved struct */
            skipws();
            let iscmp = POS + 1 < LEN && ((at(POS) == 61 && at(POS + 1) == 61) || (at(POS) == 33 && at(POS + 1) == 61));   /* == or != borrows, does not move */
            if iscopy == 0 && isref == 0 && iselem == 0 && iscmp == 0 && (POS >= LEN || at(POS) != 46) { mark_moved(o, l); }   /* bare read consumes (moves) a non-Copy struct */
            let gidx = sym_idx(o, l); if gidx >= 0 { DSGAK = SGAK[gidx as usize]; DSGAT = SGAT[gidx as usize]; DSGAK2 = SGAK2[gidx as usize]; DSGAT2 = SGAT2[gidx as usize]; } else { DSGAK = 0 - 1; DSGAT = 0 - 1; DSGAK2 = 0 - 1; DSGAT2 = 0 - 1; }   /* std-hoist P0.1/P0.3: hand this local's recorded type-arg kind (+ one nested level) to dispatch */
            return dispatch_struct(sv, styp);
        }
        if sv >= 0 && sym_flt(o, l) == 1 {                                      /* R2ci: float local -- only `NAME as <int-type>` is representable (no soft-float) */
            skipws();
            let fok = POS + 1 < LEN && at(POS) == 97 && at(POS + 1) == 115 && (POS + 2 >= LEN || !is_idc(at(POS + 2)));
            if fok == false { die_unsupported(o, l); }
            return sv;                                                          /* sv holds the truncated int; parse_cast applies the `as <int>` widening */
        }
        if sv >= 0 && sym_byref(o, l) == 1 { return opt_postfix(eload(sv)); }   /* R2bg: by-ref captured scalar -> deref the address for a read */
        if sv >= 0 { if sym_i64(o, l) == 1 { EIS64 = 1; } if sym_u(o, l) == 1 { EISU = 1; } if sym_fltv(o, l) == 1 { EISFLT = 1; } if sym_forref(o, l) == 1 { EWASREF = 1; } if sym_sref1(o, l) == 1 { EWASBOXREF = 1; } if sym_bool(o, l) == 1 { EISBOOL = 1; } if sym_u8(o, l) == 1 { EIS8 = 1; } if sym_char(o, l) == 1 { EISCHAR = 1; } if sym_into(o, l) >= 0 { INTO_TARGET = sym_into(o, l); } ETUPTY = sym_tupty(o, l); let sen = sym_enum(o, l);
            if sen > 0 { skipws(); let ds = POS; if POS < LEN && at(POS) == 46 { POS = POS + 1; if rd_ident() { let mmo = ID_O; let mml = ID_L; skipws(); if POS < LEN && at(POS) == 40 { POS = POS + 1;
                if mml == 5 && at(mmo) == 99 && at(mmo + 1) == 108 && at(mmo + 2) == 111 && at(mmo + 3) == 110 && at(mmo + 4) == 101 && method_exists(0 - ((sen - 1) + 2), mmo, mml) == 0 { skipws(); eat_ch(41); return emit_enum_clone(sen - 1, sv); }   /* LAYER5 P-DERIVE-RECURSIVE: derived `.clone()` (no user impl) on an enum -> copy the cell (was: call to a non-existent <Enum>_clone) */
                if mml == 6 && at(mmo) == 97 && at(mmo + 1) == 115 && at(mmo + 2) == 95 && ((at(mmo + 3) == 114 && at(mmo + 4) == 101 && at(mmo + 5) == 102) || (at(mmo + 3) == 109 && at(mmo + 4) == 117 && at(mmo + 5) == 116)) && method_exists(0 - ((sen - 1) + 2), mmo, mml) == 0 { skipws(); eat_ch(41); let r = opt_postfix(sv); EISENUM = 1; EIENUMTY = sen - 1; return r; }   /* LAYER5 P-BOX-AS-REF: `.as_ref()`/`.as_mut()` on a Box<Enum> (bound sub-node) is IDENTITY -- Box is a transparent pointer, so `&Enum` == the cell. Was: dispatched to a non-existent <Enum>_as_ref -> crash. Now `match a.as_ref() { .. }` matches on the child cell (like `&**a`). */
                let egi = sym_idx(o, l); if egi >= 0 { DSGAK = SGAK[egi as usize]; DSGAT = SGAT[egi as usize]; } else { DSGAK = 0 - 1; DSGAT = 0 - 1; }   /* std-hoist GAP1: hand the enum local's type-arg kind to the method dispatch */
                return emit_enum_mcall(sen - 1, mmo, mml, sv); } } POS = ds; } }   /* R2: `e.method(..)` on a user enum -> call EnumName_method */
            let soi = sym_idx(o, l); if soi >= 0 { EOPTK = SOPTK[soi as usize]; EOPTVELT = SOPTVELT[soi as usize]; EISOPT = SISOPT[soi as usize]; EOPTRES = SOPTRES[soi as usize]; EOPTREF = SOPTREF[soi as usize]; } else { EOPTK = 0; EOPTVELT = 0 - 1; EISOPT = 0; }   /* container-kind K1/(a): restore bound Option payload-kind + is-Option */
            if EISOPT == 1 { skipws(); if POS < LEN && at(POS) == 46 { let tks = POS; POS = POS + 1;
                if rd_ident() && id_is(116, 97, 107, 101, 0, 4) { skipws(); if POS < LEN && at(POS) == 40 { POS = POS + 1; skipws(); eat_ch(41);
                    /* RUSTC-FE DS: `o.take()` on a bound Option local -- return the OLD Option (a copy of the [disc,payload] cell) + set the ORIGINAL cell's disc to 0 (None) IN PLACE. sv IS the cell pointer, so the store clears o. Was: fell to opt_postfix (a value read) -> returned the payload but left o Some -> `o.is_none()` stayed false. rustc's OnDrop::disable does self.0.take(). */
                    let nc = halloc(8); let c4 = ecr(4);
                    let d0 = eload(sv); println!("  store v{} v{}", nc, d0);
                    let p0 = eload(ebin("add", sv, c4)); println!("  store v{} v{}", ebin("add", nc, c4), p0);
                    let z = ecr(0); println!("  store v{} v{}", sv, z);
                    EISOPT = 1; return opt_postfix(nc);
                } POS = tks; } else { POS = tks; } } }
            let stup = sym_tupty(o, l); skipws(); let baretup = stup >= 0 && !(POS < LEN && at(POS) == 46);   /* DBG: a BARE tuple local (no trailing `.field`) -> restore EISTUP so `{:?}` prints (a, b) not the cell pointer */
            let r = opt_postfix(sv); if sen > 0 { EISENUM = 1; EIENUMTY = sen - 1; }
            if baretup == true { ETUPTY = stup; ETUPN = TUPTY_N[stup as usize]; EITY_PIN = ty_intern(11, 0, stup); }
            return r; }  /* local scalar (Option cell / i64 / for-ref / bool / enum / u8) */
        let gi = g_find(o, l);
        if gi >= 0 {
            skipws();
            if POS < LEN && at(POS) == 91 {                                        /* GLOBAL_ARRAY[i] read */
                POS = POS + 1; let iv = parse_expr(); eat_ch(93);
                let base = egref(o, l); let es = GE[gi];
                let mut off = iv;
                if es != 1 { let c = ec(es); off = ebin("mul", iv, c); }
                let ad = ebin("add", base, off);
                if es == 1 { return eloadb(ad); }
                if es == 8 { EIS64 = 1; return eload8(ad); }   /* C-S0-I64-INDEXED-32: 64-bit load AND tag the value 64-bit, per the FI64 idiom */
                return eload(ad);
            }
            if GK[gi] == 3 { let d = emit_str_literal(GSTART[gi as usize], GCNT[gi as usize]); skipws(); if POS < LEN && at(POS) == 46 { EISSTR = 1; return dispatch_string(d); } EISSTR = 1; return d; }   /* LAYER3 M3: &str const -> emit its literal into the __str_arena (descriptor + EISSTR); route a following .method() to dispatch_string */
            if GK[gi] == 1 { return egref(o, l); }                                 /* bare global array -> base pointer */
            /* C-S0-I64-INDEXED-32: an i64/u64/usize/isize scalar static is an 8-byte cell -- 64-bit load, and tag
               the value so `{}` and any downstream op treat it as 64-bit. */
            let gr = egref(o, l); if GE[gi] == 8 { EIS64 = 1; return eload8(gr); } return eload(gr);   /* global scalar read */
        }
        -1
    }
}
fn parse_addr() -> i32 {
    unsafe {
        skipws(); if POS < LEN && at(POS) == 42 { POS = POS + 1; return parse_atom(); }   /* &*x -> x */
        if POS < LEN && at(POS) == 38 { POS = POS + 1; return parse_atom(); }   /* STATEFUL-ITER: `&&x` (e.g. Some(&&x) vs it.peek()) -- nested ref is transparent; the inner value (not an address) so it matches peek's value payload */
        if POS < LEN && at(POS) == 124 { return parse_atom(); }   /* LAYER5 P-CLOSURE-BY-REF: `&|x| body` -- a closure literal produces a {code,env} fat-ptr cell (already a pointer); `&` is transparent. Was: fell through parse_addr's `&NAME` path -> mis-handled -> `apply(v, &|x| ..)` (closure passed by ref) crashed while by-value worked. */
        if POS + 3 < LEN && at(POS) == 109 && at(POS + 1) == 111 && at(POS + 2) == 118 && at(POS + 3) == 101 { let mut q = POS + 4; while q < LEN && (at(q) == 32 || at(q) == 9) { q = q + 1; } if q < LEN && at(q) == 124 { return parse_atom(); } }   /* `&move |x| body` -- transparent too */
        if POS < LEN && is_dig(at(POS)) { let rv = parse_atom(); if EISSTR == 0 && EISSTRING == 0 { EWASREF = 1; } return rv; }   /* R2ck: &<literal>.method() -- a value, not a place; & transparent (e.g. &3.to_string()). D004 micro-edge: `&<int-literal>` is a ref to an rvalue temporary that cruft represents by VALUE (a scalar `&prim` is passed value-transparent -- the BLANK_PRIM_SELF / generic-`&T`-prim convention relies on it, so it must NOT become an address). Mark EWASREF so a subsequent `*` / a `let r = &7` binding treats the deref as a no-op (r IS the value), matching rustc's `*r == 7`; was: `let r=&7; *r` did a real eload(7) -> crash. Scoped out for a `&3.to_string()` String result (its own path no-ops the deref). Resolves the direct/by-value-match forms; a `&<literal>` stored in an Option/Vec then deref'd BY-REF (`match &Some(&7){Some(e)=>**e}`) stays the value-vs-address residual -- pathological, and irreconcilable with the primitive-by-value convention. */
        if POS < LEN && at(POS) == 40 { return parse_atom(); }   /* R2ck: &(expr) -- a value, not a place; & is transparent */
        { let ps = POS; if rd_ident() && id_is(102, 111, 114, 109, 97, 6) && POS < LEN && at(POS) == 33 { return emit_format(); } POS = ps; }   /* &format!(..) -> String descriptor */
        { let ps = POS; if rd_ident() && is_vec_macro(ID_O, ID_L) && POS < LEN && at(POS) == 33 { POS = ps; return parse_atom(); } POS = ps; }   /* LAYER9 P-REF-VEC-MACRO-ARG: `&vec![..]` -- a vec! macro is a heap [buf,len] descriptor (already a pointer), so `&` is TRANSPARENT; route through parse_atom (which builds the descriptor). Was: `vec` fell through to the `&NAME` path -> treated as a local ident -> the `![..]` left in the stream -> a garbage descriptor + a shifted arg slot (a following scalar/enum arg read wrong / segfaulted). Mirrors the `&format!` transparency above; a bound `let v=vec![..]; &v` already worked (this is the temporary form). */
        if POS < LEN && at(POS) == 91 {                                            /* E3: &[e0, e1, ..] array-literal slice -> [ptr,len] Vec-like descriptor */
            let buf = parse_atom(); let n = EARRLEN;
            let desc = halloc(8);
            let z = ecr(0); let a0 = ebin("add", desc, z); println!("  store v{} v{}", a0, buf);
            let c4 = ecr(4); let a1 = ebin("add", desc, c4); let nv = ec(n); println!("  store v{} v{}", a1, nv);
            EISVEC = 1; return desc;
        }
        let nstart = POS;
        rd_ident();
        let mut ismut = 0;
        if is_kw_mut() { rd_ident(); ismut = 1; }                                   /* &mut NAME */
        let o = ID_O; let l = ID_L;
        let gi = g_find(o, l);
        skipws();
        if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = nstart; return parse_atom(); }   /* R1: `&Enum::Variant{..}(..)` / `&Type::assoc()` -- heap value, & is transparent */
        if POS < LEN && at(POS) == 123 && ty_find(o, l) >= 0 { POS = nstart; return parse_atom(); }      /* R1: `&Struct { .. }` literal -- & is transparent */
        if POS < LEN && at(POS) == 40 && ty_find(o, l) >= 0 && sym_find(o, l) < 0 { POS = nstart; return parse_atom(); }   /* S1 P-TRAIT-GENERIC-DISPATCH: `&Tuple(args)` tuple-struct constructor -- & transparent, so EITYP=Ty flows as the generic type-arg (was: fell through -> styp=i32 default -> wrong monomorph) */
        if ismut == 0 && ty_find(o, l) >= 0 && sym_find(o, l) < 0 && (POS >= LEN || at(POS) != 40) { POS = nstart; return parse_atom(); }   /* R2cj: `&UnitStruct` value (no local of that name) -- route through parse_atom's unit-struct branch (& transparent) so EITYP is set for generic dispatch */
        if POS < LEN && at(POS) == 40 && sym_find(o, l) < 0 && fn_is(o, l) == 1 && (fnret_enum(o, l) >= 0 || fnret_st(o, l) >= 0 || fnret(o, l) == 2 || fnret(o, l) == 3) { POS = nstart; return parse_atom(); }   /* LAYER5 P-REF-TO-ENUM-CALL: `&f(args)` where f returns a HEAP value (enum/struct/Vec/String) -- the result is already a cell/descriptor pointer, so `&` is transparent; route through parse_atom so the call emits + keeps its kind. Was: fell through and mis-treated `f` as a local -> `eval(&mk(5))` inline read 0. GATED to heap returns: a SCALAR-returning fn's `&f()` is a ref-to-temporary the callee derefs, so it must keep the address-of path (else `add(&sq(3),..)` derefs the scalar as a pointer -> crash). */
        if POS < LEN && at(POS) == 46 {                                            /* &NAME.field -> field value (container descriptor) */
            let sv2 = sym_find(o, l); let stp2 = sym_styp(o, l);
            if sv2 >= 0 && stp2 >= 0 { let g2 = sym_idx(o, l); if g2 >= 0 { DSGAK = SGAK[g2 as usize]; DSGAT = SGAT[g2 as usize]; DSGAK2 = SGAK2[g2 as usize]; DSGAT2 = SGAT2[g2 as usize]; } else { DSGAK = 0 - 1; DSGAT = 0 - 1; DSGAK2 = 0 - 1; DSGAT2 = 0 - 1; } return dispatch_struct(sv2, stp2); }
            if sv2 >= 0 && gi < 0 && sym_kind(o, l) == 0 && sym_enum(o, l) <= 0 { POS = nstart; return parse_atom(); }   /* R2ck: &scalar.method() (e.g. &count.to_string()) -- a value; route the postfix through parse_atom/opt_postfix, & transparent */
        }
        if POS < LEN && at(POS) == 91 {                                            /* &NAME[i] */
            if gi < 0 && sym_kind(o, l) == 2 {                                     /* &vec[..] on a local Vec */
                POS = POS + 1; skipws();
                let mut a = ec(0); if !(POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46) { a = parse_expr(); }   /* LAYER2 P3-sub: `&v[..]` / `&v[..hi]` -- a leading `..` omits the low bound (a=0), else parse it */
                skipws();
                if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 {           /* E3: &v[a..b] / &v[a..] / &v[..] / &v[..b] sub-slice -> [ptr+a*4, len] descriptor (shares the buffer) */
                    POS = POS + 2; skipws();
                    let mut hasb = 1; let mut b = 0;
                    if POS < LEN && at(POS) == 93 { hasb = 0; } else { b = parse_expr(); }
                    eat_ch(93);
                    let dv = sym_find(o, l); let buf = eload(dv); let c4 = ecr(4);
                    let aoff = ebin("mul", a, c4); let nptr = ebin("add", buf, aoff);
                    let mut nlen = 0;
                    if hasb == 1 { nlen = ebin("sub", b, a); } else { let ld = ebin("add", dv, c4); let len = eload(ld); nlen = ebin("sub", len, a); }
                    let desc = halloc(8);
                    let z = ecr(0); let d0 = ebin("add", desc, z); println!("  store v{} v{}", d0, nptr);
                    let d1 = ebin("add", desc, c4); println!("  store v{} v{}", d1, nlen);
                    EISVEC = 1; return desc;
                }
                eat_ch(93);                                                        /* &vec[i] inner element (Vec<Vec<..>> or Vec<Struct>) */
                let dv = sym_find(o, l); let p = eload(dv); let c4 = ecr(4); let off = ebin("mul", a, c4); let ea = ebin("add", p, off);
                let vi3 = sym_idx(o, l); if vi3 >= 0 && SVELT[vi3 as usize] >= 0 { EITYP = SVELT[vi3 as usize]; return eload(ea); }   /* R1: &vec_of_struct[i] -> struct-typed element */
                EISVEC = 1; return eload(ea);
            }
            if gi < 0 && sym_kind(o, l) == 5 {                                     /* R6: &a[lo..hi] / &a[i] on a local fixed array (the local IS the buffer) */
                POS = POS + 1; skipws();
                let mut a = ec(0); if !(POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46) { a = parse_expr(); }   /* LAYER2 P3-sub: leading `..` omits the low bound */
                skipws();
                let buf = sym_find(o, l); let c4 = ecr(4);
                if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 {           /* &a[lo..hi] / &a[lo..] -> [ptr+lo*4, len] descriptor over the array buffer */
                    POS = POS + 2; skipws();
                    let mut hasb = 1; let mut b = 0;
                    if POS < LEN && at(POS) == 93 { hasb = 0; } else { b = parse_expr(); }
                    eat_ch(93);
                    let aoff = ebin("mul", a, c4); let nptr = ebin("add", buf, aoff);
                    let mut nlen = 0;
                    if hasb == 1 { nlen = ebin("sub", b, a); } else { let cw = ec(sym_arrlen(o, l)); nlen = ebin("sub", cw, a); }
                    let desc = halloc(8);
                    let z = ecr(0); let d0 = ebin("add", desc, z); println!("  store v{} v{}", d0, nptr);
                    let d1 = ebin("add", desc, c4); println!("  store v{} v{}", d1, nlen);
                    EISVEC = 1; return desc;
                }
                eat_ch(93);                                                        /* &a[i] -> address of element i */
                let off = ebin("mul", a, c4); return ebin("add", buf, off);
            }
            if gi < 0 && (sym_kind(o, l) == 1 || sym_kind(o, l) == 3) { return emit_str_slice(sym_find(o, l)); }   /* STDCOLL N3: &w[lo..hi] on a &str/String -> sub-slice descriptor (before the scalar-index fallthrough, which mis-parses `0..3`) */
            POS = POS + 1; let iv = parse_expr(); eat_ch(93);
            if gi >= 0 {
                let base = egref(o, l); let es = GE[gi];
                let mut off = iv;
                if es != 1 { let c = ec(es); off = ebin("mul", iv, c); }
                return ebin("add", base, off);
            }
            return -1;
        }
        if gi >= 0 { return egref(o, l); }                                         /* &global-scalar -> gref */
        let sk = sym_kind(o, l);
        if sk == 4 { EISCLO = 1; return sym_find(o, l); }                           /* W1b: &closure (`&g` for a &dyn Fn param) -> the fat pointer, transparent */
        if sk == 1 || sk == 3 { EISSTR = 1; return sym_find(o, l); }                /* &str / &String -> descriptor value (the `&w[lo..hi]` slice is caught in the &NAME[ block above) */
        if sk == 2 { let vi2 = sym_idx(o, l); if vi2 >= 0 && SVECDYN[vi2 as usize] == 1 { EISDYNVEC = 1; } if vi2 >= 0 && SVELT[vi2 as usize] >= 0 { EVELT = SVELT[vi2 as usize]; } if vi2 >= 0 { DVELTY = SVELTY[vi2 as usize]; EVELENUM = SVELENUM[vi2 as usize]; } return sym_find(o, l); }   /* &Vec -> descriptor value (flag dyn / struct-element; TI3: element Ty; R24b: element enum-type) */
        if sk == 5 {                                                               /* R6: &a (whole local fixed array) -> [ptr, len] slice descriptor */
            let buf = sym_find(o, l); let desc = halloc(8);
            let z = ecr(0); let d0 = ebin("add", desc, z); println!("  store v{} v{}", d0, buf);
            let c4 = ecr(4); let d1 = ebin("add", desc, c4); let cw = ec(sym_arrlen(o, l)); println!("  store v{} v{}", d1, cw);
            EISVEC = 1; return desc;
        }
        if sk == 6 { EISBYTES = 1; return sym_find(o, l); }                         /* BVK: &[u8] byte-slice local (SK=6) -> descriptor value with the byte tag, so `for x in &v` reads sbytes=1 -> stride-1 byte loads (was: fell to scalar-ref -> 4-byte stride -> wrong sum) */
        if sk == 7 { EISHM = 1; let mt = sym_ty(o, l); if mt >= 0 && TYTAG[mt as usize] == 18 { EITY_PIN = mt; } return sym_find(o, l); }                            /* S3: &HashMap -> descriptor (for `for (k,v) in &m`); LAYER2 P6: surface a BTreeMap's btree Ty so the for-in sorts */
        if sk == 13 { EISITER = 1; return sym_find(o, l); }                         /* RUSTC-FE LEX: &mut Chars cursor -> the cell pointer (transparent, like &Vec/&str), so a callee `.next()` advances the CALLER's shared cursor. Was: fell to the scalar `eref(slot)` path -> the callee got &slot not the cursor -> `adv(&mut it)` did not advance `it`. rustc_lexer threads `chars: &mut Chars` through unescape. */
        let stp = sym_styp(o, l);
        if stp >= 0 { EITYP = stp; return sym_find(o, l); }                         /* &struct -> struct pointer + type (for generic dispatch) */
        let sen = sym_enum(o, l);
        if sen > 0 { EISENUM = 1; EIENUMTY = sen - 1; return sym_find(o, l); }       /* R2: &enum -> cell pointer (transparent; the enum local's value IS the cell) */
        let soi = sym_idx(o, l);
        if soi >= 0 && SISOPT[soi as usize] == 1 { EISOPT = 1; EOPTK = SOPTK[soi as usize]; EOPTRES = SOPTRES[soi as usize]; EOPTREF = SOPTREF[soi as usize]; return sym_find(o, l); }   /* DEFECT-b fix: `&Option` local -> the cell pointer (transparent, like &enum; an Option<T> local IS a heap cell [disc,payload]). Was: fell to `eref(slot)` (address-of the slot) -> a callee `match r { Some=>.. }` did `load(&slot)` = the cell ptr not the disc -> always None. &UserEnum already hit the sen>0 branch above; Option was the untagged residual. */
        if ismut == 1 { EMUTBOR = 1; EMUTBOR_O = o; EMUTBOR_L = l; }                 /* R3: &mut on a scalar local -- record for the aliasing check */
        let slot = sym_find(o, l); return eref(slot);                              /* &local scalar -> ref(slot) */
    }
}
/* MEM: parse `&mut NAME` (a simple local) and return NAME's slot; capture kind/struct-type for the caller. */
fn mref_slot() -> i32 {
    unsafe {
        skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; }
        skipws(); rd_ident();
        if id_is(109, 117, 116, 0, 0, 3) { skipws(); rd_ident(); }   /* consumed `mut` -> read the name */
        let no = ID_O; let nl = ID_L;
        MREFK = sym_kind(no, nl); MREFSTY = sym_styp(no, nl); MREFADDR = 0;
        skipws();
        if POS < LEN && at(POS) == 46 {                              /* `&mut p.field` -> field memory address */
            let pptr = sym_find(no, nl); let pty = sym_styp(no, nl);
            POS = POS + 1; rd_ident(); let fi = fld_find(pty, ID_O, ID_L);
            MREFADDR = 1;
            return ebin("add", pptr, ec(FSB[fi as usize]));
        }
        sym_find(no, nl)
    }
}
/* R2 (value-model meet): narrow `a` to `modv` (=2^bits) with wraparound; if `signed`, sign-extend from `half` (=2^(bits-1)). */
fn emit_narrow(a: i32, modv: i32, half: i32, signed: i32) -> i32 {
    unsafe {
        let m = ebin("mod", a, ec(modv)); let s = ebin("add", m, ec(modv)); let u = ebin("mod", s, ec(modv));   /* u in [0, modv) even for negative a */
        if signed == 0 { return u; }
        let ishigh = ebin("ge", u, ec(half)); let sub = ebin("sub", u, ec(modv));
        let res = NC; NC = NC + 1; ecopy(res, u);
        println!("  brz v{}", ishigh); ckreset(); ecopy(res, sub); println!("  pool"); ckreset();
        res
    }
}
fn parse_cast() -> i32 {
    let mut a = parse_atom();
    /* `as TYPE`* -- the value-model width meet: `as char` (display flag), integer narrowing (trunc/sign-reinterpret), i64<->i32. */
    unsafe {
        loop {
            skipws();
            if POS + 1 < LEN && at(POS) == 97 && at(POS + 1) == 115 && (POS + 2 >= LEN || !is_idc(at(POS + 2))) {
                let wasenum = EISENUM; let mut wasi64 = EIS64; let wasflt = EISFLT;
                POS = POS + 2; rd_ident();
                EISBOOL = 0; EISENUM = 0;   /* a cast to a numeric/char type is no longer a bool/enum */
                if wasflt == 1 && !(ID_L == 3 && at(ID_O) == 102) { let e9 = emit_i64_lit(1000000000); a = ebin8(3, a, e9); EISFLT = 0; wasi64 = 1; }   /* SOFTFLOAT: `<f64> as <int>` -> scaled / 1e9 = integer part (truncate toward zero); now a plain i64 for the narrowing below */
                if ID_L == 4 && at(ID_O) == 99 && at(ID_O + 1) == 104 && at(ID_O + 2) == 97 && at(ID_O + 3) == 114 { if wasi64 == 1 { die_unsupported(ID_O, ID_L); } EISCHAR = 1; }   /* R2: only u8 casts to char; i64 as char -> reject like rustc */
                else {
                    EISCHAR = 0; EISU = 0; EIS8 = 0;   /* a numeric cast clears unsignedness + u8-width; the u8/u64 branches re-set */
                    if wasenum == 1 { a = eload(a); }   /* R2: `EnumVariant as i32` -> the discriminant (cell[0]) */
                    let t0 = at(ID_O); let t1 = at(ID_O + 1); let t2 = at(ID_O + 2);
                    if ID_L == 2 && t0 == 117 && t1 == 56 { a = emit_narrow(a, 256, 128, 0); EIS64 = 0; EIS8 = 1; }        /* u8 (subsequent arith masks 0xFF) */
                    else if ID_L == 2 && t0 == 105 && t1 == 56 { a = emit_narrow(a, 256, 128, 1); EIS64 = 0; }   /* i8 -- sign reinterpret */
                    else if ID_L == 3 && t0 == 117 && t1 == 49 && t2 == 54 { a = emit_narrow(a, 65536, 32768, 0); EIS64 = 0; }   /* u16 */
                    else if ID_L == 3 && t0 == 105 && t1 == 49 && t2 == 54 { a = emit_narrow(a, 65536, 32768, 1); EIS64 = 0; }   /* i16 */
                    else if ID_L == 3 && (t0 == 117 || t0 == 105) && t1 == 51 && t2 == 50 {                       /* u32 / i32 */
                        if wasi64 == 1 { let mask = ec64raw(4294967295); a = ebin("and", a, mask); EIS64 = 0; }   /* i64 -> i32: low 32 bits */
                    }
                    else if ID_L == 3 && (t0 == 117 || t0 == 105) && t1 == 54 && t2 == 52 {          /* u64 / i64 -- widen the 32-bit value: i64 sign-extends (convs), u64 zero-extends (conv) */
                        if wasi64 == 0 { if t0 == 117 { a = emit_conv(a); } else { a = emit_convs(a); } }
                        EIS64 = 1; if t0 == 117 { EISU = 1; } else { EISU = 0; }
                    }
                    else if ID_L == 3 && t0 == 102 && t1 == 54 && t2 == 52 {          /* SOFTFLOAT: `<int> as f64` -> fixed-point scaled i64 (value * 1e9) */
                        if wasi64 == 0 { a = emit_convs(a); }                          /* widen the 32-bit int to i64 first */
                        let e9 = emit_i64_lit(1000000000); a = ebin8(2, a, e9);       /* a * 1e9 */
                        EIS64 = 1; EISU = 0; EISFLT = 1;
                    }
                }
            } else { break; }
        }
    }
    a
}
fn parse_mul() -> i32 {
    let mut a = parse_cast();
    unsafe {
        let mut a64 = EIS64;
        let mut a8 = EIS8;                                               /* value-width: u8 operand -> mask the result &0xFF (mod distributes over the chain) */
        let mut aty = EITYP;                                             /* A: left operand's struct type, for operator overloading */
        let mut afl = EISFLT;                                            /* SOFTFLOAT: left operand is a fixed-point f64 */
        loop {
            let c = peek();
            if c == 47 && afl == 1 {                                     /* SOFTFLOAT: fixed-point f64 division a/b (both operands scaled by 1e9). The naive (a*1e9)/b overflows i64 once the dividend value > ~9.2 (a*1e9 = value*1e18). Split into integer + fractional parts so the big 1e9 factor never multiplies the whole dividend: result = (a/b)*1e9 + ((a%b)*1e9)/b. Now only the DIVISOR magnitude (rare) bounds overflow, not the dividend. */
                POS = POS + 1; let b = parse_cast();
                let q = ebin8(3, a, b);                                  /* a/b (scales cancel -> integer part of value_a/value_b) */
                let e9 = emit_i64_lit(1000000000); let qs = ebin8(2, q, e9);
                let rem = ebin8(4, a, b);                                /* a%b */
                let e9b = emit_i64_lit(1000000000); let rn = ebin8(2, rem, e9b); let rf = ebin8(3, rn, b);
                a = ebin8(0, qs, rf);
                a64 = 1; afl = 1; aty = -1;
            }
            else if c == 42 && afl == 1 {                                /* SOFTFLOAT: fixed-point f64 multiply a*b = value_a*value_b*1e9. Both operands scaled by 1e9, so a*b is 1e18-scaled and a raw i64 (a*b)/1e9 overflows once value_a*value_b > ~9.2. Split the 1e9 divide across both operands first: ((a/1e3)*(b/1e3))/1e3 keeps the intermediate at value*1e12 (fits to ~9e6 products) at the cost of <1e-6 precision. */
                POS = POS + 1; let b = parse_cast();
                let e3 = emit_i64_lit(1000);
                let ah = ebin8(3, a, e3); let e3b = emit_i64_lit(1000); let bh = ebin8(3, b, e3b);
                let prod = ebin8(2, ah, bh); let e3c = emit_i64_lit(1000); a = ebin8(3, prod, e3c);
                a64 = 1; afl = 1; aty = -1;
            }
            else if (c == 42 || c == 47 || c == 37) && aty >= 0 {             /* A: `a * b` / `a / b` / `a % b` on a struct type -> Mul/Div/Rem trait method */
                POS = POS + 1; let b = parse_cast();
                let mut r = -1;
                if c == 42 { r = op_method(aty, a, b, 109, 117, 108); } else if c == 47 { r = op_method(aty, a, b, 100, 105, 118); } else { r = op_method(aty, a, b, 114, 101, 109); }   /* mul / div / rem */
                if r >= 0 { a = r; aty = EITYP; a64 = 0; a8 = 0; }
                else { let b64 = EIS64; let b8 = EIS8; if b8 == 1 { a8 = 1; } let w = w64(a64, b64); if c == 42 { a = i64op("mul", a, a64, b, b64, w); } else if c == 47 { a = i64op("div", a, a64, b, b64, w); } else { a = i64op("mod", a, a64, b, b64, w); } a64 = w; aty = -1; }
            }
            else if c == 42 { POS = POS + 1; let b = parse_cast(); let b64 = EIS64; let b8 = EIS8; if b8 == 1 { a8 = 1; } let w = w64(a64, b64); a = i64op("mul", a, a64, b, b64, w); a64 = w; aty = EITYP; }
            else if c == 47 { POS = POS + 1; let b = parse_cast(); let b64 = EIS64; let b8 = EIS8; if b8 == 1 { a8 = 1; } let w = w64(a64, b64); a = i64op("div", a, a64, b, b64, w); a64 = w; aty = EITYP; }
            else if c == 37 { POS = POS + 1; let b = parse_cast(); let b64 = EIS64; let b8 = EIS8; if b8 == 1 { a8 = 1; } let w = w64(a64, b64); a = i64op("mod", a, a64, b, b64, w); a64 = w; aty = EITYP; }
            else { break; }
        }
        if a8 == 1 && a64 == 0 { a = ebin("and", a, ec(255)); }   /* value-width: u8 arithmetic result masks to 0xFF */
        EIS64 = a64; EIS8 = a8; EISFLT = afl;                     /* SOFTFLOAT: propagate the fixed-point float flag */
    }
    a
}
fn parse_add() -> i32 {
    let mut a = parse_mul();
    unsafe {
        let mut a64 = EIS64;
        let mut a8 = EIS8;                                               /* value-width: u8 operand -> mask the result &0xFF */
        let mut aty = EITYP;                                             /* A: left operand's struct type, for operator overloading */
        let mut astr = EISSTRING;                                        /* R2: `String + &str` concatenation */
        loop {
            let c = peek();
            if c == 43 && astr == 1 {                                    /* R2: String + &str/String -> concat into a fresh String */
                POS = POS + 1; let b = parse_mul();
                let nd = emit_str_new(); emit_str_push_str(nd, a); emit_str_push_str(nd, b);
                a = nd; EISSTRING = 1; EISSTR = 1; astr = 1; aty = -1; a8 = 0;
            }
            else if (c == 43 || c == 45) && aty >= 0 {                        /* A: `a + b` / `a - b` on a struct type -> Add/Sub trait method */
                POS = POS + 1; let b = parse_mul();
                let mut r = -1;
                if c == 43 { r = op_method(aty, a, b, 97, 100, 100); } else { r = op_method(aty, a, b, 115, 117, 98); }   /* add / sub */
                if r >= 0 { a = r; aty = EITYP; a64 = 0; a8 = 0; }
                else { let b64 = EIS64; let b8 = EIS8; if b8 == 1 { a8 = 1; } let w = w64(a64, b64); if c == 43 { a = i64op("add", a, a64, b, b64, w); } else { a = i64op("sub", a, a64, b, b64, w); } a64 = w; aty = -1; }
            }
            else if c == 43 { POS = POS + 1; let b = parse_mul(); let b64 = EIS64; let b8 = EIS8; if b8 == 1 { a8 = 1; } let w = w64(a64, b64); a = i64op("add", a, a64, b, b64, w); a64 = w; aty = EITYP; }
            else if c == 45 { POS = POS + 1; let b = parse_mul(); let b64 = EIS64; let b8 = EIS8; if b8 == 1 { a8 = 1; } let w = w64(a64, b64); a = i64op("sub", a, a64, b, b64, w); a64 = w; aty = EITYP; }
            else { break; }
        }
        if a8 == 1 && a64 == 0 { a = ebin("and", a, ec(255)); }   /* value-width: u8 arithmetic result masks to 0xFF */
        EIS64 = a64; EIS8 = a8;
    }
    a
}
/* Backend detour: bitwise operators route to the existing i64 bit-ops (and8/or8/xor8/shl8/shr8) -- the
   backend has no i32 bit-ops, but x86 32-bit results have zero high bits, so the 64-bit op is correct for
   i32 values. Rust precedence: << >>  >  &  >  ^  >  |  (all below + - and above comparisons). */
fn parse_shift() -> i32 {
    let mut a = parse_add();
    unsafe {
        let mut a64 = EIS64;
        let au = EISU;
        loop {
            skipws();
            if POS + 1 < LEN && at(POS) == 60 && at(POS + 1) == 60 { POS = POS + 2; let b = parse_add(); let b64 = EIS64; a = ebin8raw("shl", a, b); a64 = w64(a64, b64); }
            else if POS + 1 < LEN && at(POS) == 62 && at(POS + 1) == 62 { POS = POS + 2; let b = parse_add(); let b64 = EIS64;
                if au == 1 { a = ebin8raw("shu", a, b); }                          /* MEET-3: u64 >> -- logical shr, no sign-extend */
                else { let sa = emit_convs(a); a = ebin8raw("shr", sa, b); }       /* R2: arithmetic (signed) >> -- sign-extend so sar reads the true sign bit */
                a64 = w64(a64, b64); }
            else { break; }
        }
        EIS64 = a64; EISU = au;
    }
    a
}
fn parse_bitand() -> i32 {
    let mut a = parse_shift();
    unsafe {
        let mut a64 = EIS64;
        loop {
            skipws();
            if POS < LEN && at(POS) == 38 && (POS + 1 >= LEN || at(POS + 1) != 38) { POS = POS + 1; let b = parse_shift(); let b64 = EIS64; a = ebin8raw("and", a, b); a64 = w64(a64, b64); }
            else { break; }
        }
        EIS64 = a64;
    }
    a
}
fn parse_bitxor() -> i32 {
    let mut a = parse_bitand();
    unsafe {
        let mut a64 = EIS64;
        loop {
            skipws();
            if POS < LEN && at(POS) == 94 { POS = POS + 1; let b = parse_bitand(); let b64 = EIS64; a = ebin8raw("xor", a, b); a64 = w64(a64, b64); }
            else { break; }
        }
        EIS64 = a64;
    }
    a
}
fn parse_bitor() -> i32 {
    let mut a = parse_bitxor();
    unsafe {
        let mut a64 = EIS64;
        loop {
            skipws();
            if POS < LEN && at(POS) == 124 && (POS + 1 >= LEN || at(POS + 1) != 124) { POS = POS + 1; let b = parse_bitxor(); let b64 = EIS64; a = ebin8raw("or", a, b); a64 = w64(a64, b64); }
            else { break; }
        }
        EIS64 = a64;
    }
    a
}
/* A4 derive(PartialEq): structs compare field-wise (AND of per-word eq over TSZ), enums compare discriminant.
   neg=1 negates (for !=). Nested aggregate/String fields compare by pointer (a practical-parity limitation). */
fn emit_struct_eq(a: i32, b: i32, ty: i32) -> i32 {
    unsafe {
        let words = TSZ[ty as usize] / 4;
        let mut acc = ec(1);
        let mut i = 0;
        while i < words {
            let off = ec(i * 4);
            let ai = eload(ebin("add", a, off)); let bi = eload(ebin("add", b, off));
            let eqi = ebin("eq", ai, bi); acc = ebin("mul", acc, eqi);
            i = i + 1;
        }
        acc
    }
}
/* operator-trait: derive(Ord/PartialOrd) struct LEXICOGRAPHIC compare a<b (isgt=0) / a>b (isgt=1),
   oreq=1 for <=/>= (equal -> true). Branchless: first differing field decides; width-aware (i64 fields). */
fn emit_struct_cmp(a: i32, b: i32, ty: i32, isgt: i32, oreq: i32) -> i32 {
    unsafe {
        let mut res = ec(0);
        let mut decided = ecr(0);
        let mut n = 0;
        loop {
            let fi = fld_nth(ty, n); if fi < 0 { break; }
            let off = ec(FSB[fi as usize]);
            let aad = ebin("add", a, off); let bad = ebin("add", b, off);
            let mut af = 0; let mut bf = 0; let mut is64 = 0;
            if FI64[fi as usize] == 1 { af = eload8(aad); bf = eload8(bad); is64 = 1; } else { af = eload(aad); bf = eload(bad); }
            let ltf = i64op("lt", af, is64, bf, is64, 0);
            let gtf = i64op("gt", af, is64, bf, is64, 0);
            let dirf = if isgt == 1 { gtf } else { ltf };
            let nef = ebin("add", ltf, gtf);
            let notdec = ebin("sub", ec(1), decided);
            let take = ebin("mul", notdec, nef);
            res = ebin("add", res, ebin("mul", take, dirf));
            decided = ebin("add", decided, take);
            n = n + 1;
        }
        let notdec2 = ebin("sub", ec(1), decided);
        let r = ebin("add", res, ebin("mul", notdec2, ec(oreq)));
        r
    }
}
fn parse_cmp() -> i32 {
    let a = parse_bitor();
    unsafe {
        let a64 = EIS64; let a_typ = EITYP; let a_enum = EISENUM; let a_enumty = EIENUMTY; let au = EISU; let a_str = EISSTR; let a_string = EISSTRING; let a_opt = EISOPT;   /* TI3e-k: capture operand-a string-kind before parsing b clobbers the flags */
        if eat2(61, 61) {
            let bb = parse_bitor(); let b64 = EIS64; let b_typ = EITYP; let b_enum = EISENUM; let b_str = EISSTR; let b_string = EISSTRING; let b_opt = EISOPT;
            if (a_str == 1 || a_string == 1) && (b_str == 1 || b_string == 1) && a_enum == 0 && b_enum == 0 { let r = emit_streq_inline(a, bb); EISBOOL = 1; return r; }   /* TI3e-k: `&str`/`String` `==` -> byte-equality (was a descriptor-pointer compare -> always false). LAYER6 P-DERIVE-EQ-STRING-PAYLOAD: gate on !enum -- `let a = T::Id(String::from(..))` leaks EISSTRING from the payload arg onto the enum-tagged value, so an enum operand must fall through to the enum-eq branch below, not byte-compare the enum CELL pointer (load(cell)=disc=0 -> NULL buf -> segfault). */
            if a_opt == 1 && b_opt == 1 { let r = emit_opt_eq(a, bb); EISBOOL = 1; return r; }   /* STATEFUL-ITER: `Option == Option` (e.g. it.peek()==Some(&&x)) -> structural (disc + scalar payload), was a cell-pointer compare -> always false */
            if a_enum == 1 || b_enum == 1 {   /* LAYER5 P-DERIVE-RECURSIVE-EQ: payload-carrying enum -> structural `<Enum>_eq`; payload-less -> disc-only; non-derivable payload -> honest REJECT (no silent miscompile) */
                if a_enum == 1 && a_enumty >= 0 && enum_has_payload(a_enumty) == 1 {
                    if enum_eq_derivable(a_enumty) == 0 || enum_has_user_eq(a_enumty) == 1 { die_unsupported(ENO[a_enumty as usize], ENL[a_enumty as usize]); }
                    let r = emit_enum_eq_call(a_enumty, a, bb); EISBOOL = 1; EISENUM = 0; EIENUMTY = 0 - 1; return r;
                }
                let da = eload(a); let db = eload(bb); let r = ebin("eq", da, db); EISBOOL = 1; EISENUM = 0; EIENUMTY = 0 - 1; return r; }   /* enum: disc == disc (payload-less) */   /* AST-2 P-ENUM-EQ-RESULT-TAG-LEAK: clear EISENUM/EIENUMTY -- the `==` RESULT is a bool, not an enum; a leaked EISENUM made `(a==b) as i32` treat the 0/1 as an enum cell -> deref -> SEGFAULT (the direct-bool Display path survived because it prioritizes EISBOOL). */
            if a_typ >= 0 && b_typ >= 0 && a_typ != b_typ { die_mismatch(); }   /* R2bn: comparing two DIFFERENT struct types -> E0308 REJECT */
            if a_typ >= 0 && b_typ >= 0 && a_typ == b_typ { if TPEQ[a_typ as usize] == 0 { die_unsupported(TSO[a_typ as usize], TSL[a_typ as usize]); } let r = emit_struct_eq(a, bb, a_typ); EISBOOL = 1; return r; }               /* struct: field-wise (R2: reject if no #[derive(PartialEq)]) */
            let bu = EISU; if au == 1 && bu == 1 { WUNS = 1; } let r = i64op("eq", a, a64, bb, b64, 0); EISBOOL = 1; return r;
        }
        if eat2(33, 61) {
            let bb = parse_bitor(); let b64 = EIS64; let b_typ = EITYP; let b_enum = EISENUM; let b_str = EISSTR; let b_string = EISSTRING; let b_opt = EISOPT;
            if (a_str == 1 || a_string == 1) && (b_str == 1 || b_string == 1) && a_enum == 0 && b_enum == 0 { let eqd = emit_streq_inline(a, bb); let z = ecr(0); let r = ebin("eq", eqd, z); EISBOOL = 1; return r; }   /* TI3e-k: `&str`/`String` `!=` -> negated byte-equality. LAYER6: gate on !enum (see the `==` branch). */
            if a_opt == 1 && b_opt == 1 { let eqd = emit_opt_eq(a, bb); let z = ecr(0); let r = ebin("eq", eqd, z); EISBOOL = 1; return r; }   /* STATEFUL-ITER: `Option != Option` */
            if a_enum == 1 || b_enum == 1 {   /* LAYER5 P-DERIVE-RECURSIVE-EQ: `!=` mirrors `==` (negated) */
                if a_enum == 1 && a_enumty >= 0 && enum_has_payload(a_enumty) == 1 {
                    if enum_eq_derivable(a_enumty) == 0 || enum_has_user_eq(a_enumty) == 1 { die_unsupported(ENO[a_enumty as usize], ENL[a_enumty as usize]); }
                    let eqd = emit_enum_eq_call(a_enumty, a, bb); let z = ecr(0); let r = ebin("eq", eqd, z); EISBOOL = 1; EISENUM = 0; EIENUMTY = 0 - 1; return r;
                }
                let da = eload(a); let db = eload(bb); let eqd = ebin("eq", da, db); let z = ecr(0); let r = ebin("eq", eqd, z); EISBOOL = 1; EISENUM = 0; EIENUMTY = 0 - 1; return r; }   /* AST-2 P-ENUM-EQ-RESULT-TAG-LEAK (!= arm): the `!=` result is a bool -- clear the enum tag (see the `==` arm) */
            if a_typ >= 0 && b_typ >= 0 && a_typ != b_typ { die_mismatch(); }   /* R2bn: comparing two DIFFERENT struct types -> E0308 REJECT */
            if a_typ >= 0 && b_typ >= 0 && a_typ == b_typ { if TPEQ[a_typ as usize] == 0 { die_unsupported(TSO[a_typ as usize], TSL[a_typ as usize]); } let eqd = emit_struct_eq(a, bb, a_typ); let z = ecr(0); let r = ebin("eq", eqd, z); EISBOOL = 1; return r; }
            let bu = EISU; if au == 1 && bu == 1 { WUNS = 1; } let r = i64op("ne", a, a64, bb, b64, 0); EISBOOL = 1; return r;
        }
        if eat2(60, 61) { let bb = parse_bitor(); let b64 = EIS64; let bu = EISU; let bty = EITYP; if a_typ >= 0 && a_typ == bty { let r = emit_struct_cmp(a, bb, a_typ, 0, 1); EISBOOL = 1; return r; } let un = if au == 1 && bu == 1 { 1 } else { 0 }; let opn = if un == 1 { "leu" } else { "le" }; WUNS = un; let r = i64op(opn, a, a64, bb, b64, 0); EISBOOL = 1; return r; }
        if eat2(62, 61) { let bb = parse_bitor(); let b64 = EIS64; let bu = EISU; let bty = EITYP; if a_typ >= 0 && a_typ == bty { let r = emit_struct_cmp(a, bb, a_typ, 1, 1); EISBOOL = 1; return r; } let un = if au == 1 && bu == 1 { 1 } else { 0 }; let opn = if un == 1 { "geu" } else { "ge" }; WUNS = un; let r = i64op(opn, a, a64, bb, b64, 0); EISBOOL = 1; return r; }
        let c = peek();
        if c == 60 { POS = POS + 1; let bb = parse_bitor(); let b64 = EIS64; let bu = EISU; let bty = EITYP; if a_typ >= 0 && a_typ == bty { let r = emit_struct_cmp(a, bb, a_typ, 0, 0); EISBOOL = 1; return r; } let un = if au == 1 && bu == 1 { 1 } else { 0 }; let opn = if un == 1 { "ltu" } else { "lt" }; WUNS = un; let r = i64op(opn, a, a64, bb, b64, 0); EISBOOL = 1; return r; }
        if c == 62 { POS = POS + 1; let bb = parse_bitor(); let b64 = EIS64; let bu = EISU; let bty = EITYP; if a_typ >= 0 && a_typ == bty { let r = emit_struct_cmp(a, bb, a_typ, 1, 0); EISBOOL = 1; return r; } let un = if au == 1 && bu == 1 { 1 } else { 0 }; let opn = if un == 1 { "gtu" } else { "gt" }; WUNS = un; let r = i64op(opn, a, a64, bb, b64, 0); EISBOOL = 1; return r; }
    }
    a
}
/* && and || via short-circuit result slots */
fn parse_and() -> i32 {
    let mut a = parse_cmp();
    unsafe {
        while eat2(38, 38) {
            let res = ec(0);
            println!("  brz v{}", a); ckreset();
            let b = parse_cmp();
            let one = ecr(1); let zz = ecr(0);
            let nb = ebin("ne", b, zz);
            ecopy(res, nb);
            println!("  pool"); ckreset();
            let dummy = one;
            a = res;
        }
    }
    a
}
fn parse_expr_impl() -> i32 {
    let mut a = parse_and();
    unsafe {
        while eat2(124, 124) {
            let res = ec(1);
            let zz = ecr(0);
            let na = ebin("eq", a, zz);
            println!("  brz v{}", na); ckreset();
            let b = parse_and();
            let zz2 = ecr(0);
            let nb = ebin("ne", b, zz2);
            ecopy(res, nb);
            println!("  pool"); ckreset();
            a = res;
        }
    }
    a
}
fn parse_expr() -> i32 { let r = parse_expr_impl(); unsafe { let p = EITY_PIN; EITY = ty_of_flags(); if p >= 0 { EITY = p; } EITY_PIN = 0 - 1; } r }   /* TI1 + FLAG-RETIRE inversion scaffold: EITY is normally derived from the flags (ty_of_flags), but a producer that has already set the value's interned Ty directly can PIN it (EITY_PIN = t) so parse_expr does not clobber it. This is the transition seam that makes EITY producer-authoritative -- the prerequisite for deleting a flag (whose write currently feeds ty_of_flags). Inert until a producer pins. */

/* emit a print!/println! (macro name already consumed); eats through the closing `)` but NOT a trailing `;`
   so it works both as a statement and as a match-arm body. */
fn emit_push_span(dst: i32, o: i32, l: i32) { unsafe { let mut j = 0; while j < l { let cv = ec(at(o + j)); emit_str_push_byte(dst, cv); j = j + 1; } } }
/* long-tail: {:?} Debug for a struct -> `Name { f: v, g: w }` (scalars via __push_int, str fields quoted,
   nested structs recurse). */
fn emit_struct_debug(dst: i32, ptr: i32, ty: i32) {
    unsafe {
        emit_push_span(dst, TSO[ty as usize], TSL[ty as usize]);
        let sp = ecr(32); emit_str_push_byte(dst, sp); let lb = ecr(123); emit_str_push_byte(dst, lb); let sp2 = ecr(32); emit_str_push_byte(dst, sp2);
        let mut first = 1; let mut i = 0;
        while i < NFLD {
            if FSST[i as usize] == ty {
                if first == 0 { let cm = ecr(44); emit_str_push_byte(dst, cm); let s3 = ecr(32); emit_str_push_byte(dst, s3); }
                first = 0;
                emit_push_span(dst, FSO[i as usize], FSL[i as usize]);
                let co = ecr(58); emit_str_push_byte(dst, co); let s4 = ecr(32); emit_str_push_byte(dst, s4);
                let off = ec(FSB[i as usize]); let val = eload(ebin("add", ptr, off));
                let fk = FFK[i as usize]; let fty = FSTY[i as usize];
                if fk == 3 || fk == 1 { let q = ecr(34); emit_str_push_byte(dst, q); emit_str_push_str(dst, val); emit_str_push_byte(dst, q); }
                else if fty >= 0 { emit_struct_debug(dst, val, fty); }
                else if FSENL[i as usize] > 0 && enum_find(FSEN[i as usize], FSENL[i as usize]) >= 0 { emit_enum_dbg_body(dst, val, enum_find(FSEN[i as usize], FSENL[i as usize]), 1); }   /* RES-3 P-STRUCT-ENUM-FIELD-DEBUG: an ENUM field in a derived-Debug struct -> inline the enum's Debug body (prints the variant name) instead of __push_int (the cell pointer). viacall=1 recurses <Enum>_dbg for any Box<ThisEnum> sub-field. Mirrors the fty>=0 struct-field recursion + the enum-variant Box-field dbg. */
                else { println!("  v{}: i32 = call __push_int v{} v{}", NC, dst, val); NC = NC + 1; }
            }
            i = i + 1;
        }
        let s5 = ecr(32); emit_str_push_byte(dst, s5); let rb = ecr(125); emit_str_push_byte(dst, rb);
    }
}
/* long-tail: {:?} Debug for an enum cell -> `Variant` or `Variant(payload)` (runtime disc -> name switch).
   viacall: for a Box<ThisEnum> (VFKIND==1) recursive field, recurse `call <Enum>_dbg dst child` instead of
   printing the child pointer (used inside the synthetic <Enum>_dbg fn -- inlining would infinitely recurse the compiler). */
fn emit_enum_dbg_body(dst: i32, cell: i32, en: i32, viacall: i32) {
    unsafe {
        let disc = eload(cell);
        let mut v = 0;
        while v < NVAR {
            if VEN[v as usize] == en {
                let dv = ec(VDISC[v as usize]); let m = ebin("eq", disc, dv);
                println!("  brz v{}", m); ckreset();
                emit_push_span(dst, VO[v as usize], VL[v as usize]);
                if VHAS[v as usize] == 1 {
                    let nf = VNF[v as usize]; let isstruct = VSTRUCT[v as usize];
                    if isstruct == 1 { let sp = ecr(32); emit_str_push_byte(dst, sp); let lb2 = ecr(123); emit_str_push_byte(dst, lb2); let sp2 = ecr(32); emit_str_push_byte(dst, sp2); }   /* " { " */
                    else { let lp = ecr(40); emit_str_push_byte(dst, lp); }   /* "(" */
                    let mut fi = 0;
                    while fi < nf {
                        if fi > 0 { let cm = ecr(44); emit_str_push_byte(dst, cm); let spc = ecr(32); emit_str_push_byte(dst, spc); }   /* ", " */
                        if isstruct == 1 { emit_push_span(dst, VFNO[(v * 8 + fi) as usize], VFNL[(v * 8 + fi) as usize]); let co = ecr(58); emit_str_push_byte(dst, co); let spc2 = ecr(32); emit_str_push_byte(dst, spc2); }   /* "name: " */
                        let pv = if vpay_i64(v, fi) == 1 { eload8(ebin("add", cell, ec(vpay_off(v, fi)))) } else { eload(ebin("add", cell, ec(vpay_off(v, fi)))) };   /* P-ENUM-I64-PAYLOAD-OFFSET: width- and offset-correct field read */
                        if VFKIND[(v * 8 + fi) as usize] == 1 && viacall == 1 { print!("  v{}: i32 = call ", NC); emit_span(ENO[en as usize], ENL[en as usize]); print!("_dbg v{} v{}", dst, pv); println!(""); NC = NC + 1; }   /* LAYER5 P-DERIVE-DEBUG: Box<ThisEnum> recursive field -> recurse the synthetic <Enum>_dbg (was __push_int -> child pointer) */
                        else if VFSTR[(v * 8 + fi) as usize] == 1 { let q = ecr(34); emit_str_push_byte(dst, q); emit_str_push_str(dst, pv); emit_str_push_byte(dst, q); }   /* R34: String/&str payload -> quoted */
                        else { println!("  v{}: i32 = call __push_int v{} v{}", NC, dst, pv); NC = NC + 1; }
                        fi = fi + 1;
                    }
                    if isstruct == 1 { let sp3 = ecr(32); emit_str_push_byte(dst, sp3); let rb2 = ecr(125); emit_str_push_byte(dst, rb2); }   /* " }" */
                    else { let rp = ecr(41); emit_str_push_byte(dst, rp); }   /* ")" */
                }
                println!("  pool"); ckreset();
            }
            v = v + 1;
        }
    }
}
fn emit_enum_debug(dst: i32, cell: i32, en: i32) {   /* {:?} site: a recursive enum routes to the synthetic <Enum>_dbg (runtime recursion); a flat enum inlines the body */
    unsafe {
        if enum_is_recursive(en) == 1 { print!("  v{}: i32 = call ", NC); emit_span(ENO[en as usize], ENL[en as usize]); print!("_dbg v{} v{}", dst, cell); println!(""); NC = NC + 1; return; }
        emit_enum_dbg_body(dst, cell, en, 0);
    }
}
fn emit_enum_dbg_fn(en: i32) {   /* LAYER5 P-DERIVE-DEBUG: synthesize `fn <Enum>_dbg(dst, cell)->i32` -- appends the Debug rendering of the enum cell into the String builder dst, recursing on Box<ThisEnum> fields */
    unsafe {
        NC = 0; NSYM = 0; ckreset();
        print!("fn "); emit_span(ENO[en as usize], ENL[en as usize]); print!("_dbg"); println!("");
        println!("  v0: i32 = param"); println!("  v1: i32 = param");
        NC = 2;
        emit_enum_dbg_body(0, 1, en, 1);
        let z = ecr(0); println!("  ret v{}", z);
    }
}
/* S2: push arg #idx into String dst formatted per ftype (0 Display, x/X/b/o radix, ? Debug). */
fn emit_push_val(dst: i32, idx: i32, ftype: i32) {
    unsafe {
        if ftype == 63 && AITYP[idx as usize] >= 0 { emit_struct_debug(dst, AV[idx as usize], AITYP[idx as usize]); return; }   /* {:?} on a struct */
        if ftype == 63 && AITYP[idx as usize] < 0 - 1 { emit_enum_debug(dst, AV[idx as usize], 0 - AITYP[idx as usize] - 2); return; }   /* {:?} on an enum */
        if AS[idx as usize] == 1 {
            if ftype == 63 { let q = ecr(34); emit_str_push_byte(dst, q); emit_str_push_str(dst, AV[idx as usize]); emit_str_push_byte(dst, q); }
            else { emit_str_push_str(dst, AV[idx as usize]); }
        }
        else if AC[idx as usize] == 1 {
            if ftype == 63 { let q = ecr(39); emit_str_push_byte(dst, q); emit_str_push_byte(dst, AV[idx as usize]); emit_str_push_byte(dst, q); }
            else { emit_str_push_byte(dst, AV[idx as usize]); }
        }
        else if AB[idx as usize] == 1 {
            let v = AV[idx as usize]; let lb = LBL; LBL = LBL + 1;
            println!("  brz v{}", v); ckreset();
            let t = ec(116); emit_str_push_byte(dst, t); let r = ec(114); emit_str_push_byte(dst, r); let u = ecr(117); emit_str_push_byte(dst, u); let e = ecr(101); emit_str_push_byte(dst, e);
            println!("  goto __Lf{}", lb); ckreset(); println!("  pool"); ckreset();
            let f = ec(102); emit_str_push_byte(dst, f); let a2 = ecr(97); emit_str_push_byte(dst, a2); let l2 = ecr(108); emit_str_push_byte(dst, l2); let s2 = ecr(115); emit_str_push_byte(dst, s2); let e2 = ecr(101); emit_str_push_byte(dst, e2);
            println!("  label __Lf{}", lb); ckreset();
        }
        else if ftype == 120 { let b = ecr(16); let u = ecr(0); println!("  v{}: i32 = call __push_radix v{} v{} v{} v{}", NC, dst, AV[idx as usize], b, u); NC = NC + 1; }
        else if ftype == 88 { let b = ecr(16); let u = ecr(1); println!("  v{}: i32 = call __push_radix v{} v{} v{} v{}", NC, dst, AV[idx as usize], b, u); NC = NC + 1; }
        else if ftype == 98 { let b = ecr(2); let u = ecr(0); println!("  v{}: i32 = call __push_radix v{} v{} v{} v{}", NC, dst, AV[idx as usize], b, u); NC = NC + 1; }
        else if ftype == 111 { let b = ecr(8); let u = ecr(0); println!("  v{}: i32 = call __push_radix v{} v{} v{} v{}", NC, dst, AV[idx as usize], b, u); NC = NC + 1; }
        else if A64[idx as usize] == 1 { if AU[idx as usize] == 1 { println!("  v{}: i32 = call __push_uint64 v{} v{}", NC, dst, AV[idx as usize]); } else { println!("  v{}: i32 = call __push_int64 v{} v{}", NC, dst, AV[idx as usize]); } NC = NC + 1; }
        else { println!("  v{}: i32 = call __push_int v{} v{}", NC, dst, AV[idx as usize]); NC = NC + 1; }
    }
}
fn emit_pad(dst: i32, count: i32, fill: i32) {   /* push `fill` `count` times (count/fill are vNs; count<=0 -> nothing) */
    unsafe {
        let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
        println!("  loop"); ckreset();
        let cnd = ebin("lt", i, count); println!("  brz v{}", cnd); ckreset();
        emit_str_push_byte(dst, fill);
        println!("  cont"); ckreset(); let one = ecr(1); let ni = ebin("add", i, one); ecopy(i, ni);
        println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
    }
}
/* S2: format!("lit {} lit", args..) -> a String; supports {N} positional, {:x/X/b/o} radix, {:?} debug,
   width + fill + align (<>^) + zero-pad, i64 args; precision {.N} is skipped. {{ }} are literal braces. */
fn emit_format() -> i32 {
    unsafe {
        eat_ch(33); eat_ch(40); skipws(); eat_ch(34);
        let fo = POS;
        while POS < LEN && at(POS) != 34 { if at(POS) == 92 { POS = POS + 1; } POS = POS + 1; }
        let fl = POS - fo; POS = POS + 1;
        let mut na = 0;
        skipws();
        while eat_ch(44) { skipws(); if peek() == 41 { break; } let a = parse_expr(); AV[na as usize] = a; AS[na as usize] = EISSTR; AC[na as usize] = EISCHAR; A64[na as usize] = EIS64; AU[na as usize] = EISU; AB[na as usize] = EISBOOL; AITYP[na as usize] = EITYP; if EISENUM == 1 { AITYP[na as usize] = 0 - (EIENUMTY + 2); } na = na + 1; }
        eat_ch(41);
        let desc = halloc(12); let buf = halloc(64); println!("  store v{} v{}", desc, buf); let __cc8=ec(8); println!("  store v{} v{}", ebin("add",desc,__cc8), ec(64));   /* STR-GROW: cap@8 */
        let mut hole = 0;
        let mut k = fo;
        while k < fo + fl {
            let c = at(k);
            if c == 123 {
                if k + 1 < fo + fl && at(k + 1) == 123 { let lb = ecr(123); emit_str_push_byte(desc, lb); k = k + 2; }
                else {
                    k = k + 1;                                                       /* past { */
                    let mut posidx = 0 - 1;
                    while k < fo + fl && is_dig(at(k)) { if posidx < 0 { posidx = 0; } posidx = posidx * 10 + (at(k) - 48); k = k + 1; }
                    if posidx < 0 && k < fo + fl && is_idst(at(k)) {                 /* {name} inline named arg (2021) -> resolve the local, append as an arg */
                        let no = k; while k < fo + fl && is_idc(at(k)) { k = k + 1; } let nl = k - no;
                        let sv = sym_find(no, nl);
                        AV[na as usize] = sv; AS[na as usize] = 0; AC[na as usize] = 0;
                        if sym_kind(no, nl) == 1 || sym_kind(no, nl) == 3 { AS[na as usize] = 1; }
                        AB[na as usize] = sym_bool(no, nl); A64[na as usize] = sym_i64(no, nl); AU[na as usize] = sym_u(no, nl); AITYP[na as usize] = 0 - 1;
                        posidx = na; na = na + 1;
                    }
                    let mut ftype = 0; let mut width = 0; let mut fill = 32; let mut align = 0;
                    if k < fo + fl && at(k) == 58 {                                  /* :spec */
                        k = k + 1;
                        if k + 1 < fo + fl && (at(k + 1) == 60 || at(k + 1) == 62 || at(k + 1) == 94) { fill = at(k); align = at(k + 1); k = k + 2; }
                        else if k < fo + fl && (at(k) == 60 || at(k) == 62 || at(k) == 94) { align = at(k); k = k + 1; }
                        if k < fo + fl && at(k) == 48 { fill = 48; k = k + 1; }
                        while k < fo + fl && is_dig(at(k)) { width = width * 10 + (at(k) - 48); k = k + 1; }
                        if k < fo + fl && at(k) == 46 { k = k + 1; while k < fo + fl && is_dig(at(k)) { k = k + 1; } }
                        if k < fo + fl && at(k) != 125 { ftype = at(k); k = k + 1; }
                    }
                    while k < fo + fl && at(k) != 125 { k = k + 1; }
                    k = k + 1;                                                       /* past } */
                    let mut idx = hole; if posidx >= 0 { idx = posidx; } else { hole = hole + 1; }
                    if width == 0 { emit_push_val(desc, idx, ftype); }
                    else {
                        let tmp = halloc(12); let tbuf = halloc(64); println!("  store v{} v{}", tmp, tbuf); let __cc8=ec(8); println!("  store v{} v{}", ebin("add",tmp,__cc8), ec(64));   /* STR-GROW: cap@8 */
                        emit_push_val(tmp, idx, ftype);
                        let c4 = ecr(4); let tlen = eload(ebin("add", tmp, c4));
                        let wc = ec(width); let padn = ebin("sub", wc, tlen);
                        let fillc = ec(fill);
                        let mut al = align; if al == 0 { if AS[idx as usize] == 1 || AC[idx as usize] == 1 || AB[idx as usize] == 1 { al = 60; } else { al = 62; } }
                        if al == 62 { emit_pad(desc, padn, fillc); emit_str_push_str(desc, tmp); }
                        else if al == 60 { emit_str_push_str(desc, tmp); emit_pad(desc, padn, fillc); }
                        else { let two = ecr(2); let half = ebin("div", padn, two); let other = ebin("sub", padn, half); emit_pad(desc, half, fillc); emit_str_push_str(desc, tmp); emit_pad(desc, other, fillc); }
                    }
                }
            } else if c == 125 {
                if k + 1 < fo + fl && at(k + 1) == 125 { let rb = ecr(125); emit_str_push_byte(desc, rb); k = k + 2; } else { k = k + 1; }
            } else if c == 92 && k + 1 < fo + fl { let e = at(k + 1); let mut dc = e; if e == 110 { dc = 10; } else if e == 116 { dc = 9; } else if e == 114 { dc = 13; } else if e == 48 { dc = 0; } let cv = ec(dc); emit_str_push_byte(desc, cv); k = k + 2; }   /* LAYER7 P-FORMAT-STR-ESCAPE: decode a backslash escape in the format-string LITERAL text (\n \t \r \0 -> the control byte; \" \\ \' -> the escaped char) -- mirrors emit_str_literal (1436). Was: the raw `\` byte pushed verbatim, so `format!("\"{}\"",x)` printed `\"..\"` and `\t`/`\n` printed literally. */
            else { let cv = ec(c); emit_str_push_byte(desc, cv); k = k + 1; }
        }
        EIS8 = 0; EISCHAR = 0; EISBOOL = 0; EIS64 = 0; EISU = 0; EISENUM = 0; EISOPT = 0; EISVEC = 0; EISARR = 0; EITYP = 0 - 1;   /* LAYER7 P-FORMAT-ARG-TAG-LEAK: the format! RESULT is a String, not the last arg's value-kind. A `u8`/char/bool/i64/enum LAST arg left its flag set (e.g. `format!("{}", op)` with op:u8 left EIS8=1), so the format! result String was mis-tagged u8 -> a later read of `s` masked the DESCRIPTOR POINTER `& 255` -> a garbage pointer -> __print_str crash. Clear the arg value-kind flags before tagging the result String. Mirrors the struct-literal / enum-ctor tag-leak clears. */
        EISSTRING = 1; EISSTR = 1; desc
    }
}
fn emit_putc_lit(ch: i32) { unsafe { let c = ec(ch); println!("  v{}: i32 = call __putc v{}", NC, c); NC = NC + 1; } }   /* fmt: emit one literal char */
fn emit_dbg_payload(v: i32, pk: i32) {   /* container-kind (a): Option/Result payload for {:?} -- &str quoted, else the integer */
    unsafe {
        if pk == 1 { emit_putc_lit(34); let p = eload(v); let c4 = ecr(4); let ln = eload(ebin("add", v, c4)); println!("  v{}: i32 = call __print_str v{} v{}", NC, p, ln); NC = NC + 1; emit_putc_lit(34); }
        else { println!("  v{}: i32 = call __print_int v{}", NC, v); NC = NC + 1; }
    }
}
fn emit_opt_debug(cell: i32, isres: i32, pk: i32) {   /* container-kind (a): {:?} on Option/Result -> Some(x)/None/Ok(x)/Err(x) via a runtime disc check */
    unsafe {
        let disc = eload(cell); let c4 = ecr(4); let payload = eload(ebin("add", cell, c4));
        let lb = LBL; LBL = LBL + 1;
        println!("  brz v{}", disc); ckreset();
        if isres == 1 { emit_putc_lit(79); emit_putc_lit(107); } else { emit_putc_lit(83); emit_putc_lit(111); emit_putc_lit(109); emit_putc_lit(101); }
        emit_putc_lit(40); emit_dbg_payload(payload, pk); emit_putc_lit(41);
        println!("  goto __Lod{}", lb); ckreset(); println!("  pool"); ckreset();
        if isres == 1 { emit_putc_lit(69); emit_putc_lit(114); emit_putc_lit(114); emit_putc_lit(40); emit_dbg_payload(payload, pk); emit_putc_lit(41); }
        else { emit_putc_lit(78); emit_putc_lit(111); emit_putc_lit(110); emit_putc_lit(101); }
        println!("  label __Lod{}", lb); ckreset();
    }
}
fn emit_sign_pos(av: i32) { unsafe { let z = ecr(0); let ge = ebin("ge", av, z); println!("  brz v{}", ge); ckreset(); emit_putc_lit(43); println!("  pool"); ckreset(); } }   /* fmt: `+` -> emit '+' when the value is non-negative (a `-` already comes from the int printer) */
/* SOFTFLOAT: print a fixed-point f64 (scaled i64 = value*1e9). prec<0 -> default (trim trailing frac
   zeros); prec>=0 -> round to N decimals. Positive values only (no sign handling -- corpus carve-out). */
fn emit_fixed_float(av: i32, prec: i32) {
    unsafe {
        if prec >= 0 {
            let mut dv: i64 = 1; let mut k = 0; while k < 9 - prec { dv = dv * 10; k = k + 1; }
            let divisor = emit_i64_lit(dv); let half = emit_i64_lit(dv / 2);
            let avph = ebin8(0, av, half);                 /* av + half (unnested -- nested calls mis-pass NC) */
            let rup = ebin8(3, avph, divisor);             /* round-half-up = (av+half)/divisor */
            /* SOFTFLOAT: correct half-UP -> half-to-EVEN (Rust's Display/`{:.N}` uses banker's rounding). On an
               EXACT tie -- the dropped remainder `av % divisor` equals `dv/2` -- with an even quotient, round DOWN. */
            let rem = ebin8(4, av, divisor);
            println!("  v{}: i32 = conv v{}", NC, rem); let remc = NC; NC = NC + 1;   /* inline conv (the econv32 CALL mis-passes its arg through the self-compile) */
            let tie = ebin("eq", remc, ec((dv / 2) as i32));
            let two = emit_i64_lit(2); let q = ebin8(3, av, divisor); let qm = ebin8(4, q, two);
            println!("  v{}: i32 = conv v{}", NC, qm); let qmc = NC; NC = NC + 1;
            let qeven = ebin("eq", qmc, ec(0));
            let cond = ebin("and", tie, qeven);
            let one = emit_i64_lit(1); let rdec = ebin8(1, rup, one);
            let rounded = NC; NC = NC + 1; ecopy64(rounded, rup);
            println!("  brz v{}", cond); ckreset();                   /* cond!=0 (tie & even) -> use the decremented (round-down) value */
            ecopy64(rounded, rdec);
            println!("  pool"); ckreset();
            let mut pw: i64 = 1; let mut k2 = 0; while k2 < prec { pw = pw * 10; k2 = k2 + 1; }
            let pow = emit_i64_lit(pw);
            let ipart = ebin8(3, rounded, pow);
            println!("  v{}: i32 = call __print_int64 v{}", NC, ipart); NC = NC + 1;
            if prec > 0 {
                emit_putc_lit(46);
                let fmod = ebin8(4, rounded, pow);
                println!("  v{}: i32 = conv v{}", NC, fmod); let frac = NC; NC = NC + 1;
                let cprec = ec(prec);
                println!("  v{}: i32 = call __print_padz v{} v{}", NC, frac, cprec); NC = NC + 1;
            }
        } else {
            let e9 = emit_i64_lit(1000000000);
            let ipart = ebin8(3, av, e9);
            println!("  v{}: i32 = call __print_int64 v{}", NC, ipart); NC = NC + 1;
            let fmod = ebin8(4, av, e9);
            println!("  v{}: i32 = conv v{}", NC, fmod); let frac = NC; NC = NC + 1;
            let z = ecr(0); let gt = ebin("gt", frac, z);
            println!("  brz v{}", gt); ckreset();
            emit_putc_lit(46);
            let c9 = ecr(9); println!("  v{}: i32 = call __strip_frac v{} v{}", NC, frac, c9); NC = NC + 1;
            println!("  pool"); ckreset();
        }
    }
}
fn emit_print(nlf: bool) {
    unsafe {
        eat_ch(33); eat_ch(40); skipws();
        if peek() == 41 { eat_ch(41); if nlf { let cv = ecr(10); println!("  v{}: i32 = call __putc v{}", NC, cv); NC = NC + 1; } return; }   /* TI3e: empty print!()/println!() -- no format string; emit just the newline (println) or nothing (print). Was: eat_ch(34) missed the absent quote, so the format scan read the trailing source `); }` as literal chars. */
        eat_ch(34);
        let fo = POS;
        while POS < LEN && at(POS) != 34 { if at(POS) == 92 { POS = POS + 1; } POS = POS + 1; }
        let fl = POS - fo; POS = POS + 1;
        let mut na = 0;
        skipws();
        while eat_ch(44) { skipws(); if peek() == 41 { break; } if const_ovf_at() == 1 { die_overflow(); } VELT = 0 - 1; let a = parse_expr();   /* CONSTEVAL R2: a const-foldable print arg that overflows i32 -> REJECT */ AV[na as usize] = a; AS[na as usize] = EISSTR; AC[na as usize] = EISCHAR; A64[na as usize] = EIS64; AU[na as usize] = EISU; AB[na as usize] = EISBOOL; AITYP[na as usize] = EITYP; AVEC[na as usize] = EISVEC; AVELT[na as usize] = VELT; AVELTY[na as usize] = DVELTY; AVELENUM[na as usize] = DVELENUM; ATUP[na as usize] = if TYTAG[EITY as usize] == 11 { 1 } else { 0 }; ATUPN[na as usize] = ETUPN; ATUPTY[na as usize] = ETUPTY; AOPT[na as usize] = EISOPT; AOPTRES[na as usize] = EOPTRES; AOPTK[na as usize] = EOPTK; AFLT[na as usize] = EISFLT; if EISENUM == 1 { AITYP[na as usize] = 0 - (EIENUMTY + 2); } na = na + 1; }
        eat_ch(41);
        let mut hole = 0;
        let mut k = fo;
        while k < fo + fl {
            let c = at(k);
            if c == 123 && k + 1 < fo + fl && at(k + 1) == 123 { emit_putc_lit(123); k = k + 2; }   /* LAYER10 P-FORMAT-BRACE-ESCAPE: `{{` -> a literal `{` (was: treated `{{` as a placeholder -> consumed arg 0 / printed garbage). emit_format (the format!->String path) already handled this; the println!/print! path did not. */
            else if c == 125 && k + 1 < fo + fl && at(k + 1) == 125 { emit_putc_lit(125); k = k + 2; }   /* LAYER10 P-FORMAT-BRACE-ESCAPE: `}}` -> a literal `}` (was: printed `}}` verbatim). */
            else if c == 123 {
                k = k + 1; let mut isdebug = 0; let mut ishex = 0;
                let namestart = k;                                                 /* R1: inline named arg `{x}` / `{x:>5}` -- collect the leading identifier */
                while k < fo + fl && at(k) != 125 && at(k) != 58 && is_idc(at(k)) { k = k + 1; }
                let mut inline_name = 0; if k - namestart > 0 && is_idst(at(namestart)) { inline_name = 1; }
                let mut fwidth = 0; let mut fpad = 32; let mut fright = 0; let mut sawcolon = 0;   /* R6: width / pad char (space default) / right-align / after `:` */
                let mut isbin = 0; let mut isoct = 0;                              /* R2: {:b} binary / {:o} octal */
                let mut fsign = 0; let mut falt = 0; let mut inprec = 0; let mut ishexup = 0; let mut isexp = 0;   /* fmt: `+` force sign / `#` alt prefix / `.N` precision (ignored for int) / `X` uppercase hex */
                let mut fprec = 0 - 1;   /* SOFTFLOAT: `.N` precision value (-1 = none) */
                let mut fleft = 0;   /* fmt: `<` left-align (pad AFTER the value) */
                let mut fwarg = 0; let mut fwargidx = 0;   /* fmt: `{:N$}` -- width taken from positional arg N at runtime (not a literal) */
                while k < fo + fl && at(k) != 125 {
                    let ch = at(k);
                    if ch == 63 { isdebug = 1; }
                    else if ch == 120 { ishex = 1; }
                    else if ch == 88 { ishex = 1; ishexup = 1; }                   /* `X` -> uppercase hex */
                    else if ch == 98 && sawcolon == 1 { isbin = 1; }               /* `b` after `:` -> binary */
                    else if ch == 111 && sawcolon == 1 { isoct = 1; }              /* `o` after `:` -> octal */
                    else if ch == 101 && sawcolon == 1 { isexp = 1; }              /* P-FLOAT R1: e */
                    else if ch == 58 { sawcolon = 1; }                             /* `:` */
                    else if ch == 43 && sawcolon == 1 { fsign = 1; }               /* fmt: `+` force sign on non-negative */
                    else if ch == 35 && sawcolon == 1 { falt = 1; }                /* fmt: `#` alternate form (0x/0b/0o prefix) */
                    else if ch == 46 && sawcolon == 1 { inprec = 1; fprec = 0; }    /* fmt: `.` -> precision follows */
                    else if ch >= 48 && ch <= 57 && inprec == 1 { fprec = fprec * 10 + (ch - 48); }   /* SOFTFLOAT: precision digits `.N` */
                    else if sawcolon == 1 {
                        if ch == 62 { fright = 1; }                                /* `>` right-align */
                        else if ch == 60 { fleft = 1; }                           /* `<` left-align (pad after) */
                        else if ch == 94 { }                                      /* `^` center -- unsupported (pad-before) */
                        else if ch == 48 && fwidth == 0 && inprec == 0 { fpad = 48; }             /* leading `0` -> zero-pad */
                        else if ch >= 48 && ch <= 57 && inprec == 0 { fwidth = fwidth * 10 + (ch - 48); }   /* width digit (not precision) */
                        else if ch == 36 && inprec == 0 { fwargidx = fwidth; fwidth = 0; fwarg = 1; }   /* fmt: `{:N$}` -- the digits were an ARG INDEX, not a literal width; take the width from arg N at runtime */
                    }
                    k = k + 1;
                }
                k = k + 1;
                if inline_name == 1 {                                              /* R1: evaluate the inline-named local into this hole's arg slot */
                    let sp = POS; POS = namestart; let iv = parse_atom();
                    AV[hole as usize] = iv; AS[hole as usize] = EISSTR; AC[hole as usize] = EISCHAR; A64[hole as usize] = EIS64; AU[hole as usize] = EISU; AB[hole as usize] = EISBOOL; AITYP[hole as usize] = EITYP;
                    if EISENUM == 1 { AITYP[hole as usize] = 0 - (EIENUMTY + 2); }
                    POS = sp;
                }
                if isdebug == 1 && AOPT[hole as usize] == 1 {                       /* container-kind (a): {:?} on Option/Result -> Some(x)/None/Ok(x)/Err(x) */
                    { let ak = AOPTK[hole as usize]; let pk = if ak == 1 || ak == 3 { 1 } else { 0 }; emit_opt_debug(AV[hole as usize], AOPTRES[hole as usize], pk); }   /* DBG: quote a &str/String Option payload */
                } else if isdebug == 1 && AVEC[hole as usize] == 1 && AVELT[hole as usize] >= 0 {   /* DBG: {:?} on Vec<Struct> -> [P { .. }, P { .. }] (runtime loop, emit_struct_debug per elem) */
                    let ty = AVELT[hole as usize];
                    let d = AV[hole as usize]; let buf = eload(d); let c4 = ecr(4); let ln = eload(ebin("add", d, c4));
                    let tmp = halloc(12); let tbuf = halloc(65536); println!("  store v{} v{}", tmp, tbuf);   /* empty String [ptr,len=0] */ let __cc8=ec(8); println!("  store v{} v{}", ebin("add",tmp,__cc8), ec(65536));   /* STR-GROW: cap@8 */
                    let lb = ecr(91); emit_str_push_byte(tmp, lb);
                    let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
                    println!("  loop"); ckreset(); let cnd = ebin("lt", i, ln); println!("  brz v{}", cnd); ckreset();
                    let z2 = ecr(0); let gt = ebin("gt", i, z2); println!("  brz v{}", gt); ckreset();
                    let cm = ecr(44); emit_str_push_byte(tmp, cm); let sp = ecr(32); emit_str_push_byte(tmp, sp);
                    println!("  pool"); ckreset();
                    let elem = eload(ebin("add", buf, ebin("mul", i, c4)));
                    emit_struct_debug(tmp, elem, ty);
                    println!("  cont"); ckreset(); let one = ecr(1); ecopy(i, ebin("add", i, one)); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                    let rb = ecr(93); emit_str_push_byte(tmp, rb);
                    let p = eload(tmp); let ln2 = eload(ebin("add", tmp, c4));
                    println!("  v{}: i32 = call __print_str v{} v{}", NC, p, ln2); NC = NC + 1;
                } else if isdebug == 1 && AVEC[hole as usize] == 1 && AVELENUM[hole as usize] >= 0 {   /* R23b: {:?} on a Vec<Enum> -> [A(1), B, ..] (runtime loop, emit_enum_debug per elem) */
                    let en = AVELENUM[hole as usize];
                    let d = AV[hole as usize]; let buf = eload(d); let c4 = ecr(4); let ln = eload(ebin("add", d, c4));
                    let tmp = halloc(12); let tbuf = halloc(65536); println!("  store v{} v{}", tmp, tbuf); let __cc8=ec(8); println!("  store v{} v{}", ebin("add",tmp,__cc8), ec(65536));   /* STR-GROW: cap@8 */
                    let lb = ecr(91); emit_str_push_byte(tmp, lb);
                    let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
                    println!("  loop"); ckreset(); let cnd = ebin("lt", i, ln); println!("  brz v{}", cnd); ckreset();
                    let z2 = ecr(0); let gt = ebin("gt", i, z2); println!("  brz v{}", gt); ckreset();
                    let cm = ecr(44); emit_str_push_byte(tmp, cm); let sp = ecr(32); emit_str_push_byte(tmp, sp);
                    println!("  pool"); ckreset();
                    let elem = eload(ebin("add", buf, ebin("mul", i, c4)));
                    emit_enum_debug(tmp, elem, en);
                    println!("  cont"); ckreset(); let one = ecr(1); ecopy(i, ebin("add", i, one)); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                    let rb = ecr(93); emit_str_push_byte(tmp, rb);
                    let p = eload(tmp); let ln2 = eload(ebin("add", tmp, c4));
                    println!("  v{}: i32 = call __print_str v{} v{}", NC, p, ln2); NC = NC + 1;
                } else if isdebug == 1 && AVEC[hole as usize] == 1 && AVELTY[hole as usize] >= 0 && TYTAG[AVELTY[hole as usize] as usize] == 11 {   /* DBG: {:?} on a Vec<(..)> tuple-element -> [(a, b), (a, b), ..] (was: fell to the scalar Vec printer -> element pointers) */
                    let velty = AVELTY[hole as usize]; let tupid = TYAUX[velty as usize]; let arity = TUPTY_N[tupid as usize];
                    let d = AV[hole as usize]; let buf = eload(d); let c4 = ecr(4); let ln = eload(ebin("add", d, c4));
                    let lb = ecr(91); println!("  v{}: i32 = call __putc v{}", NC, lb); NC = NC + 1;
                    let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
                    println!("  loop"); ckreset(); let cnd = ebin("lt", i, ln); println!("  brz v{}", cnd); ckreset();
                    let z2 = ecr(0); let gt = ebin("gt", i, z2); println!("  brz v{}", gt); ckreset();
                    let cm = ecr(44); println!("  v{}: i32 = call __putc v{}", NC, cm); NC = NC + 1; let sp = ecr(32); println!("  v{}: i32 = call __putc v{}", NC, sp); NC = NC + 1;
                    println!("  pool"); ckreset();
                    let elem = eload(ebin("add", buf, ebin("mul", i, c4)));
                    let lp = ecr(40); println!("  v{}: i32 = call __putc v{}", NC, lp); NC = NC + 1;
                    let mut fj = 0;                                               /* per-field kind (like R10 scalar tuples) so a Vec<(&str,i32)> quotes the str field instead of the int-only __dbg_els printing a pointer */
                    while fj < arity {
                        if fj > 0 { let cm2 = ecr(44); println!("  v{}: i32 = call __putc v{}", NC, cm2); NC = NC + 1; let sp2 = ecr(32); println!("  v{}: i32 = call __putc v{}", NC, sp2); NC = NC + 1; }
                        let fk = tupty_fk(tupid, fj);
                        let fv = eload(ebin("add", elem, ec(fj * 4)));
                        if fk == 1 || fk == 3 {
                            let q = ecr(34); println!("  v{}: i32 = call __putc v{}", NC, q); NC = NC + 1;
                            let fp = eload(fv); let fl2 = eload(ebin("add", fv, c4));
                            println!("  v{}: i32 = call __print_str v{} v{}", NC, fp, fl2); NC = NC + 1;
                            let q2 = ecr(34); println!("  v{}: i32 = call __putc v{}", NC, q2); NC = NC + 1;
                        } else if fk == 5 { println!("  v{}: i32 = call __print_int64 v{}", NC, fv); NC = NC + 1; }
                        else { println!("  v{}: i32 = call __print_int v{}", NC, fv); NC = NC + 1; }
                        fj = fj + 1;
                    }
                    let rp = ecr(41); println!("  v{}: i32 = call __putc v{}", NC, rp); NC = NC + 1;
                    println!("  cont"); ckreset(); let one = ecr(1); ecopy(i, ebin("add", i, one)); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                    let rb = ecr(93); println!("  v{}: i32 = call __putc v{}", NC, rb); NC = NC + 1;
                } else if isdebug == 1 && AVEC[hole as usize] == 1 && AVELTY[hole as usize] >= 0 && (TYTAG[AVELTY[hole as usize] as usize] == 3 || TYTAG[AVELTY[hole as usize] as usize] == 4) {   /* DBG: {:?} on a Vec<&str>/Vec<String> -> ["a", "b", ..] (was: scalar printer -> element descriptor pointers) */
                    let d = AV[hole as usize]; let buf = eload(d); let c4 = ecr(4); let ln = eload(ebin("add", d, c4));
                    let lb = ecr(91); println!("  v{}: i32 = call __putc v{}", NC, lb); NC = NC + 1;
                    let i = NC; let z = ecr(0); ecopy(i, z); NC = NC + 1;
                    println!("  loop"); ckreset(); let cnd = ebin("lt", i, ln); println!("  brz v{}", cnd); ckreset();
                    let z2 = ecr(0); let gt = ebin("gt", i, z2); println!("  brz v{}", gt); ckreset();
                    let cm = ecr(44); println!("  v{}: i32 = call __putc v{}", NC, cm); NC = NC + 1; let sp = ecr(32); println!("  v{}: i32 = call __putc v{}", NC, sp); NC = NC + 1;
                    println!("  pool"); ckreset();
                    let elem = eload(ebin("add", buf, ebin("mul", i, c4)));   /* str descriptor [ptr,len] */
                    let q = ecr(34); println!("  v{}: i32 = call __putc v{}", NC, q); NC = NC + 1;
                    let p = eload(elem); let ln3 = eload(ebin("add", elem, c4));
                    println!("  v{}: i32 = call __print_str v{} v{}", NC, p, ln3); NC = NC + 1;
                    let q2 = ecr(34); println!("  v{}: i32 = call __putc v{}", NC, q2); NC = NC + 1;
                    println!("  cont"); ckreset(); let one = ecr(1); ecopy(i, ebin("add", i, one)); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                    let rb = ecr(93); println!("  v{}: i32 = call __putc v{}", NC, rb); NC = NC + 1;
                } else if isdebug == 1 && AVEC[hole as usize] == 1 {                       /* R2cg: {:?} on a Vec<i32> -> [e0, e1, ..] */
                    let d = AV[hole as usize]; let buf = eload(d); let c4 = ecr(4); let ln = eload(ebin("add", d, c4));
                    let lb = ecr(91); println!("  v{}: i32 = call __putc v{}", NC, lb); NC = NC + 1;
                    let z = ecr(0); println!("  v{}: i32 = call __dbg_els v{} v{} v{}", NC, buf, z, ln); NC = NC + 1;
                    let rb = ecr(93); println!("  v{}: i32 = call __putc v{}", NC, rb); NC = NC + 1;
                } else if isdebug == 1 && ATUP[hole as usize] == 1 {                 /* R2cg: {:?} on a tuple -> (e0, e1, ..) with PER-FIELD kind (was int-only __dbg_els -> &str/String fields printed a pointer) */
                    let cell = AV[hole as usize]; let tupty = ATUPTY[hole as usize]; let arity = ATUPN[hole as usize];
                    let lp = ecr(40); println!("  v{}: i32 = call __putc v{}", NC, lp); NC = NC + 1;
                    let mut fi = 0;
                    while fi < arity {
                        if fi > 0 { let cm = ecr(44); println!("  v{}: i32 = call __putc v{}", NC, cm); NC = NC + 1; let sp = ecr(32); println!("  v{}: i32 = call __putc v{}", NC, sp); NC = NC + 1; }
                        let fk = tupty_fk(tupty, fi);
                        let fv = eload(ebin("add", cell, ec(fi * 4)));
                        if fk == 1 || fk == 3 {                                       /* &str / String field -> quoted */
                            let q = ecr(34); println!("  v{}: i32 = call __putc v{}", NC, q); NC = NC + 1;
                            let c4 = ecr(4); let p = eload(fv); let ln = eload(ebin("add", fv, c4));
                            println!("  v{}: i32 = call __print_str v{} v{}", NC, p, ln); NC = NC + 1;
                            let q2 = ecr(34); println!("  v{}: i32 = call __putc v{}", NC, q2); NC = NC + 1;
                        } else if fk == 5 { println!("  v{}: i32 = call __print_int64 v{}", NC, fv); NC = NC + 1; }   /* i64 field */
                        else { println!("  v{}: i32 = call __print_int v{}", NC, fv); NC = NC + 1; }               /* i32/scalar field */
                        fi = fi + 1;
                    }
                    let rp = ecr(41); println!("  v{}: i32 = call __putc v{}", NC, rp); NC = NC + 1;
                } else if isdebug == 1 && AITYP[hole as usize] >= 0 {                      /* {:?} on a struct -> build into a temp String, print it */
                    let tmp = halloc(12); let tbuf = halloc(64); println!("  store v{} v{}", tmp, tbuf); let __cc8=ec(8); println!("  store v{} v{}", ebin("add",tmp,__cc8), ec(64));   /* STR-GROW: cap@8 */
                    emit_struct_debug(tmp, AV[hole as usize], AITYP[hole as usize]);
                    let p = eload(tmp); let c4 = ecr(4); let ln = eload(ebin("add", tmp, c4));
                    println!("  v{}: i32 = call __print_str v{} v{}", NC, p, ln); NC = NC + 1;
                } else if isdebug == 1 && AITYP[hole as usize] < 0 - 1 {            /* {:?} on an enum */
                    let tmp = halloc(12); let tbuf = halloc(64); println!("  store v{} v{}", tmp, tbuf); let __cc8=ec(8); println!("  store v{} v{}", ebin("add",tmp,__cc8), ec(64));   /* STR-GROW: cap@8 */
                    emit_enum_debug(tmp, AV[hole as usize], 0 - AITYP[hole as usize] - 2);
                    let p = eload(tmp); let c4 = ecr(4); let ln = eload(ebin("add", tmp, c4));
                    println!("  v{}: i32 = call __print_str v{} v{}", NC, p, ln); NC = NC + 1;
                } else if isdebug == 1 && AS[hole as usize] == 1 {                  /* {:?} on &str -> "str" */
                    let q = ecr(34); println!("  v{}: i32 = call __putc v{}", NC, q); NC = NC + 1;
                    let d = AV[hole as usize]; let p = eload(d); let c4 = ecr(4); let ln = eload(ebin("add", d, c4));
                    println!("  v{}: i32 = call __print_str v{} v{}", NC, p, ln); NC = NC + 1;
                    let q2 = ecr(34); println!("  v{}: i32 = call __putc v{}", NC, q2); NC = NC + 1;
                } else if AB[hole as usize] == 1 {                                        /* A4: bool -> print true/false */
                    let v = AV[hole as usize]; let lb = LBL; LBL = LBL + 1;
                    println!("  brz v{}", v); ckreset();
                    let t = ec(116); println!("  v{}: i32 = call __putc v{}", NC, t); NC = NC + 1;
                    let r = ec(114); println!("  v{}: i32 = call __putc v{}", NC, r); NC = NC + 1;
                    let u = ecr(117); println!("  v{}: i32 = call __putc v{}", NC, u); NC = NC + 1;
                    let e = ecr(101); println!("  v{}: i32 = call __putc v{}", NC, e); NC = NC + 1;
                    println!("  goto __Lb{}", lb); ckreset();
                    println!("  pool"); ckreset();
                    let f = ec(102); println!("  v{}: i32 = call __putc v{}", NC, f); NC = NC + 1;
                    let a2 = ecr(97); println!("  v{}: i32 = call __putc v{}", NC, a2); NC = NC + 1;
                    let l2 = ecr(108); println!("  v{}: i32 = call __putc v{}", NC, l2); NC = NC + 1;
                    let s2 = ecr(115); println!("  v{}: i32 = call __putc v{}", NC, s2); NC = NC + 1;
                    let e2 = ecr(101); println!("  v{}: i32 = call __putc v{}", NC, e2); NC = NC + 1;
                    println!("  label __Lb{}", lb); ckreset();
                } else if AS[hole as usize] == 1 {
                    let d = AV[hole as usize];
                    let p = eload(d);
                    let c4 = ecr(4); let da = ebin("add", d, c4); let ln = eload(da);
                    if fright == 1 && fwidth > 0 {                                /* R6: `{:>N}` right-align a &str (pad-before with spaces) */
                        let cw = ec(fwidth); let pad = ebin("sub", cw, ln); let cpad = ec(fpad);
                        print!("  v{}: i32 = call __emit_pad v{} v{}", NC, pad, cpad); println!(""); NC = NC + 1;
                    }
                    println!("  v{}: i32 = call __print_str v{} v{}", NC, p, ln); NC = NC + 1;
                } else if AC[hole as usize] == 1 {
                    println!("  v{}: i32 = call __putc v{}", NC, AV[hole as usize]); NC = NC + 1;
                } else if AFLT[hole as usize] == 1 {
                    emit_fixed_float(AV[hole as usize], fprec);   /* SOFTFLOAT: fixed-point f64 -> decimal (default trim, or `.N` rounded) */
                } else if A64[hole as usize] == 1 && ishex == 0 && isbin == 0 && isoct == 0 {
                    if AU[hole as usize] == 1 { println!("  v{}: i32 = call __print_uint64 v{}", NC, AV[hole as usize]); } else { println!("  v{}: i32 = call __print_int64 v{}", NC, AV[hole as usize]); } NC = NC + 1;   /* i64 hex/bin/oct falls through to the radix printers (32-bit; fits for small i64) */
                } else if isexp == 1 {
                    println!("  v{}: i32 = call __print_exp v{}", NC, AV[hole as usize]); NC = NC + 1;
                } else if ishex == 1 {
                    if fsign == 1 { emit_sign_pos(AV[hole as usize]); }           /* fmt: {:+x} */
                    if falt == 1 { emit_putc_lit(48); emit_putc_lit(120); }       /* fmt: {:#x} -> 0x */
                    if ishexup == 1 { println!("  v{}: i32 = call __print_hexup v{}", NC, AV[hole as usize]); } else { println!("  v{}: i32 = call __print_hex v{}", NC, AV[hole as usize]); } NC = NC + 1;   /* R6: {:x}/{:X} */
                } else if isbin == 1 { if fsign == 1 { emit_sign_pos(AV[hole as usize]); } if falt == 1 { emit_putc_lit(48); emit_putc_lit(98); } let bs = ecr(2); println!("  v{}: i32 = call __print_radix v{} v{}", NC, AV[hole as usize], bs); NC = NC + 1;   /* R2: {:b}, fmt: {:#b} -> 0b */
                } else if isoct == 1 { if fsign == 1 { emit_sign_pos(AV[hole as usize]); } if falt == 1 { emit_putc_lit(48); emit_putc_lit(111); } let bs = ecr(8); println!("  v{}: i32 = call __print_radix v{} v{}", NC, AV[hole as usize], bs); NC = NC + 1;   /* R2: {:o}, fmt: {:#o} -> 0o */
                } else if AU[hole as usize] == 1 {                                 /* value-width: u32 (unsigned 32-bit) -> zero-extend low 32 + unsigned decimal (e.g. wrapping_sub -> 4294967289 not -7) */
                    let ze = emit_conv(AV[hole as usize]);
                    println!("  v{}: i32 = call __print_uint64 v{}", NC, ze); NC = NC + 1;
                } else {
                    let av = AV[hole as usize];
                    if fsign == 1 { emit_sign_pos(av); }                          /* fmt: {:+} -> leading '+' on non-negatives */
                    if (fwidth > 0 || fwarg == 1) && fleft == 0 {                                 /* R6: width/zero-pad on a decimal int (numbers right-align by default -> pad BEFORE) */
                        print!("  v{}: i32 = call __digits v{}", NC, av); println!(""); let dg = NC; NC = NC + 1;
                        let cw = if fwarg == 1 { AV[fwargidx as usize] } else { ec(fwidth) }; let pad = ebin("sub", cw, dg); let cpad = ec(fpad);   /* fmt: {:N$} -> runtime width from arg N */
                        print!("  v{}: i32 = call __emit_pad v{} v{}", NC, pad, cpad); println!(""); NC = NC + 1;
                    }
                    println!("  v{}: i32 = call __print_int v{}", NC, av); NC = NC + 1;
                    if (fwidth > 0 || fwarg == 1) && fleft == 1 {                                 /* fmt: `{:<N}` left-align -> pad AFTER the value (fill = space) */
                        print!("  v{}: i32 = call __digits v{}", NC, av); println!(""); let dg = NC; NC = NC + 1;
                        let cw = if fwarg == 1 { AV[fwargidx as usize] } else { ec(fwidth) }; let pad = ebin("sub", cw, dg); let cpad = ecr(32);
                        print!("  v{}: i32 = call __emit_pad v{} v{}", NC, pad, cpad); println!(""); NC = NC + 1;
                    }
                }
                hole = hole + 1;
            } else if c == 92 && k + 1 < fo + fl { let e = at(k + 1); let mut dc = e; if e == 110 { dc = 10; } else if e == 116 { dc = 9; } else if e == 114 { dc = 13; } else if e == 48 { dc = 0; } let cv = ec(dc); println!("  v{}: i32 = call __putc v{}", NC, cv); NC = NC + 1; k = k + 2;   /* LAYER7 P-FORMAT-STR-ESCAPE (println path): decode a backslash escape in a bare format-string literal (\n \t \r \0 -> control byte; \" \\ \' -> the char) -- was pushed verbatim so `println!("\"x\"")` printed `\"x\"`. Mirrors emit_str_literal + the emit_format literal branch. */
            } else {
                let cv = ec(c);
                println!("  v{}: i32 = call __putc v{}", NC, cv); NC = NC + 1;
                k = k + 1;
            }
        }
        if nlf { let cv = ecr(10); println!("  v{}: i32 = call __putc v{}", NC, cv); NC = NC + 1; }
    }
}

/* K4b: assert! / assert_eq! / assert_ne! -- on failure, write "assertion failed" to stderr and exit 101 */
fn skip_to_rparen() {
    unsafe {
        let mut d = 0;
        while POS < LEN {
            let c = at(POS);
            if c == 40 { d = d + 1; POS = POS + 1; }
            else if c == 41 { if d == 0 { return; } d = d - 1; POS = POS + 1; }
            else { POS = POS + 1; }
        }
    }
}
fn emit_fail_exit() {
    unsafe {
        /* write "assertion failed\n" to stderr, then exit(101) -- rust's panic code. This indexes a b"..."
           byte-slice literal (m[i]/m.len()), which the re-frozen tier-0 now compiles correctly (see the seed's
           ecopy fix); before the re-freeze this broke the selfhost fixed point. */
        let m = b"assertion failed";
        let cur = eload(egref_str_cursor());
        let base = ebin("add", egref_str_arena(), cur);
        let mut i = 0;
        while i < m.len() {
            let ci = ec(i); let ad = ebin("add", base, ci); let cv = ec(m[i]);
            println!("  storeb v{} v{}", ad, cv);
            i = i + 1;
        }
        let cnl = ec(m.len()); let anl = ebin("add", base, cnl); let vnl = ecr(10); println!("  storeb v{} v{}", anl, vnl);
        let c1 = ecr(1); let c2 = ecr(2); let cl = ec(m.len() + 1);
        println!("  v{}: i32 = syscall v{} v{} v{} v{}", NC, c1, c2, base, cl); NC = NC + 1;   /* write(2, base, len+1) */
        emit_flush_call();
        let c60 = ecr(60); let c101 = ecr(101);
        println!("  v{}: i32 = syscall v{} v{}", NC, c60, c101); NC = NC + 1;                  /* exit(101) */
    }
}
fn emit_assert_cmp(faileq: i32) {   /* faileq=1: assert_ne (fail when equal); faileq=0: assert_eq (fail when not equal) */
    unsafe {
        eat_ch(33); eat_ch(40);
        let a = parse_expr(); skipws(); eat_ch(44); let b = parse_expr();
        skip_to_rparen(); eat_ch(41);
        let mut cond = 0;
        if faileq == 1 { cond = ebin("eq", a, b); } else { cond = ebin("ne", a, b); }
        println!("  brz v{}", cond); ckreset();
        emit_fail_exit();
        println!("  pool"); ckreset();
    }
}
fn emit_assert_bool() {
    unsafe {
        eat_ch(33); eat_ch(40);
        let c = parse_expr();
        skip_to_rparen(); eat_ch(41);
        let z = ecr(0);
        let cond = ebin("eq", c, z);   /* 1 when c==0 (assertion false) */
        println!("  brz v{}", cond); ckreset();
        emit_fail_exit();
        println!("  pool"); ckreset();
    }
}

fn parse_stmt() {
    unsafe {
        skipws();
        while POS + 1 < LEN && at(POS) == 35 && at(POS + 1) == 91 {                  /* R-either: `#[allow(..)]` etc -- a STATEMENT-position attribute (was mis-parsed as a `attr(..)` call -> E0425). Skip it (and any stacked attrs). */
            POS = POS + 1; let mut d = 0;
            loop { if POS >= LEN { break; } let c = at(POS);
                if c == 91 { d = d + 1; POS = POS + 1; }
                else if c == 93 { d = d - 1; POS = POS + 1; if d == 0 { break; } }
                else { POS = POS + 1; } }
            skipws();
        }
        { let isv = POS;                                                            /* REACH: a nested ITEM (fn/impl/struct/enum/trait inside a fn body) is prepass-registered; skip its source here rather than mis-parse it as a closure/call expr (produced garbage + a `where F: Fn(..)` bound drifted as `cannot find fn Fn`). Scan (paren/bracket-depth-aware) to the body `{` (past generics/params/where/`-> ret`) then the balanced body, OR a `;`-terminated unit/tuple struct/type. */
            if rd_ident() && (is_kw_fn() || is_kw_impl() || is_kw_struct() || is_kw_enum() || is_kw_trait()) {
                let mut pd = 0;
                loop {
                    skipws();                                                   /* skipws also skips line and block comments so a brace inside a comment does not count */
                    if POS >= LEN { break; }
                    let c = at(POS);
                    if c == 34 || c == 39 { let bp = POS; skip_tok(); if POS <= bp { POS = bp + 1; } }   /* string/char literal -- one token; braces inside do not count */
                    else if c == 40 || c == 91 { pd = pd + 1; POS = POS + 1; }
                    else if c == 41 || c == 93 { if pd > 0 { pd = pd - 1; } POS = POS + 1; }
                    else if pd == 0 && c == 59 { POS = POS + 1; break; }        /* ';' unit/tuple struct / type alias */
                    else if pd == 0 && c == 123 { let e = find_block_end(POS); if e >= 0 { POS = e + 1; } else { POS = POS + 1; } break; }   /* body '{' -> balanced end (string/char/comment-aware find_block_end) */
                    else { POS = POS + 1; }
                }
                return;
            }
            POS = isv;
        }
        if POS + 1 < LEN && at(POS) == 39 && is_idst(at(POS + 1)) {                 /* R1: `'label: <loop>` -- allocate a break flag, push, parse the loop, pop (guard vs a `'c'` char literal via the `:`) */
            let lsv = POS; POS = POS + 1; rd_ident(); let lo = ID_O; let ll = ID_L;
            skipws();
            if POS < LEN && at(POS) == 58 {
                POS = POS + 1;
                let f = NC; let z = ecr(0); ecopy(f, z); NC = NC + 1;
                let cf2 = NC; let z3 = ecr(0); ecopy(cf2, z3); NC = NC + 1;
                LBLO[NLBL as usize] = lo; LBLL[NLBL as usize] = ll; LBLFLAG[NLBL as usize] = f;
                LBLCFLAG[NLBL as usize] = cf2; LBLLVL[NLBL as usize] = LDEPTH + 1; LBLRES[NLBL as usize] = NLRES; NLBL = NLBL + 1;
                parse_stmt();
                NLBL = NLBL - 1;
                return;
            }
            POS = lsv;
        }
        if POS < LEN && at(POS) == 42 {                                            /* *p = e  (deref store) or *p; or `*p` value tail */
            let starpos = POS;
            POS = POS + 1; let p = parse_atom(); skipws();
            if POS < LEN && at(POS) == 61 && (POS + 1 >= LEN || at(POS + 1) != 61) {
                POS = POS + 1; let e = parse_expr(); eat_ch(59);
                println!("  store v{} v{}", p, e);
                return;
            } else if POS + 1 < LEN && at(POS + 1) == 61 && (at(POS) == 43 || at(POS) == 45 || at(POS) == 42 || at(POS) == 47 || at(POS) == 124 || at(POS) == 38 || at(POS) == 94) {   /* *p += -= *= /= |= &= ^= e */
                let opc = at(POS); POS = POS + 2; let e = parse_expr(); eat_ch(59);
                let cur = eload(p);
                let mut nv = 0;
                if opc == 43 { nv = ebin("add", cur, e); } else if opc == 45 { nv = ebin("sub", cur, e); } else if opc == 42 { nv = ebin("mul", cur, e); } else if opc == 47 { nv = ebin("div", cur, e); } else if opc == 124 { nv = ebin("or", cur, e); } else if opc == 38 { nv = ebin("and", cur, e); } else { nv = ebin("xor", cur, e); }
                println!("  store v{} v{}", p, nv);
                return;
            }
            /* R1 keystone: not a store -> `*p` is a VALUE. Rewind to the `*` and route through the general
               expr path so a tail `{*x}` is RETURNED (was: `else { eat_ch(59); }` discarded it -> ret 0). */
            POS = starpos;
            let v = parse_expr();
            skipws();
            if eat_ch(59) { let d = v; } else { skipws(); if POS < LEN && at(POS) == 125 { emit_tail(v); } }
            return;
        }
        if POS < LEN && at(POS) == 123 { parse_block(); return; }                  /* R2: bare `{ ... }` block statement -- own scope (parse_block saves/restores NSYM) */
        let save = POS;
        if !rd_ident() {
            /* not an ident-led stmt: an expression statement or tail */
            POS = save;
            let v = parse_expr();
            skipws();
            if eat_ch(59) { let d = v; } else { skipws(); if POS < LEN && at(POS) == 125 { emit_tail(v); } }
            return;
        }
        let o = ID_O; let l = ID_L;
        if is_kw_let() {
            skipws();
            if POS < LEN && at(POS) == 40 {                                        /* R23: let (a, b, ..) = EXPR ; tuple destructure (PAT: now recursive -- nested tuples + `_`) */
                let patpos = POS;
                let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } POS = POS + 1; }   /* skip the whole pattern to `)` */
                skipws();
                if POS < LEN && at(POS) == 58 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 40 { let mut d2 = 1; POS = POS + 1; while POS < LEN && d2 > 0 { if at(POS) == 40 { d2 = d2 + 1; } if at(POS) == 41 { d2 = d2 - 1; } POS = POS + 1; } } }
                eat_ch(61);
                ETUPSLICE = 0; let cell = parse_expr(); let tslice = ETUPSLICE; eat_ch(59);
                let savepos = POS; POS = patpos; bind_tuple_pat(cell, tslice, ETUPTY); POS = savepos;
                return;
            }
            if POS < LEN && at(POS) == 91 {                                        /* PAT: `let [a, b, c] = EXPR ;` / `let [head, rest @ ..] = EXPR ;` array/slice destructure */
                POS = POS + 1;
                let mut pn = 0; let mut resti = 0 - 1;
                loop {
                    skipws(); if POS < LEN && at(POS) == 93 { POS = POS + 1; break; }
                    if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 { POS = POS + 2; TNO[pn as usize] = 0 - 1; TNL[pn as usize] = 0; resti = pn; pn = pn + 1; }   /* bare `..` rest (unbound) */
                    else { rd_ident(); let no = ID_O; let nl = ID_L; skipws();
                        if POS < LEN && at(POS) == 64 { POS = POS + 1; skipws(); if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 { POS = POS + 2; resti = pn; } }   /* `name @ ..` -> named rest */
                        TNO[pn as usize] = no; TNL[pn as usize] = nl; pn = pn + 1;
                    }
                    skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; }
                }
                skipws(); if POS < LEN && at(POS) == 58 { POS = POS + 1; while POS < LEN && at(POS) != 61 { POS = POS + 1; } }   /* skip a `: [T; N]` annotation */
                eat_ch(61);
                let cell = parse_expr(); let alen = EARRLEN; eat_ch(59);
                let mut i = 0;
                while i < pn {
                    let no = TNO[i as usize]; let nl = TNL[i as usize];
                    if i == resti {
                        if no >= 0 {                                               /* named rest -> a [ptr,len] slice descriptor over the tail */
                            let d = halloc(8); let z = ecr(0); let base = ebin("add", cell, ec(i * 4)); println!("  store v{} v{}", ebin("add", d, z), base);
                            let c4 = ecr(4); let cnt = ec(alen - i); println!("  store v{} v{}", ebin("add", d, c4), cnt);
                            let slot = NC; ecopy(slot, d); NC = NC + 1; sym_add(no, nl, slot); SK[(NSYM - 1) as usize] = 2;   /* slice -> .len()/[i] via dispatch_vec */
                        }
                    } else if no >= 0 && !(nl == 1 && at(no) == 95) {              /* skip `_` */
                        let el = eload(ebin("add", cell, ec(i * 4)));
                        let slot = NC; ecopy(slot, el); NC = NC + 1; sym_add(no, nl, slot);
                    }
                    i = i + 1;
                }
                return;
            }
            let ssave = POS;                                                       /* R2: `let Ty { f, g } = EXPR ;` struct destructure */
            if POS < LEN && is_idst(at(POS)) {
                rd_ident(); let dto = ID_O; let dtl = ID_L;
                let dty = ty_find(dto, dtl);
                skipws();
                if dty >= 0 && POS < LEN && at(POS) == 123 {
                    POS = POS + 1;
                    let mut tn = 0;
                    loop {
                        skipws(); if POS < LEN && at(POS) == 125 { POS = POS + 1; break; }
                        if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 {      /* R2ce: `.. ` rest pattern -- skip remaining fields to `}` */
                            POS = POS + 2; skipws();
                            while POS < LEN && at(POS) != 125 { POS = POS + 1; }
                            if POS < LEN { POS = POS + 1; }
                            break;
                        }
                        rd_ident(); TNO[tn as usize] = ID_O; TNL[tn as usize] = ID_L;
                        BNO[tn as usize] = ID_O; BNL[tn as usize] = ID_L;             /* R2ce: default bind-name == field-name */
                        skipws();
                        if POS < LEN && at(POS) == 58 { POS = POS + 1; skipws(); rd_ident(); BNO[tn as usize] = ID_O; BNL[tn as usize] = ID_L; }   /* R2ce: `field: bind` rename */
                        tn = tn + 1;
                        skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; }
                    }
                    skipws(); eat_ch(61);
                    let cell = parse_expr(); eat_ch(59);
                    let mut i = 0;
                    while i < tn {
                        let fi = fld_find(dty, TNO[i as usize], TNL[i as usize]);
                        let off = ec(FSB[fi as usize]); let ad = ebin("add", cell, off); let el = eload(ad);
                        let slot = NC; ecopy(slot, el); NC = NC + 1; sym_add(BNO[i as usize], BNL[i as usize], slot);
                        let fty = FSTY[fi as usize]; if fty >= 0 { SST[(NSYM - 1) as usize] = fty; }
                        i = i + 1;
                    }
                    return;
                }
                POS = ssave;
            }
            let lsave = POS;                                                       /* R1: `let PAT(x) = EXPR else { .. };` (let-else) or refutable destructure */
            rd_ident(); let mut ph_o = ID_O; let mut ph_l = ID_L; let mut qual = 0;
            skipws();
            if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); rd_ident(); ph_o = ID_O; ph_l = ID_L; qual = 1; }
            let lvi = var_find(ph_o, ph_l); let lod = opt_disc(ph_o, ph_l);
            if lvi >= 0 || lod >= 0 {                                              /* refutable head: Some/Ok/None/Err or a user variant */
                let mut want = 0; let mut lhas = 0;
                if lvi >= 0 { want = VDISC[lvi as usize]; lhas = VHAS[lvi as usize]; } else { want = lod; lhas = opt_has(ph_o, ph_l); }
                let mut xo = 0; let mut xl = 0; let mut bindx = 0; let mut stripref = 0;
                skipws();
                if POS < LEN && at(POS) == 40 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; stripref = 1; skipws(); } rd_ident(); xo = ID_O; xl = ID_L; bindx = 1; while POS < LEN && at(POS) != 41 { POS = POS + 1; } eat_ch(41); }   /* P-CONTROL-FLOW-LET: `Some(&x)` strips one ref level */
                skipws(); eat_ch(61);
                let cell = parse_expr();
                let coptref = EOPTREF;   /* P-CONTROL-FLOW-LET: `let Some(n) = v.first()` -- Option<&T>, payload is a ref (`*n` no-op) */
                skipws(); let es = POS; let haselse = rd_ident() && is_kw_else();
                if haselse == false { POS = es; }
                if haselse {
                    let disc = eload(cell); let wc = ec(want); let cnd = ebin("eq", disc, wc);
                    let lc = LBL; LBL = LBL + 1;
                    println!("  brz v{}", cnd); ckreset();
                    if bindx == 1 && lhas == 1 { let c4 = ecr(4); let pv = eload(ebin("add", cell, c4)); let xslot = NC; ecopy(xslot, pv); NC = NC + 1; sym_add(xo, xl, xslot); if coptref == 1 && stripref == 0 { SFORREF[(NSYM - 1) as usize] = 1; } }   /* P-CONTROL-FLOW-LET: ref-Option payload is a ref unless `&x` stripped it */
                    println!("  goto __Le{}", lc); ckreset(); println!("  pool"); ckreset();
                    parse_block();
                    println!("  label __Le{}", lc); ckreset();
                    skipws(); if POS < LEN && at(POS) == 59 { POS = POS + 1; }
                    return;
                }
                if bindx == 1 && lhas == 1 { let c4 = ecr(4); let pv = eload(ebin("add", cell, c4)); let xslot = NC; ecopy(xslot, pv); NC = NC + 1; sym_add(xo, xl, xslot); if coptref == 1 && stripref == 0 { SFORREF[(NSYM - 1) as usize] = 1; } }   /* irrefutable destructure, no else -- P-CONTROL-FLOW-LET ref-Option payload */
                eat_ch(59);
                return;
            }
            POS = lsave;
            rd_ident(); let cklet_mut = if is_kw_mut() { rd_ident(); 1 } else { 0 };   /* mut? then the name is in ID (CONSTEVAL: only IMMUTABLE lets are const-tracked) */
            /* if we consumed `mut`, ID now holds the name; else ID holds the name already */
            let no = ID_O; let nl = ID_L;
            /* optional `: TYPE` */
            skipws();
            let mut ann64 = 0; let mut anndyn = 0; let mut annu8 = 0; let mut annu = 0; let mut annhmvec = 0;
            let mut anngak_l = 0; let mut anngat_l = 0 - 1; let mut annhasg = 0;   /* std-hoist P0.3: a `let x: B<&str>` annotation's type-arg kind */
            let mut annvelty = 0 - 1;   /* TI3e-d: a `let v: Vec<String>/Vec<&str>` annotation's element interned Ty (feeds SVELTY when the RHS carries no elements, e.g. Vec::new()). */
            let mut annvecvec = 0;   /* P-VECVEC: a `let v: Vec<Vec<..>>` annotation -> element is a Vec descriptor (SVELT sentinel -3), for Vec::new()+push where the RHS carries no literal */
            let mut annvelenum = 0 - 1;   /* R38: a `let v: Vec<Enum>` annotation's element enum-type (feeds SVELENUM when the RHS is Vec::new()+push, so {:?} restores variant names instead of pointers) */
            let mut annvelstruct = 0 - 1;   /* LAYER7 P-VEC-STRUCT-ELEM: a `let v: Vec<Struct>` / `Vec<Box<Struct>>` / `Vec<P<Struct>>` annotation's element STRUCT-type (feeds SVELT so `v[i].field` / `for a in v.iter() { a.field }` see the element as a struct -- was: element untagged -> `.field` dropped, index/iter returned the raw element pointer). */
            let mut annvelhm = 0;   /* D006 P-VEC-HASHMAP-ELEM: a `let v: Vec<HashMap<..>>`/`Vec<BTreeMap<..>>` annotation -> element is a HashMap (feeds SVELT sentinel -4 so `v[i].get/insert/len` dispatch through dispatch_hashmap) */
            let mut annarr64 = 0;   /* C-S0I64-LOCALARR: `let a: [i64;N]/[u64;N]` annotation -> 8-byte element (step B). Also fixes the annotation being mis-parsed as the value (prints 0). */
            let mut annvelu8 = 0;   /* RUSTC-FE byte-vec-repr LIFT: a `let v: Vec<u8>` annotation -> bind v as a BYTE-vec (SK=6, dispatch_bytes, 1-byte) so a LOCAL Vec<u8> shares the ONE 1-byte representation with a Vec<u8> FIELD / &[u8] / as_bytes().to_vec() (was: SK=2/dispatch_vec/4-byte -> a local built then moved into a field mismatched). Pin-Art: pinart-bytevec-repr-2026-08-04.md. */
            if POS < LEN && at(POS) == 58 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; } if POS < LEN && at(POS) == 91 { POS = POS + 1; skipws(); let eo = POS; while POS < LEN && is_idc(at(POS)) { POS = POS + 1; } let el = POS - eo; if el == 3 && at(eo) == 105 && at(eo + 1) == 54 && at(eo + 2) == 52 { annarr64 = 1; } if el == 3 && at(eo) == 117 && at(eo + 1) == 54 && at(eo + 2) == 52 { annarr64 = 1; } let mut adep = 1; while POS < LEN && adep > 0 { if at(POS) == 91 { adep = adep + 1; } if at(POS) == 93 { adep = adep - 1; } POS = POS + 1; } ID_L = 0; } rd_ident();
                if ID_L == 2 && at(ID_O) == 102 && at(ID_O + 1) == 110 && POS < LEN && at(POS) == 40 {   /* LAYER3 M4: `let f: fn(A,..)->R = ..` fn-pointer type annotation -- consume `(args)->ret` up to `=` (the RHS closure/fn-value supplies the binding kind) */
                    let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } POS = POS + 1; }
                    skipws(); if POS + 1 < LEN && at(POS) == 45 && at(POS + 1) == 62 { POS = POS + 2; while POS < LEN && at(POS) != 61 { POS = POS + 1; } }
                }
                if ID_L == 2 && at(ID_O) == 117 && at(ID_O + 1) == 56 { annu8 = 1; }   /* u8 */
                if ID_L == 3 && at(ID_O) == 105 && at(ID_O + 1) == 54 && at(ID_O + 2) == 52 { ann64 = 1; }   /* i64 */
                if ID_L == 3 && at(ID_O) == 117 && at(ID_O + 1) == 54 && at(ID_O + 2) == 52 { ann64 = 1; annu = 1; }   /* u64 */
                if ID_L == 3 && at(ID_O) == 117 && at(ID_O + 1) == 51 && at(ID_O + 2) == 50 { annu = 1; }   /* value-width: u32 -> unsigned (32-bit); Display prints unsigned (rzrs.rs itself has no u32 -> self-host-safe) */
                if ID_L == 3 && at(ID_O) == 100 && at(ID_O + 1) == 121 && at(ID_O + 2) == 110 { anndyn = 1; rd_ident(); }   /* &dyn Trait -> skip trait name */
                let annty = ty_find(ID_O, ID_L); if annty >= 0 { INTO_TARGET = annty; }   /* R2cc: `let x: T = expr.into()` -- T is a user struct, dispatch `.into()` to T_from */
                if is_string_ty(ID_O, ID_L) { ANNSTR = 1; }   /* TI3e-i: `let s: String = ...` -> a char-iterator .collect() in the RHS builds a String */
                let annen = enum_find(ID_O, ID_L);   /* std-hoist GAP1: `let g: ROpt<&str>` enum annotation -> capture the type-arg kind */
                skipws(); if POS < LEN && at(POS) == 60 { let gs = POS + 1; let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 && at(POS - 1) != 45 { d = d - 1; } POS = POS + 1; }   /* the `>` of a `->` (return arrow, e.g. `Box<dyn Fn(i32)->i32>`) is NOT a generic close -- guard on the preceding `-` */
                    if is_vec_ty(ID_O, ID_L) { scan_ta_kind(gs); if ANNGAK == 3 { annvelty = 3; } else if ANNGAK == 1 { annvelty = 2; } else if ANNGAK == 2 { annvecvec = 1; }
                        let mut eo = gs; while eo < POS && (at(eo) == 32 || at(eo) == 38) { eo = eo + 1; } let mut el = eo; while el < POS && is_idc(at(el)) { el = el + 1; }
                        if is_box_ident(eo, el - eo) && el < POS && at(el) == 60 { eo = el + 1; while eo < POS && (at(eo) == 32 || at(eo) == 38) { eo = eo + 1; } el = eo; while el < POS && is_idc(at(el)) { el = el + 1; } }   /* LAYER6 P-VEC-BOX-ENUM-ELEM (LAYER7: also the P alias via is_box_ident): see THROUGH `Box<Enum>`/`P<T>` -- a Box is a transparent pointer to the cell (same repr as `Vec<Enum>`), so `let v: Vec<Box<Enum>>`/`Vec<P<Expr>>` carries the inner element type -> {:?}/index/for recurse (was: read `Box`, enum_find/ty_find fails -> element printed as a raw pointer). */
                        let een = enum_find(eo, el - eo); if een >= 0 { annvelenum = een; }   /* R38: `let v: Vec<Enum>` -> capture the element enum-type */
                        let est = ty_find(eo, el - eo); if est >= 0 { annvelstruct = est; } if el - eo == 2 && at(eo) == 117 && at(eo + 1) == 56 { annvelu8 = 1; } if (el - eo == 7 && at(eo) == 72 && at(eo + 1) == 97 && at(eo + 2) == 115 && at(eo + 3) == 104 && at(eo + 4) == 77 && at(eo + 5) == 97 && at(eo + 6) == 112) || (el - eo == 8 && at(eo) == 66 && at(eo + 1) == 84 && at(eo + 2) == 114 && at(eo + 3) == 101 && at(eo + 4) == 101 && at(eo + 5) == 77) { annvelhm = 1; }   /* RUSTC-FE byte-vec-repr LIFT: Vec<u8> element -> byte-vec; D006: Vec<HashMap>/Vec<BTreeMap> -> annvelhm */ }   /* LAYER7 P-VEC-STRUCT-ELEM: `let v: Vec<Struct>`/`Vec<Box<Struct>>`/`Vec<P<Struct>>` -> capture the element struct-type */
                    if annty >= 0 && TGPL[annty as usize] > 0 { scan_ta_kind(gs); anngak_l = ANNGAK; anngat_l = ANNGAT; annhasg = 1; }   /* std-hoist P0.3: `let x: B<&str>` generic-struct annotation -> capture the type-arg kind */
                    else if annen >= 0 && ENGPL[annen as usize] > 0 { scan_ta_kind(gs); anngak_l = ANNGAK; anngat_l = ANNGAT; annhasg = 1; }   /* std-hoist GAP1: generic-enum annotation `ROpt<&str>` -> capture the type-arg kind (payloadless RNone) */
                    let mut gp = gs; while gp + 2 < POS { if at(gp) == 100 && at(gp + 1) == 121 && at(gp + 2) == 110 { WANTDYN = 1; } if at(gp) == 86 && at(gp + 1) == 101 && at(gp + 2) == 99 { annhmvec = 1; } gp = gp + 1; } } }   /* A: `Vec<Box<dyn Trait>>` -> coerce vec! elements to fat pointers; nestgen: `HashMap<_, Vec<_>>` -> mark Vec values */
            eat_ch(61);
            let ck_save = POS; CVALID = 1; let ck_v = ceval_expr(); skipws(); let ck_ok = if CVALID == 1 && POS < LEN && at(POS) == 59 { 1 } else { 0 }; POS = ck_save;   /* CONSTEVAL R2/R3: peek-fold the RHS; if it is a pure integer const expr ending at `;`, this immutable local is const-known (set after sym_add) */
            if const_ovf_at() == 1 { die_overflow(); }   /* CONSTEVAL R2: `let c = <const expr that overflows i32>` -> REJECT */
            skipws();                                                          /* R2ci: `let NAME = D.D ;` -- bind a float by its truncated int (only `NAME as <int>` uses allowed later) */
            let ffs = POS;
            if POS < LEN && is_dig(at(POS)) {
                let mut ffn: i64 = 0;
                while POS < LEN && (is_dig(at(POS)) || at(POS) == 95) { if at(POS) != 95 { ffn = ffn * 10 + ((at(POS) - 48) as i64); } POS = POS + 1; }
                if POS < LEN && at(POS) == 46 && POS + 1 < LEN && is_dig(at(POS + 1)) {
                    POS = POS + 1;
                    while POS < LEN && (is_dig(at(POS)) || at(POS) == 95) { POS = POS + 1; }
                    while POS < LEN && is_idc(at(POS)) { POS = POS + 1; }
                    skipws();
                }
                POS = ffs;                                                     /* SOFTFLOAT: fall through -- the general RHS parse_expr handles the float literal via the fixed-point scaled representation (EISFLT), captured into SFLTV below */
            }
            if POS < LEN && at(POS) == 123 {                                   /* CTRL: `let x = { stmts; tail }` block-expression -> evaluate into x's slot (was parsed as a block stmt -> tail went to ret) */
                let bslot = NC; NC = NC + 1; parse_branch_into(bslot);
                skipws(); eat_ch(59); sym_add(no, nl, bslot); return;
            }
            EVELSTR_RET = 0;   /* R14 P-CF-ACC: reset the non-leaking fn-return Vec<String> element channel before the RHS, so only this RHS's own call sets it (no leak from a prior binding/struct field). */
            EANNARR64 = annarr64;
            let e = parse_expr();
            EANNARR64 = 0;
            INTO_TARGET = 0 - 1;   /* R2cc: consume the `.into()` target after the RHS */
            ANNSTR = 0;   /* TI3e-i: consume the String-annotation collect signal after the RHS */
            let eoptk = EOPTK; let eoptvelt = EOPTVELT;   /* container-kind K1: an Option RHS's payload kind */
            let eisopt = EISOPT; let eoptres = EOPTRES; let eoptref = EOPTREF;   /* container-kind (a); RUSTC-FE: capture the Option-payload-ref flag for the binding */
            let e8 = EIS8;   /* value-width: a u8/i8 RHS (`let x = 100u8`) -> the local is u8 */
            let ety = EITYP;
            let evec = EISVEC;
            let eiter = EISITER; let eitervelty = DVELTY;   /* STATEFUL-ITER: RHS is a bare `.iter()`/`.peekable()` -> bind a cursor (SK=9) */
            let esplit = ESPLIT;   /* STATEFUL-ITER: RHS is splitn(n,sep) -> str-cursor (SK=10) */
            let ewasref = EWASREF;   /* R2: RHS was a ref payload (find/get/first via unwrap/?) -> `let x = ..` binds a ref, `*x` a no-op */
            let estring = EISSTRING;
            let eclo = EISCLO; let ecretclo = ECRETCLO;
            let estr = EISSTR;
            let ewasg = EWASGARG; let egak0 = EGAK; let egat0 = EGAT;   /* std-hoist P0.1: RHS was a generic-struct literal carrying a concrete type-arg */
            let ewasg2 = EWASGARG2; let egak2 = EGAK2; let egat2 = EGAT2;   /* std-hoist P0.3: the RHS's NESTED (T-of-T) type-arg kind */
            let earr = EISARR;
            let eannarr64_saved = annarr64;
            let earrlen = EARRLEN;
            let earrelt = EARRELT;
            let ebytes = EISBYTES;
            let eflt = EISFLT;   /* SOFTFLOAT: RHS is a fixed-point f64 -> mark the local (SFLTV) so reads restore EISFLT and Display formats it as a decimal */
            let ebool = EISBOOL;
            let echar = EISCHAR;   /* R12: a char RHS (`let c = x as char`) -> the local Displays as a character */
            let etupty = ETUPTY;   /* tuple-field-kind: a tuple RHS -> record its type id so `t.N` restores field kinds */
            let eenum = EISENUM;
            let eenumty = EIENUMTY;
            let ehm = EISHM;
            let evdq = EISVDQ;   /* STDCOLL N1: RHS is a VecDeque -> SK=11 local */
            let eset = EISSET;   /* STDCOLL N2: RHS is a HashSet -> SK=12 local */
            let emb = EMUTBOR; let embo = EMUTBOR_O; let embl = EMUTBOR_L;   /* R3: RHS is `&mut X` -- for the aliasing check */
            let e64 = w64(ann64, EIS64);
            let eu = if annu == 1 { 1 } else { EISU };   /* MEET-3: u64-typed local */
            let mut ev = e;
            if eiter == 1 && evec == 1 {   /* STATEFUL-ITER: build a cursor [buf,len,pos=0] from the vec descriptor so it.next()/.peek() can advance a stored pos */
                let cur = halloc(12); let c4c = ecr(4); let c8c = ecr(8);
                println!("  store v{} v{}", cur, eload(ev));
                println!("  store v{} v{}", ebin("add", cur, c4c), eload(ebin("add", ev, c4c)));
                println!("  store v{} v{}", ebin("add", cur, c8c), ec(0));
                ev = cur;
            }
            if ann64 == 1 && EIS64 == 0 { if eu == 1 { ev = emit_conv(ev); } else { ev = emit_convs(ev); } }   /* R2: widen a 32-bit RHS into an i64/u64-annotated local -- sign-extend (i64) / zero-extend (u64) so negatives keep their sign */
            if anndyn == 1 && eclo == 0 { let fat = halloc(8); println!("  store v{} v{}", fat, e); let c4d = ecr(4); let etyc = ec(ety); println!("  store v{} v{}", ebin("add", fat, c4d), etyc); ev = fat; }   /* &dyn coercion -> [data, type-index]. eclo==0: `Box<dyn Fn>` over a closure is NOT a trait-object fat pointer -- the closure cell {code,env} already IS the value (SK=4 path below). */
            eat_ch(59);
            let slot = NC;
            if e64 == 1 { ecopy64(slot, ev); } else { ecopy(slot, ev); }
            /* CONSTEVAL R2/R3: record the const-known value on the just-added symbol (after sym_add below) via ck_ok/cklet_mut */
            NC = NC + 1;
            sym_add(no, nl, slot);
            if ck_ok == 1 && cklet_mut == 0 { SCKNOWN[(NSYM - 1) as usize] = 1; SCKVAL[(NSYM - 1) as usize] = ck_v; }   /* CONSTEVAL R2/R3: immutable const-int local -> const-let-propagated */
            if emb == 1 {                                                     /* R3: `let B = &mut X` -- reject a second live &mut to X (NLL: only if the prior borrower is used after here) */
                let xi = sym_idx(embo, embl);
                if xi >= 0 {
                    let po = SMBORO[xi as usize]; let pl = SMBORL[xi as usize];
                    if po >= 0 && !span_eq_at(po, pl, no, nl) && mut_used_later(po, pl) { die_borrow(); }
                    SMBORO[xi as usize] = no; SMBORL[xi as usize] = nl;
                }
            }
            if e64 == 1 { SI64[(NSYM - 1) as usize] = 1; }    /* R17: i64-typed local */
            if eu == 1 { SU64[(NSYM - 1) as usize] = 1; }     /* MEET-3: u64-typed local */
            if ety >= 0 { SST[(NSYM - 1) as usize] = ety; }   /* R1: struct-typed local */
            if ety >= 0 && ewasg == 1 { SGAK[(NSYM - 1) as usize] = egak0; SGAT[(NSYM - 1) as usize] = egat0;   /* std-hoist P0.1: record the generic-struct local's concrete type-arg kind */
                if ewasg2 == 1 { SGAK2[(NSYM - 1) as usize] = egak2; SGAT2[(NSYM - 1) as usize] = egat2; } }   /* std-hoist P0.3: record one nested (T-of-T) level */
            else if ety >= 0 && annhasg == 1 { SGAK[(NSYM - 1) as usize] = anngak_l; SGAT[(NSYM - 1) as usize] = anngat_l; }   /* std-hoist P0.3: no RHS record -> use the let-annotation's `B<..>` type-arg kind */
            if evec == 1 { if annvelu8 == 1 { SK[(NSYM - 1) as usize] = 6; EISBYTES = 1; } else { SK[(NSYM - 1) as usize] = 2; } }     /* R4: Vec-typed local; RUSTC-FE byte-vec-repr LIFT: a `Vec<u8>` local is SK=6 (byte-vec, dispatch_bytes, 1-byte) -- unified with the field/&[u8]/as_bytes representation */
            if eiter == 1 && evec == 1 { SK[(NSYM - 1) as usize] = 9; SVELTY[(NSYM - 1) as usize] = eitervelty; }   /* STATEFUL-ITER: cursor local (it.next()/.peek() dispatch); record element Ty for the payload kind */
            if esplit == 1 { SK[(NSYM - 1) as usize] = 10; }   /* STATEFUL-ITER: splitn str-cursor local */
            if eiter == 1 && evec == 0 && echar == 1 { SK[(NSYM - 1) as usize] = 13; }   /* P-CHARS-ITER: bare `let mut it = s.chars()` -> char-cursor local (cell built in the chars() handler; it.next() -> Some(char)) */
            if ewasref == 1 { SFORREF[(NSYM - 1) as usize] = 1; }   /* R2: ref-payload binding (find/get/first unwrap/?) -> `*x` no-op */
            if estring == 1 && evec == 0 && eenum == 0 { SK[(NSYM - 1) as usize] = 3; }  /* R6: String-typed local. TI3e: guard evec==0 -- a Vec<String> binding must stay Vec-kind (SK=2), else EISSTRING bubbling from the last element contaminates the Vec local to String-kind, so `&v` skips the sk==2 Vec path and DVELTY never carries the element Ty. Mirrors the &str guard below. LAYER6 P-DERIVE-EQ-STRING-PAYLOAD: guard eenum==0 too -- `let a = T::Id(String::from(..))` leaks EISSTRING from the payload arg onto the enum binding; without this it got SK=3 (String) instead of SENUM (enum), so `a == b` byte-compared the enum CELL pointer -> load(cell)=disc=0 -> NULL buf -> segfault, and derive PartialEq never routed to <Enum>_eq. */
            if eclo == 1 && ety < 0 { SK[(NSYM - 1) as usize] = 4; SCRETCLO[(NSYM - 1) as usize] = ecretclo; }     /* R10: closure-typed local (+ returns-a-closure). ety<0: a struct literal whose SOLE field is a closure leaves EISCLO=1 bubbling from the last field -- it must stay struct-kind (SST), else `s.f` skips the struct field-read path (mirrors the Vec<String> evec==0 guard). */
            if earr == 1 { SK[(NSYM - 1) as usize] = 5; SARRLEN[(NSYM - 1) as usize] = earrlen; if earrelt >= 0 { SARRELT[(NSYM - 1) as usize] = earrelt; } if EARRVTY >= 0 { SARRVTY[(NSYM - 1) as usize] = EARRVTY; } SARR64[(NSYM - 1) as usize] = eannarr64_saved; }   /* TI3e-m: record the array element interned Ty */     /* R11: array-typed local (+ R1: its element count for .len(); R2ar: element struct-type) */
            if ebytes == 1 { SK[(NSYM - 1) as usize] = 6; }   /* R25: &[u8] byte-slice local */
            if ehm == 1 { SK[(NSYM - 1) as usize] = 7; if annhmvec == 1 { SARRELT[(NSYM - 1) as usize] = 1; } }       /* S3: HashMap local (SARRELT=1 marks Vec-valued -> nestgen m[k] is a Vec) */
            if evdq == 1 { SK[(NSYM - 1) as usize] = 11; }   /* STDCOLL N1: VecDeque local -> dispatch_vecdeque */
            if eflt == 1 { SFLTV[(NSYM - 1) as usize] = 1; SI64[(NSYM - 1) as usize] = 1; }   /* SOFTFLOAT: fixed-point f64 local (i64-width scaled value) */
            if eset == 1 { SK[(NSYM - 1) as usize] = 12; }   /* STDCOLL N2: HashSet local -> dispatch_hashset */
            if ebool == 1 { SBOOL[(NSYM - 1) as usize] = 1; } /* A4: bool-typed local */
            if echar == 1 { SCHAR[(NSYM - 1) as usize] = 1; } /* R12: char-typed local */
            if etupty >= 0 { STUPTY[(NSYM - 1) as usize] = etupty; } /* tuple-field-kind: tuple-typed local */
            STY[(NSYM - 1) as usize] = ty_of_flags(); /* TI0: record the binding's interned Ty (parallel to the flag side-band; not yet consumed) */
            if EITY >= 0 && (TYTAG[EITY as usize] == 18 || TYTAG[EITY as usize] == 19) { STY[(NSYM - 1) as usize] = EITY; }   /* LAYER2 P6: a BTreeMap/BTreeSet local carries its btree Ty (maps aren't in ty_of_flags) -> the for-in reads it to sort */
            if annu8 == 1 || e8 == 1 { SU8[(NSYM - 1) as usize] = 1; }   /* R1: u8-typed local (annotation OR a u8/i8-suffix RHS) -- wrapping/saturating/checked mask 0xFF */
            if eenum == 1 { SENUM[(NSYM - 1) as usize] = eenumty + 1;
                if ewasg == 1 { SGAK[(NSYM - 1) as usize] = egak0; SGAT[(NSYM - 1) as usize] = egat0; }   /* std-hoist GAP1: record the generic-enum local's concrete type-arg (from its payload) */
                else if annhasg == 1 { SGAK[(NSYM - 1) as usize] = anngak_l; SGAT[(NSYM - 1) as usize] = anngat_l; }   /* std-hoist GAP1: payloadless RNone -> use the let-annotation's type-arg kind */
                else { SGAK[(NSYM - 1) as usize] = 0; SGAT[(NSYM - 1) as usize] = 0 - 1; } }   /* std-hoist GAP1: no kind info -> reset (SGAK is never cleared by sym_add) */
            if anndyn == 1 && eclo == 0 { SK[(NSYM - 1) as usize] = 8; SST[(NSYM - 1) as usize] = 0 - 1; }   /* long-tail: &dyn local (eclo==0: a `Box<dyn Fn>` closure stays SK=4, set above) */
            if estr == 1 && evec == 0 && estring == 0 && eclo == 0 && earr == 0 && ety < 0 { SK[(NSYM - 1) as usize] = 1; }   /* &str local (only when not a struct/other kind) */
            if WANTDYN == 1 && evec == 1 { SVECDYN[(NSYM - 1) as usize] = 1; }   /* A: Vec<Box<dyn>> local -- elements are fat pointers */
            if evec == 1 && VELT >= 0 { SVELT[(NSYM - 1) as usize] = VELT; }     /* R1: Vec<Struct> local -- record the element struct-type */
            if evec == 1 && VELENUM >= 0 { SVELENUM[(NSYM - 1) as usize] = VELENUM; }   /* R23b: Vec<Enum> local -- record the element enum-type for {:?}/for/index */
            if evec == 1 && VELENUM < 0 && annvelenum >= 0 { SVELENUM[(NSYM - 1) as usize] = annvelenum; }   /* R38: `let v: Vec<Enum> = Vec::new()` + push -- annotation is authoritative when the RHS carries no enum element */
            if evec == 1 { SVELTY[(NSYM - 1) as usize] = VELTY; }                /* TI3: Vec local -- record the element Ty */
            if evec == 1 && annvelty >= 0 { SVELTY[(NSYM - 1) as usize] = annvelty; }   /* TI3e-d: a `let v: Vec<String>/Vec<&str>` annotation is authoritative for the element Ty -- covers Vec::new()+push where the RHS carries no elements (VELTY unset). */
            if evec == 1 && annvelty < 0 && EVELSTR_RET > 0 { SVELTY[(NSYM - 1) as usize] = EVELSTR_RET; }   /* R14 P-CF-ACC: `let c = f()` where f `-> Vec<String>`/`-> Vec<&str>` -- thread the fn-return element display-kind so `c[k]`/`{}` dispatch String/&str (annotation still wins). Non-leaking: EVELSTR_RET is reset before every let-RHS and set only by this RHS's own rk==2 call. Fixes P-CF-ACC + repro D (was: element untagged -> printed the descriptor pointer -> crash at scale). */
            if evec == 1 && EVELSTR == 1 { SVELT[(NSYM - 1) as usize] = 0 - 2; }   /* R2ax: Vec<&str> local -- element is a &str descriptor pointer (SVELT sentinel -2) */
            if evec == 1 && VELVEC == 1 && VELT < 0 && EVELSTR == 0 { SVELT[(NSYM - 1) as usize] = 0 - 3; }   /* R2bx: Vec<Vec<..>> local -- element is itself a Vec descriptor (SVELT sentinel -3) */
            if evec == 1 && annvecvec == 1 { SVELT[(NSYM - 1) as usize] = 0 - 3; }   /* P-VECVEC: `let g: Vec<Vec<..>> = Vec::new()` annotation -- element is a Vec descriptor even without a literal RHS (SVELT sentinel -3) */
            if evec == 1 && annvelhm == 1 { SVELT[(NSYM - 1) as usize] = 0 - 4; }   /* D006 P-VEC-HASHMAP-ELEM: `let v: Vec<HashMap<..>> = Vec::new()` -- element is a HashMap descriptor (SVELT sentinel -4) so `v[i].get/insert/len` dispatch through dispatch_hashmap */
            if evec == 1 && VELT < 0 && annvecvec == 0 && annvelstruct >= 0 { SVELT[(NSYM - 1) as usize] = annvelstruct; }   /* LAYER7 P-VEC-STRUCT-ELEM: `let v: Vec<Struct> = Vec::new()` (+push) / `= vec![Struct{..}]` -- the annotation is authoritative for the element struct-type when the RHS carried none, so `v[i].field` / `for a in v.iter() { a.field }` dispatch struct field/method access. */
            SOPTK[(NSYM - 1) as usize] = eoptk; SOPTVELT[(NSYM - 1) as usize] = eoptvelt;   /* container-kind K1 */
            SISOPT[(NSYM - 1) as usize] = eisopt; SOPTRES[(NSYM - 1) as usize] = eoptres; SOPTREF[(NSYM - 1) as usize] = eoptref;   /* container-kind (a); RUSTC-FE: persist Option<&T> payload-ref-ness */
            WANTDYN = 0; ESPLIT = 0;   /* STATEFUL-ITER: consume the splitn str-cursor flag so it doesn't leak to the next binding */
            return;
        }
        if is_kw_if() {
            skipws();
            let isave = POS; let mut islet = 0;
            if rd_ident() && is_kw_let() { islet = 1; } else { POS = isave; }
            if islet == 1 {                                    /* R1: `if let PAT(x) = EXPR { body } [else { .. }]` (Option/Result/enum-variant) */
                skipws();
                if POS < LEN && at(POS) == 91 {                 /* PAT: `if let [slice] = arr/vec` -- irrefutable on a fixed array; REFUTABLE on a Vec `v[..]` (length guard -> else) */
                    let patpos = POS;
                    let mut d = 1; let mut arity = 0; let mut prest = 0;   /* P3: pre-scan the pattern -- element count + rest detection (for the Vec length guard) */
                    POS = POS + 1;
                    while POS < LEN && d > 0 {
                        skipws();
                        if at(POS) == 91 { d = d + 1; POS = POS + 1; }
                        else if at(POS) == 93 { d = d - 1; POS = POS + 1; }
                        else if at(POS) == 46 && POS + 1 < LEN && at(POS + 1) == 46 { prest = 1; POS = POS + 2; }
                        else if at(POS) == 44 { POS = POS + 1; }
                        else if at(POS) == 95 { POS = POS + 1; arity = arity + 1; }
                        else if is_idst(at(POS)) { rd_ident(); arity = arity + 1; }
                        else { POS = POS + 1; }
                    }
                    skipws(); eat_ch(61); skipws();
                    if POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); }   /* P3+: `if let [..] = &v[..]` -- the leading & is transparent (the slice descriptor is the same) */
                    rd_ident(); let ao = ID_O; let al = ID_L; skipws();
                    let mut vecslice = 0;
                    if prest == 0 && POS + 2 < LEN && at(POS) == 91 && at(POS + 1) == 46 && at(POS + 2) == 46 && sym_kind(ao, al) == 2 { vecslice = 1; POS = POS + 3; skipws(); eat_ch(93); }   /* P3: `NAME[..]` full-range slice of a Vec local -- refutable fixed-arity destructure */
                    if vecslice == 1 {
                        let desc = sym_find(ao, al); let ptr = eload(desc); let lenv = eload(ebin("add", desc, ec(4)));
                        let cnd = ebin("eq", lenv, ec(arity));   /* refutable: run `then` only when the Vec length matches the pattern arity */
                        let after = POS; let mark = NSYM;
                        println!("  brz v{}", cnd); ckreset();
                        POS = patpos; bind_slice_pat(ptr, arity); POS = after;   /* no rest/trailing (prest==0) -> bind_slice_pat only offsets from ptr; arity is unused there */
                        parse_block(); NSYM = mark;
                        skipws(); let s2v = POS;
                        if rd_ident() && is_kw_else() { let lb = LBL; LBL = LBL + 1; println!("  goto __Le{}", lb); ckreset(); println!("  pool"); ckreset(); skipws(); let s3v = POS; if rd_ident() && is_kw_if() { POS = s3v; parse_stmt(); } else { POS = s3v; parse_block(); } println!("  label __Le{}", lb); ckreset(); } else { POS = s2v; println!("  pool"); ckreset(); }
                        return;
                    }
                    let buf = sym_find(ao, al); let arrlen = sym_arrlen(ao, al);
                    let after = POS; let mark = NSYM;
                    POS = patpos; bind_slice_pat(buf, arrlen); POS = after;
                    parse_block();
                    NSYM = mark;
                    skipws(); let s2 = POS;
                    if rd_ident() && is_kw_else() { skipws(); if POS < LEN && at(POS) == 123 { let mut d2 = 1; POS = POS + 1; while POS < LEN && d2 > 0 { if at(POS) == 123 { d2 = d2 + 1; } if at(POS) == 125 { d2 = d2 - 1; } POS = POS + 1; } } } else { POS = s2; }
                    return;
                }
                rd_ident(); let mut po = ID_O; let mut pl = ID_L;
                skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); rd_ident(); po = ID_O; pl = ID_L; }   /* E::V */
                let vi = var_find(po, pl);
                let mut want = 0; let mut has = 0;
                if vi >= 0 { want = VDISC[vi as usize]; has = VHAS[vi as usize]; } else { want = opt_disc(po, pl); has = opt_has(po, pl); }
                let mut xo = 0; let mut xl = 0; let mut bindx = 0; let mut tuppos = 0; let mut tupbind = 0;
                skipws();
                if POS < LEN && at(POS) == 40 { POS = POS + 1; skipws();
                    if POS < LEN && at(POS) == 40 {   /* P3+: nested tuple payload `Some((a, b))` -- remember the inner tuple span, bind its fields at the payload site */
                        tuppos = POS; tupbind = 1; bindx = 1;
                        let mut dd = 1; POS = POS + 1; while POS < LEN && dd > 0 { if at(POS) == 40 { dd = dd + 1; } if at(POS) == 41 { dd = dd - 1; } POS = POS + 1; }
                        while POS < LEN && at(POS) != 41 { POS = POS + 1; } eat_ch(41);
                    } else { if POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); } rd_ident(); xo = ID_O; xl = ID_L; bindx = 1; while POS < LEN && at(POS) != 41 { POS = POS + 1; } eat_ch(41); }
                }   /* first payload binding (R2bo: strip a leading & -- `Some(&v)` copy-binds through the ref) */
                skipws(); eat_ch(61); skipws();
                let mut ilref = 0;
                if POS < LEN && at(POS) == 38 { let mut q = POS + 1; while q < LEN && at(q) == 32 { q = q + 1; } if q < LEN && is_idst(at(q)) { POS = POS + 1; skipws(); ilref = 1; } }   /* D004-CORE IC-1 P-BYREF-IFLET-SCRUTINEE-TRANSPARENT: `if let Some(x) = &opt` -- a reference to the boxed Option cell is transparent; skip the leading `&` so `cell` is the cell pointer (not eref=address-of-slot), else `eload(cell)` reads the slot not the disc -> the arm never fires. ilref governs the payload by-ref bind (`*x` no-op). The statement-if-let dual of the parse_match IC-1 fix + the 7842 slice `&`-strip. */
                let cell = parse_expr();
                let scrk = EOPTK;   /* P1: built-in Option/Result payload kind (3 String / 1 &str) from the scrutinee, for the Some(s)/Err(e) binder */
                let scrres = EOPTRES;   /* P1: Result(1)/Option(0) -- kind applies to Some (Option) or Err (Result), never Ok */
                let sctup = if EOPTTY >= 0 && TYTAG[EOPTTY as usize] == 11 { TYAUX[EOPTTY as usize] } else { 0 - 1 };   /* P3+: Option payload is a tuple -> its tuple-type id, for per-field kind restore in `Some((k,v))` (e.g. split_once -> two &str) */
                let disc = eload(cell); let wc = ec(want); let cnd = ebin("eq", disc, wc);
                println!("  brz v{}", cnd); ckreset();
                let mark = NSYM;
                if bindx == 1 && has == 1 && tupbind == 1 {   /* P3+: nested tuple payload -- the Some payload is a tuple cell [f0@0, f1@4]; brz above guards the deref (then-block only runs on Some) */
                    let c4 = ecr(4); let pv = eload(ebin("add", cell, c4));
                    let save = POS; POS = tuppos + 1; let mut tf = 0;
                    loop {
                        skipws(); if POS >= LEN || at(POS) == 41 { break; }
                        if at(POS) == 95 { POS = POS + 1; }
                        else { if at(POS) == 38 { POS = POS + 1; skipws(); } rd_ident(); let to = ID_O; let tl = ID_L; let tvv = eload(ebin("add", pv, ec(tf * 4))); let tsl = NC; ecopy(tsl, tvv); NC = NC + 1; sym_add(to, tl, tsl); if sctup >= 0 { let fk = tupty_fk(sctup, tf); if fk == 1 { SK[(NSYM - 1) as usize] = 1; } else if fk == 3 { SK[(NSYM - 1) as usize] = 3; } else if fk == 2 { SK[(NSYM - 1) as usize] = 2; } else if fk == 5 { SI64[(NSYM - 1) as usize] = 1; } } }   /* P3+: restore each tuple field's kind from the Option payload tuple-type */
                        tf = tf + 1; skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; }
                    }
                    POS = save;
                }
                else if bindx == 1 && has == 1 { let c4 = ecr(4); let pad = ebin("add", cell, c4); let pv = if vi >= 0 && VFI64[(vi * 8) as usize] == 1 { eload8(pad) } else { eload(pad) }; let xslot = NC; ecopy(xslot, pv); NC = NC + 1; sym_add(xo, xl, xslot);   /* P-ENUM-I64-PAYLOAD-WIDTH: 8-byte load for an i64 payload */ if EOPTREF == 1 || ilref == 1 { SFORREF[(NSYM - 1) as usize] = 1; if ilref == 1 { SREF1[(NSYM - 1) as usize] = 1; } } if vi >= 0 && VFSTR[(vi * 8) as usize] == 1 { SK[(NSYM - 1) as usize] = 3; } else if vi < 0 && EOPTREF == 0 && (scrk == 3 || scrk == 1) && ((scrres == 0 && want == 1) || (scrres == 1 && want == 0)) { SK[(NSYM - 1) as usize] = scrk; } if vi >= 0 && VFI64[(vi * 8) as usize] == 1 { SI64[(NSYM - 1) as usize] = 1; } }   /* R2: Option<&T> payload -> `*x` no-op; P1: String/&str payload (user-enum via VFSTR, built-in Option-Some / Result-Err via scrk) -> restore string kind. LAYER6 P-ENUM-I64-PAYLOAD-WIDTH: an i64/isize single payload binder is tagged SI64 so `*v`-based arithmetic is 64-bit (was 32-bit-truncated). */
                let mvbase = MVLOGN;                       /* MOVE-FLOW: if-let then-branch log base (threshold = mark, the pre-payload NSYM) */
                parse_block();
                let sstash = mv_stash(mvbase, mark);        /* stash then's OUTER moves */
                mv_undo(mvbase);                           /* else starts from the pre-then moved-set */
                NSYM = mark;
                skipws(); let s2b = POS;
                if rd_ident() && is_kw_else() {
                    let lb = LBL; LBL = LBL + 1;
                    println!("  goto __Le{}", lb); ckreset(); println!("  pool"); ckreset();
                    skipws(); let s3 = POS;
                    if rd_ident() && is_kw_if() { POS = s3; parse_stmt(); } else { POS = s3; parse_block(); }
                    println!("  label __Le{}", lb); ckreset();
                } else { POS = s2b; println!("  pool"); ckreset(); }
                mv_replay(sstash);                         /* union then's moves back in (post-if moved-set) */
                return;
            }
            let c = parse_expr();
            println!("  brz v{}", c); ckreset();
            /* DEFECT-h fix: a no-else `if` has type () -- its then-block value is ALWAYS discarded, so parse it
               with TAILDST=-1 (discard, like a loop body) so a value-producing tail (a `match`/`if`/block) does
               NOT leak a spurious `ret`. Was: the then-block's tail match emit_tail'd with TAILDST==0 -> `ret
               match_value`, short-circuiting the fn tail when the if is a mid-block statement (`if c { match .. }
               ; tail`). A with-else if keeps the ambient TAILDST (a tail if-expr's branches still ret). */
            let ifte = find_block_end(POS);
            let mut ifae = ifte + 1; while ifae < LEN && (at(ifae) == 32 || at(ifae) == 10 || at(ifae) == 9 || at(ifae) == 13) { ifae = ifae + 1; }
            let ifhaselse = ifae + 3 < LEN && at(ifae) == 101 && at(ifae + 1) == 108 && at(ifae + 2) == 115 && at(ifae + 3) == 101 && (ifae + 4 >= LEN || !is_idc(at(ifae + 4)));
            let ift = TAILDST;
            if ifhaselse == false { TAILDST = -1; }
            let mvth = NSYM; let mvbase = MVLOGN;          /* MOVE-FLOW: outer-symbol threshold + then-branch log base */
            parse_block();
            let sstash = mv_stash(mvbase, mvth);           /* stash then's OUTER moves */
            mv_undo(mvbase);                               /* else starts from the pre-then moved-set */
            TAILDST = ift;
            skipws();
            let s2 = POS;
            if rd_ident() && is_kw_else() {
                let lb = LBL; LBL = LBL + 1;
                println!("  goto __Le{}", lb); ckreset();
                println!("  pool"); ckreset();
                skipws();
                let s3 = POS;
                if rd_ident() && is_kw_if() { POS = s3; parse_stmt(); } else { POS = s3; parse_block(); }
                println!("  label __Le{}", lb); ckreset();
            } else { POS = s2; println!("  pool"); ckreset(); }
            mv_replay(sstash);                             /* union then's moves back in (post-if moved-set) */
            return;
        }
        if is_kw_while() {
            skipws();
            let wsave = POS;
            let mut islet = 0;
            if rd_ident() && is_kw_let() { islet = 1; } else { POS = wsave; }
            if islet == 1 {                                    /* R20: while let PAT(x) = EXPR { .. } */
                skipws(); rd_ident();                          /* Some / Ok / None / Err */
                let po = ID_O; let pl = ID_L;
                let want = opt_disc(po, pl);                   /* 1 = Some/Ok, 0 = None/Err */
                let has = opt_has(po, pl);
                let mut xo = 0; let mut xl = 0; let mut bindx = 0;
                skipws();
                if POS < LEN && at(POS) == 40 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); } rd_ident(); if id_is(109, 117, 116, 0, 0, 3) { skipws(); rd_ident(); } xo = ID_O; xl = ID_L; bindx = 1; skipws(); eat_ch(41); }   /* P-CONTROL-FLOW-LET: skip `mut`/`&` in `Some(mut inner)` -- was bound as "mut" */
                skipws(); eat_ch(61);
                println!("  loop"); ckreset();
                let cell = parse_expr();
                let scty = EITYP;                             /* R2an: while let Some(x)=vec.pop() -> x is struct-typed */
                let selt = DVELT; let dvty = DVELTY;          /* P-CONTROL-FLOW-LET: `while let Some(inner)=vec_of_vecs.pop()` -- thread the popped element's KIND to inner (Vec/String/&str), from dispatch_vec's DVELT/DVELTY */
                let disc = eload(cell);
                let wc = ec(want); let cnd = ebin("eq", disc, wc);
                println!("  brz v{}", cnd); ckreset();
                let mark = NSYM;
                if bindx == 1 && has {
                    let c4 = ecr(4); let pa = ebin("add", cell, c4); let pv = eload(pa);
                    let xslot = NC; ecopy(xslot, pv); NC = NC + 1; sym_add(xo, xl, xslot);
                    if scty >= 0 { SST[(NSYM - 1) as usize] = scty; }   /* R2an: struct-typed payload -> x.field dispatches */
                    if selt == 0 - 3 { SK[(NSYM - 1) as usize] = 2; }   /* P-CONTROL-FLOW-LET: Vec payload -> inner.len()/.pop()/[i] dispatch */
                    else if dvty >= 0 && TYTAG[dvty as usize] == 4 { SK[(NSYM - 1) as usize] = 3; }   /* String payload */
                    else if dvty >= 0 && TYTAG[dvty as usize] == 3 { SK[(NSYM - 1) as usize] = 1; }   /* &str payload */
                }
                parse_loop_body();
                NSYM = mark;
                println!("  cont"); ckreset(); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                return;
            }
            println!("  loop"); ckreset();
            let c = parse_expr();
            println!("  brz v{}", c); ckreset();
            parse_loop_body();
            println!("  cont"); ckreset();
            println!("  jmp"); ckreset();
            println!("  endloop"); ckreset();
            return;
        }
        if is_kw_loop() { emit_loop_val(); return; }   /* statement loop (break-value slot allocated but unused) */
        if is_kw_for() {
            /* for VAR in START..END (range) OR for [ (i,x) ] in &vec / vec.iter()[.enumerate()] (collection) */
            skipws();
            let mut ivo = 0; let mut ivl = 0; let mut istuple = 0; let mut istupdest = 0; let mut tdn = 0;
            if POS < LEN && at(POS) == 40 {               /* (i, x) tuple pattern (enumerate) */
                istuple = 1; POS = POS + 1; skipws();
                rd_ident(); ivo = ID_O; ivl = ID_L;
                skipws(); eat_ch(44); skipws();
                if POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); }   /* R2: `for (i,&x) in ..enumerate()` -- skip the `&` sub-pattern; the element binds by value */
                rd_ident();
                skipws(); eat_ch(41);
            } else { skipws();
                let mut refp = 0;
                if POS < LEN && at(POS) == 38 { POS = POS + 1; refp = 1; skipws(); }   /* R25: `for &byte in ..` ref pattern */
                if refp == 1 && POS < LEN && at(POS) == 40 {                            /* R2: `for &(a,b)` tuple-element destructure */
                    istupdest = 1; POS = POS + 1;
                    loop {
                        skipws(); if POS < LEN && at(POS) == 41 { POS = POS + 1; break; }
                        rd_ident(); TNO[tdn as usize] = ID_O; TNL[tdn as usize] = ID_L; tdn = tdn + 1;
                        skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; }
                    }
                } else { rd_ident(); } }
            let vo = ID_O; let vl = ID_L;
            rd_ident();                                   /* `in` */
            skipws();
            let imsave = POS;                             /* R2at: for [ (i,) ] x in v.iter_mut() [.enumerate()] -- bind x to the element ADDRESS */
            let mut imok = 0; let mut imdv = 0; let mut imarr = 0; let mut imlen = 0;
            if rd_ident() {
                let imo = ID_O; let iml = ID_L;
                let imsk = sym_kind(imo, iml);
                if imsk == 2 || imsk == 5 {   /* R-either: `.iter_mut()` over a Vec (sk=2) OR a fixed ARRAY (sk=5) -- the array case fell through (Vec-only) -> the general parse mishandled `arr.iter_mut()` and the loop body over-consumed (parse-drift). */
                    skipws();
                    if POS < LEN && at(POS) == 46 {
                        POS = POS + 1; rd_ident();
                        if id_is(105, 116, 101, 114, 95, 8) { skipws(); eat_ch(40); eat_ch(41); imok = 1; imdv = sym_find(imo, iml); if imsk == 5 { imarr = 1; imlen = sym_arrlen(imo, iml); } }
                    }
                }
            }
            if imok == 0 { POS = imsave; }
            if imok == 1 {
                skipws();
                if POS < LEN && at(POS) == 46 {           /* optional .enumerate() */
                    let es2 = POS; POS = POS + 1; rd_ident();
                    if id_is(101, 110, 117, 109, 101, 9) { skipws(); eat_ch(40); eat_ch(41); } else { POS = es2; }
                }
                let c4 = ecr(4); let len = if imarr == 1 { ec(imlen) } else { eload(ebin("add", imdv, c4)) };   /* R-either: array length is compile-time; Vec length is descriptor[+4] */
                let islot = NC; let z = ecr(0); ecopy(islot, z); NC = NC + 1;
                let xslot = NC; NC = NC + 1;
                let mark = NSYM;
                if istuple == 1 { sym_add(ivo, ivl, islot); }
                sym_add(vo, vl, xslot);                    /* x = element ADDRESS; NOT a for-ref, so *x loads/stores through it */
                println!("  loop"); ckreset();
                let cnd = ebin("lt", islot, len); println!("  brz v{}", cnd); ckreset();
                let buf = if imarr == 1 { imdv } else { eload(imdv) };   /* R-either: a local array IS its buffer; a Vec holds the buffer at descriptor[0] */
                let c4b = ecr(4); let off = ebin("mul", islot, c4b); let ea = ebin("add", buf, off);
                ecopy(xslot, ea);
                parse_loop_body();
                println!("  cont"); ckreset();
                let one = ecr(1); let ni = ebin("add", islot, one); ecopy(islot, ni);
                println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                NSYM = mark;
                return;
            }
            if istuple == 0 {                             /* for v in MAP.values_mut() -> bind v to each value SLOT ADDRESS (deref-writable) */
                let vmsave = POS;
                let mut vmok = 0; let mut vmmap = 0;
                if rd_ident() {
                    let vmo = ID_O; let vml = ID_L;
                    if sym_kind(vmo, vml) == 7 {
                        skipws();
                        if POS < LEN && at(POS) == 46 {
                            POS = POS + 1; rd_ident();
                            if id_is(118, 97, 108, 117, 101, 10) { skipws(); eat_ch(40); eat_ch(41); vmok = 1; vmmap = sym_find(vmo, vml); }
                        }
                    }
                }
                if vmok == 0 { POS = vmsave; }
                if vmok == 1 {
                    let c4 = ecr(4); let len = eload(vmmap); let ents = eload(ebin("add", vmmap, c4));
                    let islot = NC; let z = ecr(0); ecopy(islot, z); NC = NC + 1;
                    let xslot = NC; NC = NC + 1;
                    let mark = NSYM; sym_add(vo, vl, xslot);   /* x = value slot ADDRESS */
                    println!("  loop"); ckreset();
                    let cnd = ebin("lt", islot, len); println!("  brz v{}", cnd); ckreset();
                    let c8 = ecr(8); let ea = ebin("add", ents, ebin("mul", islot, c8)); let c4b = ecr(4); let va = ebin("add", ea, c4b);
                    ecopy(xslot, va);
                    parse_loop_body();
                    println!("  cont"); ckreset();
                    let one = ecr(1); let ni = ebin("add", islot, one); ecopy(islot, ni);
                    println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                    NSYM = mark;
                    return;
                }
            }
            if istuple == 1 && POS < LEN && at(POS) == 34 {   /* P-STRING: for (i,c) in "literal".char_indices() -> (byte-index, char) over the literal (ASCII) */
                let lsave = POS;
                POS = POS + 1; let so = POS;
                while POS < LEN && at(POS) != 34 { if at(POS) == 92 { POS = POS + 1; } POS = POS + 1; }
                let sl = POS - so; POS = POS + 1; skipws();
                let mut cik = 0;
                if POS < LEN && at(POS) == 46 { POS = POS + 1; rd_ident(); if id_is(99, 104, 97, 114, 95, 12) { skipws(); eat_ch(40); eat_ch(41); cik = 1; } }   /* char_indices (len 12, prefix "char_") */
                if cik == 1 {
                    let dv = emit_str_literal(so, sl);
                    let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let sp = eload(dv);
                    let islot = NC; let z = ecr(0); ecopy(islot, z); NC = NC + 1;
                    let cslot = NC; NC = NC + 1;
                    let mark = NSYM; sym_add(ivo, ivl, islot); sym_add(vo, vl, cslot); SCHAR[(NSYM - 1) as usize] = 1;   /* i = byte index, c = char */
                    println!("  loop"); ckreset();
                    let cnd = ebin("lt", islot, len); println!("  brz v{}", cnd); ckreset();
                    let sa = ebin("add", sp, islot); let byte = eloadb(sa); ecopy(cslot, byte);
                    parse_loop_body();
                    println!("  cont"); ckreset();
                    let one = ecr(1); let ni = ebin("add", islot, one); ecopy(islot, ni);
                    println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                    NSYM = mark; return;
                }
                POS = lsave;
            }
            if istuple == 0 && POS < LEN && at(POS) == 34 {   /* for c in "literal".chars()/.bytes() -> bytewise over the literal */
                let lsave = POS;
                POS = POS + 1; let so = POS;
                while POS < LEN && at(POS) != 34 { if at(POS) == 92 { POS = POS + 1; } POS = POS + 1; }
                let sl = POS - so; POS = POS + 1; skipws();
                let mut lok = 0;
                if POS < LEN && at(POS) == 46 { POS = POS + 1; rd_ident(); if id_is(99, 104, 97, 114, 115, 5) || id_is(98, 121, 116, 101, 115, 5) { skipws(); eat_ch(40); eat_ch(41); lok = 1; } }
                if lok == 1 {
                    let dv = emit_str_literal(so, sl);
                    let c4 = ecr(4); let len = eload(ebin("add", dv, c4)); let sp = eload(dv);
                    let islot = NC; let z = ecr(0); ecopy(islot, z); NC = NC + 1;
                    let cslot = NC; NC = NC + 1;
                    let mark = NSYM; sym_add(vo, vl, cslot);
                    println!("  loop"); ckreset();
                    let cnd = ebin("lt", islot, len); println!("  brz v{}", cnd); ckreset();
                    let sa = ebin("add", sp, islot); let byte = eloadb(sa); ecopy(cslot, byte);
                    parse_loop_body();
                    println!("  cont"); ckreset();
                    let one = ecr(1); let ni = ebin("add", islot, one); ecopy(islot, ni);
                    println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                    NSYM = mark; return;
                }
                POS = lsave;
            }
            if istuple == 0 && POS < LEN && at(POS) == 34 {   /* R37: for w in "literal".split_whitespace() -> yield whitespace-delimited &str descriptors (the literal-receiver dual of the NAME branch below; was: literal fell through to the generic iterable path -> garbage pointers) */
                let lsave = POS;
                POS = POS + 1; let so = POS;
                while POS < LEN && at(POS) != 34 { if at(POS) == 92 { POS = POS + 1; } POS = POS + 1; }
                let sl = POS - so; POS = POS + 1; skipws();
                let mut sok = 0;
                if POS < LEN && at(POS) == 46 { POS = POS + 1; rd_ident(); if id_is(115, 112, 108, 105, 116, 16) { skipws(); eat_ch(40); eat_ch(41); sok = 1; } }
                if sok == 1 {
                    let sdv = emit_str_literal(so, sl);
                    let c4 = ecr(4);
                    let sp = eload(sdv);
                    let len = eload(ebin("add", sdv, c4));
                    let islot = NC; let z = ecr(0); ecopy(islot, z); NC = NC + 1;
                    let sslot = NC; NC = NC + 1;
                    let wslot = NC; NC = NC + 1;
                    let mark = NSYM; sym_add(vo, vl, wslot); SK[(NSYM - 1) as usize] = 1;   /* loop var is a &str */
                    println!("  loop"); ckreset();
                    println!("  loop"); ckreset();
                    let lt1 = ebin("lt", islot, len);
                    let b1 = eloadb(ebin("add", sp, islot));
                    let ws1 = ebin("or", ebin("or", ebin("eq", b1, ec(32)), ebin("eq", b1, ec(9))), ebin("eq", b1, ec(10)));
                    let go1 = ebin("mul", lt1, ws1); println!("  brz v{}", go1); ckreset();
                    let one = ecr(1); ecopy(islot, ebin("add", islot, one));
                    println!("  cont"); ckreset(); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                    ecopy(sslot, islot);
                    println!("  loop"); ckreset();
                    let lt2 = ebin("lt", islot, len);
                    let b2 = eloadb(ebin("add", sp, islot));
                    let ws2 = ebin("or", ebin("or", ebin("eq", b2, ec(32)), ebin("eq", b2, ec(9))), ebin("eq", b2, ec(10)));
                    let notws = ebin("eq", ws2, ec(0));
                    let go2 = ebin("mul", lt2, notws); println!("  brz v{}", go2); ckreset();
                    let one2 = ecr(1); ecopy(islot, ebin("add", islot, one2));
                    println!("  cont"); ckreset(); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                    let wlen = ebin("sub", islot, sslot);
                    println!("  brz v{}", wlen); ckreset();
                    let desc = halloc(8);
                    let wptr = ebin("add", sp, sslot);
                    println!("  store v{} v{}", desc, wptr);
                    println!("  store v{} v{}", ebin("add", desc, c4), wlen);
                    ecopy(wslot, desc);
                    parse_loop_body();
                    println!("  cont"); ckreset(); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                    NSYM = mark;
                    return;
                }
                POS = lsave;
            }
            if istuple == 0 {                             /* R22: for c in NAME.chars() -> bytewise iteration over a &str/String */
                let csave = POS;
                let mut cok = 0; let mut cdv = 0; let mut ischars = 0;
                if rd_ident() {
                    let no2 = ID_O; let nl2 = ID_L; let k = sym_kind(no2, nl2);
                    skipws();
                    if (k == 1 || k == 3) && POS < LEN && at(POS) == 46 {
                        POS = POS + 1; rd_ident();
                        if id_is(99, 104, 97, 114, 115, 5) { skipws(); eat_ch(40); eat_ch(41); cok = 1; cdv = sym_find(no2, nl2); ischars = 1; }   /* chars() -- bytewise (ASCII), loop var is a char */
                        else if id_is(98, 121, 116, 101, 115, 5) { skipws(); eat_ch(40); eat_ch(41); cok = 1; cdv = sym_find(no2, nl2); }   /* bytes() -- bytewise, loop var is a u8 */
                    }
                }
                if cok == 0 { POS = csave; }
                if cok == 1 {
                    let c4 = ecr(4); let lad = ebin("add", cdv, c4); let len = eload(lad);
                    let sp = eload(cdv);
                    let islot = NC; let z = ecr(0); ecopy(islot, z); NC = NC + 1;
                    let cslot = NC; NC = NC + 1;
                    let mark = NSYM; sym_add(vo, vl, cslot);
                    if ischars == 1 { SCHAR[(NSYM - 1) as usize] = 1; }   /* R22: for c in .chars() -> char (Display prints the character). .bytes() stays an untagged scalar u8 value (b as u32 / arithmetic works as before). */
                    println!("  loop"); ckreset();
                    let cnd = ebin("lt", islot, len);
                    println!("  brz v{}", cnd); ckreset();
                    let sa = ebin("add", sp, islot); let byte = eloadb(sa); ecopy(cslot, byte);
                    parse_loop_body();
                    println!("  cont"); ckreset();
                    let one = ecr(1); let ni = ebin("add", islot, one); ecopy(islot, ni);
                    println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                    NSYM = mark;
                    return;
                }
            }
            if istuple == 0 {                             /* R2aq: for w in NAME.split_whitespace() -> yield whitespace-delimited &str [ptr,len] descriptors (SK=1) */
                let ssave = POS;
                let mut sok = 0; let mut sdv = 0;
                if rd_ident() {
                    let no4 = ID_O; let nl4 = ID_L; let k4 = sym_kind(no4, nl4);
                    skipws();
                    if (k4 == 1 || k4 == 3) && POS < LEN && at(POS) == 46 {
                        POS = POS + 1; rd_ident();
                        if id_is(115, 112, 108, 105, 116, 16) { skipws(); eat_ch(40); eat_ch(41); sok = 1; sdv = sym_find(no4, nl4); }   /* split_whitespace */
                    }
                }
                if sok == 0 { POS = ssave; }
                if sok == 1 {
                    let c4 = ecr(4);
                    let sp = eload(sdv);
                    let len = eload(ebin("add", sdv, c4));
                    let islot = NC; let z = ecr(0); ecopy(islot, z); NC = NC + 1;
                    let sslot = NC; NC = NC + 1;             /* word-start snapshot */
                    let wslot = NC; NC = NC + 1;             /* current word descriptor */
                    let mark = NSYM; sym_add(vo, vl, wslot); SK[(NSYM - 1) as usize] = 1;   /* loop var is a &str */
                    println!("  loop"); ckreset();                       /* outer: one iteration per word */
                    println!("  loop"); ckreset();                       /* skip a run of whitespace: continue while in-bounds AND whitespace */
                    let lt1 = ebin("lt", islot, len);
                    let b1 = eloadb(ebin("add", sp, islot));
                    let ws1 = ebin("or", ebin("or", ebin("eq", b1, ec(32)), ebin("eq", b1, ec(9))), ebin("eq", b1, ec(10)));
                    let go1 = ebin("mul", lt1, ws1); println!("  brz v{}", go1); ckreset();
                    let one = ecr(1); ecopy(islot, ebin("add", islot, one));
                    println!("  cont"); ckreset(); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                    ecopy(sslot, islot);                      /* word start (points at non-ws or end) */
                    println!("  loop"); ckreset();                       /* take a run of non-whitespace: continue while in-bounds AND not whitespace */
                    let lt2 = ebin("lt", islot, len);
                    let b2 = eloadb(ebin("add", sp, islot));
                    let ws2 = ebin("or", ebin("or", ebin("eq", b2, ec(32)), ebin("eq", b2, ec(9))), ebin("eq", b2, ec(10)));
                    let notws = ebin("eq", ws2, ec(0));
                    let go2 = ebin("mul", lt2, notws); println!("  brz v{}", go2); ckreset();
                    let one2 = ecr(1); ecopy(islot, ebin("add", islot, one2));
                    println!("  cont"); ckreset(); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                    let wlen = ebin("sub", islot, sslot);     /* word length (0 == none left -> break outer) */
                    println!("  brz v{}", wlen); ckreset();
                    let desc = halloc(8);
                    let wptr = ebin("add", sp, sslot);
                    println!("  store v{} v{}", desc, wptr);
                    println!("  store v{} v{}", ebin("add", desc, c4), wlen);
                    ecopy(wslot, desc);
                    parse_loop_body();
                    println!("  cont"); ckreset(); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                    NSYM = mark;
                    return;
                }
            }
            if istuple == 0 {                             /* TI3e-n: for p in NAME.split(SEP) -> yield sep-delimited &str [ptr,len] descriptors (single-char sep; keeps empty fields) */
                let spsave = POS;
                let mut spok = 0; let mut spdv = 0; let mut sep = 0;
                if rd_ident() {
                    let no5 = ID_O; let nl5 = ID_L; let k5 = sym_kind(no5, nl5);
                    skipws();
                    if (k5 == 1 || k5 == 3) && POS < LEN && at(POS) == 46 {
                        POS = POS + 1; rd_ident();
                        if id_is(115, 112, 108, 105, 116, 5) {   /* split(SEP) */
                            skipws(); eat_ch(40); skipws();
                            if POS < LEN && at(POS) == 34 { POS = POS + 1; sep = at(POS); while POS < LEN && at(POS) != 34 { if at(POS) == 92 { POS = POS + 1; } POS = POS + 1; } POS = POS + 1; }   /* "x" */
                            else if POS < LEN && at(POS) == 39 { POS = POS + 1; sep = at(POS); while POS < LEN && at(POS) != 39 { POS = POS + 1; } POS = POS + 1; }   /* 'x' */
                            skipws(); eat_ch(41);
                            spok = 1; spdv = sym_find(no5, nl5);
                        }
                    }
                }
                if spok == 0 { POS = spsave; }
                if spok == 1 {
                    let c4 = ecr(4);
                    let sp = eload(spdv);
                    let len = eload(ebin("add", spdv, c4));
                    let islot = NC; let z = ecr(0); ecopy(islot, z); NC = NC + 1;
                    let sslot = NC; NC = NC + 1;             /* field-start snapshot */
                    let wslot = NC; NC = NC + 1;             /* current field descriptor */
                    let mark = NSYM; sym_add(vo, vl, wslot); SK[(NSYM - 1) as usize] = 1;   /* loop var is a &str */
                    let csep = ec(sep);
                    println!("  loop"); ckreset();                       /* outer: one iteration per field */
                    ecopy(sslot, islot);                      /* field start = i */
                    println!("  loop"); ckreset();                       /* inner: advance i to next sep or end */
                    let lti = ebin("lt", islot, len);
                    let bi = eloadb(ebin("add", sp, islot));
                    let notsep = ebin("eq", ebin("eq", bi, csep), ec(0));
                    let goi = ebin("mul", lti, notsep); println!("  brz v{}", goi); ckreset();
                    let one = ecr(1); ecopy(islot, ebin("add", islot, one));
                    println!("  cont"); ckreset(); println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                    let flen = ebin("sub", islot, sslot);     /* field length */
                    let desc = halloc(8);
                    let fptr = ebin("add", sp, sslot);
                    println!("  store v{} v{}", desc, fptr);
                    println!("  store v{} v{}", ebin("add", desc, c4), flen);
                    ecopy(wslot, desc);
                    parse_loop_body();
                    println!("  cont"); ckreset();
                    let more = ebin("lt", islot, len);        /* more input after this field? */
                    println!("  brz v{}", more); ckreset();              /* i >= len -> this was the last field, exit */
                    let one2 = ecr(1); ecopy(islot, ebin("add", islot, one2));   /* skip the separator */
                    println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                    NSYM = mark;
                    return;
                }
            }
            if istuple == 0 {                             /* R2ai: for c in NAME.chunks(n) / NAME.windows(n) -> yield sub-slice [ptr,len] descriptors */
                let wsave = POS;
                let mut wok = 0; let mut wwin = 0; let mut wexact = 0;
                let mut wbuf = 0; let mut wlen = 0; let mut wn = 0;
                if rd_ident() {
                    let no3 = ID_O; let nl3 = ID_L; let k3 = sym_kind(no3, nl3);
                    skipws();
                    if (k3 == 2 || k3 == 5) && POS < LEN && at(POS) == 46 {
                        let msave = POS; POS = POS + 1; rd_ident();
                        let ischunks = id_is(99, 104, 117, 110, 107, 6);   /* chunks */
                        let isexact = id_is(99, 104, 117, 110, 107, 12);   /* chunks_exact (drops the final partial chunk) */
                        let iswin = id_is(119, 105, 110, 100, 111, 7);     /* windows */
                        if ischunks || isexact || iswin {
                            if isexact == 1 { wexact = 1; }
                            skipws(); eat_ch(40); wn = parse_expr(); eat_ch(41);
                            let dv = sym_find(no3, nl3);
                            if k3 == 2 { let c4 = ecr(4); wbuf = eload(dv); wlen = eload(ebin("add", dv, c4)); }
                            else { wbuf = dv; wlen = ec(sym_arrlen(no3, nl3)); }
                            wok = 1; if iswin { wwin = 1; }
                        } else { POS = msave; }
                    }
                }
                if wok == 0 { POS = wsave; }
                if wok == 1 {
                    let islot = NC; let z = ecr(0); ecopy(islot, z); NC = NC + 1;   /* start (chunks) / window index */
                    let dslot = NC; NC = NC + 1;                                    /* current sub-slice descriptor */
                    let mark = NSYM; sym_add(vo, vl, dslot); SK[(NSYM - 1) as usize] = 2;   /* loop var is a Vec-like slice */
                    println!("  loop"); ckreset();
                    let c4 = ecr(4);
                    let mut cnd = 0;
                    if wwin == 1 { let one = ecr(1); let cnt = ebin("add", ebin("sub", wlen, wn), one); cnd = ebin("lt", islot, cnt); }
                    else if wexact == 1 { cnd = ebin("le", ebin("add", islot, wn), wlen); }   /* chunks_exact: only full chunks -- islot + n <= len */
                    else { cnd = ebin("lt", islot, wlen); }
                    println!("  brz v{}", cnd); ckreset();
                    let desc = halloc(8);
                    let sub = ebin("add", wbuf, ebin("mul", islot, c4));
                    println!("  store v{} v{}", desc, sub);
                    let mut thislen = wn;
                    if wwin == 0 && wexact == 0 { let rem = ebin("sub", wlen, islot); let lt = ebin("lt", rem, wn); thislen = ebin("add", wn, ebin("mul", ebin("sub", rem, wn), lt)); }   /* chunks: last chunk = min(n, len-start), branchless. chunks_exact: always exactly n (no partial). */
                    println!("  store v{} v{}", ebin("add", desc, c4), thislen);
                    ecopy(dslot, desc);
                    parse_loop_body();
                    println!("  cont"); ckreset();
                    let mut step = ecr(1); if wwin == 0 { step = wn; }
                    let ni = ebin("add", islot, step); ecopy(islot, ni);
                    println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                    NSYM = mark;
                    return;
                }
            }
            let sv0 = parse_expr();
            EISITER = 0;   /* LAYER7 P-ITER-FLAG-LEAK: the for-loop CONSUMES the `.iter()` iterable -- clear the "possible cursor terminus" flag it set (line 3905) so it does not LEAK past this statement/function into a later `let mut v = Vec::new()` (which read the stale flag and mis-tagged v as a stateful-iter cursor SK=9 instead of a Vec SK=2 -> `&v` / for-iter over v broke). Cross-fn leak: go()'s for-iter over a &Vec param poisoned main's first Vec binding. */
            let s64 = EIS64;                              /* value-width: i64 range start -> the loop var is i64 */
            let earr = EISARR;                            /* R2cf: `for x in a` over a bare fixed array -> wrap in a [ptr,len] descriptor */
            let earrlen = EARRLEN; let earrelt = EARRELT;
            let sbytes = EISBYTES;                        /* R25: iterating a &[u8] -> stride-1 byte loads */
            let shm = EISHM;
            let sbtree = if EISHM == 1 && EITY >= 0 && TYTAG[EITY as usize] == 18 { 1 } else { 0 };   /* LAYER2 P6: iterating a BTreeMap -> sort the entries by key before yielding */
            let sdynvec = EISDYNVEC;                      /* A: iterating a Vec<Box<dyn>> -> loop var is a fat pointer (SK=8) */
            let mut svelt = EVELT;                         /* R1: iterating a Vec<Struct> -> loop var is struct-typed */
            let svelenum = EVELENUM;                       /* R24b: iterating a Vec<Enum> -> loop var is enum-typed (for {:?}/match) */
            let mut dvelty = DVELTY;                           /* TI3: element Ty of the iterated Vec (robust: reflects the SOURCE Vec's element type, tuple ONLY for real tuple-Vecs -- not enumerate) */
            if dvelty < 0 { if earr == 1 && EARRVTY >= 0 { dvelty = EARRVTY; } else if EISVEC == 1 && VELTY >= 0 { dvelty = VELTY; } }   /* TI3e-f: a FRESH vec!/array literal iterable (e.g. `for (a,b) in [(1,2),..]`) carries its element interned Ty in EARRVTY/VELTY, not DVELTY (which is only set for bound Vec vars). Guard dvelty<0 so a bound-var loop's correct DVELTY is never overwritten by a stale value. Lets the tuple-destructure (TYTAG==11) fire instead of the enumerate fallback. */
            let mut sv = sv0;
            if earr == 1 { let d = halloc(8); let z = ecr(0); println!("  store v{} v{}", ebin("add", d, z), sv0); let c4 = ecr(4); let cw = ec(earrlen); println!("  store v{} v{}", ebin("add", d, c4), cw); sv = d; if earrelt >= 0 { svelt = earrelt; } }   /* R2cf: array -> [ptr,len] descriptor for the collection form */
            skipws();
            if shm == 1 && istuple == 1 {                 /* S3: for (k, v) in &map -> iterate entries */
                let c4 = ecr(4); let len = eload(sv); let ents = eload(ebin("add", sv, c4));
                if sbtree == 1 { print!("  v{}: i32 = call __pair_sort v{} v{}", NC, ents, len); println!(""); NC = NC + 1; }   /* LAYER2 P6: BTreeMap -> sort entries by key ascending before iterating */
                let islot = NC; let z = ecr(0); ecopy(islot, z); NC = NC + 1;
                let kslot = NC; NC = NC + 1; let vslot = NC; NC = NC + 1;
                let mark = NSYM; sym_add(ivo, ivl, kslot); mark_forref(); sym_add(vo, vl, vslot); mark_forref();
                println!("  loop"); ckreset();
                let cnd = ebin("lt", islot, len); println!("  brz v{}", cnd); ckreset();
                let c8 = ecr(8); let ea = ebin("add", ents, ebin("mul", islot, c8));
                let kk = eload(ea); ecopy(kslot, kk);
                let c4b = ecr(4); let vv = eload(ebin("add", ea, c4b)); ecopy(vslot, vv);
                parse_loop_body();
                println!("  cont"); ckreset(); let one = ecr(1); let ni = ebin("add", islot, one); ecopy(islot, ni);
                println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                NSYM = mark;
                return;
            }
            if istuple == 0 && POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 {   /* range form: START..[=]END */
                POS = POS + 2;
                let mut incl = 0;
                if POS < LEN && at(POS) == 61 { POS = POS + 1; incl = 1; }
                let ev = parse_expr(); let ev64 = EIS64;
                let r64 = if s64 == 1 || ev64 == 1 { 1 } else { 0 };   /* value-width: i64 range -> 64-bit loop var (compare/increment/accumulate) */
                let slot = NC; if r64 == 1 { ecopy64(slot, sv); } else { ecopy(slot, sv); } NC = NC + 1;
                let mark = NSYM; sym_add(vo, vl, slot); if r64 == 1 { SI64[(NSYM - 1) as usize] = 1; }
                println!("  loop"); ckreset();
                let mut c = 0;
                if incl == 1 { c = i64op("le", slot, r64, ev, ev64, 0); } else { c = i64op("lt", slot, r64, ev, ev64, 0); }
                println!("  brz v{}", c); ckreset();
                parse_loop_body();
                println!("  cont"); ckreset();
                let one = ecr(1); let ni = if r64 == 1 { emit_binop("add", slot, one, 1, 1) } else { ebin("add", slot, one) }; if r64 == 1 { ecopy64(slot, ni); } else { ecopy(slot, ni); }
                println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
                NSYM = mark;
                return;
            }
            /* collection form: sv is a Vec descriptor -> iterate elements, binding VAR = v[i] (+ index if tuple) */
            if POS < LEN && at(POS) == 46 {               /* optional `.enumerate()` */
                let es = POS; POS = POS + 1; rd_ident();
                if id_is(101, 110, 117, 109, 101, 9) { skipws(); eat_ch(40); eat_ch(41); } else { POS = es; }
            }
            let dv = sv;
            let c4 = ecr(4); let lenad = ebin("add", dv, c4); let len = eload(lenad);
            let islot = NC; let z = ecr(0); ecopy(islot, z); NC = NC + 1;
            let xslot = NC; NC = NC + 1;
            let mut ftupty = 0 - 1;                        /* TI3: `for (a,b) in &vec_of_tuples` -> destructure (element Ty is a tuple), NOT enumerate. Robust: enumerate over a scalar Vec has a scalar element Ty. */
            if istuple == 1 && dvelty >= 0 && TYTAG[dvelty as usize] == 11 {
                TNO[0] = ivo; TNL[0] = ivl; TNO[1] = vo; TNL[1] = vl; tdn = 2; istupdest = 1; istuple = 0; ftupty = TYAUX[dvelty as usize];
            }
            let mark = NSYM;
            if istuple == 1 { sym_add(ivo, ivl, islot); }
            if istupdest == 1 {
                let mut ti = 0;
                while ti < tdn { let sl = NC; NC = NC + 1; TDSL[ti as usize] = sl; sym_add(TNO[ti as usize], TNL[ti as usize], sl);
                    SFORREF[(NSYM - 1) as usize] = 1;   /* R2: `for (i,x) in &vec_of_tuples` -- each field yields `&F` (the `&vec` element is a ref) -> `*i` is a no-op on the bound value */
                    if ftupty >= 0 { let fk = tupty_fk(ftupty, ti); if fk == 1 { SK[(NSYM - 1) as usize] = 1; } else if fk == 3 { SK[(NSYM - 1) as usize] = 3; } else if fk == 2 { SK[(NSYM - 1) as usize] = 2; } else if fk == 4 { SCHAR[(NSYM - 1) as usize] = 1; } else if fk == 5 { SI64[(NSYM - 1) as usize] = 1; } }   /* TI3: restore each destructured field's kind from the tuple Ty */
                    ti = ti + 1; }
            } else { sym_add(vo, vl, xslot); mark_forref();
                if dvelty >= 0 && TYTAG[dvelty as usize] == 3 { SK[(NSYM - 1) as usize] = 1; SFORREF[(NSYM - 1) as usize] = 0; }   /* TI3: `for s in &vec_of_&str` -- tag s as &str (s.len()/s.method()). */
                if dvelty >= 0 && TYTAG[dvelty as usize] == 4 { SK[(NSYM - 1) as usize] = 3; SFORREF[(NSYM - 1) as usize] = 0; }   /* TI3e: `for s in &vec_of_String` -- el = the {buf@0,len@4} descriptor pointer; tag s String-kind so s.len() reads eload(s+4). Enabled by the 5799 evec==0 guard that keeps DVELTY carrying the String element Ty. */
            }
            if sdynvec == 1 { SK[(NSYM - 1) as usize] = 8; }   /* A: `for s in &vec_of_dyn` -- s is a fat pointer, s.method() -> dispatch_dyn */
            if svelt >= 0 { SST[(NSYM - 1) as usize] = svelt; SFORREF[(NSYM - 1) as usize] = 0; SELEMREF[(NSYM - 1) as usize] = 1; }   /* R1: `for p in &vec_of_struct` -- p is struct-typed (p.field dispatches); S1-AST: SELEMREF move-exempt (a `&Struct` element, never moved) */
            if svelenum >= 0 && istupdest == 0 && istuple == 0 { SENUM[(NSYM - 1) as usize] = svelenum + 1; }   /* R24b: `for e in &vec_of_enum` -- e is enum-typed (`{:?}`/`match e` dispatch) */
            println!("  loop"); ckreset();
            let cnd = ebin("lt", islot, len);
            println!("  brz v{}", cnd); ckreset();
            let buf = eload(dv);
            let mut el = 0;
            if sbytes == 1 { let ea = ebin("add", buf, islot); el = eloadb(ea); }
            else { let c4b = ecr(4); let off = ebin("mul", islot, c4b); let ea = ebin("add", buf, off); el = eload(ea); }
            if istupdest == 1 {
                let mut ti = 0;
                while ti < tdn { let off = ec(ti * 4); let ad = ebin("add", el, off); let ev = eload(ad); ecopy(TDSL[ti as usize], ev); ti = ti + 1; }
            } else { ecopy(xslot, el); }
            parse_loop_body();
            println!("  cont"); ckreset();
            let one = ecr(1); let ni = ebin("add", islot, one); ecopy(islot, ni);
            println!("  jmp"); ckreset(); println!("  endloop"); ckreset();
            NSYM = mark;
            return;
        }
        if is_kw_return() {
            skipws();
            if POS < LEN && at(POS) == 59 { let z = ecr(0); POS = POS + 1; if INMAIN == 1 { emit_flush_call(); } println!("  ret v{}", z); }
            else { let mut v = parse_expr(); eat_ch(59); if CUR_RET_I64 == 1 && EIS64 == 0 { if CUR_RET_U64 == 1 { v = emit_conv(v); } else { v = emit_convs(v); } } if INMAIN == 1 { emit_flush_call(); } println!("  ret v{}", v); }   /* R2: widen a 32-bit return into an i64/u64-returning fn; R3: flush before main's explicit return */
            return;
        }
        if is_kw_break() {                                                         /* R1: `break;` / `break value;` / `break 'label;` */
            skipws();
            if POS < LEN && at(POS) == 39 {                                        /* break 'label [value] -- set that label's flag, store the value into the labeled loop's slot; brk propagates out via parse_loop_body */
                POS = POS + 1; rd_ident(); let lo = ID_O; let ll = ID_L;
                let mut k = NLBL - 1; let mut found = -1;
                loop { if k < 0 { break; } if span_eq_at(LBLO[k as usize], LBLL[k as usize], lo, ll) { found = k; break; } k = k - 1; }
                if found >= 0 { let one = ecr(1); ecopy(LBLFLAG[found as usize], one); }
                skipws();
                if POS < LEN && at(POS) != 59 {                                     /* R2ch: `break 'label value` -- assign into the LABELED loop's result slot */
                    let bv = parse_expr();
                    if found >= 0 { let ts = LBLRES[found as usize]; lvf_capture(ts); ecopy(LRES[ts as usize], bv); }
                }
                eat_ch(59); println!("  brk"); return;
            }
            if POS < LEN && at(POS) != 59 {
                let bv = parse_expr();
                if NLRES > 0 { lvf_capture(NLRES - 1); ecopy(LRES[(NLRES - 1) as usize], bv); }
            }
            eat_ch(59); println!("  brk"); return;
        }
        if is_kw_continue() {                                                      /* R2az: `continue;` / `continue 'label;` */
            skipws();
            if POS < LEN && at(POS) == 39 {
                POS = POS + 1; rd_ident(); let lo = ID_O; let ll = ID_L;
                let mut k = NLBL - 1; let mut found = -1;
                loop { if k < 0 { break; } if span_eq_at(LBLO[k as usize], LBLL[k as usize], lo, ll) { found = k; break; } k = k - 1; }
                eat_ch(59);
                if found >= 0 && LBLLVL[found as usize] == LDEPTH { println!("  cnt"); return; }
                if found >= 0 { let one = ecr(1); ecopy(LBLCFLAG[found as usize], one); }
                println!("  brk"); return;
            }
            eat_ch(59); println!("  cnt"); return;
        }
        if is_kw_unsafe() { parse_block(); return; }
        if is_print() || is_println() {
            let nlf = is_println();
            emit_print(nlf); eat_ch(59);
            return;
        }
        if is_assert2() && POS < LEN && at(POS) == 33 {                            /* assert_eq! / assert_ne! */
            let mut f = 0; if at(ID_O + 7) == 110 { f = 1; }                       /* +7 == 'n' -> assert_ne (fail when equal) */
            emit_assert_cmp(f); eat_ch(59); return;
        }
        if is_assert() && POS < LEN && at(POS) == 33 {                            /* assert! */
            emit_assert_bool(); eat_ch(59); return;
        }
        skipws();
        /* struct field write `NAME.field... = EXPR ;` (lookahead for a top-level `=` distinguishes it
           from a method-call statement `p.m(args);`) */
        if sym_styp(o, l) >= 0 && POS < LEN && at(POS) == 46 {
            let mut q = POS; let mut depth = 0; let mut isasg = 0; let mut isca = 0;
            while q < LEN {
                let ch = at(q);
                if ch == 40 || ch == 91 || ch == 123 { depth = depth + 1; q = q + 1; }
                else if ch == 41 || ch == 93 || ch == 125 { if depth == 0 { q = LEN; } else { depth = depth - 1; q = q + 1; } }
                else if depth == 0 && ch == 59 { q = LEN; }
                else if depth == 0 && (ch == 43 || ch == 45 || ch == 42 || ch == 47 || ch == 124 || ch == 38 || ch == 94) && q + 1 < LEN && at(q + 1) == 61 { isca = ch; q = LEN; }   /* R2/R2bd: field `op=` (+ - * / | & ^) */
                else if depth == 0 && ch == 61 && (q + 1 >= LEN || at(q + 1) != 61) && at(q - 1) != 60 && at(q - 1) != 62 && at(q - 1) != 33 && at(q - 1) != 61 { isasg = 1; q = LEN; }   /* R2bv: `>= <= != ==` trailing `=` is a comparison, not a field assignment (Wave-2 bug 1: `self.n==0` method tail) */
                else { q = q + 1; }
            }
            if isca != 0 {                                                          /* R2: `obj.field += / -= / *= / /= EXPR ;` (was a silent no-op) */
                let sv = sym_find(o, l); let styp2 = sym_styp(o, l);
                FLV_I64 = 0; let ad = field_lvalue(sv, styp2);
                skipws(); POS = POS + 2; let e = parse_expr(); let e64 = EIS64; let f64 = FLV_I64;   /* value-width: 64-bit field op= (was 32-bit or/store -> dropped bit 32-63, e.g. bitset `bits |= 1u64<<63`) */
                let cur = if f64 == 1 { eload8(ad) } else { eload(ad) }; let mut nv = cur;
                if isca == 43 { nv = emit_binop("add", cur, e, f64, e64); } else if isca == 45 { nv = emit_binop("sub", cur, e, f64, e64); } else if isca == 42 { nv = emit_binop("mul", cur, e, f64, e64); } else if isca == 47 { nv = emit_binop("div", cur, e, f64, e64); } else if isca == 124 { nv = emit_binop("or", cur, e, f64, e64); } else if isca == 38 { nv = emit_binop("and", cur, e, f64, e64); } else { nv = emit_binop("xor", cur, e, f64, e64); }
                eat_ch(59);
                if f64 == 1 { println!("  store8 v{} v{}", ad, nv); } else { println!("  store v{} v{}", ad, nv); }
                return;
            }
            if isasg == 1 {
                let sv = sym_find(o, l); let styp2 = sym_styp(o, l);
                let ad = field_lvalue(sv, styp2);
                eat_ch(61); let e = parse_expr(); eat_ch(59);
                println!("  store v{} v{}", ad, e);
                return;
            }
            POS = save; let v = parse_expr(); skipws(); if eat_ch(59) { let d = v; } else { skipws(); if POS < LEN && at(POS) == 125 { emit_tail(v); } }
            return;
        }
        /* R11: ARRAY[i] = EXPR ; (local fixed-array element store) */
        if POS < LEN && at(POS) == 91 && sym_kind(o, l) == 5 {
            let av = sym_find(o, l); let e64 = SARR64[sym_idx(o, l) as usize];
            POS = POS + 1; let iv = parse_expr(); eat_ch(93); skipws();
            let more0 = POS < LEN && at(POS) == 91;
            let st64 = if e64 == 1 && more0 == false { 1 } else { 0 };
            let cw0 = if st64 == 1 { ecr(8) } else { ecr(4) };
            let mut ad = ebin("add", av, ebin("mul", iv, cw0));
            while POS < LEN && at(POS) == 91 { let base = eload(ad); POS = POS + 1; let jv = parse_expr(); eat_ch(93); let c4b = ecr(4); ad = ebin("add", base, ebin("mul", jv, c4b)); skipws(); }   /* R2: nested a[i][j] -- load inner-array ptr, index again */
            if POS < LEN && at(POS) == 61 && (POS + 1 >= LEN || at(POS + 1) != 61) {
                POS = POS + 1; let e = parse_expr(); eat_ch(59);
                if st64 == 1 { println!("  store8 v{} v{}", ad, e); } else { println!("  store v{} v{}", ad, e); }
                return;
            }
            if POS + 1 < LEN && at(POS + 1) == 61 && (at(POS) == 43 || at(POS) == 45 || at(POS) == 42 || at(POS) == 47) {   /* R2: a[i] op= e (compound element assign) */
                let opc = at(POS); POS = POS + 2; let e = parse_expr(); eat_ch(59);
                let cur = eload(ad); let mut nv = 0;
                if opc == 43 { nv = ebin("add", cur, e); } else if opc == 45 { nv = ebin("sub", cur, e); } else if opc == 42 { nv = ebin("mul", cur, e); } else { nv = ebin("div", cur, e); }
                println!("  store v{} v{}", ad, nv);
                return;
            }
            POS = save; let v = parse_expr(); skipws(); if eat_ch(59) { let d = v; } else { skipws(); if POS < LEN && at(POS) == 125 { emit_tail(v); } } return;
        }
        /* R4: VEC[i] = EXPR ; (local Vec element store) */
        if POS < LEN && at(POS) == 91 && sym_kind(o, l) == 2 {
            let dv = sym_find(o, l);
            POS = POS + 1; let iv = parse_expr(); eat_ch(93); skipws();
            let p = eload(dv); let c4 = ecr(4); let off = ebin("mul", iv, c4); let mut ad = ebin("add", p, off);
            while POS < LEN && at(POS) == 91 {                                    /* R2bp: Vec<Vec<..>> chained store m[i][j].. = e -- deref inner Vec descriptor */
                let inner_desc = eload(ad); let inner_buf = eload(inner_desc);
                POS = POS + 1; let jv = parse_expr(); eat_ch(93); skipws();
                let c4b = ecr(4); ad = ebin("add", inner_buf, ebin("mul", jv, c4b));
            }
            if POS < LEN && at(POS) == 61 && (POS + 1 >= LEN || at(POS + 1) != 61) {
                POS = POS + 1; let e = parse_expr(); eat_ch(59);
                println!("  store v{} v{}", ad, e);
                return;
            }
            POS = save; let v = parse_expr(); skipws(); if eat_ch(59) { let d = v; } else { skipws(); if POS < LEN && at(POS) == 125 { emit_tail(v); } } return;
        }
        /* GLOBAL_ARRAY[i] = EXPR ; */
        if POS < LEN && at(POS) == 91 && g_find(o, l) >= 0 {
            POS = POS + 1; let iv = parse_expr(); eat_ch(93); skipws();
            if POS < LEN && at(POS) == 61 && (POS + 1 >= LEN || at(POS + 1) != 61) {
                POS = POS + 1; let e = parse_expr(); eat_ch(59);
                let gi = g_find(o, l); let base = egref(o, l); let es = GE[gi];
                let mut off = iv;
                if es != 1 { let c = ec(es); off = ebin("mul", iv, c); }
                let ad = ebin("add", base, off);
                if es == 1 { println!("  storeb v{} v{}", ad, e); } else { if es == 8 { println!("  store8 v{} v{}", ad, e); } else { println!("  store v{} v{}", ad, e); } }
                return;
            }
            POS = save; let v = parse_expr(); skipws(); if eat_ch(59) { let d = v; } else { skipws(); if POS < LEN && at(POS) == 125 { emit_tail(v); } } return;
        }
        /* NAME = EXPR ;  (global scalar or local) */
        if POS + 2 < LEN && at(POS + 2) == 61 && ((at(POS) == 60 && at(POS + 1) == 60) || (at(POS) == 62 && at(POS + 1) == 62)) {   /* R2bd: local `t <<= / >>= e` */
            let shc = at(POS); POS = POS + 3; let vi64s = sym_i64(o, l);
            let gwi = g_find(o, l); let mut vi64s = vi64s; if gwi >= 0 { if GE[gwi as usize] == 8 { vi64s = 1; } }   /* C-S0-I64-INDEXED-32: a global's width is in GE, not sym_i64 */
            let e = parse_expr(); eat_ch(59);
            let gi = g_find(o, l);
            let mut base = 0;
            if gi >= 0 { base = egref(o, l); } else { let slot = sym_find(o, l); base = eref(slot); }
            let mut cur = 0;
            if vi64s == 1 { println!("  v{}: i64 = load8 v{}", NC, base); cur = NC; NC = NC + 1; } else { cur = eload(base); }
            let mut nv = 0;
            if vi64s == 1 {
                if shc == 60 { nv = ebin8raw("shl", cur, e); } else { nv = ebin8raw("shr", cur, e); }
                println!("  store8 v{} v{}", base, nv);
            } else {
                if shc == 60 { nv = ebin("shl", cur, e); } else { nv = ebin("shr", cur, e); }
                println!("  store v{} v{}", base, nv);
            }
            return;
        }
        if POS + 1 < LEN && at(POS + 1) == 61 && (at(POS) == 43 || at(POS) == 45 || at(POS) == 42 || at(POS) == 47 || at(POS) == 124 || at(POS) == 38 || at(POS) == 94) {   /* R1/R2bd: local `t += -= *= /= |= &= ^= e` -- |= &= ^= were silent no-ops */
            let opc0 = at(POS); let sty0 = sym_styp(o, l);
            if sty0 >= 0 && (opc0 == 43 || opc0 == 45 || opc0 == 42 || opc0 == 47) {   /* operator-trait: struct `a += b` -> impl AddAssign etc: call Type_<op>_assign(&mut self, rhs) */
                POS = POS + 2; let rhs = parse_expr(); eat_ch(59);
                let selfp = sym_find(o, l);
                print!("  v{}: i32 = call ", NC); emit_span(TSO[sty0 as usize], TSL[sty0 as usize]);
                if opc0 == 43 { print!("_add_assign"); } else if opc0 == 45 { print!("_sub_assign"); } else if opc0 == 42 { print!("_mul_assign"); } else { print!("_div_assign"); }
                println!(" v{} v{}", selfp, rhs); NC = NC + 1;
                return;
            }
            let opc = at(POS); POS = POS + 2; let vi64 = sym_i64(o, l);
            let gwi = g_find(o, l); let mut vi64 = vi64; if gwi >= 0 { if GE[gwi as usize] == 8 { vi64 = 1; } }   /* C-S0-I64-INDEXED-32: a global's width is in GE, not sym_i64 */
            let e = parse_expr(); eat_ch(59);
            let gi = g_find(o, l);
            let mut base = 0;
            if gi >= 0 { base = egref(o, l); } else { let slot = sym_find(o, l); if sym_byref(o, l) == 1 { base = slot; } else { base = eref(slot); } }   /* R2bg: byref -> slot IS the address */
            let mut cur = 0;
            if vi64 == 1 { println!("  v{}: i64 = load8 v{}", NC, base); cur = NC; NC = NC + 1; } else { cur = eload(base); }
            let mut nv = 0;
            if vi64 == 1 {
                if opc == 43 { nv = ebin8raw("add", cur, e); } else if opc == 45 { nv = ebin8raw("sub", cur, e); } else if opc == 42 { nv = ebin8raw("mul", cur, e); } else if opc == 47 { nv = ebin8raw("div", cur, e); } else if opc == 124 { nv = ebin8raw("or", cur, e); } else if opc == 38 { nv = ebin8raw("and", cur, e); } else { nv = ebin8raw("xor", cur, e); }
                println!("  store8 v{} v{}", base, nv);
            } else {
                if opc == 43 { nv = ebin("add", cur, e); } else if opc == 45 { nv = ebin("sub", cur, e); } else if opc == 42 { nv = ebin("mul", cur, e); } else if opc == 47 { nv = ebin("div", cur, e); } else if opc == 124 { nv = ebin("or", cur, e); } else if opc == 38 { nv = ebin("and", cur, e); } else { nv = ebin("xor", cur, e); }
                println!("  store v{} v{}", base, nv);
            }
            return;
        }
        if POS < LEN && at(POS) == 61 && (POS + 1 >= LEN || at(POS + 1) != 61) {
            let vi64 = sym_i64(o, l);
            let gwi = g_find(o, l); let mut vi64 = vi64; if gwi >= 0 { if GE[gwi as usize] == 8 { vi64 = 1; } }   /* C-S0-I64-INDEXED-32: a global's width is in GE, not sym_i64 */
            POS = POS + 1; let e = parse_expr(); eat_ch(59);
            mark_unmoved(o, l);   /* MOVEBORROW: `NAME = expr` reassigns -> NAME is no longer moved (legal mut rebind after a move). LAYER7: un-mark AFTER the RHS so `lhs = f(lhs)` (self-consuming reassign, e.g. Pratt `lhs = Box::new(Bin(op,lhs,rhs))`) reinitializes lhs rather than the RHS move leaving it moved. */
            let gi = g_find(o, l);
            let mut base = 0;
            if gi >= 0 { base = egref(o, l); } else { let slot = sym_find(o, l); if sym_byref(o, l) == 1 { base = slot; } else { base = eref(slot); } }   /* R2bg: byref -> slot IS the address */
            if vi64 == 1 { println!("  store8 v{} v{}", base, e); } else { println!("  store v{} v{}", base, e); }
            return;
        }
        /* expression statement (e.g. a call) -- rewind and parse */
        POS = save;
        let v = parse_expr();
        skipws();
        if eat_ch(59) { let d = v; } else { skipws(); if POS < LEN && at(POS) == 125 { emit_tail(v); } }
    }
}

/* POS-SANITY (P-ACCUMULATED-STATE-PARSE-DRIFT): find the matching `}` for the `{` at `open`, LEXER-AWARE
   (skip_tok skips strings/chars/comments as single tokens, so `{`/`}` inside `"..{}.."`/`'}'`/`// }` are not
   counted). Returns the index of the matching `}`, or -1 if unbalanced/EOF (then the guard is disabled --
   NEVER false-fire on valid code). POS is restored. */
fn find_block_end(open: i32) -> i32 {
    unsafe {
        let save = POS; POS = open;
        let mut d = 0;
        loop {
            skipws();
            if POS >= LEN { POS = save; return 0 - 1; }
            let c = at(POS);
            if c == 123 { d = d + 1; POS = POS + 1; }
            else if c == 125 { d = d - 1; POS = POS + 1; if d == 0 { let e = POS - 1; POS = save; return e; } }
            else { skip_tok(); }
        }
    }
}
fn die_parse_drift(blk: i32) { unsafe {   /* POS-SANITY: a statement over-consumed PAST its enclosing block's `}` -> a parse desync. Fail CLEANLY at the DRIFT SITE (naming the block) instead of letting POS run into later items and mis-parse a where-clause/signature ~100 lines downstream (the slippery accumulated-state class). */
    eputc(101); eputc(114); eputc(114); eputc(111); eputc(114); eputc(58); eputc(32);   /* "error: " */
    eputc(112); eputc(97); eputc(114); eputc(115); eputc(101); eputc(32);               /* "parse " */
    eputc(100); eputc(114); eputc(105); eputc(102); eputc(116); eputc(32);              /* "drift " */
    __syscall(1, 2, &SRC[blk as usize], 48);                                            /* 48 bytes at the block open -- identifies the over-consuming body */
    eput_at(blk);
    eputc(10);
    __syscall(60, 1, 0, 0);
} }
fn parse_block() {
    unsafe {
        let blkopen = POS;
        let blkend = find_block_end(POS);
        eat_ch(123);
        let mark = NSYM;
        loop {
            skipws();
            if POS >= LEN { break; }
            if at(POS) == 125 { POS = POS + 1; break; }
            if blkend >= 0 && POS > blkend { die_parse_drift(blkopen); }   /* over-consumed past this block's `}` */
            let before = POS;
            parse_stmt();
            if POS == before { POS = POS + 1; }   /* safety: never spin on an unhandled construct */
        }
        NSYM = mark;
    }
}

/* parse one function: fn NAME ( params ) [-> RET] { body } */
/* params (pos at `(`) + optional `-> RET` + body. self_ty >= 0 injects `&self` bound to that struct type. */
fn parse_pb(self_ty: i32) {
    unsafe {
        CUR_RET_I64 = 0; CUR_RET_U64 = 0; CUR_RET_RES = 0;   /* R2: reset per-fn i64-return flag before parsing this fn's signature; CHECK N4: reset the Result/Option return kind */
        /* R-either: pre-scan a `where F: Fn(..)` clause BEFORE the params, so a `f: F` param is tagged as a
           closure (SK=4). The where-clause follows the params in source, but the param loop checks
           is_fn_generic on `f: F` first -- so look ahead (paren-depth aware) and append its Fn bounds now. */
        { let mut p = POS; let mut pd = 0;
          loop {
            if p >= LEN { break; }
            let c = at(p);
            if c == 40 { pd = pd + 1; }
            else if c == 41 { pd = pd - 1; }
            else if pd == 0 && c == 123 { break; }   /* body brace -> no where clause */
            else if pd == 0 && c == 119 && p + 4 < LEN && at(p + 1) == 104 && at(p + 2) == 101 && at(p + 3) == 114 && at(p + 4) == 101 && (p + 5 >= LEN || !is_idc(at(p + 5))) && (p == 0 || !is_idc(at(p - 1))) {
                let ws = p + 5; let mut we = ws; while we < LEN && at(we) != 123 { we = we + 1; }
                scan_where_fn_bounds(ws, we); break;
            }
            p = p + 1;
          } }
        eat_ch(40);
        if self_ty >= 0 || IMPL_SELF_ENUM >= 0 || BLANK_PRIM_SELF == 1 {
            /* consume `&self`/`self`/`&mut self` as the first param */
            skipws(); let mut selfref = 0; if POS < LEN && at(POS) == 38 { POS = POS + 1; selfref = 1; if is_kw_mut_next() { } }
            if selfref == 0 { is_kw_mut_next(); }   /* R1: `mut self` by-value -- skip the leading `mut` (was: rd_ident read `mut` -> is_kw_self false -> self unbound = ref v-1) */
            skipws(); let sp = POS;
            if rd_ident() && is_kw_self() {
                println!("  v{}: i32 = param", NC);
                sym_add(ID_O, ID_L, NC);
                if self_ty >= 0 { SST[(NSYM - 1) as usize] = self_ty; }
                if BLANK_PRIM_SELF == 1 { SFORREF[(NSYM - 1) as usize] = 1; }   /* R1 keystone: scalar self passed by value -> `*self` is a no-op */
                if IMPL_SELF_ENUM >= 0 { SENUM[(NSYM - 1) as usize] = IMPL_SELF_ENUM + 1; if selfref == 1 { SFORREF[(NSYM - 1) as usize] = 1; } }   /* R2: enum method -- self is a (by-ref) enum for `match self` */
                NC = NC + 1;
                skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; }
            } else { POS = sp; }
        }
        skipws();
        if peek() != 41 {
            loop {
                let mut pkind = 0; let mut psty = -1; let mut pi64 = 0; let mut pu = 0; let mut pen = 0 - 1; let mut pisref = 0; let mut pfnbool = 0; let mut pu8 = 0; let mut pinto = 0 - 1; let mut pvecvec = 0; let mut popt = 0; let mut popres = 0; let mut pbool = 0; let mut pdyn = 0;
                let mut pcg = 0; let mut cgo = 0; let mut cgl = 0;   /* P-MONO-CONST-GENERIC: this param is `[T; N]` with a const-generic length name N (cgo/cgl span) -> bind N = the array descriptor's len field */
                let mut pelt = 0 - 1;   /* R30b: `&[Struct]`/`&Vec<Struct>` param -- element struct-type, so `s.iter().map(|x| x.field/method)` / `s[i].field` dispatch */
                rd_ident();
                if is_kw_mut() { skipws(); rd_ident(); }   /* R2ck: `mut name: T` param binding -- skip the `mut`, the name follows */
                let po = ID_O; let pl = ID_L;
                skipws();
                if POS < LEN && at(POS) == 58 { POS = POS + 1;
                    skipws(); let mut pref = 0; if POS < LEN && at(POS) == 38 { POS = POS + 1; pref = 1; pisref = 1; skipws(); if POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); } if is_kw_mut_next() { } }   /* P1: &'a T -> erase 'a */
                    if POS < LEN && at(POS) == 42 { POS = POS + 1; skipws(); let mps = POS; if rd_ident() && (is_kw_mut() || is_kw_const()) { skipws(); } else { POS = mps; } }   /* P-TYPE-RAW-POINTER: `*mut T`/`*const T` param -- consume the raw-pointer prefix; the pointer is a plain scalar machine pointer (NOT pisref -- `*p` must do a real eload, unlike a value-transparent `&scalar`), the inner type T parses below as a scalar. rustc `Vec::from_raw_parts(ptr: *mut T, ..)`. */
                    skipws();
                    if POS + 2 < LEN && at(POS) == 100 && at(POS + 1) == 121 && at(POS + 2) == 110 && (POS + 3 >= LEN || !is_idc(at(POS + 3))) { pkind = 8; POS = POS + 3; skipws(); rd_ident(); if ID_L == 2 && at(ID_O) == 70 && at(ID_O + 1) == 110 { pkind = 4; let mut d = 0; loop { if POS >= LEN { break; } let ch = at(POS); if ch == 40 || ch == 91 || ch == 123 { d = d + 1; POS = POS + 1; } else if ch == 41 || ch == 93 || ch == 125 { if d == 0 { break; } d = d - 1; POS = POS + 1; } else if d == 0 && ch == 44 { break; } else { POS = POS + 1; } } } }   /* exotica: &dyn Trait param -> pkind 8; W1b: &dyn Fn(..) -> closure param pkind 4 */
                    skipws();
                    if POS < LEN && at(POS) == 40 {                                                                 /* TUPLE-PARAM: a tuple type `(A, B, ..)` in param position -- consume the balanced parens (opaque, pkind stays 0). Was unhandled: the `(` desynced the param loop -> parse-drift (`fn f(x: (u32,u32))`; library core slice split_point_of). */
                        let mut td = 1; POS = POS + 1;
                        while POS < LEN && td > 0 { if at(POS) == 40 { td = td + 1; } if at(POS) == 41 { td = td - 1; } POS = POS + 1; }
                    } else if POS < LEN && at(POS) == 91 {                                                                 /* R25/E2/E3: &[u8] byte slice=6; &[T] i32-slice=2 (Vec-like); [T;N] value array=5; A: [T; N] const-generic=2 */
                        POS = POS + 1; skipws();
                        { let mut dp = POS; if dp < LEN && at(dp) == 38 { dp = dp + 1; while dp < LEN && at(dp) == 32 { dp = dp + 1; } } if dp + 2 < LEN && at(dp) == 100 && at(dp + 1) == 121 && at(dp + 2) == 110 && (dp + 3 >= LEN || !is_idc(at(dp + 3))) { pdyn = 1; } }   /* D003 P-SLICE-DYN-PARAM: a `&[&dyn Trait]` param -- element is a dyn fat pointer (SVECDYN -> DVELT sentinel -5 so `items[i].method()` dispatches per-element via dispatch_dyn) */
                        let es2 = POS; rd_ident();
                        let mut isbyte = 0;
                        if ID_L == 2 && (at(ID_O) == 117 || at(ID_O) == 105) && at(ID_O + 1) == 56 { isbyte = 1; }   /* u8 / i8 */
                        pelt = ty_find(ID_O, ID_L);   /* R30b: `&[Struct]` element struct-type */
                        if GSUB_ACT == 1 && GSUB_STYP >= 0 && span_eq_at(ID_O, ID_L, GSUB_TPO, GSUB_TPL) { pelt = GSUB_STYP; }   /* R30b: `&[T]` in a monomorph -> the substituted struct type */
                        POS = es2;
                        let mut constgen = 0; let mut d = 1;
                        while POS < LEN && d > 0 { if at(POS) == 91 { d = d + 1; } if at(POS) == 93 { d = d - 1; }
                            if d == 1 && at(POS) == 59 { let mut q = POS + 1; while q < LEN && (at(q) == 32 || at(q) == 9) { q = q + 1; } if q < LEN && is_idst(at(q)) { constgen = 1; cgo = q; let mut qe = q; while qe < LEN && is_idc(at(qe)) { qe = qe + 1; } cgl = qe - q; pcg = 1; } }   /* A: `[T; N]` length is an identifier -> const generic; capture the length name N */
                            POS = POS + 1; }
                        if pref == 1 { if isbyte == 1 { pkind = 6; } else { pkind = 2; } } else if constgen == 1 { pkind = 2; } else { pkind = 5; }   /* A: const-generic array -> slice descriptor [ptr,len] */
                    } else {
                    rd_ident();
                    skipws(); while POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); rd_ident(); skipws(); }   /* R-either: a `::`-qualified param type (`std::vec::Vec<u8>`, `std::string::String`) -- advance ID to the LAST segment so the kind checks + the `<..>` skip below fire on it (was: read only `std`, then the `::vec::Vec<u8>` tail was left -> the param loop broke, eat_ch(41) desynced -> parse-drift). */
                    if ID_L == 3 && at(ID_O) == 115 && at(ID_O + 1) == 116 && at(ID_O + 2) == 114 { pkind = 1; }   /* &str */
                    if ID_L == 3 && at(ID_O) == 105 && at(ID_O + 1) == 54 && at(ID_O + 2) == 52 { pi64 = 1; }     /* i64 */
                    if ID_L == 3 && at(ID_O) == 117 && at(ID_O + 1) == 54 && at(ID_O + 2) == 52 { pi64 = 1; pu = 1; }     /* u64 */
                    if ID_L == 2 && at(ID_O) == 117 && at(ID_O + 1) == 56 { pu8 = 1; }     /* value-width: u8 param -> SU8 (arith masks 0xFF) */
                    if ID_L == 4 && at(ID_O) == 98 && at(ID_O + 1) == 111 && at(ID_O + 2) == 111 && at(ID_O + 3) == 108 { pbool = 1; }   /* SPAN-4 P-BOOL-PARAM-NOT-TAGGED: a `b: bool` param -> SBOOL so a read tags EISBOOL, so `!b` uses the bool NOT (`b==0`) not the integer NOT (`0-b-1` = -2). A bool LOCAL already sets SBOOL (7774); the param path forgot it -> `if !b {..}` on a param mis-branched (`!true`=-2 truthy). */
                    if id_is(79, 112, 116, 105, 111, 6) { popt = 1; popres = 0; }   /* CHECK N4: `Option<..>` param -> SISOPT=1, SOPTRES=0 (so a read sets wasres=0 -> `?` in a Result fn rejects) */
                    else if id_is(82, 101, 115, 117, 108, 6) { popt = 1; popres = 1; }   /* CHECK N4: `Result<..>` param -> SISOPT=1, SOPTRES=1 (so a read sets wasres=1 -> `?` never false-rejects) */
                    if is_vec_ty(ID_O, ID_L) { pkind = 2; skipws(); if POS < LEN && at(POS) == 60 { scan_ta_kind(POS + 1); if ANNGAK == 2 { pvecvec = 1; }
                        let mut q = POS + 1; while q < LEN && (at(q) == 32 || at(q) == 9 || at(q) == 38) { q = q + 1; }   /* R31b: `&Vec<Struct>` element struct-type -> SVELT (skip ws + a `&` for Vec<&T>) */
                        if q < LEN && is_idst(at(q)) { let mut eo = q; let mut qe = q; while qe < LEN && is_idc(at(qe)) { qe = qe + 1; } let mut el = qe - q;
                            if is_box_ident(eo, el) && qe < LEN && at(qe) == 60 { let mut bo = qe + 1; while bo < LEN && (at(bo) == 32 || at(bo) == 38) { bo = bo + 1; } let mut be = bo; while be < LEN && is_idc(at(be)) { be = be + 1; } eo = bo; qe = be; el = be - bo; }   /* LAYER7 P-VEC-STRUCT-ELEM (param): see THROUGH `&Vec<Box<Struct>>` / `&Vec<P<Struct>>` to the inner struct so the param element is struct-typed (mirror the annotation annvelstruct fix). */
                            let est = ty_find(eo, el); if est >= 0 { pelt = est; } if GSUB_ACT == 1 && GSUB_STYP >= 0 && span_eq_at(eo, el, GSUB_TPO, GSUB_TPL) { pelt = GSUB_STYP; } }
                    } }   /* &Vec; P-VECVEC: `&Vec<Vec<..>>` -> element is a Vec descriptor (SVELT=-3) so `m[r].len()`/`m[r].iter()` dispatch */
                    if is_string_ty(ID_O, ID_L) { pkind = 3; }                                                     /* &String */
                    if ID_L == 7 && at(ID_O) == 72 && at(ID_O + 1) == 97 && at(ID_O + 2) == 115 && at(ID_O + 3) == 104 && at(ID_O + 4) == 77 { pkind = 7; }   /* R2bo: &mut HashMap param -> descriptor pointer (dispatch as HashMap) */
                    if ID_L == 5 && at(ID_O) == 67 && at(ID_O + 1) == 104 && at(ID_O + 2) == 97 && at(ID_O + 3) == 114 && at(ID_O + 4) == 115 { pkind = 13; }   /* LAYER4 P-CHARS-ITER: a `Chars`/`&mut Chars<'_>` param is the char-cursor cell [buf,len,pos] -> SK=13 so `chars.next()/.clone()/.as_str()` dispatch through dispatch_char_iter (was unrecognized -> `chars.next()` parse-drifted; rustc_lexer unescape scan_unicode) */
                    psty = ty_find(ID_O, ID_L);
                    pen = enum_find(ID_O, ID_L);                                                                     /* R2: &Enum param -> scrutinee is transparent, by-ref match */
                    if GSUB_ACT == 1 && GSUB_STYP >= 0 && span_eq_at(ID_O, ID_L, GSUB_TPO, GSUB_TPL) { psty = GSUB_STYP; }   /* R9: T -> concrete struct */
                    if GSUB_ACT == 1 && GSUB_STYP == 0 - 3 && span_eq_at(ID_O, ID_L, GSUB_TPO, GSUB_TPL) { pkind = 1; }   /* KP0.5: T -> &str instance -> the T-param is a &str */
                    if GSUB_ACT == 1 && GSUB_STYP == 0 - 4 && span_eq_at(ID_O, ID_L, GSUB_TPO, GSUB_TPL) { pkind = 3; }   /* KP0.5: T -> String instance */
                    if is_fn_generic(ID_O, ID_L) == 1 { pkind = 4; if fn_generic_ret_bool(ID_O, ID_L) == 1 { pfnbool = 1; } }   /* W1b: `f: F` where <F: Fn(..)> -> closure param (fat pointer); std-hoist: remember if F returns bool */
                    if ID_L == 4 && at(ID_O) == 105 && at(ID_O + 1) == 109 && at(ID_O + 2) == 112 && at(ID_O + 3) == 108 {   /* `impl Trait` param */
                        let gsub_impl = if GSUB_ACT == 1 && span_eq_at(ID_O, ID_L, GSUB_TPO, GSUB_TPL) { 1 } else { 0 };   /* TRAIT R2: this `impl Display/Debug` param IS the monomorph's substituted type -> the GSUB kind above wins, don't force pkind=4 */
                        skipws(); let isave = POS; rd_ident();
                        if ID_L == 4 && at(ID_O) == 73 && at(ID_O + 1) == 110 && at(ID_O + 2) == 116 && at(ID_O + 3) == 111 {   /* `impl Into<T>` -> value param whose `.into()` targets T (From/Into) */
                            skipws(); if POS < LEN && at(POS) == 60 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; } rd_ident(); pinto = ty_find(ID_O, ID_L); while POS < LEN && at(POS) != 62 { POS = POS + 1; } if POS < LEN { POS = POS + 1; } }
                        } else {                                                       /* impl Display/Debug/Fn.. -> generic/closure param */
                            POS = isave; if gsub_impl == 0 { pkind = 4; }
                            let mut d = 0;
                            loop { if POS >= LEN { break; } let ch = at(POS);
                                if ch == 40 || ch == 91 || ch == 123 { d = d + 1; POS = POS + 1; }
                                else if ch == 41 || ch == 93 || ch == 125 { if d == 0 { break; } d = d - 1; POS = POS + 1; }
                                else if d == 0 && ch == 44 { break; } else { POS = POS + 1; } }
                        }
                    }
                    else if (ID_L == 2 && at(ID_O) == 70 && at(ID_O + 1) == 110) || (ID_L == 2 && at(ID_O) == 102 && at(ID_O + 1) == 110) {   /* Fn / fn(..) fn-pointer -> closure param */
                        pkind = 4;
                        let mut d = 0;
                        loop { if POS >= LEN { break; } let ch = at(POS);
                            if ch == 40 || ch == 91 || ch == 123 { d = d + 1; POS = POS + 1; }
                            else if ch == 41 || ch == 93 || ch == 125 { if d == 0 { break; } d = d - 1; POS = POS + 1; }
                            else if d == 0 && ch == 44 { break; } else { POS = POS + 1; } }
                    }
                    skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 45 && POS + 1 < LEN && at(POS + 1) == 62 { POS = POS + 2; } else { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }   /* LAYER4: ->-aware param-type generic skip -- a fn-ptr type in a generic `Option<fn(bool) -> B>` has a `->` whose `>` must NOT close the generic (was: closed early at the `->`, leaving `B>` -> parse drift; desynced c_or_byte_string in rustc_lexer) */
                    } }
                println!("  v{}: i32 = param", NC);
                sym_add(po, pl, NC);
                if pkind > 0 { SK[(NSYM - 1) as usize] = pkind; }
                if pkind == 2 && pvecvec == 1 { SVELT[(NSYM - 1) as usize] = 0 - 3; }   /* P-VECVEC: &Vec<Vec<..>> param -- element is a Vec descriptor */
                if pkind == 2 && pelt >= 0 { SVELT[(NSYM - 1) as usize] = pelt; }   /* R30b: &[Struct] param -- element struct-type (s.iter().map(|x| x.field/method)) */
                if pfnbool == 1 { SBOOL[(NSYM - 1) as usize] = 1; }   /* std-hoist: closure param whose Fn bound returns bool -> `f(x)` calli result is a bool */
                if pdyn == 1 && pkind == 2 { SVECDYN[(NSYM - 1) as usize] = 1; }   /* D003 P-SLICE-DYN-PARAM: a `&[&dyn Trait]` param -> mark the slice's elements dyn fat pointers (the Vec-local dispatch threads SVECDYN -> DVELT -5 -> dispatch_dyn per element) */
                if pbool == 1 { SBOOL[(NSYM - 1) as usize] = 1; }   /* SPAN-4 P-BOOL-PARAM-NOT-TAGGED: a `bool` param is bool-typed -> tag SBOOL (mirrors the bool-local path at the let-binding) */
                if psty >= 0 { SST[(NSYM - 1) as usize] = psty; }
                if pi64 == 1 { SI64[(NSYM - 1) as usize] = 1; }
                if pu == 1 { SU64[(NSYM - 1) as usize] = 1; }
                if pu8 == 1 { SU8[(NSYM - 1) as usize] = 1; }   /* value-width: u8 param */
                if pinto >= 0 { SINTO[(NSYM - 1) as usize] = pinto; }   /* From/Into: `impl Into<T>` param -> `.into()` targets T */
                if popt == 1 { SISOPT[(NSYM - 1) as usize] = 1; SOPTRES[(NSYM - 1) as usize] = popres; }   /* CHECK N4: Option/Result param -> record is-Option + Result-vs-Option so `?` distinguishes the operand */
                let mut pty = 0; if pi64 == 1 { pty = 1; } else if pkind == 1 { pty = 2; } else if pkind == 3 { pty = 3; } else if pkind == 2 { pty = ty_intern(8, 0, 0); } else if psty >= 0 { pty = ty_intern(12, 0, psty); } STY[(NSYM - 1) as usize] = pty;   /* TI2: params carry their interned Ty (the call boundary; prerequisite for call/return propagation) */
                if pen >= 0 { SENUM[(NSYM - 1) as usize] = pen + 1; }                       /* R2: enum-typed param */
                if pen >= 0 && pisref == 1 { SFORREF[(NSYM - 1) as usize] = 1; }            /* R2: &Enum param is a reference -> by-ref match, `*v` no-op */
                if psty >= 0 && pisref == 1 { SFORREF[(NSYM - 1) as usize] = 1; }            /* LAYER11 P-MUT-REBORROW-MOVE: a `&Struct`/`&mut Struct` param is a REFERENCE -> mark SFORREF so a bare read is a REBORROW, not a move (the move-check at the struct-read path skips SFORREF params). Was: passing a `&mut Body` param to two fns `cp(b); dce(b)` (a pass-manager idiom) falsely flagged `b` as moved-then-used -> die_moved. */
                NC = NC + 1;
                if pcg == 1 {                                                              /* P-MONO-CONST-GENERIC: bind the const-generic length name N = the array param descriptor's len (at +4), so `N` used as a value in the body resolves (was unbound -> ret v-1) */
                    let desc = NC - 1; let c4 = ecr(4); let lenad = ebin("add", desc, c4);
                    println!("  v{}: i32 = load v{}", NC, lenad); let nval = NC; NC = NC + 1;
                    sym_add(cgo, cgl, nval);
                }
                if !eat_ch(44) { break; }
            }
        }
        eat_ch(41);
        skipws();
        if POS + 1 < LEN && at(POS) == 45 && at(POS + 1) == 62 { POS = POS + 2; skipws();
            if POS < LEN && at(POS) == 40 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } POS = POS + 1; } }   /* R23: tuple return type (T, T) */
            else if POS < LEN && at(POS) == 91 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 91 { d = d + 1; } if at(POS) == 93 { d = d - 1; } POS = POS + 1; } }   /* E2: [T; N] array return type */
            else { if POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); skipws(); } if is_kw_mut_next() { } } rd_ident();   /* R2: `-> &mut T` / `-> &'a T` -- skip the ref, lifetime, and `mut` before the type */
            if ID_L == 3 && at(ID_O + 1) == 54 && at(ID_O + 2) == 52 { if at(ID_O) == 105 { CUR_RET_I64 = 1; } else if at(ID_O) == 117 { CUR_RET_I64 = 1; CUR_RET_U64 = 1; } }   /* R2: `-> i64` / `-> u64` return type -- sign/zero-extend negatives on the ret path */
            if id_is(82, 101, 115, 117, 108, 6) { CUR_RET_RES = 1; }   /* CHECK N4: `-> Result<..>` */
            else if id_is(79, 112, 116, 105, 111, 6) { CUR_RET_RES = 2; }   /* CHECK N4: `-> Option<..>` */
            if id_is(105, 109, 112, 108, 0, 4) { skipws(); rd_ident(); }   /* A: `-> impl Trait` -> erase (read the trait name; the `<..>` generic is skipped below). Eager iterators already produce a concrete Vec, so the body returns it. */
            skipws(); if POS < LEN && at(POS) == 40 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } POS = POS + 1; } skipws(); if POS + 1 < LEN && at(POS) == 45 && at(POS + 1) == 62 { POS = POS + 2; skipws(); rd_ident(); } }   /* R2by: `-> impl Fn(Args) -> Ret` -- consume the closure arg list AND its `-> Ret`, else the leftover `(i64) -> i64` mis-parses as the body's first stmt (emitting sub/gt on v-1 -> crash) */
            skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 45 && POS + 1 < LEN && at(POS + 1) == 62 { POS = POS + 2; } else { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }   /* R10: `->`-aware -- a `-> Box<dyn Fn(i64)->i64>` return's inner `->` must not close the generic early (was: leftover `i64>{..}` mis-parsed as the body -> `gt v-1 v-1` crash) */
            skipws(); while POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); rd_ident(); skipws(); }
            skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 45 && POS + 1 < LEN && at(POS + 1) == 62 { POS = POS + 2; } else { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }   /* R-either: a `::`-qualified return type can END in generics (`io::Result<usize>`, `iter::Map<..>`); the `<..>`-skip above runs BEFORE the path-erase, so skip a TRAILING `<..>` too (`->`-aware). Was: `-> io::Result<usize> {` left POS at `<usize>{..}` -> the body over-consumed (parse-drift). */
            skipws(); while POS < LEN && at(POS) == 43 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 39 { POS = POS + 1; while POS < LEN && is_idc(at(POS)) { POS = POS + 1; } } else { rd_ident(); while POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); rd_ident(); } skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 45 && POS + 1 < LEN && at(POS + 1) == 62 { POS = POS + 2; } else { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } } } skipws(); } } }   /* LAYER4: trailing `+ <bound>` chain on an `impl Trait` return type (`-> impl Iterator<Item=Token> + '_`, `+ Send + Sync`) -- skip `+` then a lifetime (`'_`/`'a`) or a trait path(+generics), repeat; else `+ '_ {` drifts (tokenize in rustc_lexer) */
        skipws();
        if POS + 4 < LEN && at(POS) == 119 && at(POS + 1) == 104 && at(POS + 2) == 101 && at(POS + 3) == 114 && at(POS + 4) == 101 && (POS + 5 >= LEN || !is_idc(at(POS + 5))) {   /* skip a `where T: Bound, ..` clause up to the body `{` (was mis-parsed as the body -> crash) */
            while POS < LEN && at(POS) != 123 { POS = POS + 1; }
        }
        let z = ecr(0);
        parse_block();
        if INMAIN == 1 { emit_flush_call(); }   /* R3: flush before main's fallback (unit-return) ret */
        println!("  ret v{}", z);
    }
}
fn parse_fn() {
    unsafe {
        rd_ident();  /* fn */
        rd_ident();
        let no = ID_O; let nl = ID_L;
        NC = 0; NSYM = 0; ckreset();
        MVLOGN = 0; MVSTASHN = 0;   /* MOVE-FLOW: reset the per-function move-transition log + stash */
        GNFN = 0;
        skipws(); if POS < LEN && at(POS) == 60 { let gs = POS + 1; let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 45 && POS + 1 < LEN && at(POS + 1) == 62 { POS = POS + 2; } else { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } scan_fn_generics(gs, POS - 1); }   /* skip generic <T> / <const N: ..> / <F: Fn(..)->..> (skip `->` so its `>` isn't miscounted); W1b: record Fn-bound param names */
        let is_main = nl == 4 && at(no) == 109 && at(no + 1) == 97 && at(no + 2) == 105 && at(no + 3) == 110;
        if is_main { println!("ir main -> i32"); emit_static_inits(); INMAIN = 1; }   /* R3: flush before main's ret(s) */
        else { print!("fn "); emit_span(no, nl); println!(""); }
        parse_pb(-1);
        INMAIN = 0;
    }
}
/* impl [Trait for] Type { fn m(&self,..) {..} } -> emit each method as `Type_m` */
fn parse_impl() {
    unsafe {
        let mut igpo = 0; let mut igpl = 0;   /* S3 R1: impl-level generic-param name (T in `impl<T> Wrap<T>`) */
        skipws(); if POS < LEN && at(POS) == 60 {
            let gp = POS + 1; POS = gp; skipws();
            while POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; skipws(); } }   /* skip lifetimes */
            if POS < LEN && is_idst(at(POS)) { let sp = POS; if rd_ident() { igpo = ID_O; igpl = ID_L; } POS = sp; }
            POS = gp; let mut d = 1; while POS < LEN && d > 0 { if at(POS) == 45 && POS + 1 < LEN && at(POS + 1) == 62 { POS = POS + 2; } else { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }   /* R10: `->`-aware -- `impl<F: Fn(i32)->i32> A<F>` -- the bound's inner `->` must not close the generic early */
        rd_ident();
        skipws(); while POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); rd_ident(); skipws(); }   /* R-either: `::`-qualified trait/type name (`fmt::Write`, `std::fmt::Display`) -- advance ID to the LAST segment before the `<..>`/`for` checks (was: read only `fmt`, the `for` check at `::Write` failed via rd_ident-on-`:` -> hastrait stayed 0, the impl header desynced -> the first method body over-consumed). */
        let mut to = ID_O; let mut tl = ID_L;
        let traito = to; let traitl = tl; let mut hastrait = 0;                     /* A1: remember the trait name (ident before `for`) */
        skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }   /* R2cc: skip generic args on the trait name (`From<i64>`) before the `for` check, else the trait/type isn't recognised */
        skipws(); let s = POS;
        if rd_ident() && ID_L == 3 && at(ID_O) == 102 && at(ID_O + 1) == 111 && at(ID_O + 2) == 114 {   /* `for` */
            hastrait = 1; rd_ident(); skipws(); while POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); rd_ident(); skipws(); } to = ID_O; tl = ID_L;   /* R-either: the `for` TYPE can also be `::`-qualified (`impl X for std::vec::Vec<T>`) -- advance to the LAST segment */
        } else { POS = s; }
        skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
        let ty = ty_find(to, tl);
        if hastrait == 1 && ty >= 0 && traitl == 8 && at(traito) == 73 && at(traito + 1) == 116 && at(traito + 2) == 101 && at(traito + 3) == 114 && at(traito + 4) == 97 && at(traito + 5) == 116 && at(traito + 6) == 111 && at(traito + 7) == 114 {   /* TRAIT R5: `impl Iterator for T` -> register T as a materializable iterator */
            ITERTY[NITER as usize] = ty; NITER = NITER + 1;
        }
        let en = enum_find(to, tl);                                                 /* R2: `impl EnumName { .. }` -- inherent methods on an enum */
        if hastrait == 1 && ty < 0 && en < 0 {                                      /* R2: blanket `impl<T> Trait for T` -- methods recorded by prepass_blankets; skip body */
            eat_ch(123);
            let mut d = 1; while POS < LEN && d > 0 { if at(POS) == 123 { d = d + 1; } if at(POS) == 125 { d = d - 1; } POS = POS + 1; }
            return;
        }
        IMPL_SELF_ENUM = en;                                                        /* R2: bind self as this enum for the method bodies (en = -1 for structs -> no effect) */
        eat_ch(123);
        NIMPLM = 0;
        loop {
            skipws();
            if POS >= LEN { break; }
            if at(POS) == 125 { POS = POS + 1; break; }
            let itsave = POS;
            let mut atfn = 0;   /* S1: 1 if POS was advanced to the `fn` of a `const fn` method (process as a method, not an assoc const) */
            if rd_ident() {
                if is_kw_const() {
                    skipws(); let cs = POS;
                    let mut nextfn = 0; if rd_ident() { if is_kw_fn() { nextfn = 1; } }
                    if nextfn == 1 { POS = cs; atfn = 1; }   /* `const fn` -> POS at `fn`, fall to the method path */
                    else if ty >= 0 {                        /* `const NAME: T = <intlit>;` associated const */
                        POS = cs; rd_ident(); let cno = ID_O; let cnl = ID_L;
                        skipws(); if POS < LEN && at(POS) == 58 { POS = POS + 1; while POS < LEN && at(POS) != 61 { POS = POS + 1; } }
                        if POS < LEN && at(POS) == 61 { POS = POS + 1; skipws();
                            let mut neg = 0; if POS < LEN && at(POS) == 45 { neg = 1; POS = POS + 1; }
                            let mut v: i64 = 0; while POS < LEN && is_dig(at(POS)) { v = v * 10 + ((at(POS) - 48) as i64); POS = POS + 1; }
                            if neg == 1 { v = 0 - v; }
                            TCTY[NTCONST as usize] = ty; TCNO[NTCONST as usize] = cno; TCNL[NTCONST as usize] = cnl; TCVAL[NTCONST as usize] = v; NTCONST = NTCONST + 1;
                        }
                        skipws(); if POS < LEN && at(POS) == 59 { POS = POS + 1; }
                        continue;
                    }
                }
            }
            if atfn == 0 { POS = itsave; }
            { let ps2 = POS; if rd_ident() && ID_L == 3 && at(ID_O) == 112 && at(ID_O + 1) == 117 && at(ID_O + 2) == 98 { skipws(); if POS < LEN && at(POS) == 40 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } POS = POS + 1; } } } else { POS = ps2; } }   /* S1: skip a `pub`/`pub(crate)` method visibility (was: `pub fn` -> the `fn` got skip_tok'd -> method dropped) */
            let fnsave = POS;
            if rd_ident() && is_kw_fn() {
                rd_ident();
                let mo = ID_O; let ml = ID_L;
                IMPLMO[NIMPLM as usize] = mo; IMPLML[NIMPLM as usize] = ml; NIMPLM = NIMPLM + 1;
                if ty >= 0 { MITY[NMI as usize] = ty; MIMO[NMI as usize] = mo; MIML[NMI as usize] = ml; NMI = NMI + 1; }   /* dyn registry */
                NC = 0; NSYM = 0; ckreset(); GNFN = 0;
                skipws(); if POS < LEN && at(POS) == 60 { let gs = POS + 1; let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 45 && POS + 1 < LEN && at(POS + 1) == 62 { POS = POS + 2; } else { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } scan_fn_generics(gs, POS - 1); }
                if igpl > 0 && ty >= 0 && hastrait == 0 && NGIM < 256 { GIMOWN[NGIM as usize] = ty; GIMMO[NGIM as usize] = mo; GIML[NGIM as usize] = ml; GIMTPO[NGIM as usize] = igpo; GIMTPL[NGIM as usize] = igpl; GIMBODY[NGIM as usize] = POS; NGIM = NGIM + 1; }   /* S3 R1: register the generic-inherent-impl method for on-demand per-T monomorphization */
                print!("fn "); emit_span(to, tl); print!("_"); emit_span(mo, ml); println!("");
                /* self injected only if the method has a self receiver (parse_pb handles the peek) */
                parse_pb(ty);
            } else { POS = fnsave; skip_tok(); }
        }
        if hastrait == 1 {                                                          /* A1: emit each un-overridden trait default method as Type_method */
            let endpos = POS;
            let mut k = 0;
            while k < NTDM {
                if span_eq_at(TRO[k as usize], TRL[k as usize], traito, traitl) && impl_has_method(TDMO[k as usize], TDML[k as usize]) == 0 {
                    POS = TDPOS[k as usize];
                    rd_ident(); let dmo = ID_O; let dml = ID_L;
                    NC = 0; NSYM = 0; ckreset();
                    print!("fn "); emit_span(to, tl); print!("_"); emit_span(dmo, dml); println!("");
                    if ty >= 0 { MITY[NMI as usize] = ty; MIMO[NMI as usize] = dmo; MIML[NMI as usize] = dml; NMI = NMI + 1; }   /* dyn registry (default method) */
                    if ty >= 0 && TDMSTR[k as usize] != 0 { MRTOWN[NMRT as usize] = ty; MRTO[NMRT as usize] = dmo; MRTL[NMRT as usize] = dml; MRTS[NMRT as usize] = -1; MRTB[NMRT as usize] = 0; MRTV[NMRT as usize] = 0; MRTI64[NMRT as usize] = 0; MRTSTR[NMRT as usize] = TDMSTR[k as usize]; NMRT = NMRT + 1; }   /* S1 L1: register the default method's String/&str return kind for `ty` so `A.hello()` tags the result */
                    parse_pb(ty);
                }
                k = k + 1;
            }
            POS = endpos;
        }
        IMPL_SELF_ENUM = 0 - 1;                                                     /* R2: clear enum-self binding after the impl */
    }
}
fn is_kw_mut_next() -> bool { unsafe { skipws(); if POS + 2 < LEN && at(POS) == 109 && at(POS + 1) == 117 && at(POS + 2) == 116 { POS = POS + 3; true } else { false } } }
/* PAT: bind a (possibly nested) tuple pattern `( p, p, .. )` against tuple cell `cell`; p is `_`, an ident, or a nested `(...)`. */
/* PAT: bind a slice pattern `[p0, .., pk]` / `[p0, rest @ ..]` over a fixed-array buffer `buf` of length
   `arrlen`. Leading names bind arr[i]; trailing (after `..`) bind arr[arrlen-ntrail+j]; `rest @ ..` binds a
   [ptr,len] sub-slice; `_` skips. Irrefutable on a fixed array. POS at `[`. */
fn bind_slice_pat(buf: i32, arrlen: i32) {
    unsafe {
        eat_ch(91);
        let mut nlead = 0; let mut hasrest = 0; let mut ro = 0; let mut rl = 0; let mut rbind = 0; let mut ntrail = 0;
        loop {
            skipws();
            if POS >= LEN || at(POS) == 93 { break; }
            if at(POS) == 46 && POS + 1 < LEN && at(POS + 1) == 46 { POS = POS + 2; hasrest = 1; skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } break; }
            rd_ident(); let no = ID_O; let nl = ID_L; skipws();
            if POS < LEN && at(POS) == 64 { POS = POS + 1; skipws(); if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 { POS = POS + 2; } hasrest = 1; rbind = 1; ro = no; rl = nl; skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } break; }
            if !(nl == 1 && at(no) == 95) { let el = eload(ebin("add", buf, ec(nlead * 4))); let slot = NC; ecopy(slot, el); NC = NC + 1; sym_add(no, nl, slot); }
            nlead = nlead + 1;
            skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; }
        }
        if hasrest == 1 {
            loop {
                skipws();
                if POS >= LEN || at(POS) == 93 { break; }
                rd_ident(); TNO[ntrail as usize] = ID_O; TNL[ntrail as usize] = ID_L; ntrail = ntrail + 1;
                skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; }
            }
        }
        eat_ch(93);
        let mut j = 0;
        while j < ntrail {
            let idx = arrlen - ntrail + j;
            if !(TNL[j as usize] == 1 && at(TNO[j as usize]) == 95) { let el = eload(ebin("add", buf, ec(idx * 4))); let slot = NC; ecopy(slot, el); NC = NC + 1; sym_add(TNO[j as usize], TNL[j as usize], slot); }
            j = j + 1;
        }
        if rbind == 1 {
            let desc = halloc(8); println!("  store v{} v{}", desc, ebin("add", buf, ec(nlead * 4)));
            let c4 = ecr(4); println!("  store v{} v{}", ebin("add", desc, c4), ec(arrlen - nlead - ntrail));
            let slot = NC; ecopy(slot, desc); NC = NC + 1; sym_add(ro, rl, slot); SK[(NSYM - 1) as usize] = 2;
        }
    }
}
fn bind_tuple_pat(cell: i32, tslice: i32, tyid: i32) {
    unsafe {
        eat_ch(40);
        let mut i = 0;
        loop {
            skipws();
            if POS >= LEN || at(POS) == 41 { break; }
            let ad = ebin("add", cell, ec(i * 4));
            if at(POS) == 95 && (POS + 1 >= LEN || !is_idc(at(POS + 1))) { POS = POS + 1; }   /* _ wildcard */
            else if at(POS) == 40 { let inner = eload(ad); bind_tuple_pat(inner, 0, 0 - 1); }         /* nested tuple */
            else {
                if is_kw_mut_next() { } if POS < LEN && at(POS) == 38 { POS = POS + 1; }        /* skip mut / & */
                rd_ident(); let no = ID_O; let nl = ID_L; let el = eload(ad);
                let slot = NC; ecopy(slot, el); NC = NC + 1; sym_add(no, nl, slot);
                if tslice == 1 { SK[(NSYM - 1) as usize] = 2; }
                let fk = tupty_fk(tyid, i);                                                     /* tuple-field-kind: mark the bound var with the field's kind (&str/String/Vec/char/i64) */
                if fk == 1 { SK[(NSYM - 1) as usize] = 1; } else if fk == 3 { SK[(NSYM - 1) as usize] = 3; } else if fk == 2 { SK[(NSYM - 1) as usize] = 2; } else if fk == 4 { SCHAR[(NSYM - 1) as usize] = 1; } else if fk == 5 { SI64[(NSYM - 1) as usize] = 1; }
            }
            i = i + 1;
            skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; }
        }
        eat_ch(41);
    }
}

/* R2 pre-pass: register every `enum Name { V, V(T), ... }` -> enum + variant registry (discriminants 0..) */
fn prepass_enums() {
    unsafe {
        POS = 0;
        loop {
            skipws();
            if POS >= LEN { break; }
            let save = POS;
            if rd_ident() && is_kw_enum() {
                rd_ident();
                let en = NEN;
                ENO[en as usize] = ID_O; ENL[en as usize] = ID_L; ENGPO[en as usize] = 0; ENGPL[en as usize] = 0; NEN = NEN + 1;
                skipws(); if POS < LEN && at(POS) == 60 {
                    let gp = POS + 1; POS = gp; skipws();
                    while POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; skipws(); } }   /* skip leading lifetimes */
                    if POS < LEN && is_idst(at(POS)) { let sp = POS; if rd_ident() { ENGPO[en as usize] = ID_O; ENGPL[en as usize] = ID_L; } POS = sp; }   /* std-hoist GAP1: record the enum's first type-param name */
                    POS = gp; let mut d = 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }   /* R2: skip `<T>` on a generic enum `enum Maybe<T> { .. }` (was an infinite loop) */
                eat_ch(123);
                let mut disc = 0;
                loop {
                    skipws();
                    while POS < LEN && at(POS) == 35 && POS + 1 < LEN && at(POS + 1) == 91 { POS = POS + 2; let mut da = 1; while POS < LEN && da > 0 { if at(POS) == 91 { da = da + 1; } if at(POS) == 93 { da = da - 1; } POS = POS + 1; } skipws(); }   /* RUSTC-FE FC: skip `#[..]` attributes before an enum variant (was an infinite HANG: rd_ident on `#` read nothing -> the variant loop spun; real enums attribute variants, e.g. `enum Option<T> { #[lang="None"] None, .. }`) */
                    if POS < LEN && at(POS) == 125 { POS = POS + 1; break; }
                    if POS >= LEN { break; }
                    let vsave = POS;
                    rd_ident();
                    let vi = NVAR;
                    VEN[vi as usize] = en; VO[vi as usize] = ID_O; VL[vi as usize] = ID_L;
                    VDISC[vi as usize] = disc; VHAS[vi as usize] = 0; VNF[vi as usize] = 0;
                    skipws();
                    if POS < LEN && at(POS) == 40 {
                        VHAS[vi as usize] = 1;
                        let mut nf = 0;
                        let mut d = 1; POS = POS + 1;
                        let mut atstart = 1;   /* R34: at the start of a field's type -> capture its leading ident for String/&str {:?}-quoting */
                        while POS < LEN && d > 0 {
                            if atstart == 1 && d == 1 {
                                while POS < LEN && (at(POS) == 32 || at(POS) == 38 || at(POS) == 10 || at(POS) == 9) { POS = POS + 1; }   /* skip ws + leading `&` */
                                let fo = POS; while POS < LEN && is_idc(at(POS)) { POS = POS + 1; } let fl = POS - fo;
                                if nf < 8 { if is_string_ty(fo, fl) || (fl == 3 && at(fo) == 115 && at(fo + 1) == 116 && at(fo + 2) == 114) { VFSTR[(vi * 8 + nf) as usize] = 1; } if is_i64_ty(fo, fl) { VFI64[(vi * 8 + nf) as usize] = 1; } if is_vec_ty(fo, fl) { VFVEC[(vi * 8 + nf) as usize] = 1; } VFKIND[(vi * 8 + nf) as usize] = vfield_eq_kind(fo, fl, en); if is_vec_ty(fo, fl) && POS < LEN && at(POS) == 60 { let mut eo = POS + 1; while eo < LEN && (at(eo) == 32 || at(eo) == 38) { eo = eo + 1; } let mut ee = eo; while ee < LEN && is_idc(at(ee)) { ee = ee + 1; } if is_box_ident(eo, ee - eo) && ee < LEN && at(ee) == 60 { eo = ee + 1; while eo < LEN && (at(eo) == 32 || at(eo) == 38) { eo = eo + 1; } ee = eo; while ee < LEN && is_idc(at(ee)) { ee = ee + 1; } } let vest = ty_find(eo, ee - eo); if vest >= 0 { VFVELT[(vi * 8 + nf) as usize] = vest; } else if is_string_ty(eo, ee - eo) { VFVELTY[(vi * 8 + nf) as usize] = 3; } else if ee - eo == 3 && at(eo) == 115 && at(eo + 1) == 116 && at(eo + 2) == 114 { VFVELTY[(vi * 8 + nf) as usize] = 2; } } { let mut cfo = fo; let mut cfl = fl; if is_box_ident(fo, fl) && POS < LEN && at(POS) == 60 { let mut bo = POS + 1; while bo < LEN && (at(bo) == 32 || at(bo) == 38) { bo = bo + 1; } let mut be = bo; while be < LEN && is_idc(at(be)) { be = be + 1; } cfo = bo; cfl = be - bo; } VFPSTO[(vi * 8 + nf) as usize] = cfo; VFPSTL[(vi * 8 + nf) as usize] = cfl; } }   /* DEFECT-c: capture the tuple-variant field's payload type-name span (through Box) -- resolved struct-vs-enum LAZILY at the match binder. String / str (+ LAYER5 P-DERIVE-RECURSIVE-EQ per-field eq kind; LAYER6 P-ENUM-I64-PAYLOAD-WIDTH i64/isize width; LAYER6 P-STRUCT-VARIANT-FIELD-KIND Vec) */
                                nf = nf + 1; atstart = 0;
                                if POS >= LEN { break; }
                            }
                            if at(POS) == 40 { d = d + 1; }
                            if at(POS) == 41 { d = d - 1; }
                            if d == 1 && at(POS) == 44 { atstart = 1; }
                            POS = POS + 1;   /* R34: consume the current char (incl. the closing `)` when d hits 0), matching the original scan -- else the outer variant loop spins on the `)` */
                        }
                        VNF[vi as usize] = nf;
                    }
                    else if POS < LEN && at(POS) == 123 {                            /* R1: struct-shaped variant `V{ f0:T, f1:T }` -- fields stored positionally in decl order */
                        VHAS[vi as usize] = 1; VSTRUCT[vi as usize] = 1;
                        let mut nf = 0; let mut d = 1; POS = POS + 1;
                        loop {   /* R22b: record each field NAME (before its `: T`) for {:?} `V { f: v, .. }` */
                            skipws(); if POS >= LEN || at(POS) == 125 { break; }
                            let fo = POS; while POS < LEN && is_idc(at(POS)) { POS = POS + 1; } let fl = POS - fo;
                            if nf < 8 { VFNO[(vi * 8 + nf) as usize] = fo; VFNL[(vi * 8 + nf) as usize] = fl; }
                            nf = nf + 1;
                            skipws(); if POS < LEN && at(POS) == 58 { POS = POS + 1; skipws(); while POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); } let to = POS; while POS < LEN && is_idc(at(POS)) { POS = POS + 1; } let tl = POS - to; if nf <= 8 { if is_string_ty(to, tl) || (tl == 3 && at(to) == 115 && at(to + 1) == 116 && at(to + 2) == 114) { VFSTR[(vi * 8 + nf - 1) as usize] = 1; } if is_i64_ty(to, tl) { VFI64[(vi * 8 + nf - 1) as usize] = 1; } if is_vec_ty(to, tl) { VFVEC[(vi * 8 + nf - 1) as usize] = 1; } VFKIND[(vi * 8 + nf - 1) as usize] = vfield_eq_kind(to, tl, en); } }   /* R34: struct-variant field type -> String/&str quoting in {:?} (+ LAYER5 eq kind; LAYER6 i64/isize width; LAYER6 Vec) */
                            while POS < LEN && d > 0 && at(POS) != 44 { if at(POS) == 123 { d = d + 1; } if at(POS) == 125 { d = d - 1; if d == 0 { break; } } POS = POS + 1; }   /* skip `: T` to the next `,` or the closing `}` */
                            if POS < LEN && at(POS) == 44 { POS = POS + 1; }
                        }
                        if POS < LEN && at(POS) == 125 { POS = POS + 1; }
                        VNF[vi as usize] = nf;
                    }
                    skipws();
                    if POS < LEN && at(POS) == 61 { POS = POS + 1; CVALID = 1; let dv = ceval_expr(); if CVALID == 1 { disc = dv; VDISC[vi as usize] = dv; } skipws(); }   /* R2: explicit discriminant `Variant = <const>` (was an infinite loop) */
                    NVAR = NVAR + 1; disc = disc + 1;
                    skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; }
                    if POS == vsave { skip_tok(); }   /* RUSTC-FE FC fail-closed: a variant iteration that made no progress force-advances -> the loop can never spin */
                }
            } else { POS = save; skip_tok(); }
        }
    }
}

/* R2: `match SCRUT { PAT => EXPR, ... }` as an expression. enum patterns `E::V`/`E::V(x)` compare the loaded
   discriminant; integer patterns compare the scrutinee directly; `_` is the default arm. lowered to a chain
   of brz/pool tests, each arm copying its value to a shared result slot then `goto` the end label. */
/* R3b: `if C { E1 } else [if ...] { E2 }` as an EXPRESSION. Each branch block's tail expr is captured (via
   TAILDST) into a shared result slot; structure mirrors the statement-if lowering (brz / goto end / pool). */
fn parse_branch_into(dst: i32) {
    unsafe { let save = TAILDST; TAILDST = dst + 1; parse_block(); TAILDST = save; }
}
/* R2ad: shared `if let PAT(x) = EXPR` head -- parse the variant + optional `(x)` binder and the
   scrutinee, emit the discriminant test (`brz` to the else on mismatch) and bind the payload (cell+4)
   in a fresh scope; returns the NSYM mark so the caller pops the binder after its then-branch. */
fn parse_iflet_head() -> i32 {
    unsafe {
        skipws(); rd_ident();                          /* Some / Ok / None / Err / Enum::Variant */
        let mut po = ID_O; let mut pl = ID_L;
        skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); rd_ident(); po = ID_O; pl = ID_L; }   /* LAYER7 P-IFLET-EXPR-QUALIFIED-VARIANT: `if let E::V = expr { .. } else { .. }` as a VALUE expression -- resolve the qualified user-enum variant discriminant (was: only Some/Ok/None/Err via opt_disc, so a `Tok::End` pattern left POS at `::End` where `= expr` was expected -> parse-drift). Mirrors the statement-position if-let. */
        let vi = var_find(po, pl);
        let mut want = 0; let mut has = 0;
        if vi >= 0 { want = VDISC[vi as usize]; has = VHAS[vi as usize]; } else { want = opt_disc(po, pl); if opt_has(po, pl) { has = 1; } }
        let mut xo = 0; let mut xl = 0; let mut bindx = 0;
        skipws();
        if POS < LEN && at(POS) == 40 { POS = POS + 1; skipws(); rd_ident(); xo = ID_O; xl = ID_L; bindx = 1; skipws(); eat_ch(41); }
        skipws(); eat_ch(61); skipws();
        let mut ilref = 0;
        if POS < LEN && at(POS) == 38 { let mut q = POS + 1; while q < LEN && at(q) == 32 { q = q + 1; } if q < LEN && is_idst(at(q)) { POS = POS + 1; skipws(); ilref = 1; } }   /* D004-CORE IC-1 P-BYREF-IFLET-EXPR-SCRUTINEE-TRANSPARENT: `let y = if let Some(x)=&opt {..} else {..}` -- the expression-position dual of the statement if-let & match IC-1 fix: skip the transparent leading `&` so `cell` is the cell pointer, not eref=address-of-slot. */
        let cell = parse_expr();
        let disc = eload(cell);
        let wc = ec(want); let cnd = ebin("eq", disc, wc);
        println!("  brz v{}", cnd); ckreset();
        let mark = NSYM;
        if bindx == 1 && has == 1 {
            let c4 = ecr(4); let pa = ebin("add", cell, c4); let pv = eload(pa);
            let xslot = NC; ecopy(xslot, pv); NC = NC + 1; sym_add(xo, xl, xslot); if EOPTREF == 1 || ilref == 1 { SFORREF[(NSYM - 1) as usize] = 1; if ilref == 1 { SREF1[(NSYM - 1) as usize] = 1; } }
        }
        mark
    }
}
fn parse_if_arm(result: i32, endlbl: i32) {
    unsafe {
        skipws();
        let lsave = POS;
        let mut islet = 0;
        if rd_ident() && is_kw_let() { islet = 1; } else { POS = lsave; }
        if islet == 1 {                                    /* R2ad: `if let PAT(x) = EXPR { E1 } else { E2 }` as a value expression */
            let mark = parse_iflet_head();
            let mvbase = MVLOGN;                           /* MOVE-FLOW: then-branch starts here */
            parse_branch_into(result);
            let sstash = mv_stash(mvbase, mark);           /* stash then's OUTER (index < mark) moves */
            mv_undo(mvbase);                               /* else starts from the pre-then moved-set */
            NSYM = mark;
            println!("  goto __Li{}", endlbl); ckreset();
            println!("  pool"); ckreset();
            skipws();
            let s2 = POS;
            if rd_ident() && is_kw_else() {
                skipws();
                let s3 = POS;
                if rd_ident() && is_kw_if() { parse_if_arm(result, endlbl); }
                else { POS = s3; parse_branch_into(result); }
            } else { POS = s2; }
            mv_replay(sstash);                             /* union then's moves back in (post-if moved-set) */
            return;
        }
        let c = parse_expr();
        println!("  brz v{}", c); ckreset();
        let mvth = NSYM; let mvbase = MVLOGN;              /* MOVE-FLOW: outer-symbol threshold + then-branch log base */
        parse_branch_into(result);
        let sstash = mv_stash(mvbase, mvth);              /* stash then's OUTER moves */
        mv_undo(mvbase);                                  /* else starts from the pre-then moved-set */
        println!("  goto __Li{}", endlbl); ckreset();
        println!("  pool"); ckreset();
        skipws();
        let s2 = POS;
        if rd_ident() && is_kw_else() {
            skipws();
            let s3 = POS;
            if rd_ident() && is_kw_if() { parse_if_arm(result, endlbl); }
            else { POS = s3; parse_branch_into(result); }
        } else { POS = s2; }
        mv_replay(sstash);                                /* union then's moves back in (post-if moved-set) */
    }
}
fn parse_if_expr() -> i32 {
    unsafe {
        let result = NC; NC = NC + 1;
        let endlbl = LBL; LBL = LBL + 1;
        parse_if_arm(result, endlbl);
        println!("  label __Li{}", endlbl); ckreset();
        EITYP = -1;
        result
    }
}

/* R24: read a scalar match-pattern atom -- a char literal 'c' (with common escapes) or an integer
   literal -- returning its value and advancing POS. Used for `N =>`, `'c' =>`, and range endpoints. */
fn read_pat_scalar() -> i32 {
    unsafe {
        skipws();
        if POS < LEN && at(POS) == 39 {
            POS = POS + 1;
            let mut v = at(POS);
            if v == 92 { POS = POS + 1; let e = at(POS); if e == 110 { v = 10; } else if e == 116 { v = 9; } else if e == 114 { v = 13; } else if e == 48 { v = 0; } else if e == 117 && POS + 1 < LEN && at(POS + 1) == 123 { POS = POS + 2; v = 0; while POS < LEN && at(POS) != 125 { let h = at(POS); let hd = if h >= 48 && h <= 57 { h - 48 } else if h >= 97 && h <= 102 { h - 97 + 10 } else if h >= 65 && h <= 70 { h - 65 + 10 } else { 0 }; v = v * 16 + hd; POS = POS + 1; } } else { v = e; } }   /* LAYER6 P-CHAR-UNICODE-ESCAPE: `'\u{XXXX}'` -> decode the hex codepoint (POS lands on `}`; the outer POS+1 steps past it, mirroring the simple-escape flow). Was: `\u` fell to `else{v=e}` -> v='u' + POS stuck at `{XXXX}'` -> desync. In a PATTERN (read_pat_scalar) this made `matches!(c,'\u{0009}'|..)` (rustc_lexer is_whitespace) match EVERYTHING -> is_whitespace(any)=true -> the single-token collapse. */
            POS = POS + 1;
            if POS < LEN && at(POS) == 39 { POS = POS + 1; }
            return v;
        }
        let mut neg = 0; if POS < LEN && at(POS) == 45 { neg = 1; POS = POS + 1; skipws(); }   /* PARSE-4 P-NEGATIVE-LITERAL-PATTERN: a `-N` match/matches! pattern (`match t { -1 => .. }`, `matches!(t, -1 | -2)`) -- consume the leading `-` and negate; was: `-` is not a digit -> read 0 + POS stuck at `-N` -> the arm matched 0 and the pattern desynced (a token-kind parser using negative sentinels mis-dispatched every arm). */
        let mut n = 0;
        while POS < LEN && is_dig(at(POS)) { n = n * 10 + (at(POS) - 48); POS = POS + 1; }
        if neg == 1 { 0 - n } else { n }
    }
}
/* R2ae: recursively match a parenthesized variant payload pattern. POS is just past the open paren.
   cellptr is the enum cell pointer; payload field fi lives at cellptr+4+fi*4. Plain names bind the
   payload slot; a nested variant ctor (Some/Ok/None/E::V) ANDs its inner discriminant test into
   testv and recurses. byref threads the by-ref-match reference flag onto bound slots. Returns testv. */
fn match_pat_payload(cellptr: i32, testv0: i32, byref: i32, vpar: i32, pkind: i32, ptup: i32) -> i32 {
    unsafe {
        let mut testv = testv0;
        let psenum = if ptup <= 0 - 100000 { 0 - ptup - 100000 } else { 0 - 1 };   /* D005 P-PRESULT-BOX-ENUM-PAYLOAD-BINDER: the HIGH-NEGATIVE ptup range (<= -100000) encodes -(enumidx+100000) for an Ok(e)/Some(e) Box<Enum> success payload -> tag e SENUM. Distinct from the struct range below. */
        let psty = if ptup <= 0 - 2 && ptup > 0 - 100000 { 0 - ptup - 2 } else { 0 - 1 };   /* LAYER5 P-PRESULT-STRUCT-PAYLOAD-BINDER: ptup is overloaded -- >=0 is a tuple-type id (the `Some((k,v))` path), <= -2 (but > -100000, above the D005 enum range) encodes -(structidx+2) for an Ok(v)/Some(v) user-struct payload (a struct success-arm is never a tuple, so the two never collide; -1 stays "none"). This avoids a 7th fn param, which the 6-arg-register backend ABI cannot pass. */
        let mut fi = 0;
        loop {
            skipws(); if POS < LEN && at(POS) == 41 { break; }
            if POS < LEN && (at(POS) == 39 || is_dig(at(POS)) || (at(POS) == 45 && POS + 1 < LEN && is_dig(at(POS + 1)))) {   /* LAYER4 P-CHARS-ITER-SEG: a LITERAL payload `Some('}')` / `Ok(5)` -- TEST the payload field == the literal (with `|`-alternation + `lo..=hi` range), do NOT rd_ident it (was: rd_ident on `'` read nothing -> stale ID_L -> var_find/opt_disc span_eq_at read SRC out of bounds -> FE SEGFAULT; rustc_lexer unescape `match chars.next() { Some('}') => .. }`). */
                let off = ec(vpay_off(vpar, fi)); let pp = ebin("add", cellptr, off); let val = if vpay_i64(vpar, fi) == 1 { eload8(pp) } else { eload(pp) };   /* P-ENUM-I64-PAYLOAD-OFFSET */
                let lo = read_pat_scalar(); skipws();
                let mut t = 0;
                if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 { POS = POS + 2; if POS < LEN && at(POS) == 61 { POS = POS + 1; } let hi = read_pat_scalar(); t = ebin("mul", ebin("ge", val, ec(lo)), ebin("le", val, ec(hi))); }
                else { t = ebin("eq", val, ec(lo)); }
                skipws();
                while POS < LEN && at(POS) == 124 && (POS + 1 >= LEN || at(POS + 1) != 124) {   /* `Some('a' | 'b')` payload or-alternation */
                    POS = POS + 1; skipws(); let lo2 = read_pat_scalar(); skipws(); let mut t2 = 0;
                    if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 { POS = POS + 2; if POS < LEN && at(POS) == 61 { POS = POS + 1; } let hi2 = read_pat_scalar(); t2 = ebin("mul", ebin("ge", val, ec(lo2)), ebin("le", val, ec(hi2))); }
                    else { t2 = ebin("eq", val, ec(lo2)); }
                    t = ebin("or", t, t2); skipws();
                }
                testv = ebin("mul", testv, t);
                fi = fi + 1;
                skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; continue; } else { break; }
            }
            if POS < LEN && at(POS) == 40 {   /* P3+: nested tuple pattern `Some((a, b))` on payload field fi -- the field value is a tuple cell [f0@0, f1@4], destructure it */
                let tcell0 = eload(ebin("add", cellptr, ec(vpay_off(vpar, fi))));   /* P-ENUM-I64-PAYLOAD-OFFSET */
                let one = ecr(1); let ni = ebin("sub", one, testv);   /* guard: deref the tuple cell only when the arm test still holds, else read the (valid) cell pointer -- no wild load on a mismatched arm (e.g. the None case) */
                let tcell = ebin("add", ebin("mul", testv, tcell0), ebin("mul", ni, cellptr));
                POS = POS + 1; let mut tf = 0;
                loop {
                    skipws(); if POS >= LEN || at(POS) == 41 { break; }
                    if at(POS) == 95 { POS = POS + 1; }
                    else { if at(POS) == 38 { POS = POS + 1; skipws(); } rd_ident(); let to = ID_O; let tl = ID_L; let tvv = eload(ebin("add", tcell, ec(tf * 4))); let tsl = NC; ecopy(tsl, tvv); NC = NC + 1; sym_add(to, tl, tsl); if byref == 1 { SFORREF[(NSYM - 1) as usize] = 1; } if ptup >= 0 { let fk = tupty_fk(ptup, tf); if fk == 1 { SK[(NSYM - 1) as usize] = 1; } else if fk == 3 { SK[(NSYM - 1) as usize] = 3; } else if fk == 2 { SK[(NSYM - 1) as usize] = 2; } else if fk == 5 { SI64[(NSYM - 1) as usize] = 1; } } }   /* P3+: restore each tuple field's kind from the Option payload tuple-type (split_once -> &str) */
                    tf = tf + 1; skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; }
                }
                eat_ch(41);
            } else {
            let mut stripref = 0; if POS < LEN && at(POS) == 38 { POS = POS + 1; stripref = 1; skipws(); }   /* P-MATCH-PATTERN: `Some(&n)` -- the `&` strips one ref level, so bind n to the value directly (not by-ref) */
            rd_ident(); let mut bo = ID_O; let mut bl = ID_L;
            skipws();
            if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; rd_ident(); bo = ID_O; bl = ID_L; }
            let off = ec(vpay_off(vpar, fi)); let pp = ebin("add", cellptr, off); let val = if vpay_i64(vpar, fi) == 1 { eload8(pp) } else { eload(pp) };   /* P-ENUM-I64-PAYLOAD-WIDTH: 8-byte load for an i64 payload (the cell slot is now 8-byte) */
            let vi = var_find(bo, bl); let od = opt_disc(bo, bl);
            let mut isctor = 0; let mut idisc = 0;
            if vi >= 0 { isctor = 1; idisc = VDISC[vi as usize]; }
            else if od >= 0 { isctor = 1; idisc = od; }
            skipws();
            let mut hasparen = 0;
            if POS < LEN && at(POS) == 40 { hasparen = 1; }
            if isctor == 1 {
                let one = ecr(1); let ni = ebin("sub", one, testv);                     /* R2bu: match arms are straight-line (one brz per pool); a mismatched outer variant leaves a non-pointer in `val`. */
                let safev = ebin("add", ebin("mul", testv, val), ebin("mul", ni, cellptr));   /* guard: deref `val` only when the accumulated test still holds, else read the (always-valid) cell pointer -- no wild load */
                let idc = eload(safev); let icv = ec(idisc); let dt = ebin("eq", idc, icv);
                testv = ebin("mul", testv, dt);
                if hasparen == 1 { POS = POS + 1; testv = match_pat_payload(safev, testv, byref, if vi >= 0 { vi } else { 0 - 1 }, 0 - 1, 0 - 1); eat_ch(41); }
            } else {
                let slot = NC; if vpar >= 0 && VFI64[(vpar * 8 + fi) as usize] == 1 { ecopy64(slot, val); } else { ecopy(slot, val); } NC = NC + 1; sym_add(bo, bl, slot);   /* P-ENUM-I64-PAYLOAD-WIDTH: 64-bit copy of an i64 payload into the bound slot (was a 32-bit `or` -> truncation) */
                if byref == 1 && stripref == 0 { SFORREF[(NSYM - 1) as usize] = 1; SREF1[(NSYM - 1) as usize] = 1; }   /* by-ref payload binding is a ref (`*n` no-op) UNLESS `&n` already stripped the ref. D004 IC-3: also SREF1 -- a 2nd `*` (`**e` on Option<Box<T>>/Option<&T>) does a real load, not another no-op. */
                if vpar >= 0 && VFSTR[(vpar * 8 + fi) as usize] == 1 { SK[(NSYM - 1) as usize] = 3; }   /* P1: String/&str variant-payload field -> restore the bound slot's string kind (was: bound as bare i32 -> printed the descriptor pointer) */
                if vpar >= 0 && VFI64[(vpar * 8 + fi) as usize] == 1 { SI64[(NSYM - 1) as usize] = 1; }   /* P-ENUM-I64-PAYLOAD-WIDTH: the payload is now stored 8-byte (cell-widening) -> tag the binder i64 so print/arith are 64-bit and `*v` load8 reads the full width (was deferred while the store was 4-byte) */
                if vpar >= 0 && VFVEC[(vpar * 8 + fi) as usize] == 1 { SK[(NSYM - 1) as usize] = 2; if VFVELT[(vpar * 8 + fi) as usize] >= 0 { SVELT[(NSYM - 1) as usize] = VFVELT[(vpar * 8 + fi) as usize]; } if VFVELTY[(vpar * 8 + fi) as usize] >= 0 { SVELTY[(NSYM - 1) as usize] = VFVELTY[(vpar * 8 + fi) as usize]; } }   /* LAYER8 P-VARIANT-VEC-STRUCT-ELEM thread SVELT; DEFECT-g P-VARIANT-VEC-STRING-ELEM thread SVELTY (String=3/&str=2) so `E::V(v)=>for p in v.iter(){p}` binds p String/&str */   /* LAYER7 P-VEC-STRUCT-ELEM: a `Vec<..>` tuple-variant payload field -> tag the bound slot SK=2 so `match e { E::Call(args) => args.iter()/args.len() }` dispatches the Vec (was untagged scalar -> `.iter()` unrecognized -> parse-drift). Mirrors the struct-variant binder (9449) VFVEC restore. NOTE: do NOT also restore VFI64 here -- a single i64 tuple payload is stored in the 4-byte enum cell slot, so tagging SI64 makes `*v` do an 8-byte load8 that over-reads garbage high bits (regressed match_guard_multiarm_ast; the width is handled at the `*v` read site, not the binder, for tuple payloads). */
                if vpar >= 0 && VFKIND[(vpar * 8 + fi) as usize] == 1 { SENUM[(NSYM - 1) as usize] = VEN[vpar as usize] + 1; }   /* LAYER5 P-BOX-AS-REF: a Box<ThisEnum> recursive payload -> tag the bound child as an enum so `a.as_ref()`/`a.method()`/{:?} dispatch through the enum path (was untagged -> `a.as_ref()` hit a non-existent <Enum>_as_ref -> crash; `&**a`/`eval(a)` worked because they need no tag) */
                if vpar >= 0 && VFKIND[(vpar * 8 + fi) as usize] != 1 && VFPSTL[(vpar * 8 + fi) as usize] > 0 { let pfo = VFPSTO[(vpar * 8 + fi) as usize]; let pfl = VFPSTL[(vpar * 8 + fi) as usize]; let pst = ty_find(pfo, pfl); if pst >= 0 && enum_find(pfo, pfl) < 0 { SST[(NSYM - 1) as usize] = pst; } }   /* DEFECT-c P-VARIANT-STRUCT-PAYLOAD-BINDER: a `Box<Struct>`/`Struct` tuple-variant payload -> bind the child struct-typed (the payload word IS the struct cell pointer) so `x.field`/`&x.field` dispatches. LAZY resolve: ty_find + enum_find<0 at match time (all prepasses run -> enum_find authoritatively excludes a Box<SelfEnum>, which VFKIND==1 / the SENUM path above already handle). The user-enum dual of the psty Ok/Some struct-payload binder below. */
                else if vpar < 0 && byref == 0 && fi == 0 && (pkind == 3 || pkind == 1) { SK[(NSYM - 1) as usize] = pkind; }   /* P1: built-in Option/Result String/&str payload -> restore string kind from the scrutinee's EOPTK */
                if vpar < 0 && fi == 0 && psty >= 0 { SST[(NSYM - 1) as usize] = psty; }   /* LAYER5 P-PRESULT-STRUCT-PAYLOAD-BINDER: an Ok(v)/Some(v) whose payload is a USER STRUCT -> bind v struct-typed (the payload cell IS the struct pointer) so `v.field` dispatches, not read as a scalar */
                if vpar < 0 && fi == 0 && psenum >= 0 { SENUM[(NSYM - 1) as usize] = psenum + 1; }   /* D005 P-PRESULT-BOX-ENUM-PAYLOAD-BINDER: an Ok(e)/Some(e) whose payload is a Box<Enum>/Enum -> tag e SENUM so `e.method()` dispatches through the enum (was untagged Box pointer -> mis-dispatch -> garbage). The enum dual of the struct-payload SST binding above. */
                skipws();
                if POS < LEN && at(POS) == 64 {   /* P-MATCH-PATTERN: `x @ subpat` on a variant payload -- AND the range/literal test on `val` (mirrors the scalar @ path). Was left unparsed -> `)` desync -> empty output. */
                    POS = POS + 1; skipws();
                    let mut paren = 0; if POS < LEN && at(POS) == 40 { paren = 1; POS = POS + 1; skipws(); }
                    let lo = read_pat_scalar(); skipws();
                    let mut t = 0;
                    if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 { POS = POS + 2; if POS < LEN && at(POS) == 61 { POS = POS + 1; } let hi = read_pat_scalar(); t = ebin("mul", ebin("ge", val, ec(lo)), ebin("le", val, ec(hi))); }
                    else { t = ebin("eq", val, ec(lo)); }
                    skipws();
                    while POS < LEN && at(POS) == 124 && (POS + 1 >= LEN || at(POS + 1) != 124) {
                        POS = POS + 1; skipws(); let lo2 = read_pat_scalar(); skipws(); let mut t2 = 0;
                        if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 { POS = POS + 2; if POS < LEN && at(POS) == 61 { POS = POS + 1; } let hi2 = read_pat_scalar(); t2 = ebin("mul", ebin("ge", val, ec(lo2)), ebin("le", val, ec(hi2))); }
                        else { t2 = ebin("eq", val, ec(lo2)); }
                        t = ebin("or", t, t2); skipws();
                    }
                    if paren == 1 { if POS < LEN && at(POS) == 41 { POS = POS + 1; } }
                    testv = ebin("mul", testv, t);
                }
            }
            }
            fi = fi + 1;
            skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; }
        }
        testv
    }
}
fn parse_match() -> i32 {
    unsafe {
        skipws();
        let mut byref = 0;
        if POS < LEN && at(POS) == 38 { byref = 1;
            /* D004-CORE IC-1 P-BYREF-MATCH-SCRUTINEE-TRANSPARENT: `match &<place>` over a heap cell (Option/enum/
               struct) -- a reference to a boxed cell is TRANSPARENT (the cell pointer IS the value). Skip the leading
               `&` so the scrutinee parses to the cell pointer directly, NOT eref=address-of-slot; else `base=eload(sv)`
               reads the slot content (the cell pointer) instead of the discriminant -- one indirection short -> no enum
               arm's disc test matches -> the whole match falls through to empty output. Was: `match &o{Some(_)=>..}`
               dead (every by-ref enum/Option scrutinee). byref stays 1 to govern the payload binding (`*e` no-op).
               Scoped to `&<ident/self>` (not `&(` -- the `(&a,&b)` tuple-of-refs form is handled by tupref below). */
            let mut q = POS + 1; while q < LEN && at(q) == 32 { q = q + 1; }
            if q < LEN && is_idst(at(q)) { POS = POS + 1; skipws(); }
        }
        let mut tupref = 0;   /* LAYER9 P-BYREF-TUPLE-MATCH-PAYLOAD-DEREF: detect a tuple-of-refs scrutinee `match (&a, &b)` (the canonical structural-unify idiom). Each tuple element is then a pointer-TO-the-local (an extra indirection over a by-value tuple whose elements are the cell pointers directly); the tuple-pattern arm handler adds one `eload(elem)` and binds payloads by-ref (SFORREF) so `*v` is a no-op. Was: read disc/payload one level short -> `*v` derefed garbage -> OOB -> segfault. Detection: first non-ws char after `match` is `(`, first non-ws inside is `&`. */
        { let mut q = POS; while q < LEN && at(q) == 32 { q = q + 1; } if q < LEN && at(q) == 40 { q = q + 1; while q < LEN && at(q) == 32 { q = q + 1; } if q < LEN && at(q) == 38 { tupref = 1; } } }
        if tupref == 1 { byref = 1; }
        let mut splace = 0; let mut hasplace = 0;   /* P-MATCH-PATTERN (ref mut): a bare-local scrutinee's slot ADDRESS, for `ref`/`ref mut x` place bindings */
        let msave = POS; skipws();
        if POS < LEN && is_idst(at(POS)) { rd_ident(); let so = ID_O; let sl = ID_L; skipws(); if POS < LEN && at(POS) == 123 && sym_idx(so, sl) >= 0 { splace = eref(sym_find(so, sl)); hasplace = 1; } }   /* &-address of the scrutinee local's slot (like `&mut n`) */
        POS = msave;
        let sv = parse_expr();
        let scrk = EOPTK;                                                       /* P1: built-in Option/Result payload kind (3 String / 1 &str) from the scrutinee, for the Some(s)/Err(e) binder */
        let scrres = EOPTRES;                                                     /* P1: 1 if the scrutinee is a Result (payload kind applies to the Err arm), 0 if an Option (applies to Some) */
        let sctup = if EOPTTY >= 0 && TYTAG[EOPTTY as usize] == 11 { TYAUX[EOPTTY as usize] } else { 0 - 1 };   /* P3+: Option payload tuple-type id, for per-field kind restore in `Some((k,v))` arms (split_once -> two &str) */
        let scrsty = if EOPTTY >= 0 && TYTAG[EOPTTY as usize] == 12 { TYAUX[EOPTTY as usize] } else { 0 - 1 };   /* LAYER5 P-PRESULT-STRUCT-PAYLOAD-BINDER: the Ok/Some payload's USER-STRUCT index, so an `Ok(v)`/`Some(v)` binder restores v struct-typed (`v.field` dispatches) instead of reading the raw payload pointer */
        let scrsenum = if EOPTTY >= 0 && TYTAG[EOPTTY as usize] == 13 { TYAUX[EOPTTY as usize] } else { 0 - 1 };   /* D005 P-PRESULT-BOX-ENUM-PAYLOAD-BINDER: the Ok/Some payload's ENUM index (tag 13), so an `Ok(e)`/`Some(e)` binder tags e SENUM (`e.method()` dispatches through the enum) instead of reading the raw Box pointer */
        let scrsty2 = if EOPTERRTY >= 0 && TYTAG[EOPTERRTY as usize] == 12 { TYAUX[EOPTERRTY as usize] } else { 0 - 1 };   /* LAYER6 P-PRESULT-ERR-STRUCT-PAYLOAD: the Result ERR payload's USER-STRUCT index, so an `Err(e)` binder restores e struct-typed */
        let sarrlen = EARRLEN;                                                    /* PAT: fixed-array scrutinee length, for `[slice]` arms */
        if EWASREF == 1 { byref = 1; }                                            /* scrutinee is a &Enum param / for-ref */
        if EOPTREF == 1 { byref = 1; }                                            /* P-MATCH-PATTERN: `match v.first()`/get() -- Option<&T>, payload models a reference, so a `Some(n)` binding is by-ref (`*n` a no-op). Was: match path left it a value -> `*n` derefed the value -> segfault. */
        let sbool = EISBOOL;                                                      /* R2: scrutinee bool? (a bool match is exhaustive with true/false, no `_`) */
        eat_ch(123);
        let endlbl = LBL; LBL = LBL + 1;
        let result = NC; NC = NC + 1;
        let mut mi64 = 0;   /* LAYER6 P-ENUM-I64-PAYLOAD-WIDTH: 1 if any arm value is i64 -> the shared match-result slot must be copied 64-bit (ecopy64) and the match result tagged EIS64, else an i64 arm value (`N::Lit(v) => *v + BIG`) truncates to 32 bits at the result join. */
        let save2 = POS; let mut isenum = 0; let mut sord = 0;
        skipws();
        if POS < LEN && is_idst(at(POS)) { rd_ident(); let fo = ID_O; let fl = ID_L;
            if var_find(ID_O, ID_L) >= 0 || opt_disc(ID_O, ID_L) >= 0 { isenum = 1; }
            else { skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { isenum = 1; if is_ordering(fo, fl) { sord = 1; } } } }   /* TI3e-g: `match a.cmp(&b) { Ordering::.. }` -- Ordering scrutinee is a bare i32, not a heap enum */
        POS = save2;
        let mut base = sv;
        if isenum == 1 && sord == 0 { base = eload(sv); }   /* TI3e-g: an Ordering scrutinee is already the bare discriminant i32 -- do NOT eload (would deref -1/0/1 as a pointer -> segfault) */
        let mut menum = 0 - 1; let mut nmatched = 0; let mut sawdef = 0;           /* R4: exhaustiveness -- enum type, distinct variant arms, catch-all seen */
        let mvarmth = NSYM; let mvarmbase = MVLOGN; let mvarmstash = MVSTASHN;      /* MOVE-FLOW: outer-symbol threshold + per-arm log base + union stash -- each arm starts from the pre-match moved-set; the post-match set is the union of all arms' outer moves */
        loop {
            skipws();
            if POS < LEN && at(POS) == 125 { POS = POS + 1; break; }
            if POS >= LEN { break; }
            let armstart = POS;                                                    /* R24: progress guard -- a desync on an unsupported pattern must not spin the loop */
            let mark = NSYM;
            let mut istest = 1; let mut testv = 0;
            if at(POS) == 95 { POS = POS + 1; istest = 0; sawdef = 1; }
            else if at(POS) == 91 { bind_slice_pat(sv, sarrlen); istest = 0; sawdef = 1; }   /* PAT: `[slice]` arm on a fixed array -- irrefutable, bind by offset */
            else if at(POS) == 39 || is_dig(at(POS)) || (at(POS) == 45 && POS + 1 < LEN && is_dig(at(POS + 1))) {   /* R24: int/char literal, range lo..[=]hi, and R4 or-patterns p|p|p */
                let lo = read_pat_scalar();
                skipws();
                if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 {
                    POS = POS + 2; if POS < LEN && at(POS) == 61 { POS = POS + 1; }
                    let hi = read_pat_scalar();
                    let clo = ec(lo); let chi = ec(hi);
                    let gel = ebin("ge", base, clo); let lel = ebin("le", base, chi);
                    testv = ebin("mul", gel, lel);
                } else { let cv = ec(lo); testv = ebin("eq", base, cv); }
                skipws();
                while POS < LEN && at(POS) == 124 && (POS + 1 >= LEN || at(POS + 1) != 124) {   /* R4: `| pat` -- OR this scalar/range test in */
                    POS = POS + 1; skipws();
                    let lo2 = read_pat_scalar(); skipws();
                    let mut t2 = 0;
                    if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 {
                        POS = POS + 2; if POS < LEN && at(POS) == 61 { POS = POS + 1; }
                        let hi2 = read_pat_scalar();
                        let clo2 = ec(lo2); let chi2 = ec(hi2);
                        t2 = ebin("mul", ebin("ge", base, clo2), ebin("le", base, chi2));
                    } else { let cv2 = ec(lo2); t2 = ebin("eq", base, cv2); }
                    testv = ebin("or", testv, t2);
                    skipws();
                }
            } else if at(POS) == 91 {                                              /* R4: slice pattern [p, .., p] on a &[i32] descriptor [ptr@0, len@4] */
                POS = POS + 1;                                                     /* consume [ */
                let bstart = POS;
                let mut pre = 0; let mut post = 0; let mut hasrest = 0;
                loop {                                                             /* pass 1: count pre/post + detect `..` */
                    skipws();
                    if POS >= LEN || at(POS) == 93 { break; }
                    if at(POS) == 46 && POS + 1 < LEN && at(POS + 1) == 46 { hasrest = 1; POS = POS + 2; }
                    else { if at(POS) == 95 { POS = POS + 1; } else { rd_ident(); } if hasrest == 0 { pre = pre + 1; } else { post = post + 1; } }
                    skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; }
                }
                if POS < LEN && at(POS) == 93 { POS = POS + 1; }                   /* consume ] */
                let ptr = eload(base);
                let lenv = eload(ebin("add", base, ec(4)));
                let need = pre + post;
                if hasrest == 1 { testv = ebin("ge", lenv, ec(need)); } else { testv = ebin("eq", lenv, ec(need)); }
                let epos = POS;                                                    /* pass 2 rewinds; remember where to resume */
                POS = bstart;
                let mut idx = 0; let mut pj = 0; let mut seenrest = 0;
                loop {                                                             /* pass 2: bind each named element */
                    skipws();
                    if POS >= LEN || at(POS) == 93 { break; }
                    if at(POS) == 46 && POS + 1 < LEN && at(POS + 1) == 46 { seenrest = 1; POS = POS + 2; }
                    else {
                        if at(POS) == 95 { POS = POS + 1; }
                        else {
                            rd_ident(); let bo = ID_O; let bl = ID_L;
                            let mut ad = 0;
                            if seenrest == 0 { ad = ebin("add", ptr, ec(idx * 4)); }
                            else { let oe = ebin("add", ebin("sub", lenv, ec(post)), ec(pj)); ad = ebin("add", ptr, ebin("mul", oe, ec(4))); }
                            let elv = eload(ad);
                            let bslot = NC; ecopy(bslot, elv); NC = NC + 1; sym_add(bo, bl, bslot); SFORREF[(NSYM - 1) as usize] = 1;
                        }
                        if seenrest == 0 { idx = idx + 1; } else { pj = pj + 1; }
                    }
                    skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; }
                }
                POS = epos;
            } else if at(POS) == 40 {                                              /* R1: tuple pattern `(p0, p1, ..)` on a tuple scrutinee [f0@0, f1@4, ..] */
                POS = POS + 1;
                let mut fi = 0; let mut anytest = 0;
                loop {
                    skipws();
                    if POS >= LEN || at(POS) == 41 { break; }
                    let elem = eload(ebin("add", sv, ec(fi * 4)));   /* LAYER9 P-BYREF-TUPLE-MATCH-PAYLOAD-DEREF: for a by-ref tuple `(&a,&b)` the element is already the enum-cell pointer (`&enum` is transparent -- a boxed cell), same shape as a by-value element; the ONLY fix is byref->SFORREF (set via tupref above) so `*v` on a bound payload is a no-op. Was: byref=0 for a tuple scrutinee -> no SFORREF -> `*v` derefed the payload value -> OOB -> segfault. */
                    if at(POS) == 95 { POS = POS + 1; }                             /* _ wildcard */
                    else if at(POS) == 39 || is_dig(at(POS)) || (at(POS) == 45 && POS + 1 < LEN && is_dig(at(POS + 1))) {   /* literal element */
                        let lo = read_pat_scalar(); let cv = ec(lo); let t = ebin("eq", elem, cv);
                        if anytest == 0 { testv = t; anytest = 1; } else { testv = ebin("mul", testv, t); }
                    } else {                                                        /* binding OR variant-pattern element */
                        rd_ident(); let mut bo = ID_O; let mut bl = ID_L;
                        skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); rd_ident(); bo = ID_O; bl = ID_L; skipws(); }   /* E::V or bare V */
                        let evi = var_find(bo, bl); let eod = opt_disc(bo, bl);
                        if evi >= 0 || eod >= 0 {    /* R-either / LAYER8 P-TUPLE-NULLARY-VARIANT-ELEM: a variant-pattern element `V` or `V(subpat, ..)` in a tuple pattern -- test elem's disc (was: only tested when a `(` payload followed, so a NULLARY `(N::A, N::A)` fell to the bind branch below -> bound the variant name as a var -> the disc check was SKIPPED -> always matched). */
                            let edisc = if evi >= 0 { VDISC[evi as usize] } else { eod };
                            let ed = eload(elem); let t = ebin("eq", ed, ec(edisc));
                            if anytest == 0 { testv = t; anytest = 1; } else { testv = ebin("mul", testv, t); }
                            if POS < LEN && at(POS) == 40 {                          /* payload variant `V(subpat, ..)` -- bind the inner payload(s) */
                                POS = POS + 1;                                       /* consume ( */
                                let mut pf = 0;
                                loop {
                                    skipws(); if POS >= LEN || at(POS) == 41 { break; }
                                    let pv = eload(ebin("add", elem, ec(4 + pf * 4)));
                                    if at(POS) == 95 { POS = POS + 1; }
                                    else { rd_ident(); let pbo = ID_O; let pbl = ID_L; let ps = NC; ecopy(ps, pv); NC = NC + 1; sym_add(pbo, pbl, ps); if byref == 1 { SFORREF[(NSYM - 1) as usize] = 1; } }
                                    pf = pf + 1;
                                    skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; }
                                }
                                eat_ch(41);
                            }
                        } else {
                            let slot = NC; ecopy(slot, elem); NC = NC + 1; sym_add(bo, bl, slot);
                        }
                    }
                    fi = fi + 1;
                    skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; }
                }
                eat_ch(41);
                if anytest == 0 { istest = 0; sawdef = 1; }                         /* all wildcards/binds -> irrefutable arm */
                skipws();
                while POS < LEN && at(POS) == 124 && (POS + 1 >= LEN || at(POS + 1) != 124) {   /* LAYER4: tuple-pattern `|`-alternation `('#', _) | ('"', _)` -- OR each alternative tuple's element-eq test in (was: only the first tuple parsed, POS left at `|` -> eat_ch(=>) desynced -> arm no-progress -> R24 break -> the final `_` arm unseen -> false non-exhaustive; rustc_lexer raw_ident/c_or_byte_string) */
                    POS = POS + 1; skipws();
                    if POS < LEN && at(POS) == 40 {
                        POS = POS + 1;
                        let mut fi2 = 0; let mut atest = 0; let mut altt = 0;
                        loop {
                            skipws(); if POS >= LEN || at(POS) == 41 { break; }
                            let elem2 = eload(ebin("add", sv, ec(fi2 * 4)));
                            if at(POS) == 95 { POS = POS + 1; }
                            else if at(POS) == 39 || is_dig(at(POS)) || (at(POS) == 45 && POS + 1 < LEN && is_dig(at(POS + 1))) {
                                let lo = read_pat_scalar(); let cv = ec(lo); let t = ebin("eq", elem2, cv);
                                if atest == 0 { altt = t; atest = 1; } else { altt = ebin("mul", altt, t); }
                            } else {
                                rd_ident(); let mut bo = ID_O; let mut bl = ID_L;
                                skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; skipws(); rd_ident(); bo = ID_O; bl = ID_L; skipws(); }
                                let evi = var_find(bo, bl); let eod = opt_disc(bo, bl);
                                if evi >= 0 || eod >= 0 {
                                    let edisc = if evi >= 0 { VDISC[evi as usize] } else { eod };
                                    let ed = eload(elem2); let t = ebin("eq", ed, ec(edisc));
                                    if atest == 0 { altt = t; atest = 1; } else { altt = ebin("mul", altt, t); }
                                    if POS < LEN && at(POS) == 40 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } POS = POS + 1; } }
                                }
                            }
                            fi2 = fi2 + 1;
                            skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; }
                        }
                        eat_ch(41);
                        if atest == 1 { if istest == 1 { testv = ebin("or", testv, altt); } else { testv = altt; istest = 1; } }
                    }
                    skipws();
                }
            } else {
                rd_ident();
                let mut isref = 0;
                if id_is(114, 101, 102, 0, 0, 3) { isref = 1; skipws(); rd_ident(); if id_is(109, 117, 116, 0, 0, 3) { skipws(); rd_ident(); } }   /* P-MATCH-PATTERN: `ref x` / `ref mut x` -- skip the keyword(s); the binding is to the scrutinee PLACE */
                let bno = ID_O; let bnl = ID_L;                                    /* the (potential binding) name */
                skipws();
                let mut qual = 0;
                if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; rd_ident(); qual = 1; }   /* E::V or bare V */
                let mut vi = var_find(ID_O, ID_L);
                if qual == 1 { let qen = enum_find(bno, bnl); if qen >= 0 { let qv = var_find_in(qen, ID_O, ID_L); if qv >= 0 { vi = qv; } } }   /* LAYER6 P-QUALIFIED-VARIANT-RESOLUTION: `E::V` -> resolve V within enum E (bno/bnl), so a variant name shared with an earlier enum (Tok::Num vs Expr::Num) doesn't mis-set menum -> false non-exhaustive. */
                let od = opt_disc(ID_O, ID_L);
                let cmi = cfind(bno, bnl);                                          /* consts: a bare const name in a match arm is a value pattern, not a binding */
                if qual == 0 && vi < 0 && od < 0 && cmi >= 0 {
                    let cv = ec(CVAL[cmi as usize]); testv = ebin("eq", base, cv); istest = 1; anytest = 1;   /* const pattern: test scrutinee == const value */
                }
                else if qual == 0 && vi < 0 && od < 0 {                                 /* R4: binding pattern `x` (or `x @ subpat`) -- binds the scrutinee, always matches unless @ adds a test */
                    let bslot = NC; if isref == 1 && hasplace == 1 { ecopy(bslot, splace); } else { ecopy(bslot, base); } NC = NC + 1; sym_add(bno, bnl, bslot);   /* P-MATCH-PATTERN: `ref[ mut] x` binds x to the scrutinee's slot ADDRESS -> `*x` reads/writes the place (like `let x = &mut n`) */
                    istest = 0;
                    skipws();
                    if POS < LEN && at(POS) == 64 {                                /* `x @ subpat` -- subpat may be a `(a|b)` alternation */
                        POS = POS + 1; skipws();
                        let mut paren = 0;
                        if POS < LEN && at(POS) == 40 { paren = 1; POS = POS + 1; skipws(); }
                        let lo = read_pat_scalar(); skipws();
                        if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 {
                            POS = POS + 2; if POS < LEN && at(POS) == 61 { POS = POS + 1; }
                            let hi = read_pat_scalar();
                            let clo = ec(lo); let gel = ebin("ge", base, clo);
                            let chi = ec(hi); let lel = ebin("le", base, chi);
                            testv = ebin("mul", gel, lel);
                        } else { let cv = ec(lo); testv = ebin("eq", base, cv); }
                        skipws();
                        while POS < LEN && at(POS) == 124 && (POS + 1 >= LEN || at(POS + 1) != 124) {   /* R2ak: OR each alternative of `x @ (a|b|c)` */
                            POS = POS + 1; skipws();
                            let lo2 = read_pat_scalar(); skipws();
                            let mut t2 = 0;
                            if POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 {
                                POS = POS + 2; if POS < LEN && at(POS) == 61 { POS = POS + 1; }
                                let hi2 = read_pat_scalar();
                                let clo2 = ec(lo2); let chi2 = ec(hi2);
                                t2 = ebin("mul", ebin("ge", base, clo2), ebin("le", base, chi2));
                            } else { let cv2 = ec(lo2); t2 = ebin("eq", base, cv2); }
                            testv = ebin("or", testv, t2);
                            skipws();
                        }
                        if paren == 1 { if POS < LEN && at(POS) == 41 { POS = POS + 1; } }
                        istest = 1;
                    }
                    if istest == 0 { sawdef = 1; }                                 /* R4: a bare binding (no @) is an irrefutable catch-all */
                } else {
                    let mut disc = 0;
                    if vi >= 0 { disc = VDISC[vi as usize]; } else { disc = od; }
                    if sord == 1 { disc = ord_disc(ID_O, ID_L); }   /* TI3e-g: Ordering::Less/Equal/Greater -> -1/0/1 (the raw .cmp() value) */
                    if vi >= 0 { menum = VEN[vi as usize]; nmatched = nmatched + 1; }   /* R4: a distinct enum variant arm */
                    let cv = ec(disc); testv = ebin("eq", base, cv);
                    skipws();
                    let mut fpn = 1;   /* P-MATCH-PATTERN: first-alt payload is a NAMED binding (not `_`) -- for the E0408 consistency check across or-alternatives */
                    if POS < LEN && at(POS) == 40 {
                        POS = POS + 1;
                        let sp0 = POS; skipws(); if POS < LEN && at(POS) == 95 { fpn = 0; } POS = sp0;
                        let mut apk = 0 - 1;   /* P1: the built-in payload kind, applied only to the arm that carries it -- Option Some (disc 1) or Result Err (disc 0), never Result Ok */
                        if (scrk == 3 || scrk == 1) && vi < 0 { if scrres == 0 && disc == 1 { apk = scrk; } else if scrres == 1 && disc == 0 { apk = scrk; } }
                        let mut aptup = sctup;   /* LAYER5 P-PRESULT-STRUCT-PAYLOAD-BINDER: overload ptup -- keep the tuple-type id, else encode -(scrsty+2) for a struct SUCCESS arm (Ok/Some, disc 1). A struct success-arm is never a tuple, so sctup is -1 there and the two never collide. */
                        if scrsty >= 0 && vi < 0 && disc == 1 && sctup < 0 { aptup = 0 - (scrsty + 2); }
                        if scrsenum >= 0 && vi < 0 && disc == 1 && sctup < 0 { aptup = 0 - (scrsenum + 100000); }   /* D005: encode the Ok/Some ENUM-payload index in a HIGH-NEGATIVE range (<= -100000) so it never collides with the struct range -(sty+2) [small negatives] or the tuple range [>=0]; decoded in match_pat_payload */
                        if scrsty2 >= 0 && vi < 0 && disc == 0 && sctup < 0 { aptup = 0 - (scrsty2 + 2); }   /* LAYER6 P-PRESULT-ERR-STRUCT-PAYLOAD: the ERR (disc 0) arm's `Err(e)` payload is a user struct -> bind e struct-typed (the Err-side dual of the Ok/Some scrsty binding above) */
                        testv = match_pat_payload(sv, testv, byref, vi, apk, aptup);   /* R2ae: bind payloads, recurse into nested variant patterns Some(Some(n)); vi threads the variant index for P1 user-enum String-payload restore, apk the arm-correct built-in Option/Result payload kind, aptup the tuple-field kinds (>=0) OR the encoded Ok/Some user-struct payload type (<=-2) */
                        eat_ch(41);
                    }
                    else if POS < LEN && at(POS) == 123 {                           /* R1: struct-variant pattern `V{ f0, f1 }` -- bind positionally in decl order */
                        POS = POS + 1;
                        let mut fi = 0;
                        loop {
                            skipws(); if POS < LEN && at(POS) == 125 { break; }
                            rd_ident(); let mut bo = ID_O; let mut bl = ID_L; let mut ignore = 0;
                            skipws();
                            if POS < LEN && at(POS) == 58 { POS = POS + 1; skipws();   /* LAYER6 P-STRUCT-VARIANT-FIELD-RENAME: `field: binding` / `field: _` -- the field name sets the decl position, the binding name (or `_` ignore) is what enters scope. Was: only `rd_ident()` for the field name, leaving `: binding` unconsumed -> the `,`/`}` scan desynced -> the remaining fields never bound (pointer/garbage). */
                                if POS < LEN && at(POS) == 95 && (POS + 1 >= LEN || !is_idc(at(POS + 1))) { ignore = 1; POS = POS + 1; } else { rd_ident(); bo = ID_O; bl = ID_L; } }
                            let off = ec(vpay_off(vi, fi)); let pp = ebin("add", sv, off); let val = if vpay_i64(vi, fi) == 1 { eload8(pp) } else { eload(pp) };   /* P-ENUM-I64-PAYLOAD-OFFSET */
                            if ignore == 0 {
                                let slot = NC; ecopy(slot, val); NC = NC + 1; sym_add(bo, bl, slot);
                                if byref == 1 { SFORREF[(NSYM - 1) as usize] = 1; }    /* by-ref match: payload is a reference, `*v` is a no-op */
                                if vi >= 0 { let idx = vi * 8 + fi; if VFVEC[idx as usize] == 1 { SK[(NSYM - 1) as usize] = 2; } else if VFSTR[idx as usize] == 1 { SK[(NSYM - 1) as usize] = 3; } if VFI64[idx as usize] == 1 { SI64[(NSYM - 1) as usize] = 1; } if VFKIND[idx as usize] == 1 { SENUM[(NSYM - 1) as usize] = VEN[vi as usize] + 1; } }   /* LAYER6 P-STRUCT-VARIANT-FIELD-KIND: restore each named field's value-kind (Vec->SK2, String->SK3, i64->SI64) so `E::C{ args }=>args.len()` / a String field / an i64 field dispatch (was untagged scalar -> .len() returned the descriptor pointer, arithmetic 32-bit). Mirrors the tuple-payload binder's VFSTR/VFI64 restore. AST-4 P-STRUCT-VARIANT-BOX-ENUM-FIELD: also restore VFKIND==1 (a `Box<ThisEnum>` recursive field) -> tag SENUM so `E::B{lhs,rhs}=>lhs.eval()` dispatches through the enum (was untagged -> `lhs.eval()` on a raw pointer crashed; the tuple-variant binder already does this at the VFKIND==1 site). */
                            }
                            fi = fi + 1;
                            skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; }
                        }
                        eat_ch(125);
                    }
                    /* P-MATCH-PATTERN (or-variant): `E::A(n) | E::B(n) => ..` -- OR each alternative's
                       discriminant test into this arm; the shared payload binding is already emitted from
                       the FIRST alternative (same +4 offset), so skip the alternatives' payloads. Was: the
                       `|` was left unconsumed -> `eat_ch(=>)` desynced -> segfault. */
                    skipws();
                    while POS < LEN && at(POS) == 124 && (POS + 1 >= LEN || at(POS + 1) != 124) {
                        POS = POS + 1; skipws();
                        rd_ident();
                        if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; rd_ident(); }
                        let vi2 = var_find(ID_O, ID_L); let od2 = opt_disc(ID_O, ID_L);
                        let mut disc2 = 0; if vi2 >= 0 { disc2 = VDISC[vi2 as usize]; } else { disc2 = od2; }
                        if vi2 >= 0 { nmatched = nmatched + 1; }
                        let cv2 = ec(disc2); let t2 = ebin("eq", base, cv2); testv = ebin("or", testv, t2);
                        skipws();
                        if POS < LEN && at(POS) == 40 { POS = POS + 1; let sp1 = POS; skipws(); let mut apn = 1; if POS < LEN && at(POS) == 95 { apn = 0; } POS = sp1; if apn != fpn { die_mismatch(); }   /* E0408: a variable bound in one or-alternative but not all -> reject like rustc */
                            let mut d = 1; while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } POS = POS + 1; } }
                        skipws();
                    }
                }
            }
            skipws();
            let gs = POS;
            let ggot = rd_ident();
            if ggot && id_is(105, 102, 0, 0, 0, 2) {                               /* R4: `if GUARD` -- AND the guard (may use the pattern's bindings) into the arm test */
                if istest == 1 {   /* LAYER5 P-MATCH-GUARD-MULTIARM: GATE the guard eval behind the pattern test -- the guard runs only when the disc/sub-pattern matched, so a mismatched arm never evaluates it. Was: straight-line, so a Lit scrutinee reaching an `Expr::Add(a,b) if eval(a)==eval(b)` arm ran eval(a) on the Lit payload read as a pointer -> deref garbage -> segfault (rustc evaluates a guard only when the pattern matches). */
                    let gslot = NC; let z = ecr(0); ecopy(gslot, z); NC = NC + 1;   /* gslot = 0 (arm fails if the pattern didn't match) */
                    println!("  brz v{}", testv); ckreset();
                    let g = parse_expr(); ecopy(gslot, g);
                    println!("  pool"); ckreset();
                    testv = gslot;
                } else { let g = parse_expr(); testv = g; istest = 1; }   /* irrefutable pattern (`_`/bare bind) -> always matches, guard always evaluates */
            } else { POS = gs; }
            eat_ch(61); eat_ch(62);
            if istest == 1 { println!("  brz v{}", testv); ckreset(); }
            skipws();
            if POS < LEN && at(POS) == 123 {                                  /* `{ ... }` block arm (incl. empty `{}`) */
                let z = ecr(0); ecopy(result, z);
                parse_branch_into(result);
            } else if POS < LEN && at(POS) == 42 && (POS + 1 >= LEN || at(POS + 1) != 42) {   /* P-MATCH-PATTERN: `=> *p [op]= e` deref-(compound-)assign arm body (e.g. ref-mut `*x += 10`); no `;` -- comma/brace terminated. A `=> *x` VALUE arm rewinds to the general expr handler (which honors SFORREF). */
                let bstart = POS; POS = POS + 1; let p = parse_atom(); skipws();
                if POS < LEN && at(POS) == 61 && (POS + 1 >= LEN || at(POS + 1) != 61) { POS = POS + 1; let e = parse_expr(); println!("  store v{} v{}", p, e); let z = ecr(0); ecopy(result, z); }
                else if POS + 1 < LEN && at(POS + 1) == 61 && (at(POS) == 43 || at(POS) == 45 || at(POS) == 42 || at(POS) == 47 || at(POS) == 124 || at(POS) == 38 || at(POS) == 94) { let opc = at(POS); POS = POS + 2; let e = parse_expr(); let cur = eload(p); let mut nv = 0; if opc == 43 { nv = ebin("add", cur, e); } else if opc == 45 { nv = ebin("sub", cur, e); } else if opc == 42 { nv = ebin("mul", cur, e); } else if opc == 47 { nv = ebin("div", cur, e); } else if opc == 124 { nv = ebin("or", cur, e); } else if opc == 38 { nv = ebin("and", cur, e); } else { nv = ebin("xor", cur, e); } println!("  store v{} v{}", p, nv); let z = ecr(0); ecopy(result, z); }
                else { POS = bstart; let dv = parse_expr(); if EIS64 == 1 { mi64 = 1; ecopy64(result, dv); } else { ecopy(result, dv); } }   /* `=> *x` deref-VALUE arm -> general expr path (honors SFORREF / no-op). LAYER6 P-ENUM-I64-PAYLOAD-WIDTH: i64 arm value -> 64-bit copy. */
            } else {
                let bsave = POS;
                let bgot = rd_ident();
                let bo2 = ID_O; let bl2 = ID_L;
                let mut handled = 0;
                if bgot && is_kw_return() {                                        /* R2bm: `=> return [EXPR]` arm body -- emit ret (comma- or brace-terminated) */
                    skipws();
                    if POS < LEN && (at(POS) == 44 || at(POS) == 125) { let z = ecr(0); println!("  ret v{}", z); }
                    else { let mut v = parse_expr(); if CUR_RET_I64 == 1 && EIS64 == 0 { if CUR_RET_U64 == 1 { v = emit_conv(v); } else { v = emit_convs(v); } } println!("  ret v{}", v); }
                    handled = 1;
                }
                else if bgot && (is_print() || is_println()) { let nlf = is_println(); emit_print(nlf); let z = ecr(0); ecopy(result, z); handled = 1; }
                else if bgot {
                    skipws();
                    if POS < LEN && at(POS) == 61 && (POS + 1 >= LEN || at(POS + 1) != 61) {   /* NAME = EXPR arm */
                        POS = POS + 1;
                        let vi64 = sym_i64(bo2, bl2);
                        let rhs = parse_expr();
                        let gi = g_find(bo2, bl2);
                        let mut base = 0;
                        if gi >= 0 { base = egref(bo2, bl2); } else { let slot = sym_find(bo2, bl2); base = eref(slot); }
                        if vi64 == 1 { println!("  store8 v{} v{}", base, rhs); } else { println!("  store v{} v{}", base, rhs); }
                        let z = ecr(0); ecopy(result, z); handled = 1;
                    }
                    else if POS + 1 < LEN && at(POS + 1) == 61 && (at(POS) == 43 || at(POS) == 45 || at(POS) == 42 || at(POS) == 47) {   /* NAME op= EXPR arm (compound-assign body was a silent no-op) */
                        let opc = at(POS); POS = POS + 2;
                        let vi64 = sym_i64(bo2, bl2);
                        let e = parse_expr();
                        let gi = g_find(bo2, bl2);
                        let mut base = 0;
                        if gi >= 0 { base = egref(bo2, bl2); } else { let slot = sym_find(bo2, bl2); base = eref(slot); }
                        let mut cur = 0;
                        if vi64 == 1 { println!("  v{}: i64 = load8 v{}", NC, base); cur = NC; NC = NC + 1; } else { cur = eload(base); }
                        let mut nv = 0;
                        if vi64 == 1 {
                            if opc == 43 { nv = ebin8raw("add", cur, e); } else if opc == 45 { nv = ebin8raw("sub", cur, e); } else if opc == 42 { nv = ebin8raw("mul", cur, e); } else { nv = ebin8raw("div", cur, e); }
                            println!("  store8 v{} v{}", base, nv);
                        } else {
                            if opc == 43 { nv = ebin("add", cur, e); } else if opc == 45 { nv = ebin("sub", cur, e); } else if opc == 42 { nv = ebin("mul", cur, e); } else { nv = ebin("div", cur, e); }
                            println!("  store v{} v{}", base, nv);
                        }
                        let z = ecr(0); ecopy(result, z); handled = 1;
                    }
                }
                if handled == 0 { POS = bsave; let e = parse_expr(); if EIS64 == 1 { mi64 = 1; ecopy64(result, e); } else { ecopy(result, e); } }   /* LAYER6 P-ENUM-I64-PAYLOAD-WIDTH: i64 arm value -> 64-bit copy into the shared result slot. */
            }
            println!("  goto __Lm{}", endlbl); ckreset();
            if istest == 1 { println!("  pool"); ckreset(); }
            mv_stash(mvarmbase, mvarmth); mv_undo(mvarmbase);                      /* MOVE-FLOW: stash this arm's OUTER moves for the union, then reset to the pre-match moved-set for the next arm */
            NSYM = mark;
            skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; }
            if POS <= armstart { break; }                                         /* R24: no progress this arm -> stop rather than spin (unsupported pattern) */
        }
        mv_replay(mvarmstash);                                                    /* MOVE-FLOW: union every arm's outer moves into the post-match moved-set */
        if isenum == 1 && sawdef == 0 && menum >= 0 {                             /* R4: exhaustiveness -- an enum match with no catch-all must cover every variant */
            let mut tot = 0; let mut vi2 = 0;
            while vi2 < NVAR { if VEN[vi2 as usize] == menum { tot = tot + 1; } vi2 = vi2 + 1; }
            if nmatched < tot { die_nonexhaustive(); }
        }
        if isenum == 0 && sawdef == 0 && sbool == 0 { die_nonexhaustive(); }      /* R2: an integer match with no `_` catch-all is non-exhaustive (bool matches exempt) */
        println!("  label __Lm{}", endlbl); ckreset();
        EITYP = -1;
        if mi64 == 1 { EIS64 = 1; }   /* LAYER6 P-ENUM-I64-PAYLOAD-WIDTH: an i64-producing match is an i64 value -> `ret <match>` / `let x = match ..` / `-<match>` stay 64-bit. */
        result
    }
}

/* R9: monomorphization support */
fn gf_find(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NGF { if span_eq_at(GFO[i as usize], GFL[i as usize], o, l) { return i; } i = i + 1; } -1 }
}
fn ret_is_tparg(sig: i32, tpo: i32, tpl: i32) -> i32 {   /* KP0: 1 if the return type after `(params)` is the bare type-param T (`-> T`) */
    unsafe {
        let mut p = sig;
        if p < LEN && at(p) == 40 { let mut d = 1; p = p + 1; while p < LEN && d > 0 { if at(p) == 40 { d = d + 1; } if at(p) == 41 { d = d - 1; } p = p + 1; } }
        while p < LEN && (at(p) == 32 || at(p) == 9) { p = p + 1; }
        if p + 1 < LEN && at(p) == 45 && at(p + 1) == 62 {
            p = p + 2; while p < LEN && (at(p) == 32 || at(p) == 9) { p = p + 1; }
            let s = p; while p < LEN && is_idc(at(p)) { p = p + 1; }
            if span_eq_at(s, p - s, tpo, tpl) { return 1; }
        }
        0
    }
}
fn ret_is_opt_tparg(sig: i32, tpo: i32, tpl: i32) -> i32 {   /* KP0/K1: 1 if return is Option<T>, 2 if Result<T,..> (T = the type-param) */
    unsafe {
        let mut p = sig;
        if p < LEN && at(p) == 40 { let mut d = 1; p = p + 1; while p < LEN && d > 0 { if at(p) == 40 { d = d + 1; } if at(p) == 41 { d = d - 1; } p = p + 1; } }
        while p < LEN && (at(p) == 32 || at(p) == 9) { p = p + 1; }
        if !(p + 1 < LEN && at(p) == 45 && at(p + 1) == 62) { return 0; }
        p = p + 2; while p < LEN && (at(p) == 32 || at(p) == 9) { p = p + 1; }
        let s = p; while p < LEN && is_idc(at(p)) { p = p + 1; }
        let cl = p - s;
        let isopt = cl == 6 && at(s) == 79 && at(s + 1) == 112 && at(s + 2) == 116 && at(s + 3) == 105 && at(s + 4) == 111 && at(s + 5) == 110;
        let isres = cl == 6 && at(s) == 82 && at(s + 1) == 101 && at(s + 2) == 115 && at(s + 3) == 117 && at(s + 4) == 108 && at(s + 5) == 116;
        if !(isopt || isres) { return 0; }
        while p < LEN && (at(p) == 32 || at(p) == 9) { p = p + 1; }
        if p >= LEN || at(p) != 60 { return 0; }
        p = p + 1; while p < LEN && (at(p) == 32 || at(p) == 9) { p = p + 1; }
        let s2 = p; while p < LEN && is_idc(at(p)) { p = p + 1; }
        if span_eq_at(s2, p - s2, tpo, tpl) { if isres { return 2; } return 1; }
        0
    }
}
fn find_tparg(sig: i32, tpo: i32, tpl: i32) -> i32 {   /* index of the first param whose type is the type-param T */
    unsafe {
        let saved = POS;
        POS = sig; eat_ch(40);
        let mut idx = 0;
        loop {
            skipws();
            if POS >= LEN || at(POS) == 41 { POS = saved; return 0; }
            rd_ident();
            skipws(); eat_ch(58); skipws();
            if POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); if is_kw_mut_next() { } }
            rd_ident();
            if span_eq_at(ID_O, ID_L, tpo, tpl) { POS = saved; return idx; }
            let mut d = 0;
            loop {
                if POS >= LEN { break; }
                let ch = at(POS);
                if ch == 40 || ch == 91 || ch == 60 { d = d + 1; POS = POS + 1; }
                else if ch == 41 || ch == 93 || ch == 62 { if d == 0 { POS = POS; break; } d = d - 1; POS = POS + 1; }
                else if d == 0 && ch == 44 { break; }
                else { POS = POS + 1; }
            }
            if POS < LEN && at(POS) == 44 { POS = POS + 1; idx = idx + 1; } else { POS = saved; return 0; }
        }
    }
}
fn prepass_generics() {
    unsafe {
        POS = 0;
        loop {
            skipws();
            if POS >= LEN { break; }
            let save = POS;
            if rd_ident() && is_kw_fn() {
                rd_ident();
                let fo = ID_O; let fl = ID_L;
                let ngf_before = NGF;
                skipws();
                if POS < LEN && at(POS) == 60 {
                    POS = POS + 1; let ghs = POS; skipws();
                    /* P1: skip leading lifetime params `'a, 'b, ..` -- they erase; only a TYPE param makes this generic */
                    loop { skipws(); if POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; } } else { break; } }
                    skipws();
                    if POS < LEN && at(POS) == 62 {                                    /* lifetime-only generic -> NOT monomorphizable; treat as a normal fn */
                        POS = POS + 1;
                    } else {
                        rd_ident();
                        let tpo = ID_O; let tpl = ID_L;
                        /* W1b: is the FIRST type param Fn-bound (a closure)? then it must NOT drive monomorphization (closures pass as fat pointers uniformly) */
                        let mut firstfn = 0; let mut sd = 0; let mut sp = POS;
                        loop { if sp >= LEN { break; } let sc = at(sp);
                            if sc == 60 || sc == 40 { sd = sd + 1; }
                            else if sc == 62 { if sd == 0 { break; } sd = sd - 1; }
                            else if sc == 41 { sd = sd - 1; }
                            else if sc == 44 && sd == 0 { break; }
                            else if sd == 0 && sc == 70 && sp + 1 < LEN && at(sp + 1) == 110 { firstfn = 1; }
                            sp = sp + 1; }
                        let mut d = 1; while POS < LEN && d > 0 { if at(POS) == 45 && POS + 1 < LEN && at(POS + 1) == 62 { POS = POS + 2; } else { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
                        let ghe = POS - 1;   /* TRAIT: the closing `>` of the generic header */
                        skipws();
                        let sig = POS;
                        if firstfn == 1 {                                             /* W1b: Fn-bound sole/first type param -> emit as a normal fn (parse_fn tags f:F SK=4) */
                        } else {
                        let arg = find_tparg(sig, tpo, tpl);
                        GFO[NGF as usize] = fo; GFL[NGF as usize] = fl; GFTPO[NGF as usize] = tpo; GFTPL[NGF as usize] = tpl;
                        GFSIG[NGF as usize] = sig; GFGS[NGF as usize] = ghs; GFGE[NGF as usize] = ghe; GFARG[NGF as usize] = arg; GFRETT[NGF as usize] = ret_is_tparg(sig, tpo, tpl); GFRETOPT[NGF as usize] = ret_is_opt_tparg(sig, tpo, tpl); NGF = NGF + 1;
                        }
                    }
                }
                if NGF == ngf_before && POS < LEN && at(POS) == 40 {   /* TRAIT R2: `fn NAME(x: impl Display/Debug)` (no explicit <T>) -> monomorphize per arg type like <X: Display>; register keyed on the `impl` token */
                    let psig = POS; let mut ipos = 0 - 1; let mut q = POS + 1; let mut dep = 0;
                    loop {
                        if q >= LEN { break; }
                        let c = at(q);
                        if c == 40 || c == 60 { dep = dep + 1; }
                        else if c == 41 { if dep == 0 { break; } dep = dep - 1; }
                        else if c == 62 { dep = dep - 1; }
                        else if c == 105 && q + 5 < LEN && at(q + 1) == 109 && at(q + 2) == 112 && at(q + 3) == 108 && at(q + 4) == 32 {   /* `impl ` */
                            let mut w = q + 4; while w < LEN && at(w) == 32 { w = w + 1; }
                            if w + 1 < LEN && at(w) == 68 && (at(w + 1) == 105 || at(w + 1) == 101) { ipos = q; }   /* Display / Debug */
                        }
                        q = q + 1;
                    }
                    if ipos >= 0 {
                        let arg = find_tparg(psig, ipos, 4);
                        GFO[NGF as usize] = fo; GFL[NGF as usize] = fl; GFTPO[NGF as usize] = ipos; GFTPL[NGF as usize] = 4;
                        GFSIG[NGF as usize] = psig; GFGS[NGF as usize] = psig; GFGE[NGF as usize] = psig; GFARG[NGF as usize] = arg; GFRETT[NGF as usize] = 0; GFRETOPT[NGF as usize] = 0; NGF = NGF + 1;
                    }
                }
            } else { POS = save; skip_tok(); }
        }
    }
}
fn reg_inst(gf: i32, styp: i32) {
    unsafe {
        let mut i = 0; while i < NINST { if IGF[i as usize] == gf && ISTYP[i as usize] == styp { return; } i = i + 1; }
        IGF[NINST as usize] = gf; ISTYP[NINST as usize] = styp; NINST = NINST + 1;
    }
}
fn emit_generic_inst(gf: i32, styp: i32) {
    unsafe {
        NC = 0; NSYM = 0; ckreset();
        GSUB_ACT = 1; GSUB_TPO = GFTPO[gf as usize]; GSUB_TPL = GFTPL[gf as usize]; GSUB_STYP = styp;
        print!("fn "); emit_span(GFO[gf as usize], GFL[gf as usize]); print!("_");
        if styp == -2 { print!("i32"); } else if styp == -3 { print!("str"); } else if styp == -4 { print!("string"); } else { emit_span(TSO[styp as usize], TSL[styp as usize]); }
        println!("");
        scan_fn_generics(GFGS[gf as usize], GFGE[gf as usize]);   /* TRAIT: record this generic's Fn-bound params (e.g. a 2nd-position `F: Fn`) so parse_pb tags `f: F` as a closure (SK=4) -- the monomorph path skips the header, so GFNO was stale */
        POS = GFSIG[gf as usize];
        parse_pb(-1);
        GSUB_ACT = 0;
    }
}
fn blanket_find(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NBI { if span_eq_at(BIMO[i as usize], BIML[i as usize], o, l) { return i; } i = i + 1; } -1 }
}
/* S3 R1: a generic-inherent-impl method `Type::method` (from `impl<T> Type<T>`) -> registry index, or -1 */
fn gim_find(ty: i32, mo: i32, ml: i32) -> i32 {
    unsafe { let mut i = 0; while i < NGIM { if GIMOWN[i as usize] == ty && span_eq_at(GIMMO[i as usize], GIML[i as usize], mo, ml) { return i; } i = i + 1; } -1 }
}
/* S3 R1: register a generic-impl monomorph (gim, styp) to emit in the deferred pass (dedup). */
fn reg_gim_inst(gim: i32, styp: i32) {
    unsafe { let mut i = 0; while i < NGIMINST { if GIMINST[i as usize] == gim && GIMINSTM[i as usize] == styp { return; } i = i + 1; }
        if NGIMINST < 1024 { GIMINST[NGIMINST as usize] = gim; GIMINSTM[NGIMINST as usize] = styp; NGIMINST = NGIMINST + 1; } }
}
/* S3 R1: emit `Type_method_<styp>` per-T monomorph; GIM_ACT lets self.v resolve via GSUB_STYP. */
fn emit_gim_inst(gim: i32, styp: i32) {
    unsafe {
        let save = POS; let savenc = NC; let savesym = NSYM;
        NC = 0; NSYM = 0; ckreset(); GNFN = 0;
        GSUB_ACT = 1; GSUB_TPO = GIMTPO[gim as usize]; GSUB_TPL = GIMTPL[gim as usize]; GSUB_STYP = styp; GIM_ACT = 1;
        print!("fn "); emit_span(TSO[GIMOWN[gim as usize] as usize], TSL[GIMOWN[gim as usize] as usize]); print!("_"); emit_span(GIMMO[gim as usize], GIML[gim as usize]); print!("_");
        if styp == 0 - 3 { print!("str"); } else if styp == 0 - 4 { print!("string"); } else if styp == 0 - 2 { print!("i32"); } else { emit_span(TSO[styp as usize], TSL[styp as usize]); }
        println!("");
        POS = GIMBODY[gim as usize];
        parse_pb(GIMOWN[gim as usize]);
        GSUB_ACT = 0; GIM_ACT = 0;
        POS = save; NC = savenc; NSYM = savesym;
    }
}
fn reg_blanket_inst(bi: i32, styp: i32) {
    unsafe {
        let mut i = 0; while i < NBINST { if IBI[i as usize] == bi && IBTYP[i as usize] == styp { return; } i = i + 1; }
        IBI[NBINST as usize] = bi; IBTYP[NBINST as usize] = styp; NBINST = NBINST + 1;
    }
}
fn emit_blanket_inst(bi: i32, styp: i32) {
    unsafe {
        NC = 0; NSYM = 0; ckreset();
        GSUB_ACT = 1; GSUB_TPO = BITPO[bi as usize]; GSUB_TPL = BITPL[bi as usize]; GSUB_STYP = styp;
        print!("fn ");
        if styp == 0 - 2 { print!("i32"); } else if styp == 0 - 3 { print!("str"); } else if styp == 0 - 4 { print!("string"); } else { emit_span(TSO[styp as usize], TSL[styp as usize]); }
        print!("_"); emit_span(BIMO[bi as usize], BIML[bi as usize]); println!("");
        POS = BIBODY[bi as usize];
        if styp < 0 { BLANK_PRIM_SELF = 1; }   /* R1 keystone: prim-concrete self is a scalar value */
        parse_pb(styp);
        BLANK_PRIM_SELF = 0;
        GSUB_ACT = 0;
    }
}
/* R2: register the methods of every blanket `impl<T: Bound> Trait for T { .. }` (for-type unregistered = a type param) */
fn prepass_blankets() {
    unsafe {
        POS = 0;
        loop {
            skipws();
            if POS >= LEN { break; }
            let save = POS;
            if rd_ident() && is_kw_impl() {
                skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
                rd_ident();
                skipws(); let s2 = POS;
                let mut isfor = 0; let mut fo = 0; let mut fl = 0;
                if rd_ident() && ID_L == 3 && at(ID_O) == 102 && at(ID_O + 1) == 111 && at(ID_O + 2) == 114 { isfor = 1; rd_ident(); fo = ID_O; fl = ID_L; } else { POS = s2; }
                if isfor == 1 && ty_find(fo, fl) < 0 {                                   /* blanket: `for T` where T is a type param */
                    skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
                    eat_ch(123);
                    loop {
                        skipws();
                        if POS >= LEN { break; }
                        if at(POS) == 125 { POS = POS + 1; break; }
                        if rd_ident() && is_kw_fn() {
                            rd_ident(); let mo = ID_O; let ml = ID_L;
                            skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
                            skipws();
                            BIMO[NBI as usize] = mo; BIML[NBI as usize] = ml; BITPO[NBI as usize] = fo; BITPL[NBI as usize] = fl; BIBODY[NBI as usize] = POS;
                            { let mut q = POS; let mut rk = 0;   /* R16b: scan the sig for `-> String`/`-> &str` so a prim-receiver call can tag the result's string kind */
                              while q < LEN && at(q) != 123 {
                                  if at(q) == 45 && q + 1 < LEN && at(q + 1) == 62 { let mut r = q + 2; while r < LEN && (at(r) == 32 || at(r) == 9) { r = r + 1; } if r < LEN && at(r) == 38 { r = r + 1; while r < LEN && (at(r) == 32 || at(r) == 9) { r = r + 1; } } if is_string_ty(r, 6) { rk = 3; } else if r + 2 < LEN && at(r) == 115 && at(r + 1) == 116 && at(r + 2) == 114 { rk = 1; } q = LEN; }
                                  else { q = q + 1; }
                              }
                              BIRET[NBI as usize] = rk; }
                            if prim_int_ty(fo, fl) > 0 { BIPRIM[NBI as usize] = 0 - 2; } else { BIPRIM[NBI as usize] = 0; }   /* R1 keystone: `impl Tr for i32` is a concrete prim impl, not a true blanket */
                            NBI = NBI + 1;
                            let mut d = 0;                                               /* skip `( .. )` then the `{ body }` */
                            loop { if POS >= LEN { break; } let ch = at(POS); if ch == 40 { d = d + 1; POS = POS + 1; } else if ch == 41 { d = d - 1; POS = POS + 1; if d == 0 { break; } } else { POS = POS + 1; } }
                            while POS < LEN && at(POS) != 123 { POS = POS + 1; }
                            eat_ch(123);
                            let mut db = 1; while POS < LEN && db > 0 { if at(POS) == 123 { db = db + 1; } if at(POS) == 125 { db = db - 1; } POS = POS + 1; }
                        } else { skip_tok(); }
                    }
                } else { POS = save; skip_tok(); }
            } else { POS = save; skip_tok(); }
        }
    }
}

/* R15: impl-method return-struct type (or -1) */
fn mret_find(ty: i32, mo: i32, ml: i32) -> i32 {
    unsafe { let mut i = 0; while i < NMRT { if MRTOWN[i as usize] == ty && span_eq_at(MRTO[i as usize], MRTL[i as usize], mo, ml) { return MRTS[i as usize]; } i = i + 1; } -1 }
}
/* Wave-2 bug 3: 1 if method `ty::mo` is registered AND returns unit/() (no `-> T`); 0 if not found or has a return type */
fn mret_void(ty: i32, mo: i32, ml: i32) -> i32 {
    unsafe { let mut i = 0; while i < NMRT { if MRTOWN[i as usize] == ty && span_eq_at(MRTO[i as usize], MRTL[i as usize], mo, ml) { return MRTV[i as usize]; } i = i + 1; } 0 }
}
/* value-width: 1 if method `ty::mo` declares a `-> i64/u64` return */
fn mret_i64(ty: i32, mo: i32, ml: i32) -> i32 {
    unsafe { let mut i = 0; while i < NMRT { if MRTOWN[i as usize] == ty && span_eq_at(MRTO[i as usize], MRTL[i as usize], mo, ml) { return MRTI64[i as usize]; } i = i + 1; } 0 }
}
/* R2bv: 1 if method `ty::mo` declares a `-> bool` return */
fn mret_bool(ty: i32, mo: i32, ml: i32) -> i32 {
    unsafe { let mut i = 0; while i < NMRT { if MRTOWN[i as usize] == ty && span_eq_at(MRTO[i as usize], MRTL[i as usize], mo, ml) { return MRTB[i as usize]; } i = i + 1; } 0 }
}
/* S1 L1: 3 if method `ty::mo` returns String, 1 if &str, 0 otherwise */
fn mret_str(ty: i32, mo: i32, ml: i32) -> i32 {
    unsafe { let mut i = 0; while i < NMRT { if MRTOWN[i as usize] == ty && span_eq_at(MRTO[i as usize], MRTL[i as usize], mo, ml) { return MRTSTR[i as usize]; } i = i + 1; } 0 }
}
fn mret_enum(ty: i32, mo: i32, ml: i32) -> i32 {   /* LAYER5 P-METHOD-RETURNS-ENUM: this method's `-> UserEnum`/`-> &UserEnum` return enum index (-1 = none) */
    unsafe { let mut i = 0; while i < NMRT { if MRTOWN[i as usize] == ty && span_eq_at(MRTO[i as usize], MRTL[i as usize], mo, ml) { return MREN[i as usize]; } i = i + 1; } 0 - 1 }
}
fn mret_optk(ty: i32, mo: i32, ml: i32) -> i32 {   /* SPAN-4 P-OPTION-PAYLOAD-DISPLAY-KIND (method arm): this method's `-> Option<P>`/`-> Result<P,_>` payload display-kind (0 scalar, 1 str, 3 char) */
    unsafe { let mut i = 0; while i < NMRT { if MRTOWN[i as usize] == ty && span_eq_at(MRTO[i as usize], MRTL[i as usize], mo, ml) { return MROPTK[i as usize]; } i = i + 1; } 0 }
}
fn mret_optenum(ty: i32, mo: i32, ml: i32) -> i32 {   /* D005 (method arm): this method's `-> Result/Option<Box<Enum>/Enum>` success-payload enum index (-1 = none) */
    unsafe { let mut i = 0; while i < NMRT { if MRTOWN[i as usize] == ty && span_eq_at(MRTO[i as usize], MRTL[i as usize], mo, ml) { return MROPTENUM[i as usize]; } i = i + 1; } 0 - 1 }
}
fn mret_optref(ty: i32, mo: i32, ml: i32) -> i32 {   /* AST-1 P-OPTION-PAYLOAD-BYREF (method arm): this method's `-> Option<&T>`/`-> Option<Box<T>>` payload-is-a-reference flag */
    unsafe { let mut i = 0; while i < NMRT { if MRTOWN[i as usize] == ty && span_eq_at(MRTO[i as usize], MRTL[i as usize], mo, ml) { return MROPTREF[i as usize]; } i = i + 1; } 0 }
}
fn mret_vec(ty: i32, mo: i32, ml: i32) -> i32 {   /* LAYER7 P-METHOD-RETURNS-VEC: 1 if this method returns `-> Vec<T>`/`-> ThinVec<T>` (with MRVELT/MRVELENUM = element type) */
    unsafe { let mut i = 0; while i < NMRT { if MRTOWN[i as usize] == ty && span_eq_at(MRTO[i as usize], MRTL[i as usize], mo, ml) { if MRVEC[i as usize] == 1 { DVELT = MRVELT[i as usize]; VELT = MRVELT[i as usize]; DVELENUM = MRVELENUM[i as usize]; VELENUM = MRVELENUM[i as usize]; } return MRVEC[i as usize]; } i = i + 1; } 0 }
}
fn mret_ref(ty: i32, mo: i32, ml: i32) -> i32 {   /* LAYER5 P-METHOD-RETURNS-SLICE-ENUM-REF: 1 if this method returns a reference `-> &T` */
    unsafe { let mut i = 0; while i < NMRT { if MRTOWN[i as usize] == ty && span_eq_at(MRTO[i as usize], MRTL[i as usize], mo, ml) { return MRTREF[i as usize]; } i = i + 1; } 0 }
}
fn prepass_methods() {
    unsafe {
        POS = 0;
        loop {
            skipws();
            if POS >= LEN { break; }
            let save = POS;
            if rd_ident() && is_kw_impl() {
                let mut bp = save; while bp > 0 && (at(bp - 1) == 32 || at(bp - 1) == 9 || at(bp - 1) == 10 || at(bp - 1) == 13) { bp = bp - 1; }
                if bp >= 2 && at(bp - 1) == 62 && at(bp - 2) == 45 { POS = save; skip_tok(); continue; }   /* LAYER4: `-> impl Trait` RETURN TYPE (e.g. tokenize `-> impl Iterator<Item=Token>`) is NOT an impl BLOCK -- was mistaken for one, consuming the fn body as method registrations (a ~530-line over-consume dropping every following method) */
                let mut gpo = 0; let mut gpl = 0;                                     /* std-hoist P0.1: impl-level generic param name (e.g. T in `impl<T> B<T>`) */
                skipws(); if POS < LEN && at(POS) == 60 {
                    let gp = POS + 1; POS = gp; skipws();
                    while POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; skipws(); } }
                    if POS < LEN && is_idst(at(POS)) { let sp = POS; if rd_ident() { gpo = ID_O; gpl = ID_L; } POS = sp; }
                    POS = gp; let mut d = 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
                rd_ident(); let mut to = ID_O; let mut tl = ID_L;
                skipws(); let s = POS;
                if rd_ident() && ID_L == 3 && at(ID_O) == 102 && at(ID_O + 1) == 111 && at(ID_O + 2) == 114 { rd_ident(); to = ID_O; tl = ID_L; } else { POS = s; }
                skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
                let mut owner = ty_find(to, tl);
                if owner < 0 { let en0 = enum_find(to, tl); if en0 >= 0 { owner = 0 - (en0 + 2); } }   /* std-hoist GAP1: an impl on a user enum -> encode owner as -(en+2) so `unwrap(self)->T` registers MRTS=-3 */
                eat_ch(123);
                loop {
                    skipws();
                    if POS < LEN && at(POS) == 125 { POS = POS + 1; break; }
                    if POS >= LEN { break; }
                    let mvs = POS; if rd_ident() && id_is(112, 117, 98, 0, 0, 3) { skipws(); if POS < LEN && at(POS) == 40 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } POS = POS + 1; } skipws(); } } else { POS = mvs; }   /* LAYER4: skip a `pub` / `pub(crate)` visibility before a method's `fn`; else rd_ident consumes `pub` and the following skip_tok eats the `fn`, desyncing every visibility-qualified method (Cursor::new/as_str/bump/advance_token registered 0) */
                    if rd_ident() && is_kw_fn() {
                        rd_ident(); let mo = ID_O; let ml = ID_L;
                        skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
                        skipws(); if POS < LEN && at(POS) == 40 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } POS = POS + 1; } }
                        skipws(); let mut rs = -1; let mut rsb = 0; let mut rvoid = 1; let mut mrefret = 0;   /* rvoid: 1 until a `-> T` return type is seen; mrefret: LAYER5 P-METHOD-RETURNS-SLICE-ENUM-REF -- the return type had a leading `&` */
                        if POS + 1 < LEN && at(POS) == 45 && at(POS + 1) == 62 { rvoid = 0; POS = POS + 2; skipws();
                            if POS < LEN && at(POS) == 38 { mrefret = 1; POS = POS + 1; skipws(); if POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); skipws(); } let ms = POS; if rd_ident() && is_kw_mut() { skipws(); } else { POS = ms; } }   /* R2: skip `& [lifetime] mut` so `-> &mut C`/`-> &mut Self` register the return type (builder chain); mrefret: the return is a REFERENCE */
                            rd_ident();
                            if is_box_ident(ID_O, ID_L) && POS < LEN && at(POS) == 60 { let mut ip = POS + 1; while ip < LEN && (at(ip) == 32 || at(ip) == 38) { ip = ip + 1; } if !(ip + 2 < LEN && at(ip) == 100 && at(ip + 1) == 121 && at(ip + 2) == 110) { POS = POS + 1; skipws(); while POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); } rd_ident(); } }   /* LAYER6 P-FN-RETURNS-BOX-ENUM (method arm): `fn f(..) -> Box<T>` is a transparent pointer -- see THROUGH Box to T so a method returning `Box<Enum>`/`Box<Struct>` tags MREN/MRTS like `-> Enum`/`-> Struct` (else `let ast = p.parse()` on a `-> Box<Expr>` method bound untagged -> eval(&ast) read 0). Guard skips `Box<dyn ..>` closures. Mirrors the free-fn scan_ret_kind arm. */
                            if ID_L == 4 && at(ID_O) == 83 && at(ID_O + 1) == 101 && at(ID_O + 2) == 108 && at(ID_O + 3) == 102 { rs = owner; } else { rs = ty_find(ID_O, ID_L); }
                            if gpl > 0 && span_eq_at(gpo, gpl, ID_O, ID_L) { rs = 0 - 3; }   /* std-hoist P0.1: `-> T` returns the generic param -> resolve at the call from the value's type-arg kind */
                            if ID_L == 4 && at(ID_O) == 98 && at(ID_O + 1) == 111 && at(ID_O + 2) == 111 && at(ID_O + 3) == 108 { rsb = 1; } }   /* R2bv: -> bool */
                        let rsi64 = rvoid == 0 && ID_L == 3 && at(ID_O + 1) == 54 && at(ID_O + 2) == 52 && (at(ID_O) == 105 || at(ID_O) == 117);   /* value-width: -> i64/u64 */
                        let mut rsstr = 0; if rvoid == 0 { if is_string_ty(ID_O, ID_L) { rsstr = 3; } else if ID_L == 3 && at(ID_O) == 115 && at(ID_O + 1) == 116 && at(ID_O + 2) == 114 { rsstr = 1; }
                            else if ID_L == 6 && at(ID_O) == 82 && at(ID_O + 1) == 101 && at(ID_O + 2) == 115 && at(ID_O + 3) == 117 && at(ID_O + 4) == 108 && at(ID_O + 5) == 116 { rsstr = 4; }   /* method-? sub-arc: `-> Result<..>` return kind (code 4; consumed only by dispatch_struct's method-`?` route) */
                            else if ID_L == 6 && at(ID_O) == 79 && at(ID_O + 1) == 112 && at(ID_O + 2) == 116 && at(ID_O + 3) == 105 && at(ID_O + 4) == 111 && at(ID_O + 5) == 110 { rsstr = 5; }   /* method-? sub-arc: `-> Option<..>` return kind (code 5) */
                            else if ID_L == 4 && at(ID_O) == 99 && at(ID_O + 1) == 104 && at(ID_O + 2) == 97 && at(ID_O + 3) == 114 { rsstr = 6; } }   /* LAYER4: `-> char` return kind (code 6) -- so emit_mcall tags the result EISCHAR (was: `self.first()` char result untagged -> Displayed as int) */
                        let mut rsoptk = 0; let mut rsoptref = 0; let mut rsoptenum = 0 - 1;   /* SPAN-4/AST-1/D005: a `-> Option<P>`/`-> Result<P,_>` method -- peek the payload ident (POS is just past "Option"/"Result", at `<`) for its display-kind (str/String -> 1, char -> 3) AND whether P is a reference (`&T`/`Box<T>` -> rsoptref). A local cursor `pp`; POS is untouched for the prepass's body scan. */
                        if rsstr == 4 || rsstr == 5 { let mut pp = POS; while pp < LEN && (at(pp) == 32 || at(pp) == 9) { pp = pp + 1; }
                            if pp < LEN && at(pp) == 60 { pp = pp + 1;
                                let mut ap = pp; while ap < LEN && at(ap) == 32 { ap = ap + 1; } if ap < LEN && at(ap) == 38 { rsoptref = 1; }   /* AST-1: `&` before the payload -> by-ref */
                                while pp < LEN && (at(pp) == 32 || at(pp) == 38) { pp = pp + 1; }
                                if pp < LEN && at(pp) == 39 { pp = pp + 1; while pp < LEN && is_idc(at(pp)) { pp = pp + 1; } while pp < LEN && at(pp) == 32 { pp = pp + 1; } }
                                let po = pp; while pp < LEN && is_idc(at(pp)) { pp = pp + 1; } let pll = pp - po;
                                if pll == 3 && at(po) == 115 && at(po + 1) == 116 && at(po + 2) == 114 { rsoptk = 1; }
                                else if pll == 6 && at(po) == 83 && at(po + 1) == 116 && at(po + 2) == 114 && at(po + 3) == 105 && at(po + 4) == 110 && at(po + 5) == 103 { rsoptk = 1; }
                                else if pll == 4 && at(po) == 99 && at(po + 1) == 104 && at(po + 2) == 97 && at(po + 3) == 114 { rsoptk = 3; }
                                if pll == 3 && at(po) == 66 && at(po + 1) == 111 && at(po + 2) == 120 { rsoptref = 1; }
                                if rsoptk != 0 { rsoptref = 0; }
                                { let mut qeo = po; let mut qel = po + pll;   /* D005 (method arm): see THROUGH a `Box<Inner>` success payload to the inner enum -> rsoptenum */
                                    if pll == 3 && at(po) == 66 && at(po + 1) == 111 && at(po + 2) == 120 && qel < LEN && at(qel) == 60 { qeo = qel + 1; while qeo < LEN && (at(qeo) == 32 || at(qeo) == 38) { qeo = qeo + 1; } qel = qeo; while qel < LEN && is_idc(at(qel)) { qel = qel + 1; } }
                                    let men = enum_find(qeo, qel - qeo); if men >= 0 { rsoptenum = men; } } } }   /* AST-1: `Box<T>` payload is a pointer -> by-ref; a `&str`/char payload is a fat value (EOPTK-handled), not by-ref */
                        let mut rsen = 0 - 1; if rvoid == 0 && rs < 0 && rsstr == 0 && rsi64 == 0 && rsb == 0 { rsen = enum_find(ID_O, ID_L); }   /* LAYER5 P-METHOD-RETURNS-ENUM: `-> UserEnum`/`-> &UserEnum` (not a struct/str/i64/bool return) -> record the enum */
                        let mut rsvec = 0; let mut rsvelt = 0 - 1; let mut rsvelenum = 0 - 1;   /* LAYER7 P-METHOD-RETURNS-VEC */
                        if rvoid == 0 && is_vec_ty(ID_O, ID_L) && POS < LEN && at(POS) == 60 { rsvec = 1; rs = -1; rsen = -1;   /* `-> Vec<T>`/`-> ThinVec<T>` -- capture the element type (Box/alias see-through) for SVELT/SVELENUM threading */
                            let mut eo = POS + 1; while eo < LEN && (at(eo) == 32 || at(eo) == 38) { eo = eo + 1; } let mut ee = eo; while ee < LEN && is_idc(at(ee)) { ee = ee + 1; }
                            if is_box_ident(eo, ee - eo) && ee < LEN && at(ee) == 60 { eo = ee + 1; while eo < LEN && (at(eo) == 32 || at(eo) == 38) { eo = eo + 1; } ee = eo; while ee < LEN && is_idc(at(ee)) { ee = ee + 1; } }
                            let est = ty_find(eo, ee - eo); if est >= 0 { rsvelt = est; } let een = enum_find(eo, ee - eo); if een >= 0 { rsvelenum = een; } }
                        MRTOWN[NMRT as usize] = owner; MRTO[NMRT as usize] = mo; MRTL[NMRT as usize] = ml; MRTS[NMRT as usize] = rs; MRTB[NMRT as usize] = rsb; MRTV[NMRT as usize] = rvoid; if rsi64 { MRTI64[NMRT as usize] = 1; } else { MRTI64[NMRT as usize] = 0; } MRTSTR[NMRT as usize] = rsstr; MREN[NMRT as usize] = rsen; MRTREF[NMRT as usize] = mrefret; MRVEC[NMRT as usize] = rsvec; MRVELT[NMRT as usize] = rsvelt; MRVELENUM[NMRT as usize] = rsvelenum; MROPTK[NMRT as usize] = rsoptk; MROPTREF[NMRT as usize] = rsoptref; MROPTENUM[NMRT as usize] = rsoptenum; NMRT = NMRT + 1;
                        skipws(); while POS < LEN && at(POS) != 123 { POS = POS + 1; }
                        let mut d = 0;
                        loop { if POS >= LEN { break; } if at(POS) == 123 { d = d + 1; POS = POS + 1; } else if at(POS) == 125 { d = d - 1; POS = POS + 1; if d == 0 { break; } } else { POS = POS + 1; } }
                    } else { skip_tok(); }
                }
            } else { POS = save; skip_tok(); }
        }
    }
}

/* std-hoist P0.3: read the FIRST type-arg's kind starting at position p (inside a `<..>`) into ANNGAK/ANNGAT.
   ANNGAK: 0 scalar, 1 &str, 2 Vec, 3 String; ANNGAT: user-struct index (-1 = none). Does NOT move POS. */
fn scan_ta_kind(p: i32) {
    unsafe {
        ANNGAK = 0; ANNGAT = 0 - 1;
        let mut q = p;
        while q < LEN && (at(q) == 32 || at(q) == 9) { q = q + 1; }
        if q < LEN && at(q) == 38 { q = q + 1; while q < LEN && (at(q) == 32 || at(q) == 9) { q = q + 1; } }   /* skip `&` */
        if q < LEN && at(q) == 39 { q = q + 1; while q < LEN && is_idc(at(q)) { q = q + 1; }                  /* skip a lifetime `'a` */
            while q < LEN && (at(q) == 32 || at(q) == 9) { q = q + 1; }
            if q < LEN && at(q) == 38 { q = q + 1; while q < LEN && (at(q) == 32 || at(q) == 9) { q = q + 1; } } }
        let io = q; while q < LEN && is_idc(at(q)) { q = q + 1; } let il = q - io;
        if il <= 0 { return; }
        if is_string_ty(io, il) { ANNGAK = 3; return; }
        if is_vec_ty(io, il) { ANNGAK = 2; return; }
        if il == 3 && at(io) == 115 && at(io + 1) == 116 && at(io + 2) == 114 { ANNGAK = 1; return; }   /* str */
        let sty = ty_find(io, il); if sty >= 0 { ANNGAK = 0; ANNGAT = sty; }
    }
}
/* return-kind detection at POS (past the params): 0 scalar, 1 &str, 2 Vec, 3 String, 4 char, 5 i64. */
fn scan_ret_kind() -> i32 {
    unsafe {
        let mut rk = 0;
        RET_STY = 0 - 1; RET_REF = 0; RET_VELENUM = 0 - 1; RET_VELT = 0 - 1; RET_VELSTR = 0; RET_ENUM = 0 - 1; RET_STY2 = 0 - 1; RET_OPTK = 0; RET_OPTREF = 0; RET_OPTENUM = 0 - 1;
        if POS + 1 < LEN && at(POS) == 45 && at(POS + 1) == 62 {
            POS = POS + 2; skipws();
            if POS < LEN && at(POS) == 91 {                                        /* E2: -> [T; N] array return */
                let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 91 { d = d + 1; } if at(POS) == 93 { d = d - 1; } POS = POS + 1; }
                return 6;
            }
            let mut refret = 0;
            if POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); refret = 1; RET_REF = 1;
                if POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); skipws(); } }
            rd_ident();
            if id_is(105, 109, 112, 108, 0, 4) { rd_ident(); }   /* A: `-> impl Iterator<..>` -> the body's eager iterator is a Vec; treat the return kind as Vec so `f().sum()/.collect()` dispatch */
            if is_box_ident(ID_O, ID_L) && POS < LEN && at(POS) == 60 {   /* LAYER6 P-FN-RETURNS-BOX-ENUM: `-> Box<T>` is a transparent pointer -- see THROUGH it to the inner type T so `-> Box<Enum>`/`-> Box<Struct>` tags the caller's result like `-> Enum`/`-> Struct` (the inner enum_find/ty_find below then set RET_ENUM/RET_STY). Was: ID stayed `Box`, no check matched -> RET_ENUM=-1 -> `let ast = mk()` (a fn returning Box<Expr>) bound untagged, so `eval(&ast)` read the slot not the cell -> 0. GUARD: skip a `Box<dyn ..>`/`Box<dyn Fn>` closure (handled by the fat-ptr check below). */
                let mut ip = POS + 1; while ip < LEN && (at(ip) == 32 || at(ip) == 38) { ip = ip + 1; }
                if !(ip + 2 < LEN && at(ip) == 100 && at(ip + 1) == 121 && at(ip + 2) == 110) { POS = POS + 1; skipws(); while POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); } rd_ident(); }
            }
            let a0 = at(ID_O);
            if is_string_ty(ID_O, ID_L) { rk = 3; }
            if is_vec_ty(ID_O, ID_L) { rk = 2;
                skipws(); if POS < LEN && at(POS) == 60 { let mut eo = POS + 1; while eo < LEN && (at(eo) == 32 || at(eo) == 38) { eo = eo + 1; } let mut el = eo; while el < LEN && is_idc(at(el)) { el = el + 1; } let een = enum_find(eo, el - eo); if een >= 0 { RET_VELENUM = een; } let est = ty_find(eo, el - eo); if est >= 0 { RET_VELT = est; } if is_string_ty(eo, el - eo) { RET_VELSTR = 3; } else if el - eo == 3 && at(eo) == 115 && at(eo + 1) == 116 && at(eo + 2) == 114 { RET_VELSTR = 2; } }   /* R40: `-> Vec<Enum>`/`-> Vec<Struct>` return -- record the element type for the caller's binding {:?}. R14 P-CF-ACC: also a `-> Vec<String>`(3)/`-> Vec<&str>`(2) element display-kind. */
            }
            if id_is(73, 116, 101, 114, 97, 8) { rk = 2;
                skipws(); if POS < LEN && at(POS) == 60 { let mut eo = POS + 1; while eo < LEN && at(eo) != 61 && at(eo) != 62 { eo = eo + 1; } if eo < LEN && at(eo) == 61 { let mut so = eo + 1; while so < LEN && (at(so) == 32 || at(so) == 38) { so = so + 1; } let mut sl = so; while sl < LEN && is_idc(at(sl)) { sl = sl + 1; } let een = enum_find(so, sl - so); if een >= 0 { RET_VELENUM = een; } let est = ty_find(so, sl - so); if est >= 0 { RET_VELT = est; } } }   /* LAYER4 P-STD-ITERATOR: `-> impl Iterator<Item = Struct/Enum>` -- record the element type so the caller's `for t in gen()` binds `t` struct-typed (else `t.len` mis-reads a scalar; rustc_lexer `tokenize -> impl Iterator<Item=Token>`). Mirrors the `-> Vec<Struct>` extraction: find `=` inside `<..>`, read the element type ident. */
            }   /* Iterator -> Vec-kind */
            if ID_L >= 2 && at(ID_O) == 70 && at(ID_O + 1) == 110 { rk = 8; }   /* R1: `-> impl Fn/FnMut/FnOnce` -> closure-kind (caller binds SK=4) */
            if ID_L == 3 && a0 == 66 && at(ID_O + 1) == 111 && at(ID_O + 2) == 120 && POS < LEN && at(POS) == 60 {   /* R10: `-> Box<dyn Fn(..)->..>` -> a boxed closure is our fat-ptr cell; caller binds SK=4. Peek the `<..>` for `Fn(`. */
                let mut bp = POS + 1; let mut bd = 1; while bp < LEN && bd > 0 { if at(bp) == 60 { bd = bd + 1; } if at(bp) == 62 && at(bp - 1) != 45 { bd = bd - 1; } if at(bp) == 70 && bp + 2 < LEN && at(bp + 1) == 110 && at(bp + 2) == 40 { rk = 8; } bp = bp + 1; } }
            if refret == 1 { if ID_L == 3 { if a0 == 115 { if at(ID_O + 1) == 116 { if at(ID_O + 2) == 114 { rk = 1; } } } } }
            if ID_L == 4 { if a0 == 99 { if at(ID_O + 1) == 104 { if at(ID_O + 2) == 97 { if at(ID_O + 3) == 114 { rk = 4; } } } } }
            if ID_L == 3 { if a0 == 105 { if at(ID_O + 1) == 54 { if at(ID_O + 2) == 52 { rk = 5; } } } }
            if ID_L == 3 { if a0 == 117 { if at(ID_O + 1) == 54 { if at(ID_O + 2) == 52 { rk = 5; } } } }
            if ID_L == 4 && a0 == 98 && at(ID_O + 1) == 111 && at(ID_O + 2) == 111 && at(ID_O + 3) == 108 { rk = 7; }   /* A4: -> bool */
            if ID_L == 6 && a0 == 79 && at(ID_O + 1) == 112 && at(ID_O + 2) == 116 && at(ID_O + 3) == 105 && at(ID_O + 4) == 111 && at(ID_O + 5) == 110 { rk = 9; }    /* (a): -> Option */
            if ID_L == 6 && a0 == 82 && at(ID_O + 1) == 101 && at(ID_O + 2) == 115 && at(ID_O + 3) == 117 && at(ID_O + 4) == 108 && at(ID_O + 5) == 116 { rk = 10; }   /* (a): -> Result */
            if rk == 9 || rk == 10 { skipws(); if POS < LEN && at(POS) == 60 { let mut eo = POS + 1; while eo < LEN && (at(eo) == 32 || at(eo) == 38) { eo = eo + 1; } if eo < LEN && at(eo) == 39 { eo = eo + 1; while eo < LEN && is_idc(at(eo)) { eo = eo + 1; } while eo < LEN && at(eo) == 32 { eo = eo + 1; } } let mut el = eo; while el < LEN && is_idc(at(el)) { el = el + 1; } let est = ty_find(eo, el - eo); if est >= 0 { RET_STY = est; }
                { let mut peo = eo; let mut pel = el;   /* D005: see THROUGH a `Box<Inner>` success payload to the inner type, then record an ENUM payload (RET_OPTENUM) or a Box<Struct> payload (RET_STY) */
                    if is_box_ident(peo, pel - peo) && pel < LEN && at(pel) == 60 { peo = pel + 1; while peo < LEN && (at(peo) == 32 || at(peo) == 38) { peo = peo + 1; } pel = peo; while pel < LEN && is_idc(at(pel)) { pel = pel + 1; } }
                    let een2 = enum_find(peo, pel - peo); if een2 >= 0 { RET_OPTENUM = een2; }
                    let est2 = ty_find(peo, pel - peo); if est2 >= 0 && est < 0 { RET_STY = est2; } }
                let pl = el - eo;   /* SPAN-4: payload display-kind from the DECLARED Option/Result payload ident (str/String -> 1, char -> 3) */
                if pl == 3 && at(eo) == 115 && at(eo + 1) == 116 && at(eo + 2) == 114 { RET_OPTK = 1; }
                else if pl == 6 && at(eo) == 83 && at(eo + 1) == 116 && at(eo + 2) == 114 && at(eo + 3) == 105 && at(eo + 4) == 110 && at(eo + 5) == 103 { RET_OPTK = 1; }
                else if pl == 4 && at(eo) == 99 && at(eo + 1) == 104 && at(eo + 2) == 97 && at(eo + 3) == 114 { RET_OPTK = 3; }
                let mut rp = POS + 1; while rp < eo { if at(rp) == 38 { RET_OPTREF = 1; } rp = rp + 1; }   /* AST-1: a `&`/`&'a` before the Option/Result payload -> the payload is a reference */
                if pl == 3 && at(eo) == 66 && at(eo + 1) == 111 && at(eo + 2) == 120 { RET_OPTREF = 1; }   /* AST-1: a `Box<T>` payload is a pointer -> by-ref for deref purposes */
                if RET_OPTK != 0 { RET_OPTREF = 0; }   /* AST-1: a `&str`/`&String`/char payload is a fat VALUE descriptor (handled by EOPTK display-kind, `*x` would over-deref), NOT a by-ref pointer -- only `&i32`/`&Struct`/`&Enum`/Box are by-ref */
                if rk == 10 { let mut cp = el; let mut d = 1; while cp < LEN && d > 0 { let ch = at(cp); if ch == 60 { d = d + 1; } else if ch == 62 { d = d - 1; if d == 0 { break; } } else if ch == 44 && d == 1 { break; } cp = cp + 1; } if cp < LEN && at(cp) == 44 { let mut so = cp + 1; while so < LEN && (at(so) == 32 || at(so) == 38) { so = so + 1; } if so < LEN && at(so) == 39 { so = so + 1; while so < LEN && is_idc(at(so)) { so = so + 1; } while so < LEN && at(so) == 32 { so = so + 1; } } let mut se = so; while se < LEN && is_idc(at(se)) { se = se + 1; } let est2 = ty_find(so, se - so); if est2 >= 0 { RET_STY2 = est2; } } }   /* LAYER6 P-PRESULT-ERR-STRUCT-PAYLOAD: `-> Result<_, E>` -- scan to the top-level `,`, then read the ERR (2nd) type-arg's struct index into RET_STY2 (consumed via FRST2/EOPTERRTY for the Err(e) binder) */
            } }   /* LAYER5 P-PRESULT-STRUCT-PAYLOAD-BINDER: `-> Result<S,..>`/`-> Option<S>` -- record the first type-arg's user-struct index so the caller can flag an Ok(v)/Some(v) match binder struct-typed (mirrors the `-> Vec<Struct>` element extraction; consumed via FRST/fnret_st ONLY on the rk 9/10 EOPTTY path, never the rk==0 struct-return path) */
            RET_GAK = 0 - 99; RET_GAT = 0 - 1;
            if rk == 0 { let sty = ty_find(ID_O, ID_L); if sty >= 0 { RET_STY = sty;
                skipws(); if POS < LEN && at(POS) == 60 && TGPL[sty as usize] > 0 { scan_ta_kind(POS + 1); RET_GAK = ANNGAK; RET_GAT = ANNGAT; }   /* std-hoist P0.3: `-> B<&str>` generic-struct return -> record the type-arg kind */
            } else { let een = enum_find(ID_O, ID_L); if een >= 0 { RET_ENUM = een; } } }   /* R2cj: `-> UserStruct` (by value or `&Struct`) -> record so callers bind struct-typed; LAYER5 P-FN-RETURNS-ENUM: `-> UserEnum` -> record the enum so callers tag the result EISENUM */
        }
        rk
    }
}

/* R6: register each free fn's return kind (String=3 / Vec=2) so `let x = f(..)` propagates str/vec-ness */
fn fnret(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return FRK[i as usize]; } i = i + 1; } 0 }
}
fn fnret_ref(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return FRETREF[i as usize]; } i = i + 1; } 0 }
}
fn fnret_st(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return FRST[i as usize]; } i = i + 1; } 0 - 1 }
}
fn fnret_st2(o: i32, l: i32) -> i32 {   /* LAYER6 P-PRESULT-ERR-STRUCT-PAYLOAD: a fn's `-> Result<_, E>` ERR struct-type (-1 = none) */
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return FRST2[i as usize]; } i = i + 1; } 0 - 1 }
}
fn fnret_enum(o: i32, l: i32) -> i32 {   /* LAYER5 P-FN-RETURNS-ENUM: a fn's `-> UserEnum` return enum index (-1 = none) */
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return FREN[i as usize]; } i = i + 1; } 0 - 1 }
}
fn fnret_optk(o: i32, l: i32) -> i32 {   /* SPAN-4 P-OPTION-PAYLOAD-DISPLAY-KIND: a fn's `-> Option<P>`/`-> Result<P,_>` payload display-kind (0 scalar, 1 str, 3 char) */
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return FROPTK[i as usize]; } i = i + 1; } 0 }
}
fn fnret_optref(o: i32, l: i32) -> i32 {   /* AST-1 P-OPTION-PAYLOAD-BYREF: a fn's `-> Option<&T>`/`-> Option<Box<T>>` payload-is-a-reference flag */
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return FROPTREF[i as usize]; } i = i + 1; } 0 }
}
fn fnret_optenum(o: i32, l: i32) -> i32 {   /* D005 P-PRESULT-BOX-ENUM-PAYLOAD-BINDER: a fn's `-> Result/Option<Box<Enum>/Enum>` success-payload enum index (-1 = none) */
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return FROPTENUM[i as usize]; } i = i + 1; } 0 - 1 }
}
fn fnret_velenum(o: i32, l: i32) -> i32 {   /* R40: a fn's `-> Vec<Enum>` element enum-type (-1 = none) */
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return FRVELENUM[i as usize]; } i = i + 1; } 0 - 1 }
}
fn fnret_velt(o: i32, l: i32) -> i32 {   /* R40: a fn's `-> Vec<Struct>` element struct-type (-1 = none) */
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return FRVELT[i as usize]; } i = i + 1; } 0 - 1 }
}
fn fnret_velstr(o: i32, l: i32) -> i32 {   /* R14 P-CF-ACC: a fn's `-> Vec<String>`(3)/`-> Vec<&str>`(2) element display-kind (0 none) */
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return FRVELSTR[i as usize]; } i = i + 1; } 0 }
}
fn fnret_gak(o: i32, l: i32) -> i32 {   /* std-hoist P0.3: a fn's `-> B<..>` type-arg kind (-99 = none) */
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return FRGAK[i as usize]; } i = i + 1; } 0 - 99 }
}
fn fnret_gat(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return FRGAT[i as usize]; } i = i + 1; } 0 - 1 }
}
fn fn_dynpos(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return FDYNPOS[i as usize]; } i = i + 1; } 0 }
}
fn fn_is(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return 1; } i = i + 1; } 0 }
}
fn fn_argc(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return FNAC[i as usize]; } i = i + 1; } 0 }
}
fn fn_arrmask(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return FARRPOS[i as usize]; } i = i + 1; } 0 }
}
fn prepass_fns() {
    unsafe {
        POS = 0;
        loop {
            skipws();
            if POS >= LEN { break; }
            let save = POS;
            if rd_ident() && is_kw_fn() {
                rd_ident();
                let fo = ID_O; let fl = ID_L;
                skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 45 && POS + 1 < LEN && at(POS + 1) == 62 { POS = POS + 2; } else { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }   /* std-hoist: skip `->` as a unit so a `<F: Fn(i32)->i32>` bound's `>` doesn't prematurely close the generic header (was: left POS mid-signature -> scan_ret_kind missed `-> RVec<..>` -> the fn's struct return unregistered -> generic-closure-fn returning a struct mis-bound). Mirrors scan_ret_kind's `->` handling. */
                let mut mask = 0; let mut amask = 0; let mut nac = 0; let mut sawp = 0;
                skipws(); if POS < LEN && at(POS) == 40 { let mut d = 1; POS = POS + 1; let mut pw = 1;
                    while POS < LEN && d > 0 {
                        let c = at(POS);
                        if d == 1 && c != 41 && c != 32 && c != 9 && c != 10 && c != 13 && c != 44 { sawp = 1; }
                        if d == 1 && c == 44 { nac = nac + 1; }
                        if c == 40 || c == 91 || c == 60 {
                            if d == 1 && c == 91 {                                     /* A: entering a `[..]` param at top level -- detect `[T; ident]` const generic */
                                let mut bp = POS + 1; let mut bd = 1;
                                while bp < LEN && bd > 0 { if at(bp) == 91 { bd = bd + 1; } if at(bp) == 93 { bd = bd - 1; } if bd == 1 && at(bp) == 59 { let mut q = bp + 1; while q < LEN && (at(q) == 32 || at(q) == 9) { q = q + 1; } if q < LEN && is_idst(at(q)) { amask = amask + pw; } } bp = bp + 1; }
                            }
                            d = d + 1; POS = POS + 1;
                        }
                        else if c == 41 || c == 93 || c == 62 { d = d - 1; POS = POS + 1; }
                        else {
                            if d == 1 && c == 44 { pw = pw * 2; }
                            if d == 1 && c == 100 && POS + 2 < LEN && at(POS + 1) == 121 && at(POS + 2) == 110 && (POS + 3 >= LEN || !is_idc(at(POS + 3))) { let mut w = POS + 3; while w < LEN && (at(w) == 32 || at(w) == 9) { w = w + 1; } if !(w + 1 < LEN && at(w) == 70 && at(w + 1) == 110) { mask = mask + pw; } }   /* W1b: `dyn Fn(..)` closure param is NOT wrapped (passed as its own fat pointer) */
                            POS = POS + 1;
                        }
                    }
                }
                skipws();
                let rk = scan_ret_kind();
                if NFRN < 1024 { FNO[NFRN as usize] = fo; FNL[NFRN as usize] = fl; FRK[NFRN as usize] = rk; FRETREF[NFRN as usize] = RET_REF; FDYNPOS[NFRN as usize] = mask; FARRPOS[NFRN as usize] = amask; FRST[NFRN as usize] = RET_STY; FRST2[NFRN as usize] = RET_STY2; FRGAK[NFRN as usize] = RET_GAK; FRGAT[NFRN as usize] = RET_GAT; FRVELENUM[NFRN as usize] = RET_VELENUM; FRVELT[NFRN as usize] = RET_VELT; FRVELSTR[NFRN as usize] = RET_VELSTR; FREN[NFRN as usize] = RET_ENUM; FROPTK[NFRN as usize] = RET_OPTK; FROPTREF[NFRN as usize] = RET_OPTREF; FROPTENUM[NFRN as usize] = RET_OPTENUM; FNAC[NFRN as usize] = nac + sawp; NFRN = NFRN + 1; }
            } else { POS = save; skip_tok(); }
        }
    }
}

/* pre-pass: register every `static [mut] NAME: TYPE = INIT;` and emit its `gdef NAME <bytes>` (zeroed) */
/* E1: constant-expression evaluator over integers -- literals (dec/hex), char/byte-char, other consts,
   unary -/!, * / %, + -, with `as TYPE` casts ignored. CVALID goes 0 on anything non-constant (a call,
   an unknown name), so the prepass only registers consts it could fully fold. */
fn cfind(o: i32, l: i32) -> i32 { unsafe { let mut i = 0; while i < NCONST { if span_eq_at(CNO[i as usize], CNL[i as usize], o, l) { return i; } i = i + 1; } -1 } }
fn tconst_find(ty: i32, o: i32, l: i32) -> i32 { unsafe { let mut i = 0; while i < NTCONST { if TCTY[i as usize] == ty && span_eq_at(TCNO[i as usize], TCNL[i as usize], o, l) { return i; } i = i + 1; } -1 } }   /* TRAIT R3: (type, const-name) -> registry index */
fn chexval(c: i32) -> i32 { if c >= 48 && c <= 57 { c - 48 } else if c >= 97 && c <= 102 { c - 87 } else if c >= 65 && c <= 70 { c - 55 } else { -1 } }
fn ceval_atom() -> i32 {
    unsafe {
        skipws();
        if POS < LEN && at(POS) == 40 { POS = POS + 1; let v = ceval_expr(); skipws(); if POS < LEN && at(POS) == 41 { POS = POS + 1; } return v; }
        if POS < LEN && at(POS) == 45 { POS = POS + 1; return 0 - ceval_atom(); }
        if POS < LEN && at(POS) == 33 { POS = POS + 1; let v = ceval_atom(); if v == 0 { return 1; } return 0; }
        if POS < LEN && (at(POS) == 39 || (at(POS) == 98 && POS + 1 < LEN && at(POS + 1) == 39)) {   /* 'c' or b'c' */
            if at(POS) == 98 { POS = POS + 1; }
            POS = POS + 1; let mut v = at(POS);
            if v == 92 { POS = POS + 1; let e = at(POS); if e == 110 { v = 10; } else if e == 116 { v = 9; } else if e == 114 { v = 13; } else if e == 48 { v = 0; } else if e == 117 && POS + 1 < LEN && at(POS + 1) == 123 { POS = POS + 2; v = 0; while POS < LEN && at(POS) != 125 { let h = at(POS); let hd = if h >= 48 && h <= 57 { h - 48 } else if h >= 97 && h <= 102 { h - 97 + 10 } else if h >= 65 && h <= 70 { h - 65 + 10 } else { 0 }; v = v * 16 + hd; POS = POS + 1; } } else { v = e; } }   /* LAYER6 P-CHAR-UNICODE-ESCAPE: `'\u{XXXX}'` -> decode the hex codepoint (POS lands on `}`; the outer POS+1 steps past it, mirroring the simple-escape flow). Was: `\u` fell to `else{v=e}` -> v='u' + POS stuck at `{XXXX}'` -> desync. In a PATTERN (read_pat_scalar) this made `matches!(c,'\u{0009}'|..)` (rustc_lexer is_whitespace) match EVERYTHING -> is_whitespace(any)=true -> the single-token collapse. */
            POS = POS + 1; if POS < LEN && at(POS) == 39 { POS = POS + 1; }
            return v;
        }
        if POS < LEN && is_dig(at(POS)) {
            if at(POS) == 48 && POS + 1 < LEN && at(POS + 1) == 120 {                 /* 0x hex */
                POS = POS + 2; let mut v = 0;
                while POS < LEN && chexval(at(POS)) >= 0 { v = v * 16 + chexval(at(POS)); POS = POS + 1; }
                while POS < LEN && is_idc(at(POS)) { POS = POS + 1; }
                return v;
            }
            let mut v = 0; while POS < LEN && is_dig(at(POS)) { v = v * 10 + (at(POS) - 48); POS = POS + 1; }
            while POS < LEN && is_idc(at(POS)) { POS = POS + 1; }                     /* 5usize suffix */
            return v;
        }
        if POS < LEN && is_idst(at(POS)) {
            rd_ident(); let io = ID_O; let il = ID_L;
            let pb = pbind_find(io, il); if pb >= 0 { return PBV[pb as usize]; }   /* CONST-FN: param binding shadows consts */
            skipws();
            let gci = g_find(io, il);   /* CONSTEVAL R1: const array index `A[idx]` in a const expr -- fold + bounds-check (E0080 on OOB) */
            if gci >= 0 && GK[gci as usize] == 1 && GFOLD[gci as usize] == 1 && POS < LEN && at(POS) == 91 { POS = POS + 1; let idx = ceval_expr(); skipws(); eat_ch(93); let n = GCNT[gci as usize]; if idx < 0 || idx >= n { die_const_oob(); } return GIV[(GSTART[gci as usize] + idx) as usize]; }
            if POS < LEN && at(POS) == 40 { let cf = cfn_find(io, il); if cf >= 0 { return ceval_call(cf); } CVALID = 0; return 0; }   /* CONST-FN: call in const initializer */
            let ci = cfind(io, il);
            if ci >= 0 { return CVAL[ci as usize]; }
            if sym_ckknown(io, il) == 1 { return sym_ckval(io, il); }   /* CONSTEVAL: a const-known immutable local -> its value (folds `a+b`, `a[i]` when a/i are const-let-propagated) */
            CVALID = 0; return 0;
        }
        CVALID = 0; 0
    }
}
fn ceval_term() -> i32 {
    unsafe {
        let mut v = ceval_atom();
        loop {
            skipws();
            if POS + 2 < LEN && at(POS) == 97 && at(POS + 1) == 115 && !is_idc(at(POS + 2)) { POS = POS + 2; skipws(); rd_ident(); }   /* `as TYPE` cast -> ignore */
            else if POS < LEN && at(POS) == 42 { POS = POS + 1; v = v * ceval_atom(); }
            else if POS < LEN && at(POS) == 47 { POS = POS + 1; let r = ceval_atom(); if r != 0 { v = v / r; } }
            else if POS < LEN && at(POS) == 37 { POS = POS + 1; let r = ceval_atom(); if r != 0 { v = v % r; } }
            else { break; }
        }
        v
    }
}
fn ceval_expr() -> i32 {
    unsafe {
        let mut v = ceval_term();
        loop {
            skipws();
            if POS < LEN && at(POS) == 43 { POS = POS + 1; v = v + ceval_term(); }
            else if POS < LEN && at(POS) == 45 { POS = POS + 1; v = v - ceval_term(); }
            else { break; }
        }
        v
    }
}
/* CONSTEVAL R2: an i64 mirror of ceval that flags i32 overflow (COVFL) on each +/-/* whose result
   leaves the i32 range. Only atoms ceval also folds are handled (int literals, const-known immutable
   locals, folded consts); anything else sets CVALID=0 so a non-const expr is never flagged. */
fn ceval64_atom() -> i64 {
    unsafe {
        skipws();
        if POS < LEN && at(POS) == 40 { POS = POS + 1; let v = ceval64_expr(); skipws(); if POS < LEN && at(POS) == 41 { POS = POS + 1; } return v; }
        if POS < LEN && at(POS) == 45 { POS = POS + 1; return 0 - ceval64_atom(); }
        if POS < LEN && is_dig(at(POS)) {
            let mut v: i64 = 0; while POS < LEN && (is_dig(at(POS)) || at(POS) == 95) { if at(POS) != 95 { v = v * 10 + ((at(POS) - 48) as i64); } POS = POS + 1; }
            let sfo = POS; while POS < LEN && is_idc(at(POS)) { POS = POS + 1; }   /* type suffix (5i32/5i64/5u64) */
            let sfl = POS - sfo;
            if sfl == 3 && (at(sfo) == 105 || at(sfo) == 117) && at(sfo + 1) == 54 && at(sfo + 2) == 52 { C64SEEN = 1; }   /* i64/u64 suffix -> 64-bit const fold; an i32-range excess is not overflow */
            if sfl == 5 && (at(sfo) == 105 || at(sfo) == 117) && at(sfo + 1) == 115 && at(sfo + 2) == 105 { C64SEEN = 1; }   /* isize/usize suffix */
            if v > 2147483647 { C64SEEN = 1; }   /* a literal exceeding i32::MAX is at least i64-wide */
            return v;
        }
        if POS < LEN && is_idst(at(POS)) {
            rd_ident(); let io = ID_O; let il = ID_L;
            if sym_ckknown(io, il) == 1 { if sym_i64(io, il) == 1 || sym_u(io, il) == 1 { C64SEEN = 1; } return sym_ckval(io, il) as i64; }
            let ci = cfind(io, il); if ci >= 0 { return CVAL[ci as usize] as i64; }
            CVALID = 0; return 0;
        }
        CVALID = 0; 0
    }
}
fn ceval64_term() -> i64 {
    unsafe {
        let imax: i64 = 2147483647; let imin: i64 = 0 - imax - 1;
        let mut v = ceval64_atom();
        loop {
            skipws();
            if POS + 2 < LEN && at(POS) == 97 && at(POS + 1) == 115 && !is_idc(at(POS + 2)) { POS = POS + 2; skipws(); rd_ident(); C64SEEN = 1; }   /* `as TYPE` cast -> the arithmetic type changed; do not treat an i32-range excess as overflow */
            else if POS < LEN && at(POS) == 42 { POS = POS + 1; v = v * ceval64_atom(); if v > imax || v < imin { COVFL = 1; } }
            else if POS < LEN && at(POS) == 47 { POS = POS + 1; let r = ceval64_atom(); if r != 0 { v = v / r; } }
            else if POS < LEN && at(POS) == 37 { POS = POS + 1; let r = ceval64_atom(); if r != 0 { v = v % r; } }
            else { break; }
        }
        v
    }
}
fn ceval64_expr() -> i64 {
    unsafe {
        let imax: i64 = 2147483647; let imin: i64 = 0 - imax - 1;
        let mut v = ceval64_term();
        loop {
            skipws();
            if POS < LEN && at(POS) == 43 { POS = POS + 1; v = v + ceval64_term(); if v > imax || v < imin { COVFL = 1; } }
            else if POS < LEN && at(POS) == 45 { POS = POS + 1; v = v - ceval64_term(); if v > imax || v < imin { COVFL = 1; } }
            else { break; }
        }
        v
    }
}
/* CONSTEVAL R2: peek -- does the expression at POS fold to a const whose i32 arithmetic provably
   overflows? Restores POS. True only when fully const-foldable (CVALID) AND an op overflowed (COVFL). */
fn const_ovf_at() -> i32 {
    unsafe { let osave = POS; COVFL = 0; CVALID = 1; C64SEEN = 0; let _v = ceval64_expr(); let r = if COVFL == 1 && CVALID == 1 && C64SEEN == 0 { 1 } else { 0 }; POS = osave; COVFL = 0; C64SEEN = 0; r }
}
/* E1: register `const NAME: T = EXPR;` whose EXPR folds to an integer constant. Non-integer consts
   (struct/array/fn-call initializers) leave CVALID=0 and are skipped -- they fall through as before. */
fn cfn_find(o: i32, l: i32) -> i32 { unsafe { let mut i = 0; while i < NCFN { if span_eq_at(CFNO[i as usize], CFNL[i as usize], o, l) { return i; } i = i + 1; } -1 } }
fn pbind_find(o: i32, l: i32) -> i32 { unsafe { let mut i = PBDEPTH - 1; while i >= 0 { if span_eq_at(PBNO[i as usize], PBNL[i as usize], o, l) { return i; } i = i - 1; } -1 } }
/* CONST-FN: evaluate a call to registered const fn `cf`. POS is at `(`. Args eval'd via ceval_expr, params
   bound on the PB stack, the body re-parsed at CFBODY, then POS + depth restored. Nested/recursive calls
   stack their own bindings (pbind_find searches most-recent-first, so a callee sees only its own params). */
fn ceval_call(cf: i32) -> i32 {
    unsafe {
        eat_ch(40);
        let mut av0 = 0; let mut av1 = 0; let mut av2 = 0; let mut av3 = 0; let mut na = 0;
        loop {
            skipws(); if POS < LEN && at(POS) == 41 { break; }
            let a = ceval_expr();
            if na == 0 { av0 = a; } else if na == 1 { av1 = a; } else if na == 2 { av2 = a; } else { av3 = a; }
            na = na + 1;
            skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; }
        }
        eat_ch(41);
        let savepos = POS; let base = PBDEPTH; let np = CFNP[cf as usize];
        let mut i = 0;
        while i < np {
            PBNO[PBDEPTH as usize] = CFPO[(cf * 4 + i) as usize]; PBNL[PBDEPTH as usize] = CFPL[(cf * 4 + i) as usize];
            let mut val = 0; if i == 0 { val = av0; } else if i == 1 { val = av1; } else if i == 2 { val = av2; } else { val = av3; }
            PBV[PBDEPTH as usize] = val; PBDEPTH = PBDEPTH + 1; i = i + 1;
        }
        POS = CFBODY[cf as usize];
        let r = ceval_expr();
        PBDEPTH = base; POS = savepos;
        r
    }
}
/* CONST-FN: register every `const fn NAME(p: T, ..) -> T { EXPR }` (name, param name spans, body-expr POS). */
fn prepass_constfns() {
    unsafe {
        POS = 0;
        loop {
            skipws(); if POS >= LEN { break; }
            let save = POS;
            if rd_ident() && is_kw_const() {
                skipws();
                if rd_ident() && is_kw_fn() {
                    skipws(); rd_ident(); let no = ID_O; let nl = ID_L;
                    skipws(); eat_ch(40);
                    let cf = NCFN; CFNO[cf as usize] = no; CFNL[cf as usize] = nl;
                    let mut np = 0;
                    loop {
                        skipws(); if POS >= LEN || at(POS) == 41 { break; }
                        let pstart = POS;
                        while POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); skipws(); } }   /* RUSTC-FE FC: skip a leading `&`/`&mut`/lifetime on a param (was an infinite HANG: a `const fn` METHOD's `&self` receiver -- rd_ident did not consume `&`, the param loop spun; top-level const fns have no `&self` but impl-method const fns do, and this top-level prepass scans them) */
                        rd_ident();
                        if np < 4 { CFPO[(cf * 4 + np) as usize] = ID_O; CFPL[(cf * 4 + np) as usize] = ID_L; np = np + 1; }
                        skipws(); if POS < LEN && at(POS) == 58 { while POS < LEN && at(POS) != 44 && at(POS) != 41 { POS = POS + 1; } }
                        skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; }
                        if POS == pstart { POS = POS + 1; }   /* RUSTC-FE FC fail-closed: a param iteration that made no progress force-advances -> the loop can never spin */
                    }
                    eat_ch(41);
                    CFNP[cf as usize] = np;
                    while POS < LEN && at(POS) != 123 { POS = POS + 1; }
                    eat_ch(123); skipws();
                    let bsave = POS;
                    if rd_ident() && id_is(114, 101, 116, 117, 114, 6) { skipws(); CFBODY[cf as usize] = POS; } else { CFBODY[cf as usize] = bsave; }   /* skip optional `return` */
                    NCFN = NCFN + 1;
                    POS = bsave; let mut d = 1; while POS < LEN && d > 0 { if at(POS) == 123 { d = d + 1; } if at(POS) == 125 { d = d - 1; } POS = POS + 1; }
                } else { POS = save; skip_tok(); }
            } else { POS = save; skip_tok(); }
        }
    }
}
fn prepass_consts() {
    unsafe {
        POS = 0;
        loop {
            skipws();
            if POS >= LEN { break; }
            let save = POS;
            if rd_ident() && is_kw_const() {
                skipws();
                let s2 = POS;
                if rd_ident() && is_kw_fn() { POS = save; skip_tok(); } else {
                    POS = s2; rd_ident(); let no = ID_O; let nl = ID_L;
                    skipws();
                    let mut carr = 0; let mut cesz = 4; let mut cstr = 0;           /* R2aj: `const NAME: [T; N] = [..]` array const; LAYER3 M3: cstr = `const NAME: &str = ".."` */
                    if POS < LEN && at(POS) == 58 {
                        POS = POS + 1; skipws();
                        if POS < LEN && at(POS) == 91 {
                            carr = 1; POS = POS + 1; skipws(); rd_ident();
                            if ID_L == 2 && at(ID_O) == 117 && at(ID_O + 1) == 56 { cesz = 1; }
                            while POS < LEN && at(POS) != 93 { POS = POS + 1; } eat_ch(93); skipws();   /* CONST-ARR: skip ws before `=` (was: `const A: [T; N] = [..]` with a space around `=` missed the carr branch -> array read 0) */
                        } else {
                            let tp = POS; if POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); }   /* LAYER3 M3: `&str` -- skip `&`, check for `str` (not `String`/`str`+id) */
                            if POS + 2 < LEN && at(POS) == 115 && at(POS + 1) == 116 && at(POS + 2) == 114 && (POS + 3 >= LEN || !is_idc(at(POS + 3))) { cstr = 1; }
                            POS = tp; while POS < LEN && at(POS) != 61 && at(POS) != 59 { POS = POS + 1; }
                        }
                    }
                    if cstr == 1 && POS < LEN && at(POS) == 61 {                    /* LAYER3 M3: `const S: &str = "LIT"` -> GK=3 global; GSTART/GCNT hold the literal content span; emit_str_literal at use */
                        POS = POS + 1; skipws();
                        if POS < LEN && at(POS) == 34 {
                            POS = POS + 1; let so = POS; while POS < LEN && at(POS) != 34 { POS = POS + 1; } let sl = POS - so; if POS < LEN { POS = POS + 1; }
                            GO[NGLOB as usize] = no; GL[NGLOB as usize] = nl; GK[NGLOB as usize] = 3; GSTART[NGLOB as usize] = so; GCNT[NGLOB as usize] = sl; NGLOB = NGLOB + 1;
                        }
                    } else if carr == 1 && POS < LEN && at(POS) == 61 {                    /* register as a gdef global + replay inits, exactly like a static array (R2af) */
                        POS = POS + 1; skipws();
                        let istart = NGIV; let mut icnt = 0; let mut cnt = 0;
                        if POS < LEN && at(POS) == 91 {
                            POS = POS + 1;
                            loop { skipws(); if POS >= LEN || at(POS) == 93 || NGIV >= 8000 { break; }
                                CVALID = 1; let cv = ceval_expr(); GIV[NGIV as usize] = cv; NGIV = NGIV + 1; icnt = icnt + 1; cnt = cnt + 1;
                                skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; } }
                            eat_ch(93);
                        }
                        GO[NGLOB as usize] = no; GL[NGLOB as usize] = nl; GK[NGLOB as usize] = 1; GE[NGLOB as usize] = cesz; GFOLD[NGLOB as usize] = 1;   /* a `const` array: elements are value-foldable (E0080) since ALL inits are replayed (icnt == logical length) */
                        GSTART[NGLOB as usize] = istart; GCNT[NGLOB as usize] = icnt;
                        print!("gdef "); emit_span(no, nl); let mut bb = cnt * cesz; if bb < 4 { bb = 4; } println!(" {}", bb);
                        NGLOB = NGLOB + 1;
                    } else if POS < LEN && at(POS) == 61 {
                        POS = POS + 1; CVALID = 1;
                        let v = ceval_expr(); skipws();
                        if CVALID == 1 && POS < LEN && at(POS) == 59 { CNO[NCONST as usize] = no; CNL[NCONST as usize] = nl; CVAL[NCONST as usize] = v; NCONST = NCONST + 1; }
                    }
                    while POS < LEN && at(POS) != 59 { POS = POS + 1; }
                    eat_ch(59);
                }
            } else { POS = save; skip_tok(); }
        }
    }
}
/* A1: register each trait's DEFAULT methods (those with a { body }, not a `;` declaration). */
fn prepass_traits() {
    unsafe {
        POS = 0;
        loop {
            skipws();
            if POS >= LEN { break; }
            let save = POS;
            if rd_ident() && is_kw_trait() {
                rd_ident(); let tno = ID_O; let tnl = ID_L;                         /* trait name */
                while POS < LEN && at(POS) != 123 { POS = POS + 1; }                 /* skip generics/supertraits to the { */
                eat_ch(123);
                loop {
                    skipws();
                    if POS >= LEN { break; }
                    if at(POS) == 125 { POS = POS + 1; break; }
                    if rd_ident() && is_kw_fn() {
                        skipws(); let namepos = POS; rd_ident(); let mno = ID_O; let mnl = ID_L;
                        skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
                        if POS < LEN && at(POS) == 40 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } POS = POS + 1; } }
                        let mut dstr = 0; skipws();
                        if POS + 1 < LEN && at(POS) == 45 && at(POS + 1) == 62 { POS = POS + 2; skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); skipws(); } } rd_ident(); if is_string_ty(ID_O, ID_L) { dstr = 3; } else if ID_L == 3 && at(ID_O) == 115 && at(ID_O + 1) == 116 && at(ID_O + 2) == 114 { dstr = 1; } }   /* S1 L1: default method `-> String`/`-> &str` return kind */
                        while POS < LEN && at(POS) != 123 && at(POS) != 59 { POS = POS + 1; }
                        if POS < LEN && at(POS) == 123 {                             /* has a body -> default method */
                            TRO[NTDM as usize] = tno; TRL[NTDM as usize] = tnl; TDMO[NTDM as usize] = mno; TDML[NTDM as usize] = mnl; TDPOS[NTDM as usize] = namepos; TDMSTR[NTDM as usize] = dstr; NTDM = NTDM + 1;
                            let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 123 { d = d + 1; } if at(POS) == 125 { d = d - 1; } POS = POS + 1; }
                        } else { eat_ch(59); }
                    } else { skip_tok(); }
                }
            } else { POS = save; skip_tok(); }
        }
    }
}
fn impl_has_method(o: i32, l: i32) -> i32 { unsafe { let mut i = 0; while i < NIMPLM { if span_eq_at(IMPLMO[i as usize], IMPLML[i as usize], o, l) { return 1; } i = i + 1; } 0 } }
fn prepass_statics() {
    unsafe {
        POS = 0;
        loop {
            skipws();
            if POS >= LEN { break; }
            let save = POS;
            if rd_ident() && is_kw_static() {
                rd_ident(); if is_kw_mut() { rd_ident(); }
                let no = ID_O; let nl = ID_L;
                skipws(); eat_ch(58); skipws();
                let mut kind = 0; let mut esz = 4; let mut bytes = 4;
                if POS < LEN && at(POS) == 91 {
                    POS = POS + 1; rd_ident();
                    if ID_L == 2 && at(ID_O) == 117 && at(ID_O + 1) == 56 { esz = 1; } else { if id_w8() == 1 { esz = 8; } else { esz = 4; } }
                    eat_ch(59); skipws();
                    CVALID = 1; let n = ceval_expr();                               /* CONST-ORDER: static array size via ceval (was digit-only -> `[T; SIZE]` with a const SIZE gave n=0) */
                    skipws(); eat_ch(93);
                    kind = 1; bytes = n * esz;
                } else { rd_ident(); if id_w8() == 1 { esz = 8; bytes = 8; } else { esz = 4; bytes = 4; } kind = 0; }
                /* R2af: capture the initializer constants (list / repeat / scalar) for replay at main entry */
                let istart = NGIV; let mut icnt = 0;
                skipws();
                if POS < LEN && at(POS) == 61 {
                    POS = POS + 1; skipws();
                    if POS < LEN && at(POS) == 91 {
                        POS = POS + 1; skipws(); let elem0 = POS;
                        CVALID = 1; let c0 = ceval_expr(); skipws();
                        if POS < LEN && at(POS) == 59 {                           /* [C; N] repeat */
                            POS = POS + 1; skipws();
                            let mut nrep = 0; while POS < LEN && is_dig(at(POS)) { nrep = nrep * 10 + (at(POS) - 48); POS = POS + 1; }
                            if c0 != 0 { let mut r = 0; while r < nrep && NGIV < 8000 { GIV[NGIV as usize] = c0; NGIV = NGIV + 1; icnt = icnt + 1; r = r + 1; } }   /* expand non-zero fill only ([0;N] needs no init; avoids GIV overflow on rzrs.rs's own SRC) */
                        } else {                                                 /* [c0, c1, ..] */
                            POS = elem0; CVALID = 1;
                            loop { skipws(); if POS >= LEN || at(POS) == 93 || NGIV >= 8000 { break; }
                                let cv = ceval_expr(); GIV[NGIV as usize] = cv; NGIV = NGIV + 1; icnt = icnt + 1;
                                skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; } }
                        }
                    } else { CVALID = 1; let cv = ceval_expr(); GIV[NGIV as usize] = cv; NGIV = NGIV + 1; icnt = icnt + 1; }
                }
                while POS < LEN && at(POS) != 59 { POS = POS + 1; }
                eat_ch(59);
                GO[NGLOB as usize] = no; GL[NGLOB as usize] = nl; GK[NGLOB as usize] = kind; GE[NGLOB as usize] = esz;
                GSTART[NGLOB as usize] = istart; GCNT[NGLOB as usize] = icnt;
                print!("gdef "); emit_span(no, nl);
                let mut bb = bytes; if bb < 4 { bb = 4; }
                println!(" {}", bb);
                NGLOB = NGLOB + 1;
            } else { POS = save; skip_tok(); }
        }
    }
}
/* R2af: replay static initializers at main entry -- store non-zero consts into the zero-filled gdef region
   (nothing for all-zero initializers, so rzrs.rs's own zero statics emit no code -- self-host safe). */
fn emit_static_inits() {
    unsafe {
        let mut gi = 0;
        while gi < NGLOB {
            let cnt = GCNT[gi as usize];
            if cnt > 0 {
                let es = GE[gi as usize]; let start = GSTART[gi as usize];
                let mut any = 0; let mut j = 0;
                while j < cnt { if GIV[(start + j) as usize] != 0 { any = 1; } j = j + 1; }
                if any == 1 {
                    let base = egref(GO[gi as usize], GL[gi as usize]);
                    let mut i = 0;
                    while i < cnt {
                        let v = GIV[(start + i) as usize];
                        if v != 0 {
                            let ad = ebin("add", base, ec(i * es)); let cv = ec(v);
                            if es == 1 { println!("  storeb v{} v{}", ad, cv); } else { println!("  store v{} v{}", ad, cv); }
                        }
                        i = i + 1;
                    }
                }
            }
            gi = gi + 1;
        }
    }
}

/* K5: discover `#[test] fn NAME` functions (record name spans) and whether a user `fn main` exists */
fn prepass_tests() {
    unsafe {
        NTEST = 0; MAIN_SEEN = 0;
        let mut p = 0;
        while p + 6 < LEN {
            if at(p) == 35 && at(p + 1) == 91 && at(p + 2) == 116 && at(p + 3) == 101 && at(p + 4) == 115 && at(p + 5) == 116 && at(p + 6) == 93 {
                p = p + 7;
                while p + 2 < LEN && !(at(p) == 102 && at(p + 1) == 110 && !is_idc(at(p + 2))) { p = p + 1; }
                p = p + 2;
                while p < LEN && (at(p) == 32 || at(p) == 9 || at(p) == 10 || at(p) == 13) { p = p + 1; }
                let no = p;
                while p < LEN && is_idc(at(p)) { p = p + 1; }
                if NTEST < 128 { TESTO[NTEST as usize] = no; TESTL[NTEST as usize] = p - no; NTEST = NTEST + 1; }
            } else { p = p + 1; }
        }
        let mut q = 0;
        while q + 7 < LEN {
            if at(q) == 102 && at(q + 1) == 110 && at(q + 2) == 32 && at(q + 3) == 109 && at(q + 4) == 97 && at(q + 5) == 105 && at(q + 6) == 110 && !is_idc(at(q + 7)) { MAIN_SEEN = 1; }
            q = q + 1;
        }
    }
}
/* emit into the generated program: __putc(c) for a fixed char code c (no byte-slice reads -> tier-0-safe) */
fn emit_putc(c: i32) { unsafe { let cv = ec(c); println!("  v{}: i32 = call __putc v{}", NC, cv); NC = NC + 1; } }

/* K5b/PhaseT: if there are #[test] fns and no user main, synthesize a runner main that runs each test in an
   ISOLATED child (fork, run the test, exit; a failing assert exits 101). The parent prints a cargo-style
   `test NAME ... ok|FAILED` line per test, wait4s each child, tallies failures in a heap slot, prints a
   `<N> failed` summary, and exits with the failure count (0 = all passed). Test names are read from SRC at
   emit time and baked in as putc calls. */
fn emit_test_main() {
    unsafe {
        if NTEST == 0 { return; }
        if MAIN_SEEN == 1 { return; }
        println!("ir main -> i32");
        NC = 0;
        let stp = halloc(8);                                    /* wait4 status buffer */
        let fcp = halloc(8);                                    /* fail counter */
        let z0 = ecr(0);
        println!("  store v{} v{}", fcp, z0);                   /* *fcp = 0 */
        let mut i = 0;
        while i < NTEST {
            emit_putc(116); emit_putc(101); emit_putc(115); emit_putc(116); emit_putc(32);   /* "test " */
            let mut k = 0;
            while k < TESTL[i as usize] { emit_putc(at(TESTO[i as usize] + k)); k = k + 1; }  /* the test name */
            emit_putc(32); emit_putc(46); emit_putc(46); emit_putc(46); emit_putc(32);        /* " ... " */
            let c57 = ecr(57);
            println!("  v{}: i32 = syscall v{}", NC, c57); let pid = NC; NC = NC + 1;   /* pid = fork() */
            let zc = ecr(0);
            let cond = ebin("eq", pid, zc);                     /* child iff pid == 0 */
            println!("  brz v{}", cond); ckreset();
            /* child: silence the test's stderr (open /dev/null, dup2 -> fd 2) so a failing assert's message does
               not interleave with the parent's `test NAME ... ok|FAILED` line -- like cargo capturing test output */
            let dcur = eload(egref_str_cursor());
            let dbase = ebin("add", egref_str_arena(), dcur);
            let d0 = ecr(47); println!("  storeb v{} v{}", ebin("add", dbase, ec(0)), d0);   /* '/' */
            let d1 = ecr(100); println!("  storeb v{} v{}", ebin("add", dbase, ec(1)), d1);  /* 'd' */
            let d2 = ecr(101); println!("  storeb v{} v{}", ebin("add", dbase, ec(2)), d2);  /* 'e' */
            let d3 = ecr(118); println!("  storeb v{} v{}", ebin("add", dbase, ec(3)), d3);  /* 'v' */
            let d4 = ecr(47); println!("  storeb v{} v{}", ebin("add", dbase, ec(4)), d4);   /* '/' */
            let d5 = ecr(110); println!("  storeb v{} v{}", ebin("add", dbase, ec(5)), d5);  /* 'n' */
            let d6 = ecr(117); println!("  storeb v{} v{}", ebin("add", dbase, ec(6)), d6);  /* 'u' */
            let d7 = ecr(108); println!("  storeb v{} v{}", ebin("add", dbase, ec(7)), d7);  /* 'l' */
            let d8 = ecr(108); println!("  storeb v{} v{}", ebin("add", dbase, ec(8)), d8);  /* 'l' */
            let d9 = ecr(0); println!("  storeb v{} v{}", ebin("add", dbase, ec(9)), d9);    /* NUL */
            let oc2 = ecr(2); let ow = ecr(1); let oz = ecr(0);
            println!("  v{}: i32 = syscall v{} v{} v{} v{}", NC, oc2, dbase, ow, oz); let dfd = NC; NC = NC + 1;   /* open(/dev/null, O_WRONLY) */
            let dc33 = ecr(33); let two = ecr(2);
            println!("  v{}: i32 = syscall v{} v{} v{}", NC, dc33, dfd, two); NC = NC + 1;   /* dup2(fd, 2) */
            print!("  v{}: i32 = call ", NC); emit_span(TESTO[i as usize], TESTL[i as usize]); println!(""); NC = NC + 1;
            emit_flush_call(); let c60 = ecr(60); let ez = ecr(0);
            println!("  v{}: i32 = syscall v{} v{}", NC, c60, ez); NC = NC + 1;   /* child: exit(0) (assert failure already exited 101) */
            println!("  pool"); ckreset();
            let c61 = ecr(61); let wz1 = ecr(0); let wz2 = ecr(0);
            println!("  v{}: i32 = syscall v{} v{} v{} v{} v{}", NC, c61, pid, stp, wz1, wz2); NC = NC + 1;   /* wait4(pid,&status,0,0) */
            let st = eload(stp);
            let c256a = ecr(256);
            let dv = ebin("div", st, c256a);
            let c256b = ecr(256);
            let code = ebin("mod", dv, c256b);                  /* WEXITSTATUS = (status/256)%256 */
            let znc = ecr(0);
            let cc = ebin("ne", code, znc);                     /* failed iff code != 0 */
            let lb = LBL; LBL = LBL + 1;
            println!("  brz v{}", cc); ckreset();
            emit_putc(70); emit_putc(65); emit_putc(73); emit_putc(76); emit_putc(69); emit_putc(68);   /* "FAILED" (cc != 0) */
            let fcv = eload(fcp);
            let one = ecr(1);
            let nfc = ebin("add", fcv, one);
            println!("  store v{} v{}", fcp, nfc);              /* *fcp += 1 */
            println!("  goto __Lt{}", lb); ckreset();
            println!("  pool"); ckreset();
            emit_putc(111); emit_putc(107);                     /* "ok" (cc == 0) */
            println!("  label __Lt{}", lb); ckreset();
            emit_putc(10);                                      /* newline */
            i = i + 1;
        }
        let fc = eload(fcp);
        println!("  v{}: i32 = call __print_uint v{}", NC, fc); NC = NC + 1;   /* print the failure count */
        emit_putc(32); emit_putc(102); emit_putc(97); emit_putc(105); emit_putc(108); emit_putc(101); emit_putc(100); emit_putc(10);   /* " failed\n" */
        let fc2 = eload(fcp);
        let c60e = ecr(60);
        println!("  v{}: i32 = syscall v{} v{}", NC, c60e, fc2); NC = NC + 1;   /* exit(failcount) */
        let r = ec(0);
        println!("  ret v{}", r);
    }
}

/* LAYER4 (keeper 2026-08-01): release-mode conditional-compilation SOURCE-PREPROCESSING.
   Applied ONCE over SRC before any prepass/parse (Doc 731 alphabet-purity: purify the source alphabet upstream
   so no downstream pass re-handles cfg). Model: debug_assertions = FALSE (rustc ships the lexer in release).
     - `#[cfg(debug_assertions)]`      -> blank the attribute AND its attached item (field/stmt/block).
     - `#[cfg(not(debug_assertions))]` -> blank only the attribute (keep the item).
     - any other attr (`#[derive]`, `#[inline]`, `#[test]`, `#[cfg(feature=..)]`) -> untouched.
   Blanks with spaces (not deletion) so every byte offset / POS-based span stays valid. Scans past line/block
   comments and string literals so a `#[cfg..]` inside a comment or string is never mistaken for a real attr. */
fn cfg_span_has(s: i32, e: i32, p0: i32, p1: i32, p2: i32) -> bool {
    unsafe { let mut q = s; while q + 3 <= e { if at(q) == p0 && at(q + 1) == p1 && at(q + 2) == p2 { return true; } q = q + 1; } false }
}
fn cfg_blank(s: i32, e: i32) { unsafe { let mut i = s; while i < e { SRC[i as usize] = 32; i = i + 1; } } }
fn preprocess_cfg() {
    unsafe {
        let mut p = 0;
        while p + 1 < LEN {
            let c = at(p);
            if c == 47 && at(p + 1) == 47 { while p < LEN && at(p) != 10 { p = p + 1; } }                       /* // line comment */
            else if c == 47 && at(p + 1) == 42 { p = p + 2; while p + 1 < LEN && !(at(p) == 42 && at(p + 1) == 47) { p = p + 1; } p = p + 2; }   /* block comment */
            else if c == 34 { p = p + 1; while p < LEN && at(p) != 34 { if at(p) == 92 { p = p + 1; } p = p + 1; } p = p + 1; }                  /* "string" */
            else if c == 35 && at(p + 1) == 91 {                                                                /* #[ attribute */
                let astart = p; let mut d = 0; let mut q = p + 1;
                loop { if q >= LEN { break; } let cc = at(q); if cc == 91 { d = d + 1; q = q + 1; } else if cc == 93 { d = d - 1; q = q + 1; if d == 0 { break; } } else { q = q + 1; } }
                let aend = q;   /* just past the ] */
                let has_cfg = cfg_span_has(astart, aend, 99, 102, 103);   /* "cfg" */
                let has_dbg0 = cfg_span_has(astart, aend, 100, 101, 98);   /* "deb" (debug_assertions) */
                let has_test = cfg_span_has(astart, aend, 116, 101, 115);   /* "tes" (test) -- LAYER4: #[cfg(test)] excluded in a non-test build, like debug_assertions in release */
                let has_dbg = has_dbg0 || has_test;                        /* both `debug_assertions` and `test` are FALSE predicates in the shipped build */
                let has_not = cfg_span_has(astart, aend, 110, 111, 116);   /* "not" */
                if has_cfg && has_dbg && has_not == false {                /* #[cfg(debug_assertions)] / #[cfg(test)] -> strip attr + item */
                    cfg_blank(astart, aend);
                    let mut r = aend; while r < LEN && (at(r) == 32 || at(r) == 9 || at(r) == 10 || at(r) == 13) { r = r + 1; }
                    if r < LEN && at(r) == 123 {                           /* block item: blank the balanced {..} */
                        let bs = r; let mut b = 0;
                        loop { if r >= LEN { break; } let cc = at(r); if cc == 123 { b = b + 1; r = r + 1; } else if cc == 125 { b = b - 1; r = r + 1; if b == 0 { break; } } else { r = r + 1; } }
                        cfg_blank(bs, r);
                    } else {                                              /* field/stmt/braced item: scan to the item end at ()[] depth 0 */
                        let bs = r; let mut nd = 0;
                        loop {
                            if r >= LEN { break; }
                            let cc = at(r);
                            if cc == 40 || cc == 91 { nd = nd + 1; r = r + 1; }
                            else if cc == 41 || cc == 93 { if nd > 0 { nd = nd - 1; } r = r + 1; }
                            else if cc == 123 { let mut b = 0; loop { if r >= LEN { break; } let ccc = at(r); if ccc == 123 { b = b + 1; r = r + 1; } else if ccc == 125 { b = b - 1; r = r + 1; if b == 0 { break; } } else { r = r + 1; } } break; }   /* item has a {..} body (fn/struct/impl) -> blank through it */
                            else if nd == 0 && cc == 44 { r = r + 1; break; }   /* , (field end) */
                            else if nd == 0 && cc == 59 { r = r + 1; break; }   /* ; (stmt end) */
                            else if nd == 0 && cc == 125 { break; }             /* } (last field/stmt) -- stop before */
                            else { r = r + 1; }
                        }
                        cfg_blank(bs, r);
                    }
                    p = aend;   /* re-scan from just past the (now-blank) attr: the item is spaces, so this can't skip a later attr but avoids overshooting one (bugfix: p=r skipped a following cfg block when the item scan stopped early) */
                } else if has_cfg && has_not && has_dbg {                  /* #[cfg(not(debug_assertions))] -> blank only the attribute, keep the item */
                    cfg_blank(astart, aend); p = aend;
                } else {
                    p = aend;                                             /* other attr -> untouched */
                }
            }
            else { p = p + 1; }
        }
    }
}
/* LAYER4 P-MOD-FILE-SPLICE (S7 P-MODULE-TREE, file-module slice): resolve `mod NAME;` / `pub mod NAME;` by opening
   the sibling `NAME.rs` (cwd-relative) and grow-splicing `mod NAME { <contents> }` inline, so the already-green
   inline-mod + use-resolution substrate handles the rest. Runs AFTER preprocess_cfg (so `#[cfg(test)] mod tests;` is
   already blanked). Inert on the self-image (L6 has no `mod NAME;`) -> g2==g3 untouched. Open-fail leaves the decl
   untouched (a genuinely-needed module then surfaces as an unresolved name downstream). Syscalls via __syscall:
   open=2(path,O_RDONLY,0), read=0(fd,buf,n), close=3(fd). */
fn preprocess_modules() {
    unsafe {
        let mut p = 0;
        while p + 3 < LEN {
            let c = at(p);
            if c == 47 && at(p + 1) == 47 { while p < LEN && at(p) != 10 { p = p + 1; } }
            else if c == 47 && at(p + 1) == 42 { p = p + 2; while p + 1 < LEN && !(at(p) == 42 && at(p + 1) == 47) { p = p + 1; } p = p + 2; }
            else if c == 34 { p = p + 1; while p < LEN && at(p) != 34 { if at(p) == 92 { p = p + 1; } p = p + 1; } p = p + 1; }
            else if c == 109 && at(p + 1) == 111 && at(p + 2) == 100 && (p == 0 || is_idc(at(p - 1)) == false) && is_idc(at(p + 3)) == false {
                let mut q = p + 3; while q < LEN && (at(q) == 32 || at(q) == 9 || at(q) == 10 || at(q) == 13) { q = q + 1; }
                let no = q; while q < LEN && is_idc(at(q)) { q = q + 1; } let nl = q - no;
                let mut r = q; while r < LEN && (at(r) == 32 || at(r) == 9 || at(r) == 10 || at(r) == 13) { r = r + 1; }
                if nl > 0 && r < LEN && at(r) == 59 {                       /* `mod NAME;` file-module form */
                    let mut i = 0; while i < nl { PATHBUF[i as usize] = at(no + i) as u8; i = i + 1; }
                    PATHBUF[nl as usize] = 46; PATHBUF[(nl + 1) as usize] = 114; PATHBUF[(nl + 2) as usize] = 115; PATHBUF[(nl + 3) as usize] = 0;   /* ".rs\0" */
                    let fd = __syscall(2, &PATHBUF[0], 0, 0);              /* open(NAME.rs, O_RDONLY) */
                    if fd >= 0 {
                        let n = __syscall(0, fd, &MODBUF[0], 262144);      /* read the file */
                        __syscall(3, fd, 0, 0);                            /* close */
                        if n > 0 && LEN + n + 1 < 1500000 {
                            let grow = n + 1;                              /* `;` (1) -> `{`+contents(n)+`}` (n+2); net +n+1 */
                            let mut k = LEN - 1;
                            while k >= r + 1 { SRC[(k + grow) as usize] = SRC[k as usize]; k = k - 1; }
                            SRC[r as usize] = 123;                         /* { */
                            let mut j = 0; while j < n { SRC[(r + 1 + j) as usize] = MODBUF[j as usize]; j = j + 1; }
                            SRC[(r + 1 + n) as usize] = 125;               /* } */
                            LEN = LEN + grow;
                            p = r;                                         /* descend into the spliced body for nested `mod SUB;` */
                        } else { p = r + 1; }                             /* empty/oversize -> leave untouched */
                    } else { p = q; }                                     /* open failed -> leave `mod NAME;` untouched */
                } else { p = q; }                                         /* inline `mod NAME { .. }` / other -> scan the body */
            }
            else if c == 101 && p + 11 < LEN && at(p + 1) == 120 && at(p + 2) == 116 && at(p + 3) == 101 && at(p + 4) == 114 && at(p + 5) == 110 && (p == 0 || is_idc(at(p - 1)) == false) && is_idc(at(p + 6)) == false {
                /* RUSTC-FE CG-3 (crate-graph, Model A): `extern crate NAME;` -> rewrite to `mod NAME;` (overwrite "extern crate" with "mod"+spaces, re-scan) so the mod-splice pulls in the sibling dep STUB `NAME.rs`. Cross-crate resolution then rides the already-green inline-mod + use path. If NAME.rs is absent the mod-splice open-fails -> the dep surfaces as an unresolved name (fail-closed at the ref). Inert on L6/the corpus (no `extern crate`) -> g2==g3 untouched. */
                let mut w = p + 6; while w < LEN && (at(w) == 32 || at(w) == 9 || at(w) == 10 || at(w) == 13) { w = w + 1; }
                if w + 5 < LEN && at(w) == 99 && at(w + 1) == 114 && at(w + 2) == 97 && at(w + 3) == 116 && at(w + 4) == 101 && is_idc(at(w + 5)) == false {
                    SRC[p as usize] = 109; SRC[(p + 1) as usize] = 111; SRC[(p + 2) as usize] = 100;   /* "mod" */
                    let mut z = p + 3; while z < w + 5 { SRC[z as usize] = 32; z = z + 1; }             /* blank the rest of "extern .. crate" with spaces -> `mod   NAME;` re-scans as a mod-splice */
                } else { p = p + 1; }
            }
            else { p = p + 1; }
        }
    }
}
fn main() {
    unsafe {
        LEN = __syscall(0, 0, &SRC, 1500000);
        preprocess_modules();   /* LAYER4 P-MOD-FILE-SPLICE: splice `mod NAME;` siblings inline FIRST, so preprocess_cfg below sees the spliced module content (cursor.rs/unescape.rs #[cfg(debug_assertions)] fields/blocks + #[cfg(test)] mod tests;). A `#[cfg(test)] mod tests;` open-fails gracefully (left untouched) then gets blanked by cfg. */
        preprocess_cfg();   /* LAYER4: release-mode cfg source excision over the FULLY-SPLICED source, before every prepass/parse */
        // CFML consolidation: enlarged arenas for the SEM0 self-emit (no-free bump heap peaks ~1.6GB; peek/bump
        // enum-descriptor churn). __str_arena 32MB, __heap 1.6GB. Demand-zero BSS -- no disk/RAM cost for small programs.
        // cf.rs sets the frame base (1.8GB) above this budget. Order UNCHANGED (str_cursor/heap_cursor stay adjacent to
        // their arenas -- safe because the measured peak is below the arena size, so no exhaustion clobber).
        println!("gdef __str_arena 33554432");
        println!("gdef __str_cursor 8");
        println!("gdef __heap 2013265920");
        println!("gdef __heap_cursor 8");
        println!("gdef __obuf 1048576");                /* R3: buffered-output ring */
        println!("gdef __obuf_n 8");                    /* R3: buffered byte count */
        prepass_aliases();
        prepass_structs();
        prepass_constfns();
        prepass_consts();                                                          /* CONST-ORDER: consts/const-fns registered BEFORE enums/statics so `enum E { A=K }` discriminants and `static S: [T; SIZE]` sizes can ceval const names */
        prepass_enums();
        prepass_fns();
        prepass_methods();
        prepass_generics();
        prepass_statics();
        prepass_traits();
        prepass_blankets();
        prepass_tests();
        prepass_macros();
        emit_prelude();
        POS = 0;
        loop {
            skipws();
            if POS >= LEN { break; }
            let save = POS;
            let got = rd_ident();
            if got && id_is(109, 97, 99, 114, 111, 11) && POS < LEN && at(POS) == 33 {   /* A: `macro_rules! NAME { .. }` -- registered by prepass_macros; skip its whole block in codegen */
                POS = POS + 1; skipws(); rd_ident();
                while POS < LEN && at(POS) != 123 { POS = POS + 1; }
                eat_ch(123);
                let mut d = 1; while POS < LEN && d > 0 { if at(POS) == 123 { d = d + 1; } if at(POS) == 125 { d = d - 1; } POS = POS + 1; }
                continue;
            }
            if got && is_kw_trait() {                                                   /* A: skip the trait body wholesale -- prepass_traits already registered defaults; decls (incl. `-> Self::Item`) need no codegen */
                rd_ident();
                while POS < LEN && at(POS) != 123 { POS = POS + 1; }
                eat_ch(123);
                let mut d = 1; while POS < LEN && d > 0 { if at(POS) == 123 { d = d + 1; } if at(POS) == 125 { d = d - 1; } POS = POS + 1; }
            }
            else if got && is_kw_fn() {
                rd_ident();
                let isgen = gf_find(ID_O, ID_L) >= 0;
                POS = save;
                if isgen {                                    /* R9: generic fn -- emitted later as monomorphized instances */
                    rd_ident(); rd_ident();
                    skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }   /* skip generic <T> (A: const generics supported) */
                    while POS < LEN && at(POS) != 123 { POS = POS + 1; }
                    let mut d = 0;
                    loop { if POS >= LEN { break; } if at(POS) == 123 { d = d + 1; POS = POS + 1; } else if at(POS) == 125 { d = d - 1; POS = POS + 1; if d == 0 { break; } } else { POS = POS + 1; } }
                } else { parse_fn(); }
            }
            else if got && is_kw_impl() { parse_impl(); }
            else { POS = save; skip_tok(); }
        }
        let mut k = 0;
        while k < NINST { emit_generic_inst(IGF[k as usize], ISTYP[k as usize]); k = k + 1; }
        let mut kb = 0;
        while kb < NBINST { emit_blanket_inst(IBI[kb as usize], IBTYP[kb as usize]); kb = kb + 1; }
        let mut kg = 0;
        while kg < NGIMINST { emit_gim_inst(GIMINST[kg as usize], GIMINSTM[kg as usize]); kg = kg + 1; }   /* S3 R1: emit generic-impl method monomorphs (NGIMINST may grow for nested instances) */
        let mut ke = 0;   /* LAYER5 P-DERIVE-RECURSIVE-EQ: emit a synthetic `<Enum>_eq` for each payload-carrying enum with all-derivable fields and no user eq method */
        while ke < NEN { if enum_has_payload(ke) == 1 && enum_eq_derivable(ke) == 1 && enum_has_user_eq(ke) == 0 { emit_enum_eq_fn(ke); } ke = ke + 1; }
        let mut kd = 0;   /* LAYER5 P-DERIVE-DEBUG: emit a synthetic `<Enum>_dbg` for each recursive enum (Box<Self> field) so {:?} recurses */
        while kd < NEN { if enum_is_recursive(kd) == 1 { emit_enum_dbg_fn(kd); } kd = kd + 1; }
        let mut kc = 0;
        while kc < NCLO { emit_closure_body(kc); kc = kc + 1; }
        let mut kf = 0;
        while kf < NFPT { emit_fnptr_thunk(kf); kf = kf + 1; }
        emit_test_main();
    }
}
