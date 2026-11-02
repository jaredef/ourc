// rzir-ra.rs -- the rzir register allocator, IN RUST (port of rzir-ra.c). Per-function linear scan over two
// pools (callee-saved r12/r13 for across-call values; caller-saved r10/r11 for the rest) + marshalling regs
// (rsi/rdi/r8/r9), honored class = ARITH+CMP defs/uses + brz/brc/ret/store consumers, back-edge-aware
// intervals (loop/jmp AND backward goto), mutable-value allocation, FIFO free queues and (end,regname,value)
// expiry ordering to match the oracle exactly. Emits `@rNN ` on every def line of an assigned value.
// No bitwise ops. char* keyword literals -> byte-code matchers (word4 for <=4 chars + dedicated matchers for
// the 5-7 char keywords const/store/storeb/label/syscall); every fn <=6 params (the ABI has 6 arg registers).
static mut SRC: [u8; 157286400] = [0; 157286400];  // C-RA-BUF-8MB-OVERFLOW: twin of rzir-ra.c; see the note there
static mut SLEN: i32 = 0;
static mut OUT: [u8; 157286400] = [0; 157286400];
static mut OUTN: i32 = 0;
static mut OUTOVF: i32 = 0;   // C-RA-BUF-OVERFLOW guard: set when OUT would exceed 157286400; refuse (emit nothing -> loud downstream stub) instead of the silent OOB corruption that produced the mysterious ceiling SEGV
static mut SRCOVF: i32 = 0;   // C-RA-BUF-OVERFLOW guard (input side, mirrors cf BUFOVF): set when the INPUT would exceed SRC[157286400]; refuse rather than read OOB

static mut LOFF: [i32; 6000000] = [0; 6000000];
static mut LLEN: [i32; 6000000] = [0; 6000000];
static mut NL: i32 = 0;
static mut CUMCALL: [i32; 6000000] = [0; 6000000];

static mut DEFS: [i32; 40000] = [0; 40000];
static mut LU: [i32; 40000] = [0; 40000];
static mut OKDEF: [i32; 40000] = [0; 40000];
static mut BADUSE: [i32; 40000] = [0; 40000];
static mut AREG: [i32; 40000] = [0; 40000];
static mut ARGUSE: [i32; 40000] = [0; 40000];
static mut MAXV: i32 = 0;

static mut RGS: [i32; 256] = [0; 256];
static mut RGE: [i32; 256] = [0; 256];
static mut NRG: i32 = 0;
static mut LSTK: [i32; 64] = [0; 64];
static mut NLSTK: i32 = 0;
static mut LBO: [i32; 256] = [0; 256];
static mut LBL: [i32; 256] = [0; 256];
static mut LBLINE: [i32; 256] = [0; 256];
static mut NLBL: i32 = 0;

static mut FCS: [i32; 8] = [0; 8]; static mut FCSH: i32 = 0; static mut FCST: i32 = 0;
static mut FCC: [i32; 8] = [0; 8]; static mut FCCH: i32 = 0; static mut FCCT: i32 = 0;
static mut FCM: [i32; 8] = [0; 8]; static mut FCMH: i32 = 0; static mut FCMT: i32 = 0;
static mut AE: [i32; 8] = [0; 8]; static mut AV: [i32; 8] = [0; 8]; static mut AR: [i32; 8] = [0; 8]; static mut NA: i32 = 0;

static mut GNUM: i32 = 0;
static mut GEND: i32 = 0;
static mut GDEFOP: i32 = 0;
static mut GCONS: i32 = 0;
static mut GCNT: i32 = 0;
static mut GNOFF: i32 = 0;
static mut GNLEN: i32 = 0;

fn sb(p: i32) -> i32 { unsafe { SRC[p as usize] as i32 } }

fn read_all() -> i32 { unsafe { let mut total = 0; let mut n = __syscall(0, 0, &SRC[0], 65536); while n > 0 { total = total + n; if total + 65536 > 157286400 { SRCOVF = 1; n = 0; } else { n = __syscall(0, 0, &SRC[total as usize], 65536); } } total } }

fn isal(c: i32) -> i32 { if c >= 97 && c <= 122 { return 1; } if c >= 65 && c <= 90 { return 1; } if c >= 48 && c <= 57 { return 1; } if c == 95 { return 1; } 0 }

fn splitlines() { unsafe { NL = 0; let mut i = 0; let mut st = 0;
    while i < SLEN { if SRC[i as usize] == 10 { LOFF[NL as usize] = st; LLEN[NL as usize] = i - st; NL = NL + 1; st = i + 1; } i = i + 1; }
    if st < SLEN { LOFF[NL as usize] = st; LLEN[NL as usize] = SLEN - st; NL = NL + 1; } } }

fn skipsp(p: i32) -> i32 { let mut q = p; while sb(q) == 32 || sb(q) == 9 { q = q + 1; } q }

// whole-word match of byte codes b0..b3 (0 terminates, <=4 chars) at absolute src pos p, boundary after
fn word4(p: i32, b0: i32, b1: i32, b2: i32, b3: i32) -> i32 {
    if b0 != 0 && sb(p) != b0 { return 0; }
    if b1 != 0 && sb(p + 1) != b1 { return 0; }
    if b2 != 0 && sb(p + 2) != b2 { return 0; }
    if b3 != 0 && sb(p + 3) != b3 { return 0; }
    let mut n = 0;
    if b0 != 0 { n = 1; } if b1 != 0 { n = 2; } if b2 != 0 { n = 3; } if b3 != 0 { n = 4; }
    if isal(sb(p + n)) == 1 { return 0; }
    1
}
fn word_const(p: i32) -> i32 { if sb(p) != 99 || sb(p + 1) != 111 || sb(p + 2) != 110 || sb(p + 3) != 115 || sb(p + 4) != 116 { return 0; } if isal(sb(p + 5)) == 1 { return 0; } 1 }
fn word_store(p: i32) -> i32 { if sb(p) != 115 || sb(p + 1) != 116 || sb(p + 2) != 111 || sb(p + 3) != 114 || sb(p + 4) != 101 { return 0; } if isal(sb(p + 5)) == 1 { return 0; } 1 }
fn word_storeb(p: i32) -> i32 { if sb(p) != 115 || sb(p + 1) != 116 || sb(p + 2) != 111 || sb(p + 3) != 114 || sb(p + 4) != 101 || sb(p + 5) != 98 { return 0; } if isal(sb(p + 6)) == 1 { return 0; } 1 }
fn word_label(p: i32) -> i32 { if sb(p) != 108 || sb(p + 1) != 97 || sb(p + 2) != 98 || sb(p + 3) != 101 || sb(p + 4) != 108 { return 0; } if isal(sb(p + 5)) == 1 { return 0; } 1 }
fn word_syscall(p: i32) -> i32 { if sb(p) != 115 || sb(p + 1) != 121 || sb(p + 2) != 115 || sb(p + 3) != 99 || sb(p + 4) != 97 || sb(p + 5) != 108 || sb(p + 6) != 108 { return 0; } if isal(sb(p + 7)) == 1 { return 0; } 1 }

fn line_is4(ln: i32, b0: i32, b1: i32, b2: i32, b3: i32) -> i32 { unsafe { let p = skipsp(LOFF[ln as usize]); word4(p, b0, b1, b2, b3) } }
fn line_is_store(ln: i32) -> i32 { unsafe { let p = skipsp(LOFF[ln as usize]); word_store(p) } }
fn line_is_storeb(ln: i32) -> i32 { unsafe { let p = skipsp(LOFF[ln as usize]); word_storeb(p) } }
fn line_is_label(ln: i32) -> i32 { unsafe { let p = skipsp(LOFF[ln as usize]); word_label(p) } }

fn eq_pos(ln: i32) -> i32 { unsafe { let mut p = LOFF[ln as usize]; let e = p + LLEN[ln as usize]; while p < e { if sb(p) == 61 { return p; } p = p + 1; } 0 - 1 } }

// line contains "call "/"calli "/"param"/"jmp" -- consumers whose live-across values must be callee-saved
fn has_call(ln: i32) -> i32 { unsafe { let mut p = LOFF[ln as usize]; let e = p + LLEN[ln as usize] - 4;
    while p < e {
        if sb(p) == 99 && sb(p + 1) == 97 && sb(p + 2) == 108 && sb(p + 3) == 108 {
            if sb(p + 4) == 32 { return 1; }
            if sb(p + 4) == 105 && sb(p + 5) == 32 { return 1; }
        }
        if sb(p) == 112 && sb(p + 1) == 97 && sb(p + 2) == 114 && sb(p + 3) == 97 && sb(p + 4) == 109 { return 1; }
        if sb(p) == 106 && sb(p + 1) == 109 && sb(p + 2) == 112 { return 1; }
        p = p + 1;
    }
    0 } }

fn parse_num(p: i32) { unsafe { GNUM = 0; let mut q = p; while sb(q) >= 48 && sb(q) <= 57 { GNUM = GNUM * 10 + (sb(q) - 48); q = q + 1; } GEND = q; } }

// def line? after indent: vN : ... = OP; returns v or -1; sets GDEFOP/GCONS
fn def_of(ln: i32) -> i32 { unsafe {
    let mut p = skipsp(LOFF[ln as usize]);
    if sb(p) != 118 { return 0 - 1; }
    if sb(p + 1) < 48 || sb(p + 1) > 57 { return 0 - 1; }
    parse_num(p + 1); let v = GNUM; p = GEND;
    if sb(p) != 58 { return 0 - 1; }
    p = eq_pos(ln); if p < 0 { return 0 - 1; }
    p = skipsp(p + 1);
    GDEFOP = 0;
    if word4(p, 97, 100, 100, 0) == 1 { GDEFOP = 1; } if word4(p, 115, 117, 98, 0) == 1 { GDEFOP = 1; }   // add sub
    if word4(p, 109, 117, 108, 0) == 1 { GDEFOP = 1; } if word4(p, 97, 110, 100, 0) == 1 { GDEFOP = 1; }  // mul and
    if word4(p, 111, 114, 0, 0) == 1 { GDEFOP = 1; } if word4(p, 120, 111, 114, 0) == 1 { GDEFOP = 1; }   // or xor
    if word4(p, 115, 104, 108, 0) == 1 { GDEFOP = 1; } if word4(p, 115, 104, 114, 0) == 1 { GDEFOP = 1; } // shl shr
    if word4(p, 115, 104, 117, 0) == 1 { GDEFOP = 1; }                                                    // shu
    if word_const(p) == 1 { GDEFOP = 1; }                                                                 // const
    if word4(p, 100, 105, 118, 0) == 1 { GDEFOP = 1; } if word4(p, 109, 111, 100, 0) == 1 { GDEFOP = 1; } // div mod
    if word4(p, 100, 117, 118, 0) == 1 { GDEFOP = 1; } if word4(p, 117, 100, 109, 0) == 1 { GDEFOP = 1; } // duv udm
    if word4(p, 108, 111, 97, 100) == 1 { GDEFOP = 1; } if word4(p, 103, 101, 116, 0) == 1 { GDEFOP = 1; } // load get
    if word4(p, 103, 101, 116, 98) == 1 { GDEFOP = 1; }                                                   // getb
    if word4(p, 108, 116, 0, 0) == 1 { GDEFOP = 1; } if word4(p, 108, 101, 0, 0) == 1 { GDEFOP = 1; }     // lt le
    if word4(p, 103, 116, 0, 0) == 1 { GDEFOP = 1; } if word4(p, 103, 101, 0, 0) == 1 { GDEFOP = 1; }     // gt ge
    if word4(p, 101, 113, 0, 0) == 1 { GDEFOP = 1; } if word4(p, 110, 101, 0, 0) == 1 { GDEFOP = 1; }     // eq ne
    if word4(p, 108, 116, 117, 0) == 1 { GDEFOP = 1; } if word4(p, 103, 116, 117, 0) == 1 { GDEFOP = 1; } // ltu gtu
    if word4(p, 108, 101, 117, 0) == 1 { GDEFOP = 1; } if word4(p, 103, 101, 117, 0) == 1 { GDEFOP = 1; } // leu geu
    GCONS = GDEFOP;
    if word4(p, 114, 101, 102, 0) == 1 { GDEFOP = 1; }                                                    // ref (gcons stays)
    if word4(p, 99, 97, 108, 108) == 1 { GCONS = 2; }                                                     // call
    if word_syscall(p) == 1 { GCONS = 2; }                                                                // syscall
    v
} }

fn scan_count(p0: i32, e: i32) { unsafe { GCNT = 0; let mut p = p0;
    while p < e {
        if sb(p) == 118 && sb(p + 1) >= 48 && sb(p + 1) <= 57 {
            let mut prev = 0; if p > 0 { prev = isal(sb(p - 1)); }
            if prev == 0 { parse_num(p + 1); if isal(sb(GEND)) == 0 { GCNT = GCNT + 1; p = GEND; } else { p = p + 1; } }
            else { p = p + 1; }
        } else { p = p + 1; }
    } } }

// scan uses (\bvN\b) in [p,e); hon: 0 non-honored, 1 honored, 2 call/syscall arg. dv = line's def (not a use)
fn scan_uses(p0: i32, e: i32, ln: i32, hon: i32, dv: i32) { unsafe { let mut p = p0;
    while p < e {
        if sb(p) == 118 && sb(p + 1) >= 48 && sb(p + 1) <= 57 {
            let mut prev = 0; if p > 0 { prev = isal(sb(p - 1)); }
            if prev == 0 {
                parse_num(p + 1);
                if isal(sb(GEND)) == 0 {
                    let v = GNUM;
                    if v < 40000 {
                        LU[v as usize] = ln;
                        if v != dv { if hon == 0 { BADUSE[v as usize] = 1; } if hon == 2 { ARGUSE[v as usize] = 1; } }
                        if v > MAXV { MAXV = v; }
                    }
                    p = GEND;
                } else { p = p + 1; }
            } else { p = p + 1; }
        } else { p = p + 1; }
    } } }

// label name at line (after skip chars): sets GNOFF/GNLEN
fn name_after(ln: i32, skip: i32) { unsafe { let mut p = skipsp(LOFF[ln as usize]); p = p + skip; p = skipsp(p);
    GNOFF = p; while isal(sb(p)) == 1 { p = p + 1; } GNLEN = p - GNOFF; } }

fn names_eq(o1: i32, l1: i32, o2: i32, l2: i32) -> i32 { if l1 != l2 { return 0; } let mut i = 0; while i < l1 { if sb(o1 + i) != sb(o2 + i) { return 0; } i = i + 1; } 1 }

// register code -> name rank ('r10'<'r11'<'r12'<'r13'<'r8'<'r9'<'rdi'<'rsi')
fn rrank(c: i32) -> i32 { if c == 3 { return 0; } if c == 4 { return 1; } if c == 1 { return 2; } if c == 2 { return 3; } if c == 7 { return 4; } if c == 8 { return 5; } if c == 6 { return 6; } 7 }

fn emit_line(ln: i32) { unsafe { let mut p = LOFF[ln as usize]; let e = p + LLEN[ln as usize];
    if OUTN + LLEN[ln as usize] + 2 >= 157286400 { OUTOVF = 1; return; }
    while p < e { OUT[OUTN as usize] = SRC[p as usize]; OUTN = OUTN + 1; p = p + 1; }
    OUT[OUTN as usize] = 10; OUTN = OUTN + 1; } }

fn oc(c: i32) { unsafe { OUT[OUTN as usize] = c as u8; OUTN = OUTN + 1; } }

// emit def line with @rNN inserted before the first '='
fn emit_ann(ln: i32, code: i32) { unsafe { let mut p = LOFF[ln as usize]; let e = p + LLEN[ln as usize]; let q = eq_pos(ln);
    if OUTN + LLEN[ln as usize] + 16 >= 157286400 { OUTOVF = 1; return; }
    while p < q { OUT[OUTN as usize] = SRC[p as usize]; OUTN = OUTN + 1; p = p + 1; }
    oc(64); oc(114);                                          // @r
    if code == 1 { oc(49); oc(50); }                         // 12
    if code == 2 { oc(49); oc(51); }                         // 13
    if code == 3 { oc(49); oc(48); }                         // 10
    if code == 4 { oc(49); oc(49); }                         // 11
    if code == 5 { oc(115); oc(105); }                       // si
    if code == 6 { oc(100); oc(105); }                       // di
    if code == 7 { oc(56); }                                 // 8
    if code == 8 { oc(57); }                                 // 9
    oc(32);
    while p < e { OUT[OUTN as usize] = SRC[p as usize]; OUTN = OUTN + 1; p = p + 1; }
    OUT[OUTN as usize] = 10; OUTN = OUTN + 1; } }

fn fcc_push(c: i32) { unsafe { FCC[FCCT as usize] = c; FCCT = FCCT + 1; if FCCT == 8 { FCCT = 0; } } }
fn fcm_push(c: i32) { unsafe { FCM[FCMT as usize] = c; FCMT = FCMT + 1; if FCMT == 8 { FCMT = 0; } } }
fn fcm_pop() -> i32 { unsafe { if FCMH == FCMT { return 0; } let c = FCM[FCMH as usize]; FCMH = FCMH + 1; if FCMH == 8 { FCMH = 0; } c } }
fn fcs_push(c: i32) { unsafe { FCS[FCST as usize] = c; FCST = FCST + 1; if FCST == 8 { FCST = 0; } } }
fn fcc_pop() -> i32 { unsafe { if FCCH == FCCT { return 0; } let c = FCC[FCCH as usize]; FCCH = FCCH + 1; if FCCH == 8 { FCCH = 0; } c } }
fn fcs_pop() -> i32 { unsafe { if FCSH == FCST { return 0; } let c = FCS[FCSH as usize]; FCSH = FCSH + 1; if FCSH == 8 { FCSH = 0; } c } }
fn release(c: i32) { if c == 1 || c == 2 { fcs_push(c); return; } if c == 3 || c == 4 { fcc_push(c); return; } fcm_push(c); }

// expire actives with end < d; then insertion-sort survivors by (end, rrank, value)
fn expire(d: i32) { unsafe {
    let mut i = 0; let mut j = 0;
    while i < NA {
        if AE[i as usize] < d { release(AR[i as usize]); }
        else { AE[j as usize] = AE[i as usize]; AV[j as usize] = AV[i as usize]; AR[j as usize] = AR[i as usize]; j = j + 1; }
        i = i + 1;
    }
    NA = j;
    i = 1;
    while i < NA { let mut jj = i;
        while jj > 0 {
            let mut k = 0;
            if AE[(jj - 1) as usize] > AE[jj as usize] { k = 1; }
            else { if AE[(jj - 1) as usize] == AE[jj as usize] {
                if rrank(AR[(jj - 1) as usize]) > rrank(AR[jj as usize]) { k = 1; }
                else { if rrank(AR[(jj - 1) as usize]) == rrank(AR[jj as usize]) && AV[(jj - 1) as usize] > AV[jj as usize] { k = 1; } } } }
            if k == 1 {
                let t0 = AE[(jj - 1) as usize]; AE[(jj - 1) as usize] = AE[jj as usize]; AE[jj as usize] = t0;
                let t1 = AV[(jj - 1) as usize]; AV[(jj - 1) as usize] = AV[jj as usize]; AV[jj as usize] = t1;
                let t2 = AR[(jj - 1) as usize]; AR[(jj - 1) as usize] = AR[jj as usize]; AR[jj as usize] = t2;
                jj = jj - 1;
            } else { jj = 0; }
        }
        i = i + 1;
    }
} }

fn do_func(fs: i32, fe: i32) { unsafe {
    let mut v = 0; while v <= MAXV { DEFS[v as usize] = 0 - 1; LU[v as usize] = 0 - 1; OKDEF[v as usize] = 0; BADUSE[v as usize] = 0; AREG[v as usize] = 0; ARGUSE[v as usize] = 0; v = v + 1; }
    MAXV = 0; NRG = 0; NLSTK = 0; NLBL = 0;
    // pass 1: defs, uses, regions, call lines
    let mut nc = 0;
    let mut i = fs;
    while i < fe {
        if has_call(i) == 1 { nc = nc + 1; }
        CUMCALL[i as usize] = nc;
        let dv = def_of(i);
        let mut hon = 0;
        if dv >= 0 { hon = GCONS; }
        if line_is4(i, 98, 114, 122, 0) == 1 { hon = 1; }       // brz
        if line_is4(i, 98, 114, 99, 0) == 1 { hon = 1; }        // brc
        if line_is4(i, 114, 101, 116, 0) == 1 { hon = 1; }      // ret
        if line_is_store(i) == 1 { hon = 1; }                   // store
        if line_is_storeb(i) == 1 { hon = 1; }                  // storeb
        if line_is4(i, 112, 117, 116, 0) == 1 { hon = 1; }      // put
        if line_is4(i, 112, 117, 116, 98) == 1 { hon = 1; }     // putb
        if dv >= 0 && dv < 40000 {
            if DEFS[dv as usize] < 0 { DEFS[dv as usize] = i; OKDEF[dv as usize] = 1; if dv > MAXV { MAXV = dv; } }
            else { if i > LU[dv as usize] { LU[dv as usize] = i; } }
            if GDEFOP == 0 { OKDEF[dv as usize] = 0; }
        }
        let mut p = eq_pos(i); let pl = LOFF[i as usize]; let e = pl + LLEN[i as usize];
        if p >= 0 { p = p + 1; } else { p = pl; }
        if hon == 2 { scan_count(p, e); if GCNT > 6 { hon = 0; } }
        scan_uses(p, e, i, hon, dv);
        // regions
        if line_is4(i, 108, 111, 111, 112) == 1 { LSTK[NLSTK as usize] = i; NLSTK = NLSTK + 1; }   // loop
        if line_is4(i, 106, 109, 112, 0) == 1 { if NLSTK > 0 { NLSTK = NLSTK - 1; RGS[NRG as usize] = LSTK[NLSTK as usize]; RGE[NRG as usize] = i; NRG = NRG + 1; } }  // jmp
        if line_is_label(i) == 1 { name_after(i, 5); LBO[NLBL as usize] = GNOFF; LBL[NLBL as usize] = GNLEN; LBLINE[NLBL as usize] = i; NLBL = NLBL + 1; }
        if line_is4(i, 103, 111, 116, 111) == 1 { name_after(i, 4);       // goto
            let mut w = 0; while w < NLBL {
                if names_eq(GNOFF, GNLEN, LBO[w as usize], LBL[w as usize]) == 1 { RGS[NRG as usize] = LBLINE[w as usize]; RGE[NRG as usize] = i; NRG = NRG + 1; w = NLBL; }
                else { w = w + 1; }
            }
        }
        i = i + 1;
    }
    // pass 2: back-edge interval extension to fixpoint
    let mut chg = 1;
    while chg == 1 { chg = 0;
        v = 0; while v <= MAXV {
            if DEFS[v as usize] >= 0 { let mut e = LU[v as usize]; if e < DEFS[v as usize] { e = DEFS[v as usize]; }
                let mut k = 0; while k < NRG {
                    if DEFS[v as usize] < RGS[k as usize] && RGS[k as usize] <= e && e < RGE[k as usize] { LU[v as usize] = RGE[k as usize]; e = RGE[k as usize]; chg = 1; }
                    k = k + 1;
                }
            }
            v = v + 1;
        }
    }
    // pass 3: linear scan in line order; emit
    FCSH = 0; FCST = 0; FCCH = 0; FCCT = 0; FCMH = 0; FCMT = 0; NA = 0;
    fcs_push(1); fcs_push(2); fcc_push(3); fcc_push(4);
    fcm_push(5); fcm_push(6); fcm_push(7); fcm_push(8);
    i = fs;
    while i < fe {
        let dv = def_of(i);
        if dv >= 0 && dv < 40000 {
            if DEFS[dv as usize] == i && OKDEF[dv as usize] == 1 && BADUSE[dv as usize] == 0 && LU[dv as usize] > DEFS[dv as usize] {
                expire(i);
                let s = DEFS[dv as usize]; let en2 = LU[dv as usize];
                let mut isacross = 0;
                if en2 > s + 1 { if CUMCALL[(en2 - 1) as usize] - CUMCALL[s as usize] > 0 { isacross = 1; } }
                let mut c = 0;
                if isacross == 1 { c = fcs_pop(); }
                else { if ARGUSE[dv as usize] == 1 { c = fcc_pop(); if c == 0 { c = fcs_pop(); } }
                       else { c = fcm_pop(); if c == 0 { c = fcc_pop(); } if c == 0 { c = fcs_pop(); } } }
                if c > 0 { AREG[dv as usize] = c; AE[NA as usize] = en2; AV[NA as usize] = dv; AR[NA as usize] = c; NA = NA + 1; }
            }
        }
        if dv >= 0 && dv < 40000 && AREG[dv as usize] > 0 { emit_ann(i, AREG[dv as usize]); }
        else { emit_line(i); }
        i = i + 1;
    }
} }

fn main() { unsafe {
    SLEN = read_all(); OUTN = 0; MAXV = 39999;
    splitlines();
    let mut infunc = 0; let mut fs = 0; let mut i = 0;
    while i < NL {
        if line_is4(i, 105, 114, 0, 0) == 1 || line_is4(i, 102, 110, 0, 0) == 1 {   // ir / fn
            if infunc == 1 { do_func(fs, i); }
            else { let mut k = 0; while k < i { emit_line(k); k = k + 1; } }
            infunc = 1; fs = i;
        }
        i = i + 1;
    }
    if infunc == 1 { do_func(fs, NL); }
    else { let mut k = 0; while k < NL { emit_line(k); k = k + 1; } }
    if OUTOVF == 0 && SRCOVF == 0 { __syscall(1, 1, &OUT[0], OUTN); }
} }
