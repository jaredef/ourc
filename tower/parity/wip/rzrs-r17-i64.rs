/* rzrs.rs -- a self-hosting slice of rzrs, WRITTEN IN the Rust subset and compiled by rzrs itself.
   Reads a Rust program on stdin, emits rzir. Surface: multiple fns (params, calls, i32/bool),
   let/assign, if/else, while+break, return, arithmetic (+ - * / %), comparison (== != < <= > >=),
   logical (&& || !), parens, unary minus. Observable = the exit code (main's tail/return).
   Written in exactly the subset it compiles, so it can eventually compile ITSELF. */

static mut SRC: [u8; 200000] = [0; 200000];
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

/* module-level statics (name span -> gdef). GK 0=scalar 1=array; GE=element stride (1 u8 / 4 i32) */
static mut GO: [i32; 2048] = [0; 2048];
static mut GL: [i32; 2048] = [0; 2048];
static mut GK: [i32; 2048] = [0; 2048];
static mut GE: [i32; 2048] = [0; 2048];
static mut NGLOB: i32 = 0;
static mut SK: [i32; 8192] = [0; 8192];   /* symbol kind: 0 scalar, 1 &str */
static mut EISSTR: i32 = 0;               /* set by a producer when the just-parsed value is a &str descriptor */
static mut EISVEC: i32 = 0;               /* R4: set when the just-parsed value is a Vec descriptor (let binds SK=2) */
static mut EISSTRING: i32 = 0;            /* R6: set when the just-parsed value is an owned String (let binds SK=3) */
static mut EISARR: i32 = 0;               /* R11: set when the just-parsed value is a local fixed array (let binds SK=5); buffer pointer, a[i]=*(a+i*4) */
static mut AV: [i32; 16] = [0; 16];       /* println! arg vNs */
static mut AS: [i32; 16] = [0; 16];       /* println! arg is-str flags */
static mut AC: [i32; 16] = [0; 16];       /* R12: println! arg is-char flags (print via putc) */
static mut A64: [i32; 16] = [0; 16];      /* R17: println! arg is-i64 flags (print via __print_int64) */
static mut EISCHAR: i32 = 0;              /* R12: the just-parsed value is a char (print as a character) */
static mut EIS64: i32 = 0;                /* R17: the just-parsed value is a 64-bit (i64) value */
static mut SI64: [i32; 8192] = [0; 8192]; /* R17: per-symbol i64 flag */

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
static mut FFK: [i32; 1024] = [0; 1024];   /* R7: field container-kind (0 scalar/struct, 2 Vec, 3 String) */
static mut NFLD: i32 = 0;
static mut SST: [i32; 8192] = [0; 8192];   /* per-symbol struct type (-1 if not a struct) */
static mut EITYP: i32 = 0;                 /* struct type of the just-parsed expr (-1 if none) */
static mut TAILDST: i32 = 0;               /* R3b if-expr: holds dst+1 (0 = unset); tail expr copies to dst-1 not `ret`. 0-default so it self-hosts without a non-zero static init */

/* R2: enum registry. ENO/ENL = enum name span. per-variant: VEN owner-enum, VO/VL variant name span,
   VDISC discriminant, VHAS 1 if the variant carries a single i32 payload. an enum VALUE is an 8-byte heap
   cell: discriminant at offset 0, payload at offset 4. */
static mut ENO: [i32; 64] = [0; 64];
static mut ENL: [i32; 64] = [0; 64];
static mut NEN: i32 = 0;
static mut VEN: [i32; 256] = [0; 256];
static mut VO: [i32; 256] = [0; 256];
static mut VL: [i32; 256] = [0; 256];
static mut VDISC: [i32; 256] = [0; 256];
static mut VHAS: [i32; 256] = [0; 256];
static mut VNF: [i32; 256] = [0; 256];   /* R13: number of payload fields (0, 1, or more) */
static mut NVAR: i32 = 0;

/* R6: free-fn return-kind registry -> propagate String/Vec-ness of a call result (FRK: 0 scalar, 2 Vec, 3 String) */
static mut FNO: [i32; 512] = [0; 512];
static mut FNL: [i32; 512] = [0; 512];
static mut FRK: [i32; 512] = [0; 512];
static mut NFRN: i32 = 0;

/* R15: impl-method return-struct registry (owner type, method name span, return struct type or -1) */
static mut MRTOWN: [i32; 512] = [0; 512];
static mut MRTO: [i32; 512] = [0; 512];
static mut MRTL: [i32; 512] = [0; 512];
static mut MRTS: [i32; 512] = [0; 512];
static mut NMRT: i32 = 0;

/* R9: generic-fn registry (monomorphization). GFO/GFL name span, GFTPO/GFTPL the type-param name (e.g. T),
   GFSIG the pos of the `(` param list, GFARG the index of the first param whose type is T. Instances to emit:
   (IGF fn-index, ISTYP concrete struct type; -2 = scalar i32). GSUB_* = active T-substitution while emitting. */
static mut GFO: [i32; 128] = [0; 128];
static mut GFL: [i32; 128] = [0; 128];
static mut GFTPO: [i32; 128] = [0; 128];
static mut GFTPL: [i32; 128] = [0; 128];
static mut GFSIG: [i32; 128] = [0; 128];
static mut GFARG: [i32; 128] = [0; 128];
static mut NGF: i32 = 0;
static mut IGF: [i32; 256] = [0; 256];
static mut ISTYP: [i32; 256] = [0; 256];
static mut NINST: i32 = 0;
static mut GSUB_ACT: i32 = 0;
static mut GSUB_TPO: i32 = 0;
static mut GSUB_TPL: i32 = 0;
static mut GSUB_STYP: i32 = 0;

/* R10: closures. Each `|params| body` is lifted to a top-level fn __closure_N(env, params...) emitted
   deferred; captures = enclosing locals referenced in the body. A closure VALUE is a heap {code@0, env@4}
   fat pointer (code = faddr(__closure_N)); called via calli. Locals/params of closure type are SK=4.
   Registry arrays flattened: params CPO/CPL[cid*4+i] (<=4), captures CAO/CAL[cid*8+i] (<=8). */
static mut CBO: [i32; 256] = [0; 256];
static mut CBL: [i32; 256] = [0; 256];
static mut CBLK: [i32; 256] = [0; 256];
static mut CNP: [i32; 256] = [0; 256];
static mut CNC: [i32; 256] = [0; 256];
static mut CPO: [i32; 1024] = [0; 1024];
static mut CPL: [i32; 1024] = [0; 1024];
static mut CAO: [i32; 2048] = [0; 2048];
static mut CAL: [i32; 2048] = [0; 2048];
static mut CAK: [i32; 2048] = [0; 2048];   /* capture symbol-kind (SK) */
static mut CAS: [i32; 2048] = [0; 2048];   /* capture struct type (SST) */
static mut NCLO: i32 = 0;
static mut EISCLO: i32 = 0;

fn ty_find(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NTYP { if span_eq_at(TSO[i as usize], TSL[i as usize], o, l) { return i; } i = i + 1; } -1 }
}
fn fld_find(ty: i32, o: i32, l: i32) -> i32 {   /* -> field index, or -1 */
    unsafe { let mut i = 0; while i < NFLD { if FSST[i as usize] == ty && span_eq_at(FSO[i as usize], FSL[i as usize], o, l) { return i; } i = i + 1; } -1 }
}

fn is_print() -> bool { unsafe { ID_L == 5 && at(ID_O) == 112 && at(ID_O + 1) == 114 && at(ID_O + 2) == 105 && at(ID_O + 3) == 110 && at(ID_O + 4) == 116 } }
fn is_println() -> bool { unsafe { ID_L == 7 && at(ID_O) == 112 && at(ID_O + 4) == 116 && at(ID_O + 5) == 108 && at(ID_O + 6) == 110 } }

/* emit the fixed rzir prelude: __putc / __print_str / __print_uint / __print_int (host-free I/O). */
fn emit_prelude() {
    println!("fn __putc");
    println!("  v0: i32 = param");
    println!("  v1: i32 = const 0");
    println!("  v2: i32 = add v0 v1");
    println!("  v3: i32 = const 1");
    println!("  v4: i32 = const 1");
    println!("  v5: i32 = ref v2");
    println!("  v6: i32 = const 1");
    println!("  v7: i32 = syscall v3 v4 v5 v6");
    println!("  ret v1");
    println!("fn __print_str");
    println!("  v0: i32 = param");
    println!("  v1: i32 = param");
    println!("  v2: i32 = const 0");
    println!("  v3: i32 = const 1");
    println!("  v4: i32 = const 1");
    println!("  v5: i32 = syscall v3 v4 v0 v1");
    println!("  ret v2");
    println!("fn __print_uint");
    println!("  v0: i32 = param");
    println!("  v1: i32 = const 0");
    println!("  v2: i32 = const 10");
    println!("  v3: i32 = ge v0 v2");
    println!("  brz v3");
    println!("  v4: i32 = const 10");
    println!("  v5: i32 = div v0 v4");
    println!("  v6: i32 = call __print_uint v5");
    println!("  pool");
    println!("  v7: i32 = const 48");
    println!("  v8: i32 = const 10");
    println!("  v9: i32 = mod v0 v8");
    println!("  v10: i32 = add v7 v9");
    println!("  v11: i32 = call __putc v10");
    println!("  ret v1");
    println!("fn __print_int");
    println!("  v0: i32 = param");
    println!("  v1: i32 = const 0");
    println!("  v2: i32 = add v0 v1");
    println!("  v3: i32 = const 0");
    println!("  v4: i32 = lt v2 v3");
    println!("  brz v4");
    println!("  v5: i32 = const 45");
    println!("  v6: i32 = call __putc v5");
    println!("  v7: i32 = ref v2");
    println!("  v8: i32 = const 0");
    println!("  v9: i32 = sub v8 v2");
    println!("  store v7 v9");
    println!("  pool");
    println!("  v10: i32 = call __print_uint v2");
    println!("  ret v1");
    /* R17: __print_uint64 / __print_int64 (i64 decimal via div8/mod8) */
    println!("fn __print_uint64");
    println!("  v0: i32 = param");
    println!("  v1: i32 = const 0");
    println!("  v2: i32 = const 10");
    println!("  v3: i64 = conv v2");
    println!("  v4: i64 = ge8 v0 v3");
    println!("  brz v4");
    println!("  v5: i32 = const 10");
    println!("  v6: i64 = conv v5");
    println!("  v7: i64 = div8 v0 v6");
    println!("  v8: i32 = call __print_uint64 v7");
    println!("  pool");
    println!("  v9: i32 = const 48");
    println!("  v10: i32 = const 10");
    println!("  v11: i64 = conv v10");
    println!("  v12: i64 = mod8 v0 v11");
    println!("  v13: i64 = conv v9");
    println!("  v14: i64 = add8 v13 v12");
    println!("  v15: i32 = call __putc v14");
    println!("  ret v1");
    println!("fn __print_int64");
    println!("  v0: i32 = param");
    println!("  v1: i32 = const 0");
    println!("  v2: i64 = or8 v0 v0");
    println!("  v3: i32 = const 0");
    println!("  v4: i64 = conv v3");
    println!("  v5: i64 = lt8 v2 v4");
    println!("  brz v5");
    println!("  v6: i32 = const 45");
    println!("  v7: i32 = call __putc v6");
    println!("  v8: i32 = ref v2");
    println!("  v9: i32 = const 0");
    println!("  v10: i64 = conv v9");
    println!("  v11: i64 = sub8 v10 v2");
    println!("  store8 v8 v11");
    println!("  pool");
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
        while i < l {
            let ci = ec(i);
            let ad = ebin("add", base, ci);
            let cv = ec(at(o + i));
            println!("  storeb v{} v{}", ad, cv);
            i = i + 1;
        }
        let doff = ((l + 3) / 4) * 4;
        let dc = ec(doff);
        let desc = ebin("add", base, dc);
        println!("  store v{} v{}", desc, base);
        let c4 = ec(4);
        let da = ebin("add", desc, c4);
        let lv = ec(l);
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
        if is_idst(c) { while POS < LEN && is_idc(at(POS)) { POS = POS + 1; } return; }
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
fn is_kw_struct() -> bool { id_is(115, 116, 114, 117, 99, 6) } /* struc(t) len 6 */
fn is_kw_impl() -> bool { id_is(105, 109, 112, 108, 0, 4) }    /* impl */
fn is_kw_self() -> bool { id_is(115, 101, 108, 102, 0, 4) }    /* self */
fn is_kw_enum() -> bool { id_is(101, 110, 117, 109, 0, 4) }    /* enum */
fn is_kw_match() -> bool { id_is(109, 97, 116, 99, 104, 5) }   /* match */
fn is_kw_for() -> bool { id_is(102, 111, 114, 0, 0, 3) }       /* for */
fn is_kw_continue() -> bool { id_is(99, 111, 110, 116, 105, 8) }  /* continu(e) len 8 */

fn enum_find(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NEN { if span_eq_at(ENO[i as usize], ENL[i as usize], o, l) { return i; } i = i + 1; } -1 }
}
fn var_find(o: i32, l: i32) -> i32 {   /* -> variant index (searches all enums), or -1 */
    unsafe { let mut i = 0; while i < NVAR { if span_eq_at(VO[i as usize], VL[i as usize], o, l) { return i; } i = i + 1; } -1 }
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

/* R1: register every `struct Name { f: T, ... }` (fields, offsets, size). Every field is 4 bytes
   (a scalar value or a pointer to a heap struct). */
fn prepass_structs() {
    unsafe {
        POS = 0;
        loop {
            skipws();
            if POS >= LEN { break; }
            let save = POS;
            if rd_ident() && is_kw_struct() {
                rd_ident();
                let no = ID_O; let nl = ID_L;
                skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
                let ty = NTYP;
                TSO[ty as usize] = no; TSL[ty as usize] = nl; NTYP = NTYP + 1;
                eat_ch(123);
                let mut off = 0;
                loop {
                    skipws();
                    if POS >= LEN { break; }
                    if at(POS) == 125 { POS = POS + 1; break; }
                    rd_ident();
                    let fo = ID_O; let fl = ID_L;
                    eat_ch(58); skipws();
                    if POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); } }
                    rd_ident();
                    let fsty = ty_find(ID_O, ID_L);
                    let mut ffk = 0;
                    if is_vec_ty(ID_O, ID_L) { ffk = 2; }
                    if is_string_ty(ID_O, ID_L) { ffk = 3; }
                    skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
                    FSST[NFLD as usize] = ty; FSO[NFLD as usize] = fo; FSL[NFLD as usize] = fl; FSB[NFLD as usize] = off; FSTY[NFLD as usize] = fsty; FFK[NFLD as usize] = ffk; NFLD = NFLD + 1;
                    off = off + 4;
                    skipws();
                    if POS < LEN && at(POS) == 44 { POS = POS + 1; }
                }
                TSZ[ty as usize] = off;
            } else { POS = save; skip_tok(); }
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
fn emit_span(o: i32, l: i32) { unsafe { __syscall(1, 1, &SRC[o as usize], l); } }
fn egref(o: i32, l: i32) -> i32 { unsafe { print!("  v{}: i32 = gref ", NC); emit_span(o, l); println!(""); NC = NC + 1; NC - 1 } }
fn eload(a: i32) -> i32 { unsafe { println!("  v{}: i32 = load v{}", NC, a); NC = NC + 1; NC - 1 } }
fn eloadb(a: i32) -> i32 { unsafe { println!("  v{}: i32 = loadb v{}", NC, a); NC = NC + 1; NC - 1 } }

fn sym_add(o: i32, l: i32, v: i32) { unsafe { SO[NSYM as usize] = o; SL[NSYM as usize] = l; SV[NSYM as usize] = v; SK[NSYM as usize] = 0; SST[NSYM as usize] = -1; SI64[NSYM as usize] = 0; NSYM = NSYM + 1; } }
fn sym_i64(o: i32, l: i32) -> i32 { unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SI64[i as usize]; } i = i - 1; } 0 } }
fn sym_styp(o: i32, l: i32) -> i32 {
    unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SST[i as usize]; } i = i - 1; } -1 }
}
fn sym_kind(o: i32, l: i32) -> i32 {
    unsafe { let mut i = NSYM - 1; while i >= 0 { if span_eq_at(SO[i as usize], SL[i as usize], o, l) { return SK[i as usize]; } i = i - 1; } 0 }
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

fn ec(v: i32) -> i32 { unsafe { EISSTR = 0; EIS64 = 0; println!("  v{}: i32 = const {}", NC, v); NC = NC + 1; NC - 1 } }
fn ebin(op: &str, a: i32, b: i32) -> i32 { unsafe { EISSTR = 0; EIS64 = 0; println!("  v{}: i32 = {} v{} v{}", NC, op, a, b); NC = NC + 1; NC - 1 } }
fn eref(v: i32) -> i32 { unsafe { println!("  v{}: i32 = ref v{}", NC, v); NC = NC + 1; NC - 1 } }
fn ecopy(dst: i32, src: i32) { unsafe { let z = ec(0); println!("  v{}: i32 = add v{} v{}", dst, src, z); } }
fn ecopy64(dst: i32, src: i32) { unsafe { println!("  v{}: i64 = or8 v{} v{}", dst, src, src); } }   /* R17: 64-bit copy */
/* R17: i64 helpers */
fn emit_conv(v: i32) -> i32 { unsafe { println!("  v{}: i64 = conv v{}", NC, v); NC = NC + 1; NC - 1 } }
fn w64(a: i32, b: i32) -> i32 { let mut w = 0; if a == 1 { w = 1; } if b == 1 { w = 1; } w }   /* 1 if either operand is i64 */
fn ec64raw(v: i64) -> i32 { unsafe { EISSTR = 0; EIS64 = 0; println!("  v{}: i32 = const {}", NC, v); NC = NC + 1; NC - 1 } }   /* const from a 32-bit-range i64 */
fn emit_i64_lit(val: i64) -> i32 {
    unsafe {
        if val < 4294967296 && val >= 0 {                       /* fits in low 32 bits: const(lo signed) -> conv -> or8 */
            let mut lo = val; if lo >= 2147483648 { lo = lo - 4294967296; }
            let c = ec64raw(lo); let cv = emit_conv(c);
            println!("  v{}: i64 = or8 v{} v{}", NC, cv, cv); let r = NC; NC = NC + 1; EIS64 = 1; r
        } else {
            let hi = val / 4294967296;
            let lo = val - hi * 4294967296;
            let mut los = lo; if los >= 2147483648 { los = los - 4294967296; }
            let cl = ec64raw(los); let clv = emit_conv(cl);
            let ch = ec64raw(hi); let chv = emit_conv(ch);
            let c32 = ec64raw(32); let c32v = emit_conv(c32);
            println!("  v{}: i64 = shl8 v{} v{}", NC, chv, c32v); let sh = NC; NC = NC + 1;
            println!("  v{}: i64 = or8 v{} v{}", NC, sh, clv); let r = NC; NC = NC + 1; EIS64 = 1; r
        }
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
fn emit_tail(v: i32) { unsafe { if TAILDST > 0 { if EIS64 == 1 { ecopy64(TAILDST - 1, v); } else { ecopy(TAILDST - 1, v); } } else if TAILDST < 0 { let d = v; } else { println!("  ret v{}", v); } } }   /* TAILDST<0 = loop body: discard */
fn parse_loop_body() { unsafe { let s = TAILDST; TAILDST = -1; parse_block(); TAILDST = s; } }   /* a loop body never returns a value */

/* a call: name ( args ) -- emit `call NAME arg...` printing the name span */
fn emit_call(no: i32, nl: i32) -> i32 {
    unsafe {
        let rk = fnret(no, nl);
        let mut a0 = -1; let mut a1 = -1; let mut a2 = -1; let mut a3 = -1;
        let mut a4 = -1; let mut a5 = -1; let mut a6 = -1; let mut a7 = -1;
        let mut t0 = -1; let mut t1 = -1; let mut t2 = -1; let mut t3 = -1;
        let mut t4 = -1; let mut t5 = -1; let mut t6 = -1; let mut t7 = -1;
        let mut na = 0;
        skipws();
        if peek() != 41 {
            a0 = parse_expr(); t0 = EITYP; na = 1;
            if eat_ch(44) { a1 = parse_expr(); t1 = EITYP; na = 2;
            if eat_ch(44) { a2 = parse_expr(); t2 = EITYP; na = 3;
            if eat_ch(44) { a3 = parse_expr(); t3 = EITYP; na = 4;
            if eat_ch(44) { a4 = parse_expr(); t4 = EITYP; na = 5;
            if eat_ch(44) { a5 = parse_expr(); t5 = EITYP; na = 6;
            if eat_ch(44) { a6 = parse_expr(); t6 = EITYP; na = 7;
            if eat_ch(44) { a7 = parse_expr(); t7 = EITYP; na = 8; } } } } } } }
        }
        eat_ch(41);
        /* __syscall intrinsic -> `syscall args` (no callee name) */
        let is_sc = nl == 9 && at(no) == 95 && at(no + 1) == 95 && at(no + 2) == 115 && at(no + 3) == 121 && at(no + 4) == 115 && at(no + 5) == 99 && at(no + 6) == 97 && at(no + 7) == 108 && at(no + 8) == 108;
        let gi = gf_find(no, nl);
        if gi >= 0 {                                                              /* R9: monomorphized call NAME_TypeName */
            let ai = GFARG[gi];
            let mut styp = -1;
            if ai == 0 { styp = t0; } else if ai == 1 { styp = t1; } else if ai == 2 { styp = t2; }
            else if ai == 3 { styp = t3; } else if ai == 4 { styp = t4; } else if ai == 5 { styp = t5; }
            else if ai == 6 { styp = t6; } else { styp = t7; }
            if styp < 0 { styp = -2; }
            reg_inst(gi, styp);
            print!("  v{}: i32 = call ", NC); emit_span(no, nl); print!("_");
            if styp == -2 { print!("i32"); } else { emit_span(TSO[styp as usize], TSL[styp as usize]); }
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
        println!("");
        NC = NC + 1;
        if rk == 3 { EISSTR = 1; EISSTRING = 1; } else if rk == 2 { EISVEC = 1; } else if rk == 1 { EISSTR = 1; } else if rk == 4 { EISCHAR = 1; } else if rk == 5 { EIS64 = 1; }
        NC - 1
    }
}

/* R1: `Name { f: e, .. }` -> heap struct; store each field at its offset; EITYP = the type. */
fn struct_literal(ty: i32) -> i32 {
    unsafe {
        let base = halloc(TSZ[ty as usize]);
        eat_ch(123);
        loop {
            skipws();
            if POS >= LEN { break; }
            if at(POS) == 125 { POS = POS + 1; break; }
            rd_ident();
            let fo = ID_O; let fl = ID_L;
            let fi = fld_find(ty, fo, fl);
            eat_ch(58);
            let e = parse_expr();
            let cb = ec(FSB[fi as usize]);
            let ad = ebin("add", base, cb);
            println!("  store v{} v{}", ad, e);
            skipws();
            if POS < LEN && at(POS) == 44 { POS = POS + 1; }
        }
        EISSTR = 0; EISVEC = 0; EISSTRING = 0;   /* the value is a struct, not a field's container-kind */
        EITYP = ty;
        base
    }
}
/* emit `call Type_method [self] args...` (self_vn < 0 for an associated fn). pos is just past `(`. */
fn emit_mcall(ty: i32, mo: i32, ml: i32, self_vn: i32) -> i32 {
    unsafe {
        let rs = mret_find(ty, mo, ml);
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
        emit_span(TSO[ty as usize], TSL[ty as usize]); print!("_"); emit_span(mo, ml);
        if self_vn >= 0 { print!(" v{}", self_vn); }
        if na >= 1 { print!(" v{}", a0); }
        if na >= 2 { print!(" v{}", a1); }
        if na >= 3 { print!(" v{}", a2); }
        if na >= 4 { print!(" v{}", a3); }
        if na >= 5 { print!(" v{}", a4); }
        if na >= 6 { print!(" v{}", a5); }
        println!("");
        NC = NC + 1;
        if rs >= 0 { EITYP = rs; }
        NC - 1
    }
}
/* dispatch `.field`* then optional `.method(args)` on a struct base `bv` of type `ty`. */
fn dispatch_struct(bv: i32, ty: i32) -> i32 {
    unsafe {
        let mut cur = bv; let mut cty = ty;
        loop {
            skipws();
            if POS >= LEN || at(POS) != 46 { break; }   /* not '.' */
            let save = POS; POS = POS + 1; skipws(); rd_ident();
            let mo = ID_O; let ml = ID_L; skipws();
            if POS < LEN && at(POS) == 40 { POS = POS + 1; return emit_mcall(cty, mo, ml, cur); }  /* method */
            let fi = fld_find(cty, mo, ml);
            if fi < 0 { POS = save; break; }
            let cb = ec(FSB[fi as usize]); let ad = ebin("add", cur, cb);
            let fty = FSTY[fi as usize];
            let fk = FFK[fi as usize];
            let v = eload(ad);
            if fk == 3 { return dispatch_string(v); }                            /* R7: String field -> .push_str/.len (prints as str) */
            if fk == 2 { return dispatch_vec(v); }                               /* R7: Vec field -> .push/.len/[i] */
            if fty >= 0 { cur = v; cty = fty; } else { EITYP = -1; return v; }   /* scalar field -> done */
        }
        EITYP = cty; cur
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
            if POS >= LEN || at(POS) != 46 { return ad; }
            cur = eload(ad); cty = FSTY[fi as usize];
        }
        cur
    }
}
/* R4: Vec<i32> = an 8-byte heap descriptor [buf-ptr@0, len@4] with a fixed 1024-elem backing buffer (no
   realloc). is_vec_ty/is_vec_macro detect the `Vec`/`vec!` spellings; emit_vec_new builds a descriptor. */
fn is_vec_ty(o: i32, l: i32) -> bool { unsafe { l == 3 && at(o) == 86 && at(o + 1) == 101 && at(o + 2) == 99 } }   /* Vec */
fn is_vec_macro(o: i32, l: i32) -> bool { unsafe { l == 3 && at(o) == 118 && at(o + 1) == 101 && at(o + 2) == 99 } }/* vec */
fn emit_vec_new() -> i32 {
    unsafe {
        let d = halloc(8);
        let buf = halloc(4096);
        println!("  store v{} v{}", d, buf);
        EISVEC = 1;
        d
    }
}
fn emit_vec_push(dv: i32, e: i32) {
    unsafe {
        let c4 = ec(4); let lad = ebin("add", dv, c4); let n = eload(lad);
        let p = eload(dv); let c4b = ec(4); let off = ebin("mul", n, c4b); let ea = ebin("add", p, off);
        println!("  store v{} v{}", ea, e);
        let one = ec(1); let n1 = ebin("add", n, one);
        let c4c = ec(4); let lad2 = ebin("add", dv, c4c);
        println!("  store v{} v{}", lad2, n1);
    }
}
fn dispatch_vec(dv: i32) -> i32 {
    unsafe {
        skipws();
        if POS < LEN && at(POS) == 46 {
            POS = POS + 1; rd_ident();
            if id_is(108, 101, 110, 0, 0, 3) { skipws(); eat_ch(40); eat_ch(41); let c4 = ec(4); let ad = ebin("add", dv, c4); return eload(ad); }   /* len */
            if id_is(112, 117, 115, 104, 0, 4) { skipws(); eat_ch(40); let e = parse_expr(); eat_ch(41); emit_vec_push(dv, e); return ec(0); }        /* push */
            if id_is(112, 111, 112, 0, 0, 3) {                                                                                                        /* pop -> Option<i32> */
                skipws(); eat_ch(40); eat_ch(41);
                let c4 = ec(4); let lad = ebin("add", dv, c4); let len = eload(lad);
                let cell = halloc(8);
                let lb = LBL; LBL = LBL + 1;
                let z = ec(0); let gt = ebin("gt", len, z);
                println!("  brz v{}", gt);
                let one = ec(1); let n = ebin("sub", len, one);
                let c4b = ec(4); let lad2 = ebin("add", dv, c4b); println!("  store v{} v{}", lad2, n);
                let p = eload(dv); let c4c = ec(4); let off = ebin("mul", n, c4c); let ea = ebin("add", p, off); let val = eload(ea);
                let s1 = ec(1); println!("  store v{} v{}", cell, s1);
                let c4d = ec(4); let cad = ebin("add", cell, c4d); println!("  store v{} v{}", cad, val);
                println!("  goto __Lp{}", lb);
                println!("  pool");
                let z2 = ec(0); println!("  store v{} v{}", cell, z2);
                println!("  label __Lp{}", lb);
                return cell;
            }
            if id_is(105, 116, 101, 114, 0, 4) { skipws(); eat_ch(40); eat_ch(41); return dv; }   /* iter() -> the vec itself (for-loop iterates it) */
            return dv;
        }
        if POS < LEN && at(POS) == 91 {                                                                                                              /* v[i] read */
            POS = POS + 1; let iv = parse_expr(); eat_ch(93);
            let p = eload(dv); let c4 = ec(4); let off = ebin("mul", iv, c4); let ea = ebin("add", p, off); return eload(ea);
        }
        dv
    }
}

/* R5: postfix Option/Result ops on a cell value v -- `?`, .unwrap(), .unwrap_or(d), .is_some/none/ok/err().
   disc==1 = success (Some/Ok), disc==0 = fail (None/Err); payload at v+4. Rewinds a non-matching `.method`. */
fn opt_postfix(v: i32) -> i32 {
    unsafe {
        skipws();
        if POS < LEN && at(POS) == 63 {                                       /* v?  -> early-return on fail */
            POS = POS + 1;
            let disc = eload(v); let z = ec(0); let isfail = ebin("eq", disc, z);
            println!("  brz v{}", isfail);
            println!("  ret v{}", v);
            println!("  pool");
            let c4 = ec(4); let pa = ebin("add", v, c4); return eload(pa);
        }
        if POS < LEN && at(POS) == 46 {
            let s = POS;
            POS = POS + 1; rd_ident();
            if id_is(117, 110, 119, 114, 97, 6) {                                 /* unwrap -- panic (exit 101) on None/Err */
                skipws(); eat_ch(40); eat_ch(41);
                let disc = eload(v); let z = ec(0); let isnone = ebin("eq", disc, z);
                println!("  brz v{}", isnone);
                let c60 = ec(60); let c101 = ec(101); println!("  v{}: i32 = syscall v{} v{}", NC, c60, c101); NC = NC + 1;
                println!("  pool");
                let c4 = ec(4); let pa = ebin("add", v, c4); return eload(pa);
            }
            if id_is(117, 110, 119, 114, 97, 9) {                             /* unwrap_or(d) */
                skipws(); eat_ch(40); let d = parse_expr(); eat_ch(41);
                let disc = eload(v); let result = NC; NC = NC + 1; let lb = LBL; LBL = LBL + 1;
                println!("  brz v{}", disc);
                let c4 = ec(4); let pa = ebin("add", v, c4); let pv = eload(pa); ecopy(result, pv);
                println!("  goto __Lu{}", lb);
                println!("  pool");
                ecopy(result, d);
                println!("  label __Lu{}", lb);
                return result;
            }
            if id_is(105, 115, 95, 115, 111, 7) { skipws(); eat_ch(40); eat_ch(41); let disc = eload(v); let one = ec(1); return ebin("eq", disc, one); }   /* is_some */
            if id_is(105, 115, 95, 111, 107, 5) { skipws(); eat_ch(40); eat_ch(41); let disc = eload(v); let one = ec(1); return ebin("eq", disc, one); }    /* is_ok */
            if id_is(105, 115, 95, 110, 111, 7) { skipws(); eat_ch(40); eat_ch(41); let disc = eload(v); let z = ec(0); return ebin("eq", disc, z); }        /* is_none */
            if id_is(105, 115, 95, 101, 114, 6) { skipws(); eat_ch(40); eat_ch(41); let disc = eload(v); let z = ec(0); return ebin("eq", disc, z); }        /* is_err */
            POS = s; return v;
        }
        v
    }
}

/* R6: String = the same [buf@0, len@4] descriptor as &str (so it prints via the str path) but OWNED, with a
   fixed 4096-byte backing buffer that push_str/push mutate. Locals tracked via SK=3. */
fn is_string_ty(o: i32, l: i32) -> bool { unsafe { l == 6 && at(o) == 83 && at(o + 1) == 116 && at(o + 2) == 114 && at(o + 3) == 105 && at(o + 4) == 110 && at(o + 5) == 103 } }   /* String */
fn emit_str_new() -> i32 {
    unsafe {
        let d = halloc(8);
        let buf = halloc(4096);
        println!("  store v{} v{}", d, buf);
        EISSTR = 1; EISSTRING = 1;
        d
    }
}
fn emit_str_push_byte(dv: i32, ch: i32) {
    unsafe {
        let c4 = ec(4); let dlad = ebin("add", dv, c4); let dl = eload(dlad);
        let dp = eload(dv); let da = ebin("add", dp, dl);
        println!("  storeb v{} v{}", da, ch);
        let one = ec(1); let dl1 = ebin("add", dl, one);
        let c4b = ec(4); let dlad2 = ebin("add", dv, c4b);
        println!("  store v{} v{}", dlad2, dl1);
    }
}
fn emit_str_push_str(dv: i32, src: i32) {
    unsafe {
        let c4a = ec(4); let dlad = ebin("add", dv, c4a); let dl0 = eload(dlad);
        let dp = eload(dv); let dbase = ebin("add", dp, dl0);
        let sp = eload(src);
        let c4b = ec(4); let slad = ebin("add", src, c4b); let sl = eload(slad);
        let islot = NC; let z = ec(0); ecopy(islot, z); NC = NC + 1;
        println!("  loop");
        let cnd = ebin("lt", islot, sl);
        println!("  brz v{}", cnd);
        let sa = ebin("add", sp, islot); let byte = eloadb(sa);
        let da = ebin("add", dbase, islot);
        println!("  storeb v{} v{}", da, byte);
        let one = ec(1); let i1 = ebin("add", islot, one); ecopy(islot, i1);
        println!("  cont"); println!("  jmp"); println!("  endloop");
        let dl2 = ebin("add", dl0, sl);
        let c4c = ec(4); let dlad2 = ebin("add", dv, c4c);
        println!("  store v{} v{}", dlad2, dl2);
    }
}
fn dispatch_string(dv: i32) -> i32 {
    unsafe {
        skipws();
        if POS < LEN && at(POS) == 46 {
            let s = POS;
            POS = POS + 1; rd_ident();
            if id_is(108, 101, 110, 0, 0, 3) { skipws(); eat_ch(40); eat_ch(41); let c4 = ec(4); let ad = ebin("add", dv, c4); return eload(ad); }   /* len */
            if id_is(112, 117, 115, 104, 95, 8) { skipws(); eat_ch(40); let src = parse_expr(); eat_ch(41); emit_str_push_str(dv, src); return ec(0); }  /* push_str */
            if id_is(112, 117, 115, 104, 0, 4) { skipws(); eat_ch(40); let ch = parse_expr(); eat_ch(41); emit_str_push_byte(dv, ch); return ec(0); }    /* push */
            POS = s;
        }
        EISSTR = 1;
        dv
    }
}

/* R10: parse `|params| body` -> register + build a heap {code, env} fat pointer from captured locals. */
fn parse_closure() -> i32 {
    unsafe {
        let cid = NCLO; NCLO = NCLO + 1;
        POS = POS + 1;                                        /* first `|` */
        skipws();
        let mut np = 0;
        if POS < LEN && at(POS) == 124 { POS = POS + 1; }     /* `||` no params */
        else {
            loop {
                skipws();
                if POS < LEN && at(POS) == 124 { POS = POS + 1; break; }
                rd_ident(); CPO[(cid * 4 + np) as usize] = ID_O; CPL[(cid * 4 + np) as usize] = ID_L; np = np + 1;
                skipws();
                if POS < LEN && at(POS) == 58 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; if is_kw_mut_next() { } } rd_ident(); skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }
                skipws();
                if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { if POS < LEN && at(POS) == 124 { POS = POS + 1; } break; }
            }
        }
        CNP[cid as usize] = np;
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
                        CAK[(cid * 8 + nc) as usize] = sym_kind(s, len); CAS[(cid * 8 + nc) as usize] = sym_styp(s, len);
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
            let cv = sym_find(CAO[(cid * 8 + i) as usize], CAL[(cid * 8 + i) as usize]);
            let off = ec(i * 4); let ad = ebin("add", env, off);
            println!("  store v{} v{}", ad, cv);
            i = i + 1;
        }
        let clo = halloc(8);
        println!("  v{}: i32 = faddr __closure_{}", NC, cid); let fa = NC; NC = NC + 1;
        println!("  store v{} v{}", clo, fa);
        let c4 = ec(4); let ad2 = ebin("add", clo, c4);
        println!("  store v{} v{}", ad2, env);
        EISCLO = 1;
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
        let c4 = ec(4); let envad = ebin("add", fv, c4); let env = eload(envad);
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
        NC = 0; NSYM = 0;
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
            i = i + 1;
        }
        i = 0;
        while i < np { sym_add(CPO[(cid * 4 + i) as usize], CPL[(cid * 4 + i) as usize], envvn + 1 + i); i = i + 1; }
        let czero = ec(0);
        POS = CBO[cid as usize];
        if CBLK[cid as usize] == 1 { parse_block(); println!("  ret v{}", czero); }
        else { let r = parse_expr(); println!("  ret v{}", r); }
    }
}

fn parse_atom() -> i32 {
    unsafe {
        skipws();
        EISSTR = 0; EITYP = -1; EISVEC = 0; EISSTRING = 0; EISCLO = 0; EISARR = 0; EISCHAR = 0; EIS64 = 0;
        if POS < LEN && at(POS) == 124 { return parse_closure(); }                 /* |params| body */
        if POS < LEN && at(POS) == 34 {                                            /* "string literal" */
            POS = POS + 1; let so = POS;
            while POS < LEN && at(POS) != 34 { if at(POS) == 92 { POS = POS + 1; } POS = POS + 1; }
            let sl = POS - so; POS = POS + 1;
            return emit_str_literal(so, sl);
        }
        if eat_ch(40) { let v = parse_expr(); eat_ch(41); return v; }
        if eat_ch(33) { let v = parse_atom(); let z = ec(0); return ebin("eq", v, z); }   /* !x -> x==0 */
        if eat_ch(45) { let v = parse_atom(); let z = ec(0); return ebin("sub", z, v); }  /* -x */
        if eat_ch(38) { return parse_addr(); }                                            /* &place */
        if eat_ch(42) { let p = parse_atom(); return eload(p); }                          /* *p deref read */
        if POS < LEN && is_dig(at(POS)) {
            let mut n: i64 = 0;
            while POS < LEN && is_dig(at(POS)) { n = n * 10 + ((at(POS) - 48) as i64); POS = POS + 1; }
            if n >= 2147483648 { return emit_i64_lit(n); }
            return ec64raw(n);
        }
        if POS < LEN && at(POS) == 39 {                                                   /* char literal 'c' */
            POS = POS + 1; let mut cc = at(POS);
            if cc == 92 { POS = POS + 1; let e = at(POS); if e == 110 { cc = 10; } else { cc = e; } }
            POS = POS + 1; if POS < LEN && at(POS) == 39 { POS = POS + 1; }
            let r = ec(cc); EISCHAR = 1; return r;
        }
        if POS < LEN && at(POS) == 91 {                                            /* R11: [e; n] or [e0, e1, ...] local array literal */
            POS = POS + 1; skipws();
            let e0 = parse_expr(); skipws();
            if POS < LEN && at(POS) == 59 {                                        /* [e; n] repeat */
                POS = POS + 1; skipws();
                let mut n = 0; while POS < LEN && is_dig(at(POS)) { n = n * 10 + (at(POS) - 48); POS = POS + 1; }
                eat_ch(93);
                let mut sz = n * 4; if sz < 4 { sz = 4; }
                let buf = halloc(sz);
                let mut i = 0;
                while i < n { let off = ec(i * 4); let ad = ebin("add", buf, off); println!("  store v{} v{}", ad, e0); i = i + 1; }
                EISARR = 1; return buf;
            }
            let buf = halloc(4096);                                                /* [e0, e1, ...] list (fixed cap) */
            let z0 = ec(0); let ad0 = ebin("add", buf, z0); println!("  store v{} v{}", ad0, e0);
            let mut i = 1;
            loop {
                skipws();
                if POS < LEN && at(POS) == 44 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 93 { break; }
                    let e = parse_expr(); let off = ec(i * 4); let ad = ebin("add", buf, off); println!("  store v{} v{}", ad, e); i = i + 1;
                } else { break; }
            }
            eat_ch(93);
            EISARR = 1; return buf;
        }
        rd_ident();
        let o = ID_O; let l = ID_L;
        if is_kw_true() { return ec(1); }
        if is_kw_false() { return ec(0); }
        if is_kw_match() { return parse_match(); }
        if is_kw_if() { return parse_if_expr(); }
        if is_std(o, l) {                                                          /* std::process::exit(e) -> exit syscall */
            eat_ch(58); eat_ch(58); rd_ident();
            eat_ch(58); eat_ch(58); rd_ident();
            skipws(); eat_ch(40); let e = parse_expr(); eat_ch(41);
            let c60 = ec(60);
            println!("  v{}: i32 = syscall v{} v{}", NC, c60, e); let r = NC; NC = NC + 1;
            return r;
        }
        if opt_disc(o, l) >= 0 {                                                  /* R5: bare Some/None/Ok/Err constructor */
            let disc = opt_disc(o, l); let has = opt_has(o, l);
            let cell = halloc(8);
            let cd = ec(disc); println!("  store v{} v{}", cell, cd);
            if has {
                skipws(); eat_ch(40); let arg = parse_expr(); eat_ch(41);
                let c4 = ec(4); let ad = ebin("add", cell, c4); println!("  store v{} v{}", ad, arg);
            }
            return opt_postfix(cell);
        }
        skipws();
        let en = enum_find(o, l);
        if en >= 0 && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {        /* E::Variant [ (arg) ] constructor */
            POS = POS + 2; skipws(); rd_ident();
            let vi = var_find(ID_O, ID_L);
            let disc = VDISC[vi as usize]; let has = VHAS[vi as usize]; let nf = VNF[vi as usize];
            let mut sz = 4 + nf * 4; if sz < 8 { sz = 8; }
            let cell = halloc(sz);
            let cd = ec(disc); println!("  store v{} v{}", cell, cd);
            if has == 1 {
                skipws(); eat_ch(40);
                let mut fi = 0;
                loop {
                    skipws(); if POS < LEN && at(POS) == 41 { break; }
                    let arg = parse_expr();
                    let off = ec(4 + fi * 4); let ad = ebin("add", cell, off); println!("  store v{} v{}", ad, arg);
                    fi = fi + 1;
                    skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; }
                }
                eat_ch(41);
            }
            return cell;
        }
        if is_vec_ty(o, l) && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {  /* Vec::new() */
            POS = POS + 2; rd_ident(); skipws(); eat_ch(40); eat_ch(41);
            return emit_vec_new();
        }
        if is_vec_macro(o, l) && POS < LEN && at(POS) == 33 {                         /* vec![ e, ... ] */
            POS = POS + 1; skipws(); eat_ch(91);
            let dv = emit_vec_new();
            skipws();
            if POS < LEN && at(POS) != 93 {
                loop {
                    let e = parse_expr();
                    emit_vec_push(dv, e);
                    skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { POS = POS; break; }
                }
            }
            eat_ch(93); EISVEC = 1;
            return dv;
        }
        if is_string_ty(o, l) && POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {  /* String::new() / String::from("..") */
            POS = POS + 2; rd_ident();
            let isfrom = id_is(102, 114, 111, 109, 0, 4);   /* from */
            skipws(); eat_ch(40);
            let dv = emit_str_new();
            if isfrom { skipws(); let src = parse_expr(); emit_str_push_str(dv, src); EISSTR = 1; EISSTRING = 1; }
            eat_ch(41);
            return dv;
        }
        let ty = ty_find(o, l);
        if ty >= 0 {
            if POS < LEN && at(POS) == 123 { return struct_literal(ty); }           /* Name { .. } literal */
            if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {                 /* Name::assoc(args) */
                POS = POS + 2; skipws(); rd_ident();
                let mo = ID_O; let ml = ID_L; skipws(); eat_ch(40);
                let r = emit_mcall(ty, mo, ml, -1); EITYP = ty; return r;            /* assoc fn -- assume it returns Self */
            }
        }
        if sym_kind(o, l) == 4 && POS < LEN && at(POS) == 40 { return emit_closure_call(sym_find(o, l)); }   /* R10: closure call */
        if POS < LEN && at(POS) == 40 { POS = POS + 1; let r = emit_call(o, l); return opt_postfix(r); }   /* call [+ Option/Result postfix] */
        let styp = sym_styp(o, l);
        let sv = sym_find(o, l);
        if sv >= 0 && sym_kind(o, l) == 4 { EISCLO = 1; return sv; }                 /* R10: closure value (fat pointer) */
        if sv >= 0 && sym_kind(o, l) == 5 {                                          /* R11: local array -> a[i] read */
            skipws();
            if POS < LEN && at(POS) == 91 { POS = POS + 1; let iv = parse_expr(); eat_ch(93); let c4 = ec(4); let off = ebin("mul", iv, c4); let ad = ebin("add", sv, off); return eload(ad); }
            return sv;
        }
        if sv >= 0 && sym_kind(o, l) == 2 { return dispatch_vec(sv); }               /* R4: Vec local -> .push/.len/.pop/[i] */
        if sv >= 0 && (sym_kind(o, l) == 3 || sym_kind(o, l) == 1) { return dispatch_string(sv); }   /* R6: String/&str local -> .len (prints as str) */
        if sv >= 0 && styp >= 0 { return dispatch_struct(sv, styp); }               /* struct local/param -> field/method */
        if sv >= 0 { if sym_i64(o, l) == 1 { EIS64 = 1; } return opt_postfix(sv); }  /* local scalar (may be Option cell / i64) */
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
                return eload(ad);
            }
            if GK[gi] == 1 { return egref(o, l); }                                 /* bare global array -> base pointer */
            let gr = egref(o, l); return eload(gr);                                /* global scalar read */
        }
        -1
    }
}
fn parse_addr() -> i32 {
    unsafe {
        skipws(); if POS < LEN && at(POS) == 42 { POS = POS + 1; return parse_atom(); }   /* &*x -> x */
        rd_ident();
        if is_kw_mut() { rd_ident(); }                                             /* &mut NAME */
        let o = ID_O; let l = ID_L;
        let gi = g_find(o, l);
        skipws();
        if POS < LEN && at(POS) == 91 {                                            /* &NAME[i] */
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
        if sk == 1 || sk == 3 { EISSTR = 1; return sym_find(o, l); }                /* &str / &String -> descriptor value */
        if sk == 2 { return sym_find(o, l); }                                      /* &Vec -> descriptor value */
        let stp = sym_styp(o, l);
        if stp >= 0 { EITYP = stp; return sym_find(o, l); }                         /* &struct -> struct pointer + type (for generic dispatch) */
        let slot = sym_find(o, l); return eref(slot);                              /* &local scalar -> ref(slot) */
    }
}
fn parse_cast() -> i32 {
    let a = parse_atom();
    /* `as TYPE`* -> no-op in this i32 model, except `as char` sets the char-display flag */
    unsafe {
        loop {
            skipws();
            if POS + 1 < LEN && at(POS) == 97 && at(POS + 1) == 115 && (POS + 2 >= LEN || !is_idc(at(POS + 2))) {
                POS = POS + 2; rd_ident();
                if ID_L == 4 && at(ID_O) == 99 && at(ID_O + 1) == 104 && at(ID_O + 2) == 97 && at(ID_O + 3) == 114 { EISCHAR = 1; } else { EISCHAR = 0; }
            } else { break; }
        }
    }
    a
}
fn parse_mul() -> i32 {
    let mut a = parse_cast();
    unsafe {
        let mut a64 = EIS64;
        loop {
            let c = peek();
            if c == 42 { POS = POS + 1; let b = parse_cast(); let b64 = EIS64; let w = w64(a64, b64); a = emit_binop("mul", a, b, w, w); a64 = w; }
            else if c == 47 { POS = POS + 1; let b = parse_cast(); let b64 = EIS64; let w = w64(a64, b64); a = emit_binop("div", a, b, w, w); a64 = w; }
            else if c == 37 { POS = POS + 1; let b = parse_cast(); let b64 = EIS64; let w = w64(a64, b64); a = emit_binop("mod", a, b, w, w); a64 = w; }
            else { break; }
        }
        EIS64 = a64;
    }
    a
}
fn parse_add() -> i32 {
    let mut a = parse_mul();
    unsafe {
        let mut a64 = EIS64;
        loop {
            let c = peek();
            if c == 43 { POS = POS + 1; let b = parse_mul(); let b64 = EIS64; let w = w64(a64, b64); a = emit_binop("add", a, b, w, w); a64 = w; }
            else if c == 45 { POS = POS + 1; let b = parse_mul(); let b64 = EIS64; let w = w64(a64, b64); a = emit_binop("sub", a, b, w, w); a64 = w; }
            else { break; }
        }
        EIS64 = a64;
    }
    a
}
fn parse_cmp() -> i32 {
    let a = parse_add();
    unsafe {
        let a64 = EIS64;
        if eat2(61, 61) { let b = parse_add(); let w = w64(a64, EIS64); return emit_binop("eq", a, b, w, 0); }
        if eat2(33, 61) { let b = parse_add(); let w = w64(a64, EIS64); return emit_binop("ne", a, b, w, 0); }
        if eat2(60, 61) { let b = parse_add(); let w = w64(a64, EIS64); return emit_binop("le", a, b, w, 0); }
        if eat2(62, 61) { let b = parse_add(); let w = w64(a64, EIS64); return emit_binop("ge", a, b, w, 0); }
        let c = peek();
        if c == 60 { POS = POS + 1; let b = parse_add(); let w = w64(a64, EIS64); return emit_binop("lt", a, b, w, 0); }
        if c == 62 { POS = POS + 1; let b = parse_add(); let w = w64(a64, EIS64); return emit_binop("gt", a, b, w, 0); }
    }
    a
}
/* && and || via short-circuit result slots */
fn parse_and() -> i32 {
    let mut a = parse_cmp();
    unsafe {
        while eat2(38, 38) {
            let res = ec(0);
            println!("  brz v{}", a);
            let b = parse_cmp();
            let one = ec(1); let zz = ec(0);
            let nb = ebin("ne", b, zz);
            ecopy(res, nb);
            println!("  pool");
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
            let zz = ec(0);
            let na = ebin("eq", a, zz);
            println!("  brz v{}", na);
            let b = parse_and();
            let zz2 = ec(0);
            let nb = ebin("ne", b, zz2);
            ecopy(res, nb);
            println!("  pool");
            a = res;
        }
    }
    a
}
fn parse_expr() -> i32 { parse_expr_impl() }

/* emit a print!/println! (macro name already consumed); eats through the closing `)` but NOT a trailing `;`
   so it works both as a statement and as a match-arm body. */
fn emit_print(nlf: bool) {
    unsafe {
        eat_ch(33); eat_ch(40); skipws(); eat_ch(34);
        let fo = POS;
        while POS < LEN && at(POS) != 34 { if at(POS) == 92 { POS = POS + 1; } POS = POS + 1; }
        let fl = POS - fo; POS = POS + 1;
        let mut na = 0;
        skipws();
        while eat_ch(44) { skipws(); if peek() == 41 { break; } let a = parse_expr(); AV[na as usize] = a; AS[na as usize] = EISSTR; AC[na as usize] = EISCHAR; A64[na as usize] = EIS64; na = na + 1; }
        eat_ch(41);
        let mut hole = 0;
        let mut k = fo;
        while k < fo + fl {
            let c = at(k);
            if c == 123 {
                while k < fo + fl && at(k) != 125 { k = k + 1; }
                k = k + 1;
                if AS[hole as usize] == 1 {
                    let d = AV[hole as usize];
                    let p = eload(d);
                    let c4 = ec(4); let da = ebin("add", d, c4); let ln = eload(da);
                    println!("  v{}: i32 = call __print_str v{} v{}", NC, p, ln); NC = NC + 1;
                } else if AC[hole as usize] == 1 {
                    println!("  v{}: i32 = call __putc v{}", NC, AV[hole as usize]); NC = NC + 1;
                } else if A64[hole as usize] == 1 {
                    println!("  v{}: i32 = call __print_int64 v{}", NC, AV[hole as usize]); NC = NC + 1;
                } else {
                    println!("  v{}: i32 = call __print_int v{}", NC, AV[hole as usize]); NC = NC + 1;
                }
                hole = hole + 1;
            } else {
                let cv = ec(c);
                println!("  v{}: i32 = call __putc v{}", NC, cv); NC = NC + 1;
                k = k + 1;
            }
        }
        if nlf { let cv = ec(10); println!("  v{}: i32 = call __putc v{}", NC, cv); NC = NC + 1; }
    }
}

fn parse_stmt() {
    unsafe {
        skipws();
        if POS < LEN && at(POS) == 42 {                                            /* *p = e  (deref store) or *p; */
            POS = POS + 1; let p = parse_atom(); skipws();
            if POS < LEN && at(POS) == 61 && (POS + 1 >= LEN || at(POS + 1) != 61) {
                POS = POS + 1; let e = parse_expr(); eat_ch(59);
                println!("  store v{} v{}", p, e);
            } else { eat_ch(59); }
            return;
        }
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
            rd_ident(); if is_kw_mut() { rd_ident(); }   /* mut? then the name is in ID */
            /* if we consumed `mut`, ID now holds the name; else ID holds the name already */
            let no = ID_O; let nl = ID_L;
            /* optional `: TYPE` */
            skipws();
            let mut ann64 = 0;
            if POS < LEN && at(POS) == 58 { POS = POS + 1; skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; } rd_ident();
                if ID_L == 3 && at(ID_O) == 105 && at(ID_O + 1) == 54 && at(ID_O + 2) == 52 { ann64 = 1; }   /* i64 */
                if ID_L == 3 && at(ID_O) == 117 && at(ID_O + 1) == 54 && at(ID_O + 2) == 52 { ann64 = 1; }   /* u64 */
                skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }
            eat_ch(61);
            let e = parse_expr();
            let ety = EITYP;
            let evec = EISVEC;
            let estring = EISSTRING;
            let eclo = EISCLO;
            let estr = EISSTR;
            let earr = EISARR;
            let e64 = w64(ann64, EIS64);
            eat_ch(59);
            let slot = NC;
            if e64 == 1 { ecopy64(slot, e); } else { ecopy(slot, e); }
            NC = NC + 1;
            sym_add(no, nl, slot);
            if e64 == 1 { SI64[(NSYM - 1) as usize] = 1; }    /* R17: i64-typed local */
            if ety >= 0 { SST[(NSYM - 1) as usize] = ety; }   /* R1: struct-typed local */
            if evec == 1 { SK[(NSYM - 1) as usize] = 2; }     /* R4: Vec-typed local */
            if estring == 1 { SK[(NSYM - 1) as usize] = 3; }  /* R6: String-typed local */
            if eclo == 1 { SK[(NSYM - 1) as usize] = 4; }     /* R10: closure-typed local */
            if earr == 1 { SK[(NSYM - 1) as usize] = 5; }     /* R11: array-typed local */
            if estr == 1 && evec == 0 && estring == 0 && eclo == 0 && earr == 0 && ety < 0 { SK[(NSYM - 1) as usize] = 1; }   /* &str local (only when not a struct/other kind) */
            return;
        }
        if is_kw_if() {
            let c = parse_expr();
            println!("  brz v{}", c);
            parse_block();
            skipws();
            let s2 = POS;
            if rd_ident() && is_kw_else() {
                let lb = LBL; LBL = LBL + 1;
                println!("  goto __Le{}", lb);
                println!("  pool");
                skipws();
                let s3 = POS;
                if rd_ident() && is_kw_if() { POS = s3; parse_stmt(); } else { POS = s3; parse_block(); }
                println!("  label __Le{}", lb);
            } else { POS = s2; println!("  pool"); }
            return;
        }
        if is_kw_while() {
            println!("  loop");
            let c = parse_expr();
            println!("  brz v{}", c);
            parse_loop_body();
            println!("  cont");
            println!("  jmp");
            println!("  endloop");
            return;
        }
        if is_kw_loop() {
            println!("  loop");
            let one = ec(1);
            println!("  brz v{}", one);
            parse_loop_body();
            println!("  cont");
            println!("  jmp");
            println!("  endloop");
            return;
        }
        if is_kw_for() {
            /* for VAR in START..END (range) OR for [ (i,x) ] in &vec / vec.iter()[.enumerate()] (collection) */
            skipws();
            let mut ivo = 0; let mut ivl = 0; let mut istuple = 0;
            if POS < LEN && at(POS) == 40 {               /* (i, x) tuple pattern (enumerate) */
                istuple = 1; POS = POS + 1; skipws();
                rd_ident(); ivo = ID_O; ivl = ID_L;
                skipws(); eat_ch(44); skipws();
                rd_ident();
                skipws(); eat_ch(41);
            } else { rd_ident(); }
            let vo = ID_O; let vl = ID_L;
            rd_ident();                                   /* `in` */
            let sv = parse_expr();
            skipws();
            if istuple == 0 && POS + 1 < LEN && at(POS) == 46 && at(POS + 1) == 46 {   /* range form: START..[=]END */
                POS = POS + 2;
                let mut incl = 0;
                if POS < LEN && at(POS) == 61 { POS = POS + 1; incl = 1; }
                let ev = parse_expr();
                let slot = NC; ecopy(slot, sv); NC = NC + 1;
                let mark = NSYM; sym_add(vo, vl, slot);
                println!("  loop");
                let mut c = 0;
                if incl == 1 { c = ebin("le", slot, ev); } else { c = ebin("lt", slot, ev); }
                println!("  brz v{}", c);
                parse_loop_body();
                println!("  cont");
                let one = ec(1); let ni = ebin("add", slot, one); ecopy(slot, ni);
                println!("  jmp"); println!("  endloop");
                NSYM = mark;
                return;
            }
            /* collection form: sv is a Vec descriptor -> iterate elements, binding VAR = v[i] (+ index if tuple) */
            if POS < LEN && at(POS) == 46 {               /* optional `.enumerate()` */
                let es = POS; POS = POS + 1; rd_ident();
                if id_is(101, 110, 117, 109, 101, 9) { skipws(); eat_ch(40); eat_ch(41); } else { POS = es; }
            }
            let dv = sv;
            let c4 = ec(4); let lenad = ebin("add", dv, c4); let len = eload(lenad);
            let islot = NC; let z = ec(0); ecopy(islot, z); NC = NC + 1;
            let xslot = NC; NC = NC + 1;
            let mark = NSYM;
            if istuple == 1 { sym_add(ivo, ivl, islot); }
            sym_add(vo, vl, xslot);
            println!("  loop");
            let cnd = ebin("lt", islot, len);
            println!("  brz v{}", cnd);
            let buf = eload(dv); let c4b = ec(4); let off = ebin("mul", islot, c4b); let ea = ebin("add", buf, off); let el = eload(ea);
            ecopy(xslot, el);
            parse_loop_body();
            println!("  cont");
            let one = ec(1); let ni = ebin("add", islot, one); ecopy(islot, ni);
            println!("  jmp"); println!("  endloop");
            NSYM = mark;
            return;
        }
        if is_kw_return() {
            skipws();
            if POS < LEN && at(POS) == 59 { let z = ec(0); POS = POS + 1; println!("  ret v{}", z); }
            else { let v = parse_expr(); eat_ch(59); println!("  ret v{}", v); }
            return;
        }
        if is_kw_break() { eat_ch(59); println!("  brk"); return; }
        if is_kw_continue() { eat_ch(59); println!("  cnt"); return; }
        if is_kw_unsafe() { parse_block(); return; }
        if is_print() || is_println() {
            let nlf = is_println();
            emit_print(nlf); eat_ch(59);
            return;
        }
        skipws();
        /* struct field write `NAME.field... = EXPR ;` (lookahead for a top-level `=` distinguishes it
           from a method-call statement `p.m(args);`) */
        if sym_styp(o, l) >= 0 && POS < LEN && at(POS) == 46 {
            let mut q = POS; let mut depth = 0; let mut isasg = 0;
            while q < LEN {
                let ch = at(q);
                if ch == 40 || ch == 91 || ch == 123 { depth = depth + 1; q = q + 1; }
                else if ch == 41 || ch == 93 || ch == 125 { if depth == 0 { q = LEN; } else { depth = depth - 1; q = q + 1; } }
                else if depth == 0 && ch == 59 { q = LEN; }
                else if depth == 0 && ch == 61 && (q + 1 >= LEN || at(q + 1) != 61) { isasg = 1; q = LEN; }
                else { q = q + 1; }
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
            let av = sym_find(o, l);
            POS = POS + 1; let iv = parse_expr(); eat_ch(93); skipws();
            if POS < LEN && at(POS) == 61 && (POS + 1 >= LEN || at(POS + 1) != 61) {
                POS = POS + 1; let e = parse_expr(); eat_ch(59);
                let c4 = ec(4); let off = ebin("mul", iv, c4); let ad = ebin("add", av, off);
                println!("  store v{} v{}", ad, e);
                return;
            }
            POS = save; let v = parse_expr(); skipws(); if eat_ch(59) { let d = v; } else { skipws(); if POS < LEN && at(POS) == 125 { emit_tail(v); } } return;
        }
        /* R4: VEC[i] = EXPR ; (local Vec element store) */
        if POS < LEN && at(POS) == 91 && sym_kind(o, l) == 2 {
            let dv = sym_find(o, l);
            POS = POS + 1; let iv = parse_expr(); eat_ch(93); skipws();
            if POS < LEN && at(POS) == 61 && (POS + 1 >= LEN || at(POS + 1) != 61) {
                POS = POS + 1; let e = parse_expr(); eat_ch(59);
                let p = eload(dv); let c4 = ec(4); let off = ebin("mul", iv, c4); let ad = ebin("add", p, off);
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
                if es == 1 { println!("  storeb v{} v{}", ad, e); } else { println!("  store v{} v{}", ad, e); }
                return;
            }
            POS = save; let v = parse_expr(); skipws(); if eat_ch(59) { let d = v; } else { skipws(); if POS < LEN && at(POS) == 125 { emit_tail(v); } } return;
        }
        /* NAME = EXPR ;  (global scalar or local) */
        if POS < LEN && at(POS) == 61 && (POS + 1 >= LEN || at(POS + 1) != 61) {
            let vi64 = sym_i64(o, l);
            POS = POS + 1; let e = parse_expr(); eat_ch(59);
            let gi = g_find(o, l);
            let mut base = 0;
            if gi >= 0 { base = egref(o, l); } else { let slot = sym_find(o, l); base = eref(slot); }
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

fn parse_block() {
    unsafe {
        eat_ch(123);
        let mark = NSYM;
        loop {
            skipws();
            if POS >= LEN { break; }
            if at(POS) == 125 { POS = POS + 1; break; }
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
        eat_ch(40);
        if self_ty >= 0 {
            /* consume `&self`/`self`/`&mut self` as the first param */
            skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; if is_kw_mut_next() { } }
            skipws(); let sp = POS;
            if rd_ident() && is_kw_self() {
                println!("  v{}: i32 = param", NC);
                sym_add(ID_O, ID_L, NC); SST[(NSYM - 1) as usize] = self_ty;
                NC = NC + 1;
                skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; }
            } else { POS = sp; }
        }
        skipws();
        if peek() != 41 {
            loop {
                let mut pkind = 0; let mut psty = -1; let mut pi64 = 0;
                rd_ident();
                let po = ID_O; let pl = ID_L;
                skipws();
                if POS < LEN && at(POS) == 58 { POS = POS + 1;
                    skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; if is_kw_mut_next() { } }
                    rd_ident();
                    if ID_L == 3 && at(ID_O) == 115 && at(ID_O + 1) == 116 && at(ID_O + 2) == 114 { pkind = 1; }   /* &str */
                    if ID_L == 3 && at(ID_O) == 105 && at(ID_O + 1) == 54 && at(ID_O + 2) == 52 { pi64 = 1; }     /* i64 */
                    if ID_L == 3 && at(ID_O) == 117 && at(ID_O + 1) == 54 && at(ID_O + 2) == 52 { pi64 = 1; }     /* u64 */
                    if is_vec_ty(ID_O, ID_L) { pkind = 2; }                                                        /* &Vec */
                    if is_string_ty(ID_O, ID_L) { pkind = 3; }                                                     /* &String */
                    psty = ty_find(ID_O, ID_L);
                    if GSUB_ACT == 1 && GSUB_STYP >= 0 && span_eq_at(ID_O, ID_L, GSUB_TPO, GSUB_TPL) { psty = GSUB_STYP; }   /* R9: T -> concrete struct */
                    if (ID_L == 4 && at(ID_O) == 105 && at(ID_O + 1) == 109 && at(ID_O + 2) == 112 && at(ID_O + 3) == 108) || (ID_L == 2 && at(ID_O) == 70 && at(ID_O + 1) == 110) {   /* impl / Fn -> closure param */
                        pkind = 4;
                        let mut d = 0;
                        loop { if POS >= LEN { break; } let ch = at(POS);
                            if ch == 40 || ch == 91 || ch == 123 { d = d + 1; POS = POS + 1; }
                            else if ch == 41 || ch == 93 || ch == 125 { if d == 0 { break; } d = d - 1; POS = POS + 1; }
                            else if d == 0 && ch == 44 { break; } else { POS = POS + 1; } }
                    }
                    skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }
                println!("  v{}: i32 = param", NC);
                sym_add(po, pl, NC);
                if pkind > 0 { SK[(NSYM - 1) as usize] = pkind; }
                if psty >= 0 { SST[(NSYM - 1) as usize] = psty; }
                if pi64 == 1 { SI64[(NSYM - 1) as usize] = 1; }
                NC = NC + 1;
                if !eat_ch(44) { break; }
            }
        }
        eat_ch(41);
        skipws();
        if POS + 1 < LEN && at(POS) == 45 && at(POS + 1) == 62 { POS = POS + 2; skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; if POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); } } rd_ident();
            skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }
        let z = ec(0);
        parse_block();
        println!("  ret v{}", z);
    }
}
fn parse_fn() {
    unsafe {
        rd_ident();  /* fn */
        rd_ident();
        let no = ID_O; let nl = ID_L;
        NC = 0; NSYM = 0;
        skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }  /* skip generic <T> */
        let is_main = nl == 4 && at(no) == 109 && at(no + 1) == 97 && at(no + 2) == 105 && at(no + 3) == 110;
        if is_main { println!("ir main -> i32"); }
        else { print!("fn "); __syscall(1, 1, &SRC[no as usize], nl); println!(""); }
        parse_pb(-1);
    }
}
/* impl [Trait for] Type { fn m(&self,..) {..} } -> emit each method as `Type_m` */
fn parse_impl() {
    unsafe {
        skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
        rd_ident();
        let mut to = ID_O; let mut tl = ID_L;
        skipws(); let s = POS;
        if rd_ident() && ID_L == 3 && at(ID_O) == 102 && at(ID_O + 1) == 111 && at(ID_O + 2) == 114 {   /* `for` */
            rd_ident(); to = ID_O; tl = ID_L;
        } else { POS = s; }
        skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
        let ty = ty_find(to, tl);
        eat_ch(123);
        loop {
            skipws();
            if POS >= LEN { break; }
            if at(POS) == 125 { POS = POS + 1; break; }
            if rd_ident() && is_kw_fn() {
                rd_ident();
                let mo = ID_O; let ml = ID_L;
                NC = 0; NSYM = 0;
                skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
                print!("fn "); __syscall(1, 1, &SRC[to as usize], tl); print!("_"); __syscall(1, 1, &SRC[mo as usize], ml); println!("");
                /* self injected only if the method has a self receiver (parse_pb handles the peek) */
                parse_pb(ty);
            } else { skip_tok(); }
        }
    }
}
fn is_kw_mut_next() -> bool { unsafe { skipws(); if POS + 2 < LEN && at(POS) == 109 && at(POS + 1) == 117 && at(POS + 2) == 116 { POS = POS + 3; true } else { false } } }

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
                ENO[en as usize] = ID_O; ENL[en as usize] = ID_L; NEN = NEN + 1;
                eat_ch(123);
                let mut disc = 0;
                loop {
                    skipws();
                    if POS < LEN && at(POS) == 125 { POS = POS + 1; break; }
                    if POS >= LEN { break; }
                    rd_ident();
                    let vi = NVAR;
                    VEN[vi as usize] = en; VO[vi as usize] = ID_O; VL[vi as usize] = ID_L;
                    VDISC[vi as usize] = disc; VHAS[vi as usize] = 0; VNF[vi as usize] = 0;
                    skipws();
                    if POS < LEN && at(POS) == 40 {
                        VHAS[vi as usize] = 1;
                        let mut nf = 1;
                        let mut d = 1; POS = POS + 1;
                        while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } if d == 1 && at(POS) == 44 { nf = nf + 1; } POS = POS + 1; }
                        VNF[vi as usize] = nf;
                    }
                    NVAR = NVAR + 1; disc = disc + 1;
                    skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; }
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
fn parse_if_arm(result: i32, endlbl: i32) {
    unsafe {
        let c = parse_expr();
        println!("  brz v{}", c);
        parse_branch_into(result);
        println!("  goto __Li{}", endlbl);
        println!("  pool");
        skipws();
        let s2 = POS;
        if rd_ident() && is_kw_else() {
            skipws();
            let s3 = POS;
            if rd_ident() && is_kw_if() { parse_if_arm(result, endlbl); }
            else { POS = s3; parse_branch_into(result); }
        } else { POS = s2; }
    }
}
fn parse_if_expr() -> i32 {
    unsafe {
        let result = NC; NC = NC + 1;
        let endlbl = LBL; LBL = LBL + 1;
        parse_if_arm(result, endlbl);
        println!("  label __Li{}", endlbl);
        EITYP = -1;
        result
    }
}

fn parse_match() -> i32 {
    unsafe {
        let sv = parse_expr();
        eat_ch(123);
        let endlbl = LBL; LBL = LBL + 1;
        let result = NC; NC = NC + 1;
        let save2 = POS; let mut isenum = 0;
        skipws();
        if POS < LEN && is_idst(at(POS)) { rd_ident();
            if var_find(ID_O, ID_L) >= 0 || opt_disc(ID_O, ID_L) >= 0 { isenum = 1; }
            else { skipws(); if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { isenum = 1; } } }
        POS = save2;
        let mut base = sv;
        if isenum == 1 { base = eload(sv); }
        loop {
            skipws();
            if POS < LEN && at(POS) == 125 { POS = POS + 1; break; }
            if POS >= LEN { break; }
            let mark = NSYM;
            let mut istest = 1; let mut testv = 0;
            if at(POS) == 95 { POS = POS + 1; istest = 0; }
            else if is_dig(at(POS)) {
                let mut n = 0; while POS < LEN && is_dig(at(POS)) { n = n * 10 + (at(POS) - 48); POS = POS + 1; }
                let cv = ec(n); testv = ebin("eq", base, cv);
            } else {
                rd_ident(); skipws();
                if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 { POS = POS + 2; rd_ident(); }   /* E::V or bare V */
                let vi = var_find(ID_O, ID_L);
                let mut disc = 0;
                if vi >= 0 { disc = VDISC[vi as usize]; } else { disc = opt_disc(ID_O, ID_L); }
                let cv = ec(disc); testv = ebin("eq", base, cv);
                skipws();
                if POS < LEN && at(POS) == 40 {
                    POS = POS + 1;
                    let mut fi = 0;
                    loop {
                        skipws(); if POS < LEN && at(POS) == 41 { break; }
                        rd_ident(); let bo = ID_O; let bl = ID_L;
                        let off = ec(4 + fi * 4); let pp = ebin("add", sv, off); let val = eload(pp);
                        let slot = NC; ecopy(slot, val); NC = NC + 1; sym_add(bo, bl, slot);
                        fi = fi + 1;
                        skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; } else { break; }
                    }
                    eat_ch(41);
                }
            }
            eat_ch(61); eat_ch(62);
            if istest == 1 { println!("  brz v{}", testv); }
            skipws();
            let bsave = POS;
            let mut e = 0;
            if rd_ident() && (is_print() || is_println()) { let nlf = is_println(); emit_print(nlf); e = ec(0); }
            else { POS = bsave; e = parse_expr(); }
            ecopy(result, e);
            println!("  goto __Lm{}", endlbl);
            if istest == 1 { println!("  pool"); }
            NSYM = mark;
            skipws(); if POS < LEN && at(POS) == 44 { POS = POS + 1; }
        }
        println!("  label __Lm{}", endlbl);
        EITYP = -1;
        result
    }
}

/* R9: monomorphization support */
fn gf_find(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NGF { if span_eq_at(GFO[i as usize], GFL[i as usize], o, l) { return i; } i = i + 1; } -1 }
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
                skipws();
                if POS < LEN && at(POS) == 60 {
                    POS = POS + 1; skipws(); rd_ident();
                    let tpo = ID_O; let tpl = ID_L;
                    let mut d = 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; }
                    skipws();
                    let sig = POS;
                    let arg = find_tparg(sig, tpo, tpl);
                    GFO[NGF as usize] = fo; GFL[NGF as usize] = fl; GFTPO[NGF as usize] = tpo; GFTPL[NGF as usize] = tpl;
                    GFSIG[NGF as usize] = sig; GFARG[NGF as usize] = arg; NGF = NGF + 1;
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
        NC = 0; NSYM = 0;
        GSUB_ACT = 1; GSUB_TPO = GFTPO[gf as usize]; GSUB_TPL = GFTPL[gf as usize]; GSUB_STYP = styp;
        print!("fn "); emit_span(GFO[gf as usize], GFL[gf as usize]); print!("_");
        if styp == -2 { print!("i32"); } else { emit_span(TSO[styp as usize], TSL[styp as usize]); }
        println!("");
        POS = GFSIG[gf as usize];
        parse_pb(-1);
        GSUB_ACT = 0;
    }
}

/* R15: impl-method return-struct type (or -1) */
fn mret_find(ty: i32, mo: i32, ml: i32) -> i32 {
    unsafe { let mut i = 0; while i < NMRT { if MRTOWN[i as usize] == ty && span_eq_at(MRTO[i as usize], MRTL[i as usize], mo, ml) { return MRTS[i as usize]; } i = i + 1; } -1 }
}
fn prepass_methods() {
    unsafe {
        POS = 0;
        loop {
            skipws();
            if POS >= LEN { break; }
            let save = POS;
            if rd_ident() && is_kw_impl() {
                skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
                rd_ident(); let mut to = ID_O; let mut tl = ID_L;
                skipws(); let s = POS;
                if rd_ident() && ID_L == 3 && at(ID_O) == 102 && at(ID_O + 1) == 111 && at(ID_O + 2) == 114 { rd_ident(); to = ID_O; tl = ID_L; } else { POS = s; }
                skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
                let owner = ty_find(to, tl);
                eat_ch(123);
                loop {
                    skipws();
                    if POS < LEN && at(POS) == 125 { POS = POS + 1; break; }
                    if POS >= LEN { break; }
                    if rd_ident() && is_kw_fn() {
                        rd_ident(); let mo = ID_O; let ml = ID_L;
                        skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
                        skipws(); if POS < LEN && at(POS) == 40 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } POS = POS + 1; } }
                        skipws(); let mut rs = -1;
                        if POS + 1 < LEN && at(POS) == 45 && at(POS + 1) == 62 { POS = POS + 2; skipws();
                            if POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); }
                            rd_ident();
                            if ID_L == 4 && at(ID_O) == 83 && at(ID_O + 1) == 101 && at(ID_O + 2) == 108 && at(ID_O + 3) == 102 { rs = owner; } else { rs = ty_find(ID_O, ID_L); } }
                        MRTOWN[NMRT as usize] = owner; MRTO[NMRT as usize] = mo; MRTL[NMRT as usize] = ml; MRTS[NMRT as usize] = rs; NMRT = NMRT + 1;
                        skipws(); while POS < LEN && at(POS) != 123 { POS = POS + 1; }
                        let mut d = 0;
                        loop { if POS >= LEN { break; } if at(POS) == 123 { d = d + 1; POS = POS + 1; } else if at(POS) == 125 { d = d - 1; POS = POS + 1; if d == 0 { break; } } else { POS = POS + 1; } }
                    } else { skip_tok(); }
                }
            } else { POS = save; skip_tok(); }
        }
    }
}

/* return-kind detection at POS (past the params): 0 scalar, 1 &str, 2 Vec, 3 String, 4 char, 5 i64. */
fn scan_ret_kind() -> i32 {
    unsafe {
        let mut rk = 0;
        if POS + 1 < LEN && at(POS) == 45 && at(POS + 1) == 62 {
            POS = POS + 2; skipws();
            let mut refret = 0;
            if POS < LEN && at(POS) == 38 { POS = POS + 1; skipws(); refret = 1;
                if POS < LEN && at(POS) == 39 { POS = POS + 1; rd_ident(); skipws(); } }
            rd_ident();
            let a0 = at(ID_O);
            if is_string_ty(ID_O, ID_L) { rk = 3; }
            if is_vec_ty(ID_O, ID_L) { rk = 2; }
            if refret == 1 { if ID_L == 3 { if a0 == 115 { if at(ID_O + 1) == 116 { if at(ID_O + 2) == 114 { rk = 1; } } } } }
            if ID_L == 4 { if a0 == 99 { if at(ID_O + 1) == 104 { if at(ID_O + 2) == 97 { if at(ID_O + 3) == 114 { rk = 4; } } } } }
            if ID_L == 3 { if a0 == 105 { if at(ID_O + 1) == 54 { if at(ID_O + 2) == 52 { rk = 5; } } } }
            if ID_L == 3 { if a0 == 117 { if at(ID_O + 1) == 54 { if at(ID_O + 2) == 52 { rk = 5; } } } }
        }
        rk
    }
}

/* R6: register each free fn's return kind (String=3 / Vec=2) so `let x = f(..)` propagates str/vec-ness */
fn fnret(o: i32, l: i32) -> i32 {
    unsafe { let mut i = 0; while i < NFRN { if span_eq_at(FNO[i as usize], FNL[i as usize], o, l) { return FRK[i as usize]; } i = i + 1; } 0 }
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
                skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
                skipws(); if POS < LEN && at(POS) == 40 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 40 { d = d + 1; } if at(POS) == 41 { d = d - 1; } POS = POS + 1; } }
                skipws();
                let rk = scan_ret_kind();
                FNO[NFRN as usize] = fo; FNL[NFRN as usize] = fl; FRK[NFRN as usize] = rk; NFRN = NFRN + 1;
            } else { POS = save; skip_tok(); }
        }
    }
}

/* pre-pass: register every `static [mut] NAME: TYPE = INIT;` and emit its `gdef NAME <bytes>` (zeroed) */
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
                    if ID_L == 2 && at(ID_O) == 117 && at(ID_O + 1) == 56 { esz = 1; } else { esz = 4; }
                    eat_ch(59); skipws();
                    let mut n = 0;
                    while POS < LEN && is_dig(at(POS)) { n = n * 10 + (at(POS) - 48); POS = POS + 1; }
                    eat_ch(93);
                    kind = 1; bytes = n * esz;
                } else { rd_ident(); esz = 4; bytes = 4; kind = 0; }
                while POS < LEN && at(POS) != 59 { POS = POS + 1; }
                eat_ch(59);
                GO[NGLOB as usize] = no; GL[NGLOB as usize] = nl; GK[NGLOB as usize] = kind; GE[NGLOB as usize] = esz;
                print!("gdef "); emit_span(no, nl);
                let mut bb = bytes; if bb < 4 { bb = 4; }
                println!(" {}", bb);
                NGLOB = NGLOB + 1;
            } else { POS = save; skip_tok(); }
        }
    }
}

fn main() {
    unsafe {
        LEN = __syscall(0, 0, &SRC, 200000);
        println!("gdef __str_arena 1048576");
        println!("gdef __str_cursor 8");
        println!("gdef __heap 1048576");
        println!("gdef __heap_cursor 8");
        prepass_structs();
        prepass_enums();
        prepass_fns();
        prepass_methods();
        prepass_generics();
        prepass_statics();
        emit_prelude();
        POS = 0;
        loop {
            skipws();
            if POS >= LEN { break; }
            let save = POS;
            let got = rd_ident();
            if got && is_kw_fn() {
                rd_ident();
                let isgen = gf_find(ID_O, ID_L) >= 0;
                POS = save;
                if isgen {                                    /* R9: generic fn -- emitted later as monomorphized instances */
                    rd_ident(); rd_ident();
                    skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
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
        let mut kc = 0;
        while kc < NCLO { emit_closure_body(kc); kc = kc + 1; }
    }
}
