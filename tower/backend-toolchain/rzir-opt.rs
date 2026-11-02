// rzir-opt.rs -- the rzir optimizer, IN RUST (port of rzir-opt.c). Per-function const-fold (i32 wraparound;
// unsigned emission) + zero-idiom copy-prop + dead-value cascade + cmp/brz fusion, iterated to a fixpoint.
// Identity floor: a stream with nothing to do emits byte-identically. Stays in the chain's compile subset:
// i32 bitwise (&|^<<>>) is used directly (rx.self supports it); the two UNSIGNED cases -- logical shift `shu`
// and unsigned-decimal const printing -- go through an i64 low-32 mask (the masked value is positive, so
// arithmetic >> == logical). C's char* keyword literals become byte-code matchers (<=6 params: the ABI has
// only 6 arg registers). subst[] is explicitly -1 initialized (0-init would read as "substitute to v0").
// C-OPT-BUF-8MB-OVERFLOW: twin of rzir-opt.c; see the note there. This side is the BINDING constraint --
// it is minted by the M0 audit-floor cf, and 10M/11M/12M hang, diverge or segfault. All three move together.
static mut SRC: [u8; 157286400] = [0; 157286400];
static mut SLEN: i32 = 0;
static mut OUT: [u8; 157286400] = [0; 157286400];
static mut OUTN: i32 = 0;
static mut SCR: [u8; 157286400] = [0; 157286400];
static mut SCRN: i32 = 0;
static mut SRCOVF: i32 = 0;   // input-overflow guard: input exceeds SRC -> refuse (loud stub), never silent OOB corruption
static mut OUTOVF: i32 = 0;   // output/scratch-overflow guard: emit would exceed OUT/SCR -> refuse

static mut GLOFF: [i32; 6000000] = [0; 6000000];
static mut GLLEN: [i32; 6000000] = [0; 6000000];
static mut GNL: i32 = 0;
static mut LOFF: [i32; 6000000] = [0; 6000000];    // working window (one function)
static mut LLEN: [i32; 6000000] = [0; 6000000];
static mut NL: i32 = 0;
static mut LSRC: [i32; 6000000] = [0; 6000000];    // 0 = SRC buffer, 1 = SCR buffer
static mut DROPF: [i32; 6000000] = [0; 6000000];   // line dropped
// R1b per-line def cache: analyze() parses each live line's def ONCE (def_of is the heaviest tokenizer --
// full-line eq_pos scan + ~24 word_at matches); FOLD/COPY-PROP/FUSION/recount then read these instead of
// re-calling def_of on the same unchanged content. Valid because opt_round always analyze()s before the
// consuming passes, and line CONTENT only changes via a rewrite-then-return (repopulated next round).
static mut DVL: [i32; 6000000] = [0; 6000000];     // cached def value (-1 if not a def line)
static mut OPL: [i32; 6000000] = [0; 6000000];     // cached GOP
static mut ARL: [i32; 6000000] = [0; 6000000];     // cached GARITH
static mut CML: [i32; 6000000] = [0; 6000000];     // cached GCMP

static mut DEFS: [i32; 40000] = [0; 40000];      // first def line, -1
static mut DCOUNT: [i32; 40000] = [0; 40000];
static mut OKPURE: [i32; 40000] = [0; 40000];
static mut ISCONST: [i32; 40000] = [0; 40000];
static mut CVAL: [i32; 40000] = [0; 40000];
static mut USEC: [i32; 40000] = [0; 40000];
static mut USELN: [i32; 40000] = [0; 40000];
static mut ATAKEN: [i32; 40000] = [0; 40000];
static mut SUBST: [i32; 40000] = [0; 40000];
static mut MAXV: i32 = 0;

static mut GNUM: i32 = 0;
static mut GEND: i32 = 0;
static mut GOP: i32 = 0;       // 0 none, 1 const, 2 arith, 3 cmp, 4 ref, 5 other
static mut GARITH: i32 = 0;    // 1 add 2 sub 3 mul 4 and 5 or 6 xor 7 shl 8 shr 9 shu
static mut GCMP: i32 = 0;      // 1 lt 2 le 3 gt 4 ge 5 eq 6 ne 7 ltu 8 gtu 9 leu 10 geu
static mut GFOLD: i32 = 0;
static mut GFOLDOK: i32 = 0;
static mut GA: i32 = 0;
static mut GB: i32 = 0;
static mut NBUF: [i32; 16] = [0; 16];

fn read_all() -> i32 { unsafe { let mut total = 0; let mut n = __syscall(0, 0, &SRC[0], 65536); while n > 0 { total = total + n; if total + 65536 > 157286400 { SRCOVF = 1; n = 0; } else { n = __syscall(0, 0, &SRC[total as usize], 65536); } } total } }

fn isal(c: i32) -> i32 { if c >= 97 && c <= 122 { return 1; } if c >= 65 && c <= 90 { return 1; } if c >= 48 && c <= 57 { return 1; } if c == 95 { return 1; } 0 }

// char at working-window line i position p; virtual newline (10) past the line end so tokens never run into
// STALE scratch from a prior function.
fn lchar(i: i32, p: i32) -> i32 { unsafe { if p >= LLEN[i as usize] { return 10; } if LSRC[i as usize] == 1 { return SCR[(LOFF[i as usize] + p) as usize] as i32; } SRC[(LOFF[i as usize] + p) as usize] as i32 } }

fn skipsp(i: i32, p: i32) -> i32 { let mut q = p; while lchar(i, q) == 32 || lchar(i, q) == 9 { q = q + 1; } q }

// word by byte codes b0..b3 (0 terminates; <=4 chars) at line i pos p, with a word boundary after
fn word_at(i: i32, p: i32, b0: i32, b1: i32, b2: i32, b3: i32) -> i32 {
    if b0 != 0 && lchar(i, p) != b0 { return 0; }
    if b1 != 0 && lchar(i, p + 1) != b1 { return 0; }
    if b2 != 0 && lchar(i, p + 2) != b2 { return 0; }
    if b3 != 0 && lchar(i, p + 3) != b3 { return 0; }
    let mut n = 0;
    if b0 != 0 { n = 1; } if b1 != 0 { n = 2; } if b2 != 0 { n = 3; } if b3 != 0 { n = 4; }
    if isal(lchar(i, p + n)) == 1 { return 0; }
    1
}

// the one 5-char keyword ("const") -- can't fit in word_at's 4 byte codes without exceeding 6 params
fn word_const(i: i32, p: i32) -> i32 {
    if lchar(i, p) != 99 { return 0; }
    if lchar(i, p + 1) != 111 { return 0; }
    if lchar(i, p + 2) != 110 { return 0; }
    if lchar(i, p + 3) != 115 { return 0; }
    if lchar(i, p + 4) != 116 { return 0; }
    if isal(lchar(i, p + 5)) == 1 { return 0; }
    1
}

fn parse_num(i: i32, p: i32) { unsafe { GNUM = 0; let mut q = p; while lchar(i, q) >= 48 && lchar(i, q) <= 57 { GNUM = GNUM * 10 + (lchar(i, q) - 48); q = q + 1; } GEND = q; } }

fn eq_pos(i: i32) -> i32 { unsafe { let mut p = 0; while p < LLEN[i as usize] { if lchar(i, p) == 61 { return p; } p = p + 1; } 0 - 1 } }

fn line_is_ir(i: i32) -> i32 { let p = skipsp(i, 0); word_at(i, p, 105, 114, 0, 0) }        // ir
fn line_is_fn(i: i32) -> i32 { let p = skipsp(i, 0); word_at(i, p, 102, 110, 0, 0) }        // fn
fn line_is_brz(i: i32) -> i32 { let p = skipsp(i, 0); word_at(i, p, 98, 114, 122, 0) }      // brz

// def line vN : ... = OP; returns vN or -1; sets GOP/GARITH/GCMP
fn def_of(i: i32) -> i32 { unsafe {
    let mut p = skipsp(i, 0);
    if lchar(i, p) != 118 { return 0 - 1; }
    if lchar(i, p + 1) < 48 || lchar(i, p + 1) > 57 { return 0 - 1; }
    parse_num(i, p + 1); let v = GNUM; p = GEND;
    if lchar(i, p) != 58 { return 0 - 1; }
    p = eq_pos(i); if p < 0 { return 0 - 1; }
    p = skipsp(i, p + 1);
    GOP = 5; GARITH = 0; GCMP = 0;
    if word_const(i, p) == 1 { GOP = 1; }
    if word_at(i, p, 114, 101, 102, 0) == 1 { GOP = 4; }                                     // ref
    if word_at(i, p, 97, 100, 100, 0) == 1 { GOP = 2; GARITH = 1; }                          // add
    if word_at(i, p, 115, 117, 98, 0) == 1 { GOP = 2; GARITH = 2; }                          // sub
    if word_at(i, p, 109, 117, 108, 0) == 1 { GOP = 2; GARITH = 3; }                         // mul
    if word_at(i, p, 97, 110, 100, 0) == 1 { GOP = 2; GARITH = 4; }                          // and
    if word_at(i, p, 111, 114, 0, 0) == 1 { GOP = 2; GARITH = 5; }                           // or
    if word_at(i, p, 120, 111, 114, 0) == 1 { GOP = 2; GARITH = 6; }                         // xor
    if word_at(i, p, 115, 104, 108, 0) == 1 { GOP = 2; GARITH = 7; }                         // shl
    if word_at(i, p, 115, 104, 114, 0) == 1 { GOP = 2; GARITH = 8; }                         // shr
    if word_at(i, p, 115, 104, 117, 0) == 1 { GOP = 2; GARITH = 9; }                         // shu
    if word_at(i, p, 108, 116, 0, 0) == 1 { GOP = 3; GCMP = 1; }                             // lt
    if word_at(i, p, 108, 101, 0, 0) == 1 { GOP = 3; GCMP = 2; }                             // le
    if word_at(i, p, 103, 116, 0, 0) == 1 { GOP = 3; GCMP = 3; }                             // gt
    if word_at(i, p, 103, 101, 0, 0) == 1 { GOP = 3; GCMP = 4; }                             // ge
    if word_at(i, p, 101, 113, 0, 0) == 1 { GOP = 3; GCMP = 5; }                             // eq
    if word_at(i, p, 110, 101, 0, 0) == 1 { GOP = 3; GCMP = 6; }                             // ne
    if word_at(i, p, 108, 116, 117, 0) == 1 { GOP = 3; GCMP = 7; }                           // ltu
    if word_at(i, p, 103, 116, 117, 0) == 1 { GOP = 3; GCMP = 8; }                           // gtu
    if word_at(i, p, 108, 101, 117, 0) == 1 { GOP = 3; GCMP = 9; }                           // leu
    if word_at(i, p, 103, 101, 117, 0) == 1 { GOP = 3; GCMP = 10; }                          // geu
    v
} }

// first two vN operands after '='; returns count, sets GA/GB
fn rhs_ops(i: i32, dv: i32) -> i32 { unsafe {
    let mut p = eq_pos(i); if p < 0 { return 0; }
    p = p + 1; let mut n = 0;
    while p < LLEN[i as usize] {
        if lchar(i, p) == 118 && lchar(i, p + 1) >= 48 && lchar(i, p + 1) <= 57 {
            let mut prev = 0; if p > 0 { prev = isal(lchar(i, p - 1)); }
            if prev == 0 {
                parse_num(i, p + 1);
                if isal(lchar(i, GEND)) == 0 {
                    if n == 0 { GA = GNUM; } if n == 1 { GB = GNUM; }
                    n = n + 1; p = GEND;
                } else { p = p + 1; }
            } else { p = p + 1; }
        } else { p = p + 1; }
    }
    n
} }

// unsigned < via sign flip (i32 wraparound matches the C trick)
fn ult(a: i32, b: i32) -> i32 { let x = a + (0 - 2147483647 - 1); let y = b + (0 - 2147483647 - 1); if x < y { return 1; } 0 }

// fold GARITH(isar=1)/GCMP(isar=0) on a,b -> GFOLD; GFOLDOK=0 when unsafe
fn fold1(isar: i32, op: i32, a: i32, b: i32) { unsafe {
    GFOLDOK = 1;
    if isar == 1 {
        if op == 1 { GFOLD = a + b; return; }
        if op == 2 { GFOLD = a - b; return; }
        if op == 3 { GFOLD = a * b; return; }
        if op == 4 { GFOLD = a & b; return; }
        if op == 5 { GFOLD = a | b; return; }
        if op == 6 { GFOLD = a ^ b; return; }
        if b < 0 || b > 31 { GFOLDOK = 0; return; }
        if op == 7 { GFOLD = a << b; return; }
        if op == 8 { GFOLD = a >> b; return; }
        if op == 9 { let u: i64 = (a as i64) & 4294967295; GFOLD = (u >> b) as i32; return; }   // logical shr via low-32 mask (positive)
        GFOLDOK = 0; return;
    }
    if op == 1 { if a < b { GFOLD = 1; } else { GFOLD = 0; } return; }
    if op == 2 { if a <= b { GFOLD = 1; } else { GFOLD = 0; } return; }
    if op == 3 { if a > b { GFOLD = 1; } else { GFOLD = 0; } return; }
    if op == 4 { if a >= b { GFOLD = 1; } else { GFOLD = 0; } return; }
    if op == 5 { if a == b { GFOLD = 1; } else { GFOLD = 0; } return; }
    if op == 6 { if a != b { GFOLD = 1; } else { GFOLD = 0; } return; }
    if op == 7 { GFOLD = ult(a, b); return; }
    if op == 8 { GFOLD = ult(b, a); return; }
    if op == 9 { if ult(b, a) == 1 { GFOLD = 0; } else { GFOLD = 1; } return; }
    if op == 10 { if ult(a, b) == 1 { GFOLD = 0; } else { GFOLD = 1; } return; }
    GFOLDOK = 0;
} }

fn sc(c: i32) { unsafe { if SCRN + 1 >= 157286400 { OUTOVF = 1; return; } SCR[SCRN as usize] = c as u8; SCRN = SCRN + 1; } }

// unsigned decimal of an i32 bit pattern, via a positive i64 low-32 mask
fn scu(vv: i32) { unsafe {
    let mut u: i64 = (vv as i64) & 4294967295;
    if u == 0 { sc(48); return; }
    let mut t = 0;
    loop { let r = u - u / 10 * 10; NBUF[t as usize] = r as i32; t = t + 1; u = u / 10; if u == 0 { break; } }
    while t > 0 { t = t - 1; sc(48 + NBUF[t as usize]); }
} }

// scratch-emit "  vDV: i32 = const V" and repoint line i at it
fn rw_const(i: i32, dv: i32, v: i32) { unsafe {
    let st = SCRN;
    sc(32); sc(32); sc(118); scu(dv);                                   // "  vDV"
    sc(58); sc(32); sc(105); sc(51); sc(50); sc(32); sc(61); sc(32);    // ": i32 = "
    sc(99); sc(111); sc(110); sc(115); sc(116); sc(32); scu(v);         // "const V"
    LOFF[i as usize] = st; LLEN[i as usize] = SCRN - st; LSRC[i as usize] = 1;
} }

fn canon(v0: i32) -> i32 { unsafe { let mut v = v0; while v < 40000 && SUBST[v as usize] >= 0 { v = SUBST[v as usize]; } v } }

// emit working-window line i to OUT, canon-rewriting vN tokens; adds newline
fn emit_line(i: i32) { unsafe {
    if OUTN + LLEN[i as usize] + 64 >= 157286400 { OUTOVF = 1; return; }
    let mut p = 0;
    while p < LLEN[i as usize] {
        let c = lchar(i, p);
        if c == 118 && lchar(i, p + 1) >= 48 && lchar(i, p + 1) <= 57 {
            let mut prev = 0; if p > 0 { prev = isal(lchar(i, p - 1)); }
            if prev == 0 {
                parse_num(i, p + 1);
                if isal(lchar(i, GEND)) == 0 {
                    OUT[OUTN as usize] = 118; OUTN = OUTN + 1;
                    let mut q = canon(GNUM); let mut t = 0;
                    loop { let r = q - q / 10 * 10; NBUF[t as usize] = r; t = t + 1; q = q / 10; if q == 0 { break; } }
                    while t > 0 { t = t - 1; OUT[OUTN as usize] = (48 + NBUF[t as usize]) as u8; OUTN = OUTN + 1; }
                    p = GEND;
                } else { OUT[OUTN as usize] = c as u8; OUTN = OUTN + 1; p = p + 1; }
            } else { OUT[OUTN as usize] = c as u8; OUTN = OUTN + 1; p = p + 1; }
        } else { OUT[OUTN as usize] = c as u8; OUTN = OUTN + 1; p = p + 1; }
    }
    OUT[OUTN as usize] = 10; OUTN = OUTN + 1;
} }

// analyze the working window: fill per-value tables (subst is PERSISTENT across rounds -- reset once per fn)
fn analyze() { unsafe {
    let mut v = 0; while v <= MAXV { DEFS[v as usize] = 0 - 1; DCOUNT[v as usize] = 0; OKPURE[v as usize] = 0; ISCONST[v as usize] = 0; USEC[v as usize] = 0; USELN[v as usize] = 0 - 1; ATAKEN[v as usize] = 0; v = v + 1; }
    MAXV = 0;
    let mut i = 0;
    while i < NL {
        if DROPF[i as usize] == 0 {
            let dv = def_of(i);
            DVL[i as usize] = dv; OPL[i as usize] = GOP; ARL[i as usize] = GARITH; CML[i as usize] = GCMP;  // R1b: cache this line's def parse
            let mut isref = 0;
            if dv >= 0 && dv < 40000 {
                if dv > MAXV { MAXV = dv; }
                if DEFS[dv as usize] < 0 { DEFS[dv as usize] = i; if GOP < 5 { OKPURE[dv as usize] = 1; } }
                DCOUNT[dv as usize] = DCOUNT[dv as usize] + 1;
                if GOP == 4 { isref = 1; }
                if GOP == 1 && DCOUNT[dv as usize] == 1 {
                    let mut p = eq_pos(i); p = skipsp(i, p + 1); p = p + 5; p = skipsp(i, p);
                    let mut sg = 1; if lchar(i, p) == 45 { sg = 0 - 1; p = p + 1; }
                    parse_num(i, p); ISCONST[dv as usize] = 1; CVAL[dv as usize] = sg * GNUM;
                }
            }
            let mut p = eq_pos(i); if p < 0 { p = 0; } else { p = p + 1; }
            while p < LLEN[i as usize] {
                if lchar(i, p) == 118 && lchar(i, p + 1) >= 48 && lchar(i, p + 1) <= 57 {
                    let mut prev = 0; if p > 0 { prev = isal(lchar(i, p - 1)); }
                    if prev == 0 {
                        parse_num(i, p + 1);
                        if isal(lchar(i, GEND)) == 0 {
                            let vv = GNUM;
                            if vv < 40000 && vv != dv {
                                USEC[vv as usize] = USEC[vv as usize] + 1; USELN[vv as usize] = i;
                                if vv > MAXV { MAXV = vv; }
                                if isref == 1 { ATAKEN[vv as usize] = 1; }
                            }
                            p = GEND;
                        } else { p = p + 1; }
                    } else { p = p + 1; }
                } else { p = p + 1; }
            }
        }
        i = i + 1;
    }
    v = 0; while v <= MAXV { if DCOUNT[v as usize] != 1 || ATAKEN[v as usize] == 1 { OKPURE[v as usize] = 0; ISCONST[v as usize] = 0; } v = v + 1; }
} }

// recount uses over live lines with canon applied (for DVE)
fn recount() { unsafe {
    let mut v = 0; while v <= MAXV { USEC[v as usize] = 0; v = v + 1; }
    let mut i = 0;
    while i < NL {
        if DROPF[i as usize] == 0 {
            let dv = DVL[i as usize];                    // R1b: cached def value (was def_of(i))
            let mut p = eq_pos(i); if p < 0 { p = 0; } else { p = p + 1; }
            while p < LLEN[i as usize] {
                if lchar(i, p) == 118 && lchar(i, p + 1) >= 48 && lchar(i, p + 1) <= 57 {
                    let mut prev = 0; if p > 0 { prev = isal(lchar(i, p - 1)); }
                    if prev == 0 {
                        parse_num(i, p + 1);
                        if isal(lchar(i, GEND)) == 0 {
                            let vv = canon(GNUM);
                            if vv < 40000 && vv != dv { USEC[vv as usize] = USEC[vv as usize] + 1; }
                            p = GEND;
                        } else { p = p + 1; }
                    } else { p = p + 1; }
                } else { p = p + 1; }
            }
        }
        i = i + 1;
    }
} }

// one round; returns 1 if anything changed
fn opt_round() -> i32 { unsafe {
    analyze();
    let mut changed = 0;
    let mut i = 0;                                   // FOLD
    while i < NL {
        if DROPF[i as usize] == 0 {
            let dv = DVL[i as usize]; GOP = OPL[i as usize]; GARITH = ARL[i as usize]; GCMP = CML[i as usize];  // R1b: cached def parse
            if dv >= 0 && dv < 40000 && OKPURE[dv as usize] == 1 && ISCONST[dv as usize] == 0 && (GOP == 2 || GOP == 3) {
                let mut sar = 0; let mut sop = GCMP; if GOP == 2 { sar = 1; sop = GARITH; }
                let n = rhs_ops(i, dv);
                if n == 2 && GA < 40000 && GB < 40000 && ISCONST[GA as usize] == 1 && ISCONST[GB as usize] == 1 {
                    fold1(sar, sop, CVAL[GA as usize], CVAL[GB as usize]);
                    if GFOLDOK == 1 { rw_const(i, dv, GFOLD); changed = 1; }
                }
            }
        }
        i = i + 1;
    }
    if changed == 1 { return 1; }
    let mut did = 0;
    i = 0;                                           // COPY-PROP
    while i < NL {
        if DROPF[i as usize] == 0 {
            let dv = DVL[i as usize]; GOP = OPL[i as usize]; GARITH = ARL[i as usize]; GCMP = CML[i as usize];  // R1b: cached def parse
            if dv >= 0 && dv < 40000 && OKPURE[dv as usize] == 1 && GOP == 2 && GARITH == 1 && SUBST[dv as usize] < 0 {
                let n = rhs_ops(i, dv);
                if n == 2 && GA < 40000 && GB < 40000 {
                    let mut z = 0; let mut x = 0 - 1;
                    if ISCONST[GA as usize] == 1 && CVAL[GA as usize] == 0 { z = z + 1; x = GB; }
                    if ISCONST[GB as usize] == 1 && CVAL[GB as usize] == 0 { z = z + 1; x = GA; }
                    if z == 1 && x >= 0 {
                        if OKPURE[x as usize] == 1 || ISCONST[x as usize] == 1 { SUBST[dv as usize] = x; DROPF[i as usize] = 1; did = 1; }
                    }
                }
            }
        }
        i = i + 1;
    }
    let mut grew = 1;                                // DVE cascade
    while grew == 1 { grew = 0;
        recount();
        let mut v = 0;
        while v <= MAXV {
            if OKPURE[v as usize] == 1 && SUBST[v as usize] < 0 && USEC[v as usize] == 0 && DEFS[v as usize] >= 0 {
                if DROPF[DEFS[v as usize] as usize] == 0 { DROPF[DEFS[v as usize] as usize] = 1; grew = 1; did = 1; }
            }
            v = v + 1;
        }
    }
    if did == 1 { return 1; }
    analyze();                                       // FUSION (quiescent round)
    i = 0;
    while i < NL - 1 {
        if DROPF[i as usize] == 0 {
            let dv = DVL[i as usize]; GOP = OPL[i as usize]; GARITH = ARL[i as usize]; GCMP = CML[i as usize];  // R1b: cached def parse
            if dv >= 0 && dv < 40000 && OKPURE[dv as usize] == 1 && GOP == 3 && USEC[dv as usize] == 1 && USELN[dv as usize] == i + 1 {
                if DROPF[(i + 1) as usize] == 0 && line_is_brz(i + 1) == 1 {
                    let sop = GCMP;
                    let n = rhs_ops(i, dv);
                    if n == 2 {
                        let st = SCRN;
                        sc(32); sc(32); sc(98); sc(114); sc(99); sc(32);                 // "  brc "
                        if sop == 1 { sc(108); sc(116); }                               // lt
                        if sop == 2 { sc(108); sc(101); }                               // le
                        if sop == 3 { sc(103); sc(116); }                               // gt
                        if sop == 4 { sc(103); sc(101); }                               // ge
                        if sop == 5 { sc(101); sc(113); }                               // eq
                        if sop == 6 { sc(110); sc(101); }                               // ne
                        if sop == 7 { sc(108); sc(116); sc(117); }                      // ltu
                        if sop == 8 { sc(103); sc(116); sc(117); }                      // gtu
                        if sop == 9 { sc(108); sc(101); sc(117); }                      // leu
                        if sop == 10 { sc(103); sc(101); sc(117); }                     // geu
                        sc(32); sc(118); scu(GA); sc(32); sc(118); scu(GB);
                        LOFF[i as usize] = st; LLEN[i as usize] = SCRN - st; LSRC[i as usize] = 1;
                        DROPF[(i + 1) as usize] = 1; changed = 1;
                    }
                }
            }
        }
        i = i + 1;
    }
    changed
} }

fn do_func(fs: i32, fe: i32) { unsafe {
    NL = 0; SCRN = 0; MAXV = 39999;
    let mut i = 0; while i < 40000 { SUBST[i as usize] = 0 - 1; i = i + 1; }
    i = fs;
    while i < fe { LOFF[NL as usize] = GLOFF[i as usize]; LLEN[NL as usize] = GLLEN[i as usize]; LSRC[NL as usize] = 0; DROPF[NL as usize] = 0; NL = NL + 1; i = i + 1; }
    let mut r = 1;
    while r == 1 { r = opt_round(); }
    // NB: no analyze() here -- emit_line only reads canon()/SUBST + DROPF, never the analyze tables
    // (DEFS/OKPURE/USEC/...), so a pre-emit analyze is dead work. Removing it drops ~1 analyze/fn
    // (~20% of total analyze calls) with byte-identical output. (backend-quality R1a)
    i = 0;
    while i < NL { if DROPF[i as usize] == 0 { emit_line(i); } i = i + 1; }
} }

fn emit_global(j: i32) { unsafe {
    NL = 1; LOFF[0] = GLOFF[j as usize]; LLEN[0] = GLLEN[j as usize]; LSRC[0] = 0; DROPF[0] = 0;
    emit_line(0);
} }

fn main() { unsafe {
    SLEN = read_all(); OUTN = 0; MAXV = 39999;
    let mut i = 0; while i < 40000 { SUBST[i as usize] = 0 - 1; i = i + 1; }
    GNL = 0; i = 0; let mut st = 0;
    while i < SLEN { if SRC[i as usize] == 10 { GLOFF[GNL as usize] = st; GLLEN[GNL as usize] = i - st; GNL = GNL + 1; st = i + 1; } i = i + 1; }
    if st < SLEN { GLOFF[GNL as usize] = st; GLLEN[GNL as usize] = SLEN - st; GNL = GNL + 1; }
    let mut infunc = 0; let mut fs = 0; i = 0;
    while i < GNL {
        NL = 1; LOFF[0] = GLOFF[i as usize]; LLEN[0] = GLLEN[i as usize]; LSRC[0] = 0; DROPF[0] = 0;
        let mut hdr = 0;
        if line_is_ir(0) == 1 { hdr = 1; }
        if line_is_fn(0) == 1 { hdr = 1; }
        if hdr == 1 {
            if infunc == 1 { do_func(fs, i); }
            else { let mut j = 0; while j < i { emit_global(j); j = j + 1; } }
            infunc = 1; fs = i;
        }
        i = i + 1;
    }
    if infunc == 1 { do_func(fs, GNL); }
    else { let mut j = 0; while j < GNL { emit_global(j); j = j + 1; } }
    if SRCOVF == 0 && OUTOVF == 0 { __syscall(1, 1, &OUT[0], OUTN); }
} }
