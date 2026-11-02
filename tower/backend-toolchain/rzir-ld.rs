// rzir-ld.rs -- the derived rzir cross-file linker, IN RUST (port of rzir-ld.c, itself a port of rzir-ld.py).
// Reads the concatenated rzir units on stdin. Globals: collect `gdef NAME SIZE`, assign 8-aligned addresses
// from 0x700000, rewrite every `... = gref NAME` -> `const <addr>`, drop gdefs. Functions: split into
// `ir main`/`fn NAME` blocks, DEDUP by name, emit the main block first. Stays in the chain's compile subset
// (no bitwise ops needed -- alignment is (n+7)/8*8); string literals replaced by byte-code matchers.
static mut SRC: [u8; 157286400] = [0; 157286400];  // C-LD-BUF-8MB-OVERFLOW: twin of rzir-ld.c; see the note there
static mut SLEN: i32 = 0;
static mut OUT: [u8; 157286400] = [0; 157286400];
static mut OUTN: i32 = 0;
static mut GLOFF: [i32; 6000000] = [0; 6000000];   // line offsets in SRC (C-LD-GLOFF-500K: matched to rzir-ld.c gloff[1000000]; the Rust twin was under-sized at 300000 -> silent overflow + truncation on >300000-line inputs, e.g. SEM0 self-emit at scale. Latent: DDC corpus is small)
static mut GLLEN: [i32; 6000000] = [0; 6000000];   // line lengths (matched to rzir-ld.c gllen[1000000])
static mut GNL: i32 = 0;
static mut GSOFF: [i32; 40000] = [0; 40000];     // global symtab: name off in SRC
static mut GSLEN: [i32; 40000] = [0; 40000];     // name len
static mut GSADDR: [i64; 40000] = [0; 40000];    // assigned address
static mut NGS: i32 = 0;
static mut BSTART: [i32; 40000] = [0; 40000];    // function blocks: start line
static mut BNOFF: [i32; 40000] = [0; 40000];     // block name off
static mut BNLEN: [i32; 40000] = [0; 40000];     // block name len
static mut BMAIN: [i32; 40000] = [0; 40000];     // is `ir main`
static mut BDROP: [i32; 40000] = [0; 40000];     // duplicate -> drop
static mut NB: i32 = 0;
static mut GNUM: i64 = 0;
static mut GEND: i32 = 0;
static mut DIG: [i32; 16] = [0; 16];

fn sb(i: i32) -> i32 { unsafe { SRC[i as usize] as i32 } }

fn read_all() -> i32 { unsafe { let mut total = 0; let mut n = __syscall(0, 0, &SRC[0], 65536); while n > 0 { total = total + n; n = __syscall(0, 0, &SRC[total as usize], 65536); } total } }

fn isal(c: i32) -> i32 { if c >= 97 && c <= 122 { return 1; } if c >= 65 && c <= 90 { return 1; } if c >= 48 && c <= 57 { return 1; } if c == 95 { return 1; } 0 }

// char at line i position p; newline (10) if out of range so words never cross line boundaries
fn lc(i: i32, p: i32) -> i32 { unsafe { if p < 0 { return 10; } if p >= GLLEN[i as usize] { return 10; } SRC[(GLOFF[i as usize] + p) as usize] as i32 } }

fn skipsp(i: i32, p: i32) -> i32 { let mut q = p; while lc(i, q) == 32 || lc(i, q) == 9 { q = q + 1; } q }

// does the word given by byte codes b0..b3 sit at line i pos p with a word boundary after? a 0 byte
// terminates the pattern (NUL never occurs in a line). <=6 params -- the ABI has only 6 arg registers.
fn word_at(i: i32, p: i32, b0: i32, b1: i32, b2: i32, b3: i32) -> i32 {
    if b0 != 0 && lc(i, p) != b0 { return 0; }
    if b1 != 0 && lc(i, p + 1) != b1 { return 0; }
    if b2 != 0 && lc(i, p + 2) != b2 { return 0; }
    if b3 != 0 && lc(i, p + 3) != b3 { return 0; }
    let mut n = 0;
    if b0 != 0 { n = 1; } if b1 != 0 { n = 2; } if b2 != 0 { n = 3; } if b3 != 0 { n = 4; }
    if isal(lc(i, p + n)) == 1 { return 0; }
    1
}

fn idlen(i: i32, p: i32) -> i32 { let mut q = p; let mut k = 0; while isal(lc(i, q)) == 1 { k = k + 1; q = q + 1; } k }
// C-LD-QUALIFIED-NAME-COLLISION: block names may be qualified (`fn S::a`) and idlen stops at `:`, so `fn S::a`
// and `fn S::b` both scanned as "S" and the dedup-by-name pass deleted the second. idlen itself is unchanged:
// the global symtab and gref paths must keep stopping at `:`.
fn fnlen(i: i32, p: i32) -> i32 { let mut q = p; let mut k = 0; while isal(lc(i, q)) == 1 || lc(i, q) == 58 || lc(i, q) == 36 { k = k + 1; q = q + 1; } k }  // +36=$: mono-mangled `alloc$T` names were false-deduped (same class as `::`, C-LD-QUALIFIED-NAME-COLLISION)

fn parse_num(i: i32, p: i32) { unsafe { GNUM = 0; let mut q = p; while lc(i, q) >= 48 && lc(i, q) <= 57 { GNUM = GNUM * 10 + ((lc(i, q) - 48) as i64); q = q + 1; } GEND = q; } }

fn split_lines() { unsafe { let mut i = 0; let mut st = 0; GNL = 0;
    while i < SLEN { if SRC[i as usize] == 10 { GLOFF[GNL as usize] = st; GLLEN[GNL as usize] = i - st; GNL = GNL + 1; st = i + 1; } i = i + 1; }
    if st < SLEN { GLOFF[GNL as usize] = st; GLLEN[GNL as usize] = SLEN - st; GNL = GNL + 1; } } }

// two SRC substrings equal?
fn sub_eq(o1: i32, l1: i32, o2: i32, l2: i32) -> i32 { if l1 != l2 { return 0; } let mut k = 0; while k < l1 { if sb(o1 + k) != sb(o2 + k) { return 0; } k = k + 1; } 1 }

// global symtab index for name SRC[off..off+len); -1 none
fn gs_find(off: i32, len: i32) -> i32 { unsafe { let mut k = 0; while k < NGS { if sub_eq(GSOFF[k as usize], GSLEN[k as usize], off, len) == 1 { return k; } k = k + 1; } 0 - 1 } }

// position of the word `gref` in line i; -1 none
fn find_gref(i: i32) -> i32 { unsafe { let mut p = 0; while p < GLLEN[i as usize] { if word_at(i, p, 103, 114, 101, 102) == 1 { return p; } p = p + 1; } 0 - 1 } }

fn ec(c: i32) { unsafe { OUT[OUTN as usize] = c as u8; OUTN = OUTN + 1; } }

fn en(n0: i64) { unsafe { let mut n = n0; if n == 0 { ec(48); return; } let mut i = 0; while n > 0 { DIG[i as usize] = (48 + n % 10) as i32; n = n / 10; i = i + 1; } while i > 0 { i = i - 1; ec(DIG[i as usize]); } } }

// emit line i verbatim (no trailing newline)
fn emit_raw(i: i32) { unsafe { let mut k = 0; while k < GLLEN[i as usize] { OUT[OUTN as usize] = SRC[(GLOFF[i as usize] + k) as usize]; OUTN = OUTN + 1; k = k + 1; } } }

// emit one line of a block, transforming `... gref NAME` -> `... const ADDR`, dropping stray gdef; adds newline
fn emit_line(i: i32) { unsafe {
    let p = skipsp(i, 0);
    if word_at(i, p, 103, 100, 101, 102) == 1 { return; }        // drop stray gdef
    let gp = find_gref(i);
    if gp >= 0 {
        let sp = skipsp(i, gp + 4);
        let noff = GLOFF[i as usize] + sp;                          // name after `gref `
        let nlen = idlen(i, sp);
        let gi = gs_find(noff, nlen);
        if gi >= 0 {
            let mut k = 0; while k < gp { OUT[OUTN as usize] = SRC[(GLOFF[i as usize] + k) as usize]; OUTN = OUTN + 1; k = k + 1; }   // prefix `  vN: i32 = `
            ec(99); ec(111); ec(110); ec(115); ec(116); ec(32);     // "const "
            en(GSADDR[gi as usize]); ec(10); return;
        }
    }
    emit_raw(i); ec(10);
} }

// emit block b: lines [BSTART[b], next)
fn emit_block(b: i32) { unsafe { let mut e = GNL; if b + 1 < NB { e = BSTART[(b + 1) as usize]; } let mut i = BSTART[b as usize]; while i < e { emit_line(i); i = i + 1; } } }

fn main() { unsafe {
    SLEN = read_all(); OUTN = 0; NGS = 0; NB = 0;
    split_lines();

    // pass 1: global symtab (dedup by name), assign 8-aligned addrs from 0x700000
    // C-ARENA-LD-I32: `cur` is an i32 and global addresses are assigned from it, with `heaptop` emitted only
    // `if cur > 134217728`. Past i32::MAX the accumulation wraps NEGATIVE, that guard goes false, and the directive
    // VANISHES -- whereupon cf falls back to a 128MB frame base sitting INSIDE the program's own 2GB of globals, and
    // the program faults on its first real allocation. Measured on a direct-rzir probe: a 2040MB gdef returns 7,
    // 2044MB emits no heaptop and SEGVs. The rest of the chain is 32-bit too (cf's parse, `spsave`'s sign-extended
    // disp32, and `store_imm`'s imm32 for the gref'd addresses themselves), so raising the ceiling is a much larger
    // rung -- see locales/arena-ceiling. What is fixed HERE is the silence: overflow now REFUSES.
    let mut cur: i64 = 7340032;
    let mut arena_ovf = 0;
    let mut frame_below = 0;
    let mut want_fb = 0;
    let mut i = 0;
    while i < GNL {
        let p = skipsp(i, 0);
        if word_at(i, p, 103, 100, 101, 102) == 1 {              // gdef
            let p2 = skipsp(i, p + 4);
            let noff = GLOFF[i as usize] + p2; let nlen = idlen(i, p2);
            if gs_find(noff, nlen) < 0 {
                let p3 = skipsp(i, p2 + nlen); parse_num(i, p3);
                // B1 FRAME-BELOW is OPT-IN: the emitter that has the 64-bit pointer fixes emits `gdef __fbelow`.
                // L6 and every old-style emitter lack it -> they keep the historical frame-above layout untouched.
                if nlen == 8 && sb(noff) == 95 && sb(noff + 1) == 95 && sb(noff + 2) == 102 && sb(noff + 3) == 98 && sb(noff + 4) == 101 && sb(noff + 5) == 108 && sb(noff + 6) == 111 && sb(noff + 7) == 119 { want_fb = 1; }
                // FRAME-BELOW: __heap gets a reserved low frame region [128MB, 2176MB] below it, ONLY when opt-in
                // [FRAME_LO=128MB, FRAME_HI=2176MB] below it, so the frame stays <4GB and only the heap crosses 4GB.
                // Small programs (tiny __heap) skip this -> byte-identical layout (their arenas already sit below the
                // default 128MB frame base). `framelo` tells cf to pin fbase at 128MB instead of arena_top+16MB.
                if nlen == 6 && sb(noff) == 95 && sb(noff + 1) == 95 && sb(noff + 2) == 104 && sb(noff + 3) == 101 && sb(noff + 4) == 97 && sb(noff + 5) == 112 && want_fb == 1 {
                    if cur < 2281701376 { cur = 2281701376; }
                    frame_below = 1;
                }
                GSOFF[NGS as usize] = noff; GSLEN[NGS as usize] = nlen; GSADDR[NGS as usize] = cur; NGS = NGS + 1;
                cur = cur + (GNUM + 7) / 8 * 8;
                if cur < 0 { arena_ovf = 1; }                      // wrapped past i32::MAX
                if cur > 68719476736 { arena_ovf = 1; }             // C-ARENA-LIFT: raised toward the 4GB gref bound; cf now holds i64 heaptop + a higher fbase cap that does the LOUD refuse
            }
        }
        i = i + 1;
    }

    // pass 2: identify blocks (ir main / fn NAME), mark duplicates to drop
    i = 0;
    while i < GNL {
        let p = skipsp(i, 0);
        let mut isir = 0; let mut nameoff = 0 - 1; let mut namelen = 0;
        if word_at(i, p, 105, 114, 0, 0) == 1 {                  // ir
            let p2 = skipsp(i, p + 2);
            if word_at(i, p2, 109, 97, 105, 110) == 1 { isir = 1; nameoff = GLOFF[i as usize] + p2; namelen = fnlen(i, p2); }   // main
        } else if word_at(i, p, 102, 110, 0, 0) == 1 {          // fn
            let p2 = skipsp(i, p + 2); nameoff = GLOFF[i as usize] + p2; namelen = fnlen(i, p2);
        }
        if nameoff >= 0 {
            BSTART[NB as usize] = i; BNOFF[NB as usize] = nameoff; BNLEN[NB as usize] = namelen; BMAIN[NB as usize] = isir; BDROP[NB as usize] = 0;
            let mut j = 0; while j < NB { if sub_eq(BNOFF[j as usize], BNLEN[j as usize], nameoff, namelen) == 1 { BDROP[NB as usize] = 1; } j = j + 1; }
            NB = NB + 1;
        }
        i = i + 1;
    }

    // emit: main block first, then the rest in order (non-dropped, non-main)
    // CFML per-program frame base: when the arenas exceed the default 128MB frame base, hand the arena top
    // (cur) to cf via a `heaptop` directive so cf places the frame arena / sp-save / memsz ABOVE the arenas
    // (else the fixed frame region collides with a large program's heap). Below 128MB the default cf layout
    // is already above the arenas -> NO directive -> byte-identical output (every provenance/golden input has
    // arena_top < 128MB). opt/ra forward the top-level pre-block directive; the M0 cf never sees it. (matches
    // rzir-ld.c exactly for the byte-DDC.)
    if cur > 134217728 || arena_ovf == 1 {
        ec(104); ec(101); ec(97); ec(112); ec(116); ec(111); ec(112); ec(32);   // "heaptop "
        // C-ARENA-LD-I32: on overflow emit a POISON value, not a wrapped one -- and not, as first tried and REFUTED,
        // nothing at all: suppressing ld's output merely makes cf compile empty input into a 133-byte ELF that still
        // faults. i32::MAX drives fbase past cf's 0x7F000000 guard, so cf REFUSES and writes zero bytes.
        if arena_ovf == 1 { en(2147483647); } else { en(cur); }
        ec(10);
        if frame_below == 1 {
            ec(102); ec(114); ec(97); ec(109); ec(101); ec(108); ec(111); ec(32);   // "framelo "
            en(134217728);                                                            // FRAME_LO = 128MB (fixed frame base)
            ec(10);
        }
    }

    let mut mb = 0 - 1; let mut j = 0;
    while j < NB { if BMAIN[j as usize] == 1 && BDROP[j as usize] == 0 && mb < 0 { mb = j; } j = j + 1; }
    if mb >= 0 { emit_block(mb); }
    j = 0; while j < NB { if BMAIN[j as usize] == 0 && BDROP[j as usize] == 0 { emit_block(j); } j = j + 1; }

    __syscall(1, 1, &OUT[0], OUTN);
} }
