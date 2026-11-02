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
static mut AV: [i32; 16] = [0; 16];       /* println! arg vNs */
static mut AS: [i32; 16] = [0; 16];       /* println! arg is-str flags */

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
static mut NFLD: i32 = 0;
static mut SST: [i32; 8192] = [0; 8192];   /* per-symbol struct type (-1 if not a struct) */
static mut EITYP: i32 = 0;                 /* struct type of the just-parsed expr (-1 if none) */

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
                    skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } }
                    FSST[NFLD as usize] = ty; FSO[NFLD as usize] = fo; FSL[NFLD as usize] = fl; FSB[NFLD as usize] = off; FSTY[NFLD as usize] = fsty; NFLD = NFLD + 1;
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

fn sym_add(o: i32, l: i32, v: i32) { unsafe { SO[NSYM as usize] = o; SL[NSYM as usize] = l; SV[NSYM as usize] = v; SK[NSYM as usize] = 0; SST[NSYM as usize] = -1; NSYM = NSYM + 1; } }
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

fn ec(v: i32) -> i32 { unsafe { EISSTR = 0; println!("  v{}: i32 = const {}", NC, v); NC = NC + 1; NC - 1 } }
fn ebin(op: &str, a: i32, b: i32) -> i32 { unsafe { EISSTR = 0; println!("  v{}: i32 = {} v{} v{}", NC, op, a, b); NC = NC + 1; NC - 1 } }
fn eref(v: i32) -> i32 { unsafe { println!("  v{}: i32 = ref v{}", NC, v); NC = NC + 1; NC - 1 } }
fn ecopy(dst: i32, src: i32) { unsafe { let z = ec(0); println!("  v{}: i32 = add v{} v{}", dst, src, z); } }

/* a call: name ( args ) -- emit `call NAME arg...` printing the name span */
fn emit_call(no: i32, nl: i32) -> i32 {
    unsafe {
        let mut a0 = -1; let mut a1 = -1; let mut a2 = -1; let mut a3 = -1;
        let mut a4 = -1; let mut a5 = -1; let mut a6 = -1; let mut a7 = -1;
        let mut na = 0;
        skipws();
        if peek() != 41 {
            a0 = parse_expr(); na = 1;
            if eat_ch(44) { a1 = parse_expr(); na = 2;
            if eat_ch(44) { a2 = parse_expr(); na = 3;
            if eat_ch(44) { a3 = parse_expr(); na = 4;
            if eat_ch(44) { a4 = parse_expr(); na = 5;
            if eat_ch(44) { a5 = parse_expr(); na = 6;
            if eat_ch(44) { a6 = parse_expr(); na = 7;
            if eat_ch(44) { a7 = parse_expr(); na = 8; } } } } } } }
        }
        eat_ch(41);
        /* __syscall intrinsic -> `syscall args` (no callee name) */
        let is_sc = nl == 9 && at(no) == 95 && at(no + 1) == 95 && at(no + 2) == 115 && at(no + 3) == 121 && at(no + 4) == 115 && at(no + 5) == 99 && at(no + 6) == 97 && at(no + 7) == 108 && at(no + 8) == 108;
        if is_sc { print!("  v{}: i32 = syscall", NC); }
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
        EITYP = ty;
        base
    }
}
/* emit `call Type_method [self] args...` (self_vn < 0 for an associated fn). pos is just past `(`. */
fn emit_mcall(ty: i32, mo: i32, ml: i32, self_vn: i32) -> i32 {
    unsafe {
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
            let v = eload(ad);
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
fn parse_atom() -> i32 {
    unsafe {
        skipws();
        EISSTR = 0; EITYP = -1;
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
        if POS < LEN && is_dig(at(POS)) {
            let mut n = 0;
            while POS < LEN && is_dig(at(POS)) { n = n * 10 + (at(POS) - 48); POS = POS + 1; }
            return ec(n);
        }
        if POS < LEN && at(POS) == 39 {                                                   /* char literal 'c' */
            POS = POS + 1; let mut cc = at(POS);
            if cc == 92 { POS = POS + 1; let e = at(POS); if e == 110 { cc = 10; } else { cc = e; } }
            POS = POS + 1; if POS < LEN && at(POS) == 39 { POS = POS + 1; }
            return ec(cc);
        }
        rd_ident();
        let o = ID_O; let l = ID_L;
        if is_kw_true() { return ec(1); }
        if is_kw_false() { return ec(0); }
        skipws();
        let ty = ty_find(o, l);
        if ty >= 0 {
            if POS < LEN && at(POS) == 123 { return struct_literal(ty); }           /* Name { .. } literal */
            if POS + 1 < LEN && at(POS) == 58 && at(POS + 1) == 58 {                 /* Name::assoc(args) */
                POS = POS + 2; skipws(); rd_ident();
                let mo = ID_O; let ml = ID_L; skipws(); eat_ch(40);
                let r = emit_mcall(ty, mo, ml, -1); EITYP = ty; return r;            /* assoc fn -- assume it returns Self */
            }
        }
        if POS < LEN && at(POS) == 40 { POS = POS + 1; return emit_call(o, l); }   /* call */
        let styp = sym_styp(o, l);
        let sv = sym_find(o, l);
        if sv >= 0 && styp >= 0 { return dispatch_struct(sv, styp); }               /* struct local/param -> field/method */
        if sv >= 0 { if sym_kind(o, l) == 1 { EISSTR = 1; } return sv; }            /* local variable (str-kind sets EISSTR) */
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
        rd_ident();
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
        let slot = sym_find(o, l); return eref(slot);                              /* &local -> ref(slot) */
    }
}
fn parse_cast() -> i32 {
    let mut a = parse_atom();
    /* `as TYPE` -> no-op in this i32 model */
    unsafe {
        skipws();
        if POS + 1 < LEN && at(POS) == 97 && at(POS + 1) == 115 && (POS + 2 >= LEN || !is_idc(at(POS + 2))) {
            POS = POS + 2; rd_ident();  /* consume the type ident */
        }
    }
    a
}
fn parse_mul() -> i32 {
    let mut a = parse_cast();
    unsafe {
        loop {
            let c = peek();
            if c == 42 { POS = POS + 1; let b = parse_cast(); a = ebin("mul", a, b); }
            else if c == 47 { POS = POS + 1; let b = parse_cast(); a = ebin("div", a, b); }
            else if c == 37 { POS = POS + 1; let b = parse_cast(); a = ebin("mod", a, b); }
            else { break; }
        }
    }
    a
}
fn parse_add() -> i32 {
    let mut a = parse_mul();
    unsafe {
        loop {
            let c = peek();
            /* do not treat -> or unary as add; here only binary + - between terms */
            if c == 43 { POS = POS + 1; let b = parse_mul(); a = ebin("add", a, b); }
            else if c == 45 { POS = POS + 1; let b = parse_mul(); a = ebin("sub", a, b); }
            else { break; }
        }
    }
    a
}
fn parse_cmp() -> i32 {
    let a = parse_add();
    unsafe {
        if eat2(61, 61) { let b = parse_add(); return ebin("eq", a, b); }
        if eat2(33, 61) { let b = parse_add(); return ebin("ne", a, b); }
        if eat2(60, 61) { let b = parse_add(); return ebin("le", a, b); }
        if eat2(62, 61) { let b = parse_add(); return ebin("ge", a, b); }
        let c = peek();
        if c == 60 { POS = POS + 1; let b = parse_add(); return ebin("lt", a, b); }
        if c == 62 { POS = POS + 1; let b = parse_add(); return ebin("gt", a, b); }
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

fn parse_stmt() {
    unsafe {
        skipws();
        let save = POS;
        if !rd_ident() {
            /* not an ident-led stmt: an expression statement or tail */
            POS = save;
            let v = parse_expr();
            skipws();
            if eat_ch(59) { let d = v; } else { println!("  ret v{}", v); }
            return;
        }
        let o = ID_O; let l = ID_L;
        if is_kw_let() {
            rd_ident(); if is_kw_mut() { rd_ident(); }   /* mut? then the name is in ID */
            /* if we consumed `mut`, ID now holds the name; else ID holds the name already */
            let no = ID_O; let nl = ID_L;
            /* optional `: TYPE` */
            skipws();
            if POS < LEN && at(POS) == 58 { POS = POS + 1; rd_ident();
                skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }
            eat_ch(61);
            let e = parse_expr();
            let ety = EITYP;
            eat_ch(59);
            let slot = NC;
            ecopy(slot, e);
            NC = NC + 1;
            sym_add(no, nl, slot);
            if ety >= 0 { SST[(NSYM - 1) as usize] = ety; }   /* R1: struct-typed local */
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
            parse_block();
            println!("  cont");
            println!("  jmp");
            println!("  endloop");
            return;
        }
        if is_kw_loop() {
            println!("  loop");
            let one = ec(1);
            println!("  brz v{}", one);
            parse_block();
            println!("  cont");
            println!("  jmp");
            println!("  endloop");
            return;
        }
        if is_kw_return() {
            skipws();
            if POS < LEN && at(POS) == 59 { let z = ec(0); POS = POS + 1; println!("  ret v{}", z); }
            else { let v = parse_expr(); eat_ch(59); println!("  ret v{}", v); }
            return;
        }
        if is_kw_break() { eat_ch(59); println!("  brk"); return; }
        if is_kw_unsafe() { parse_block(); return; }
        if is_print() || is_println() {
            let nlf = is_println();
            eat_ch(33); eat_ch(40); skipws(); eat_ch(34);
            let fo = POS;
            while POS < LEN && at(POS) != 34 { if at(POS) == 92 { POS = POS + 1; } POS = POS + 1; }
            let fl = POS - fo; POS = POS + 1;
            let mut na = 0;
            skipws();
            while eat_ch(44) { skipws(); if peek() == 41 { break; } let a = parse_expr(); AV[na as usize] = a; AS[na as usize] = EISSTR; na = na + 1; }
            eat_ch(41); eat_ch(59);
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
            return;
        }
        skipws();
        /* struct field write `NAME.field... = EXPR ;` (lookahead for a top-level `=` distinguishes it
           from a method-call statement `p.m(args);`) */
        if sym_styp(o, l) >= 0 && POS < LEN && at(POS) == 46 {
            let mut q = POS; let mut depth = 0; let mut isasg = 0;
            while q < LEN {
                let ch = at(q);
                if ch == 40 || ch == 91 || ch == 123 { depth = depth + 1; }
                else if ch == 41 || ch == 93 || ch == 125 { if depth == 0 { q = LEN; } else { depth = depth - 1; } }
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
            POS = save; let v = parse_expr(); skipws(); if eat_ch(59) { let d = v; } else { println!("  ret v{}", v); }
            return;
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
            POS = save; let v = parse_expr(); skipws(); if eat_ch(59) { let d = v; } else { println!("  ret v{}", v); } return;
        }
        /* NAME = EXPR ;  (global scalar or local) */
        if POS < LEN && at(POS) == 61 && (POS + 1 >= LEN || at(POS + 1) != 61) {
            POS = POS + 1; let e = parse_expr(); eat_ch(59);
            let gi = g_find(o, l);
            if gi >= 0 { let base = egref(o, l); println!("  store v{} v{}", base, e); }
            else { let slot = sym_find(o, l); let a = eref(slot); println!("  store v{} v{}", a, e); }
            return;
        }
        /* expression statement (e.g. a call) -- rewind and parse */
        POS = save;
        let v = parse_expr();
        skipws();
        if eat_ch(59) { let d = v; } else { println!("  ret v{}", v); }
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
                let mut pis_str = 0; let mut psty = -1;
                rd_ident();
                let po = ID_O; let pl = ID_L;
                skipws();
                if POS < LEN && at(POS) == 58 { POS = POS + 1;
                    skipws(); if POS < LEN && at(POS) == 38 { POS = POS + 1; if is_kw_mut_next() { } }
                    rd_ident();
                    if ID_L == 3 && at(ID_O) == 115 && at(ID_O + 1) == 116 && at(ID_O + 2) == 114 { pis_str = 1; }
                    psty = ty_find(ID_O, ID_L);
                    skipws(); if POS < LEN && at(POS) == 60 { let mut d = 1; POS = POS + 1; while POS < LEN && d > 0 { if at(POS) == 60 { d = d + 1; } if at(POS) == 62 { d = d - 1; } POS = POS + 1; } } }
                println!("  v{}: i32 = param", NC);
                sym_add(po, pl, NC);
                if pis_str == 1 { SK[(NSYM - 1) as usize] = 1; }
                if psty >= 0 { SST[(NSYM - 1) as usize] = psty; }
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
        prepass_statics();
        emit_prelude();
        POS = 0;
        loop {
            skipws();
            if POS >= LEN { break; }
            let save = POS;
            let got = rd_ident();
            if got && is_kw_fn() { POS = save; parse_fn(); }
            else if got && is_kw_impl() { parse_impl(); }
            else { POS = save; skip_tok(); }
        }
    }
}
