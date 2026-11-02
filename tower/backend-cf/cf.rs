// cf.rs -- the rzir -> x86-64 ELF backend, IN RUST (hoisted from cfc.c, itself hoisted from the M0 cf).
// R2: HONORS rzir-ra @rNN (values in r10/r11/r12/r13/rsi/rdi/r8/r9; callee-saved r12/r13 push/pop);
// values without a reg slot to [ebx + N*8]. 32-bit ops for i32. Frame: fn does add ebx,0x8000 /
// sub ebx,0x8000; args in edi/esi/edx; calls + gotos backpatched by name. Validated DIFFERENTIALLY: its
// output ELF must RUN with the same behavior as the M0 cf on the rzir corpus (not byte-identical).
// C-CF-BUF-8MB-OVERFLOW: read_all() filled BUF with NO bounds check, and OUT is the very next static, so an
// input larger than BUF spilled rzir SOURCE TEXT directly over the ELF header in OUT -- a silently corrupt
// binary with no diagnostic. Measured 2026-08-26: cf BUF 8,000,000; the tower's own ra output 7,950,225 (0.6%
// headroom); a program 400 functions larger produced 8,002,455 and overwrote the header with text like
// ": i32 @r9@=8a" / "load v@2". This is what the closure lift tripped -- not fn count, not image size, not
// recursion, all of which were separately refuted. Raised to 16 MB AND bounds-checked, because raising a limit
// without a check only moves the cliff.
static mut BUF: [u8; 117440512] = [0; 117440512];
static mut BLEN: i32 = 0;
static mut OUT: [u8; 83886080] = [0; 83886080];
static mut OUTN: i32 = 0;
static mut BRZ_SITE: [i32; 1000] = [0; 1000];
static mut BRZ_SP: i32 = 0;
static mut LOOP_POS: [i32; 400] = [0; 400];
static mut LOOP_BD: [i32; 400] = [0; 400];
static mut LOOP_CD: [i32; 400] = [0; 400];
static mut LOOP_KD: [i32; 400] = [0; 400];
static mut LOOP_SP: i32 = 0;
static mut CNT_SITE: [i32; 8000] = [0; 8000];
static mut CNT_SP: i32 = 0;
static mut BRK_SITE: [i32; 8000] = [0; 8000];
static mut BRK_SP: i32 = 0;
static mut LBL_OFF: [i32; 12000] = [0; 12000];
static mut LBL_LEN: [i32; 12000] = [0; 12000];
static mut LBL_POS: [i32; 12000] = [0; 12000];
static mut NLBL: i32 = 0;
static mut GT_SITE: [i32; 16000] = [0; 16000];
static mut GT_OFF: [i32; 16000] = [0; 16000];
static mut GT_LEN: [i32; 16000] = [0; 16000];
static mut NGT: i32 = 0;
static mut FNAME_OFF: [i32; 16384] = [0; 16384];
static mut FNAME_LEN: [i32; 16384] = [0; 16384];
static mut FADDR: [i32; 16384] = [0; 16384];
static mut NF: i32 = 0;
static mut CF_SITE: [i32; 256000] = [0; 256000];
static mut CF_OFF: [i32; 256000] = [0; 256000];
static mut CF_LEN: [i32; 256000] = [0; 256000];
static mut NCF: i32 = 0;
static mut FA_SITE: [i32; 16384] = [0; 16384];   // faddr: absolute-address backpatch (mov eax, 0x400000+fnoff)
// CL-A1 (locales/call-abi-arity): arguments 7..N. System V passes the first six in
// rdi/rsi/rdx/rcx/r8/r9 and the rest ON THE STACK; cf marshalled six and stopped, so a 7th argument was
// silently not passed -- the callee read rdi again (measured: the trailing parameter returned the FIRST
// argument's value) or, with a reference, a garbage pointer or a segfault. Both tiers, identically, because
// this is the shared backend. XARG holds the overflow arguments of one call; NPUSH is how many callee-saved
// registers the CURRENT function pushed, which is what the incoming stack arguments sit above.
static mut XARG: [i32; 40] = [0; 40];   // 6 reg + 40 stack = 46-arg capacity (was 24/30; bumped for LLVM C++ FFI calls up to 34 args in rustc_codegen_llvm -- T3-ASM)
static mut NPUSH: i32 = 0;
static mut FA_OFF: [i32; 16384] = [0; 16384];
static mut FA_LEN: [i32; 16384] = [0; 16384];
static mut NFA: i32 = 0;

fn bufat(i: i32) -> i32 { unsafe { BUF[i as usize] as i32 } }
fn read_all() -> i32 { unsafe { let mut total = 0; let mut n = __syscall(0, 0, &BUF[0], 65536); while n > 0 { total = total + n; if total + 65536 > 117440512 { BUFOVF = 1; n = 0; } else { n = __syscall(0, 0, &BUF[total as usize], 65536); } } total } }
static mut BUFOVF: i32 = 0;
static mut FB: i32 = 0;   // B1: frame-below active (framelo present) -> use 64-bit address operands for >4GB heap pointers; else byte-identical to the historical layout   // set when the input would exceed BUF: refuses the write instead of corrupting OUT
fn eb(b: i32) { unsafe { OUT[OUTN as usize] = (b & 255) as u8; OUTN = OUTN + 1; } }
fn ed(d: i32) { eb(d); eb(d >> 8); eb(d >> 16); eb(d >> 24); }

// R2 register allocation: honor rzir-ra's @rNN. VREG[v] = ra register code (1=r12 2=r13 3=r10 4=r11 5=rsi
// 6=rdi 7=r8 8=r9), 0 = memory slot [rbx+v*8]. Per-function generation (VGEN==FGEN) avoids clearing 40000.
static mut VREG: [i32; 40000] = [0; 40000];
static mut VGEN: [i32; 40000] = [0; 40000];
static mut FGEN: i32 = 0;
static mut USED_R12: i32 = 0; static mut USED_R13: i32 = 0;   // does THIS fn use r12/r13 (set by scan_saves at the header) -> push/pop only then
// SLOT-FRAME FIX: slots live at [ebx + v*8] (raw vreg index); the per-call frame bump MUST clear the
// caller's highest slot or a callee's frame overlaps and clobbers a caller's live slot. The old fixed
// 0x20000 (131072 = 16384 slots) was < the max vreg any fn uses (up to ~32854 in a large `main`), so
// upper-half slots aliased into callee frames -- a latent corruption that surfaced under a vreg-number
// shift. FRAME_SZ is now sized to the program's global max vreg (floored at the old 131072 so small
// programs stay byte-identical). Set once by scan_frame() before emission.
static mut FRAME_SZ: i32 = 131072;
fn vreg_of(v: i32) -> i32 { unsafe { if v >= 0 && v < 40000 && VGEN[v as usize] == FGEN { VREG[v as usize] } else { 0 } } }
// x86-64 rm field (0-7) for a ra register code, and whether it needs REX.B
fn rrm(code: i32) -> i32 { if code == 1 { 4 } else if code == 2 { 5 } else if code == 3 { 2 } else if code == 4 { 3 } else if code == 5 { 6 } else if code == 6 { 7 } else if code == 7 { 0 } else { 1 } }
fn rrexb(code: i32) -> i32 { if code == 5 || code == 6 { 0 } else { 1 } }   // rsi/rdi need no REX.B; r8-r13 do
// mov <dest32/64>, r/m[v]  where dest is given by modrm-reg-base (0xC0|reg<<3 for the reg-form / 0x83|... mem form's reg bits)
// load into eax(reg=0)/ecx(reg=1)/edx(reg=2). memrm = the mem ModRM (0x83/0x8B/0x93 -> reg bits already in). regbase = 0xC0 | reg<<3.
fn load_reg32(v: i32, memrm: i32, regbase: i32) { unsafe { let r = vreg_of(v); if r == 0 { eb(139); eb(memrm); ed(v * 8); } else { if rrexb(r) == 1 { eb(65); } eb(139); eb(regbase + rrm(r)); } } }
fn load_reg64(v: i32, regbase: i32) { unsafe { let r = vreg_of(v); if r == 0 { eb(72); eb(139); eb(regbase - 61); ed(v * 8); } else { eb(72 + rrexb(r)); eb(139); eb(regbase + rrm(r)); } } }
fn load_eax(s: i32) { load_reg32(s, 131, 192); }   // reg base 0xC0 (eax)
fn load_ecx(s: i32) { load_reg32(s, 139, 200); }   // mem 0x8B, reg 0xC8 (ecx)
fn load_edx(s: i32) { load_reg32(s, 147, 208); }   // mem 0x93, reg 0xD0 (edx)
fn load_rax(s: i32) { load_reg64(s, 192); }
fn load_rcx(s: i32) { load_reg64(s, 200); }
// store rax/rdx into r/m[v] (64-bit, keeps slots/regs clean). regbase: 0xC0 (rax) / 0xD0 (rdx)
fn store_reg64(v: i32, regbase: i32) { unsafe { let r = vreg_of(v); if r == 0 { eb(72); eb(137); eb(regbase - 61); ed(v * 8); } else { eb(72 + rrexb(r)); eb(137); eb(regbase + rrm(r)); } } }
fn store_eax(s: i32) { store_reg64(s, 192); }
fn store_rax(s: i32) { store_reg64(s, 192); }
fn store_rdx(s: i32) { store_reg64(s, 208); }
fn store_edx(v: i32) { unsafe { let r = vreg_of(v); if r == 0 { eb(137); eb(147); ed(v * 8); } else { if rrexb(r) == 1 { eb(65); } eb(137); eb(208 + rrm(r)); } } }   // 32-bit edx -> r/m[v] (mod/udm i32 result; zero-extends into a reg home)
// OP eax/rax, r/m[v]  (arith/cmp with a 1-byte opcode). op = 3 add / 43 sub / 35 and / 11 or / 51 xor / 59 cmp
fn op_eax(op: i32, v: i32) { unsafe { let r = vreg_of(v); if r == 0 { eb(op); eb(131); ed(v * 8); } else { if rrexb(r) == 1 { eb(65); } eb(op); eb(192 + rrm(r)); } } }
fn op_rax(op: i32, v: i32) { unsafe { let r = vreg_of(v); if r == 0 { eb(72); eb(op); eb(131); ed(v * 8); } else { eb(72 + rrexb(r)); eb(op); eb(192 + rrm(r)); } } }
fn imul_eax(v: i32) { unsafe { let r = vreg_of(v); if r == 0 { eb(15); eb(175); eb(131); ed(v * 8); } else { if rrexb(r) == 1 { eb(65); } eb(15); eb(175); eb(192 + rrm(r)); } } }
fn imul_rax(v: i32) { unsafe { let r = vreg_of(v); if r == 0 { eb(72); eb(15); eb(175); eb(131); ed(v * 8); } else { eb(72 + rrexb(r)); eb(15); eb(175); eb(192 + rrm(r)); } } }
// idiv r/m[v]: F7 /7. mem ModRM 0xBB, reg 0xF8|rrm.  is64: prefix REX.W
fn idiv_v(v: i32, is64: i32) { unsafe { let r = vreg_of(v); if r == 0 { if is64 == 1 { eb(72); } eb(247); eb(187); ed(v * 8); } else { if is64 == 1 { eb(72 + rrexb(r)); } else { if rrexb(r) == 1 { eb(65); } } eb(247); eb(248 + rrm(r)); } } }
fn div6_v(v: i32) { unsafe { let r = vreg_of(v); if r == 0 { eb(247); eb(179); ed(v * 8); } else { if rrexb(r) == 1 { eb(65); } eb(247); eb(240 + rrm(r)); } } }   // unsigned div [v] (F7 /6)
// marshal: mov <destreg>, r/m[v]. dm = ModRM low bits for dest reg (mem form, reg<<3|011), dhi = reg-form base (0xC0|reg<<3), drex = REX.R for dest r8/r9
fn marshal(v: i32, drex: i32, dmem: i32, dregbase: i32) { unsafe { let r = vreg_of(v); if r == 0 { eb(72 + drex); eb(139); eb(dmem); ed(v * 8); } else { eb(72 + drex + rrexb(r)); eb(139); eb(dregbase + rrm(r)); } } }
// mov r/m[dv], imm32 (64-bit sign-extended clean const)
// C-ARENA-C3E: `store_imm` emits `mov r/m64, imm32`, which SIGN-extends -- so a constant in [2^31, 2^32) lands as a
// NEGATIVE 64-bit value. Measured in-register (a memory round-trip HIDES this, because `store` is 32-bit and `load8`
// zero-extends): const 3000000000 divided by 1e6 gave the sign-extended answer, not the unsigned one. The
// zero-extending form is cheap -- `mov r32, imm32` (B8+rd) zero-extends into the full 64-bit register by definition,
// so no movabs is needed below 4GB. For a memory slot, materialise through eax (cf's standing scratch) and store the
// full 64-bit register, so the slot's upper half is written rather than left stale.
fn store_imm_zx(dv: i32, val: i32) { unsafe {
    let r = vreg_of(dv);
    if r == 0 { eb(184); ed(val); eb(72); eb(137); eb(131); ed(dv * 8); }
    else { if rrexb(r) == 1 { eb(65); } eb(184 + rrm(r)); ed(val); }
} }
fn store_imm(dv: i32, val: i32) { unsafe { let r = vreg_of(dv); if r == 0 { eb(72); eb(199); eb(131); ed(dv * 8); ed(val); } else { eb(72 + rrexb(r)); eb(199); eb(192 + rrm(r)); ed(val); } } }
// CL-A1 stack-argument helpers. cf uses NO frame pointer -- locals are [rbx + v*8] -- so an incoming stack
// argument is addressed from rsp, and the only thing between rsp and the return address is whatever
// callee-saved registers this function's prologue pushed (NPUSH, decided by scan_saves before the pushes).
fn sub_rsp(n: i32) { eb(72); eb(129); eb(236); ed(n); }            // sub rsp, imm32
fn add_rsp(n: i32) { eb(72); eb(129); eb(196); ed(n); }            // add rsp, imm32
fn push_slot(v: i32) { let r = vreg_of(v); if r == 0 { eb(255); eb(179); ed(v * 8); } else { if rrexb(r) != 0 { eb(65); } eb(80 + rrm(r)); } }   // push qword [rbx+v*8] / push reg
// mov r/m[dv], <param reg pc> (rdi/rsi/rdx/rcx/r8/r9), or from the incoming stack area for pc >= 6
fn store_param(dv: i32, pc: i32) { unsafe {
    if pc >= 6 {
        // mov rax, [rsp + 8 + 8*NPUSH + 8*(pc-6)]  then store rax into the slot. The +8 skips the return address.
        eb(72); eb(139); eb(132); eb(36); ed(8 + 8 * NPUSH + 8 * (pc - 6));
        store_rax(dv);
        return;
    }
    let mut pf = 7; let mut prex = 0;
    if pc == 1 { pf = 6; } else if pc == 2 { pf = 2; } else if pc == 3 { pf = 1; } else if pc == 4 { pf = 0; prex = 4; } else if pc == 5 { pf = 1; prex = 4; }
    let r = vreg_of(dv);
    if r == 0 { eb(72 + prex); eb(137); eb(131 + pf * 8); ed(dv * 8); }
    else { eb(72 + prex + rrexb(r)); eb(137); eb(192 + pf * 8 + rrm(r)); }
} }
// store a value known to be in rax into r/m[dv], but as a 32-bit move when the op is 32-bit result. (unused; kept 64-bit)
fn patch(site: i32, target: i32) { unsafe { let r = target - site - 4; OUT[site as usize] = (r & 255) as u8; OUT[(site + 1) as usize] = ((r >> 8) & 255) as u8; OUT[(site + 2) as usize] = ((r >> 16) & 255) as u8; OUT[(site + 3) as usize] = ((r >> 24) & 255) as u8; } }
// R2 micro-opt: scan a function body [start .. next fn/ir header] for @r12/@r13 so the prologue pushes
// (and the epilogue pops) only the callee-saved regs the function actually uses.
fn scan_saves(start: i32) { unsafe {
    USED_R12 = 0; USED_R13 = 0;
    let mut p = start;
    while p < BLEN {
        let mut le = p; while le < BLEN && bufat(le) != 10 { le = le + 1; }
        let mut c = p; while c < le && bufat(c) == 32 { c = c + 1; }
        if c < le && ((bufat(c) == 102 && bufat(c + 1) == 110) || (bufat(c) == 105 && bufat(c + 1) == 114)) { return; }   // next fn/ir header
        let mut ss = p;
        while ss + 3 < le {
            if bufat(ss) == 64 && bufat(ss + 1) == 114 && bufat(ss + 2) == 49 {
                if bufat(ss + 3) == 50 { USED_R12 = 1; } else if bufat(ss + 3) == 51 { USED_R13 = 1; }
            }
            ss = ss + 1;
        }
        p = le + 1;
    }
} }
fn namelen(s: i32, e: i32) -> i32 { let mut q = s; while q < e && bufat(q) > 32 { q = q + 1; } q - s }

// SLOT-FRAME FIX: scan the whole rzir for the largest vreg index used by any function, and size the
// per-call frame bump to clear it: FRAME_SZ = max(131072, roundup((maxv+1)*8, 4096)). A `vN` token is
// `v` (118) + digits, not preceded by an identifier char (so `parse_v9` etc. do not count). Over-counting
// is safe (only enlarges the frame); under-counting is not, so the guard errs toward larger.
fn scan_frame() { unsafe {
    let mut mx = 0;
    let mut p = 0;
    while p < BLEN {
        if bufat(p) == 118 && bufat(p + 1) >= 48 && bufat(p + 1) <= 57 {
            let prev = if p > 0 { bufat(p - 1) } else { 32 };
            let mut pa = 0;
            if (prev >= 97 && prev <= 122) || (prev >= 65 && prev <= 90) || (prev >= 48 && prev <= 57) || prev == 95 { pa = 1; }
            if pa == 0 {
                let mut q = p + 1; let mut n = 0;
                while bufat(q) >= 48 && bufat(q) <= 57 { n = n * 10 + (bufat(q) - 48); q = q + 1; }
                if n > mx { mx = n; }
                p = q;
            } else { p = p + 1; }
        } else { p = p + 1; }
    }
    let mut sz = (mx + 1) * 8;
    sz = (sz + 4095) / 4096 * 4096;   // round up to a 4096 multiple
    if sz < 131072 { sz = 131072; }   // floor: small programs keep the old frame (byte-identical) + full recursion depth
    FRAME_SZ = sz;
} }

// CFML per-program frame base: ld emits a `heaptop <arena_top>` directive (a top-level pre-block line, forwarded
// by opt/ra) when a program's arenas exceed the default 128MB frame base. Scan for it; return arena_top, or 0 when
// absent (every small program -> the default layout, byte-identical to the pre-CFML output). The directive is the
// first line, but scan defensively and stop at the first real block.
fn scan_heaptop() -> i64 { unsafe {
    let mut p = 0;
    while p < BLEN {
        let mut e = p; while e < BLEN && bufat(e) != 10 { e = e + 1; }
        if e - p > 7 && bufat(p) == 104 && bufat(p + 1) == 101 && bufat(p + 2) == 97 && bufat(p + 3) == 112
            && bufat(p + 4) == 116 && bufat(p + 5) == 111 && bufat(p + 6) == 112 {
            let mut q = p + 7; while q < e && bufat(q) == 32 { q = q + 1; }
            let mut n: i64 = 0; while q < e && bufat(q) >= 48 && bufat(q) <= 57 { n = n * 10 + ((bufat(q) - 48) as i64); q = q + 1; }
            return n;
        }
        let mut c = p; while c < e && bufat(c) == 32 { c = c + 1; }
        if c < e && ((bufat(c) == 105 && bufat(c + 1) == 114) || (bufat(c) == 102 && bufat(c + 1) == 110)) { return 0; }
        p = e + 1;
    }
    0
} }

// B1 FRAME-BELOW: ld emits `framelo <fbase>` when it reserves a low frame region below a LARGE __heap (the dev
// self-image). Present -> pin the frame base at that fixed <4GB value (frame stays <4GB, only heap crosses 4GB);
// absent -> the historical frame-above layout (byte-identical for every small program).
fn scan_framelo() -> i64 { unsafe {
    let mut p = 0;
    while p < BLEN {
        let mut e = p; while e < BLEN && bufat(e) != 10 { e = e + 1; }
        if e - p > 7 && bufat(p) == 102 && bufat(p + 1) == 114 && bufat(p + 2) == 97 && bufat(p + 3) == 109
            && bufat(p + 4) == 101 && bufat(p + 5) == 108 && bufat(p + 6) == 111 {
            let mut q = p + 7; while q < e && bufat(q) == 32 { q = q + 1; }
            let mut n: i64 = 0; while q < e && bufat(q) >= 48 && bufat(q) <= 57 { n = n * 10 + ((bufat(q) - 48) as i64); q = q + 1; }
            return n;
        }
        let mut c = p; while c < e && bufat(c) == 32 { c = c + 1; }
        if c < e && ((bufat(c) == 105 && bufat(c + 1) == 114) || (bufat(c) == 102 && bufat(c + 1) == 110)) { return 0; }
        p = e + 1;
    }
    0
} }

fn main() {
    unsafe {
        BLEN = read_all();
        scan_frame();   // SLOT-FRAME FIX: size the per-call frame bump to the program's global max vreg
        // CFML per-program frame base: derive frame arena base / sp-save / memsz from the program's own arena top.
        // Default (no `heaptop` directive) reproduces the historical constants byte-for-byte. When the arenas exceed
        // 128MB, place the frame base 16MB above the (16MB-aligned) arena top, the sp-save 8MB below it, and size
        // memsz to frame_base + a 256MB frame region -- all under the ~2GB 32-bit-signed address ceiling.
        let arena_top = scan_heaptop();
        let flo = scan_framelo();
        let mut fbase: i64 = 134217728; let mut spsave: i64 = 67108864; let mut msz_byte = 64;
        // C-EAS-FBASE-CAP: `align16MB(arena_top) + 16MB` must stay <= 0x7F000000. Past that it exceeds i32::MAX and
        // WRAPS, and the wrap is silent AND backwards: p_memsz comes out SMALLER (measured: __heap 1984MB -> a
        // perfect 956-line witness; 2016MB -> p_memsz falls 4080MB to 2336MB and the program emits nothing). A
        // program whose arenas do not leave room for a frame region cannot be built correctly, so REFUSE rather
        // than emit a mis-sized image: `refuse` suppresses the final write, the chain yields an empty file, and the
        // stub checks in lib/scratch.sh and the gates report it loudly. Silent mis-sizing is the one outcome this
        // whole surface has taught not to allow.
        let mut refuse = 0;
        if BUFOVF == 1 { refuse = 1; }   // C-CF-BUF-8MB-OVERFLOW: an over-long input refuses loudly (empty output)
        // C-CF-FN-TABLE-1000: FNAME_OFF/FNAME_LEN/FADDR were [i32; 1000] and BOTH `fn`/`ir` sites wrote
        // FNAME_*[NF] with no bounds check, so a program defining more than ~1000 functions overflowed them into
        // the adjacent statics and produced a SILENTLY CORRUPT ELF -- measured: 931 fns fine, 1031 fns emitted a
        // header with e_phnum garbage (12545 / 29185 / 25601 / 257, varying with input) and no diagnostic at all.
        // Raised to 4000 AND bounds-checked, because raising a limit without a check only moves the cliff.
        // HONEST STATUS: this is HARDENING, not a fix for the corruption that prompted it. Raising the tables
        // and re-minting cf.hoist left the 1031-function case corrupting BYTE-IDENTICALLY, so the fn-table
        // diagnosis is REFUTED and the real cause is still unknown. What is landed here is a bounds check on
        // a write that genuinely had none, which is correct on its own terms; the guard has NOT been observed
        // to fire, because no probe yet reaches cf with >4000 functions (L6 truncates first).
        // Overflow now sets `refuse`, which suppresses the final write exactly as the fbase cap does, so the
        // chain yields an EMPTY file and the stub checks in the gates catch it loudly.
        if flo > 0 {
            unsafe { FB = 1; }
            // B1 FRAME-BELOW: frame pinned at the fixed <4GB base; __heap lives ABOVE the frame (ld-reserved).
            // p_memsz spans [0, arena_top] so the >4GB heap is mapped; frame addrs stay <4GB (no per-op widening).
            fbase = flo; spsave = flo - 8388608;
            let mut mszfb: i64 = arena_top + 268435456; if mszfb > 68719476736 { mszfb = 68719476736; }
            msz_byte = (mszfb / 16777216) as i32;
        } else if arena_top > 0 {
            fbase = ((arena_top + 16777215) / 16777216) * 16777216 + 16777216;   // 16MB-align up, + 16MB pad
            if fbase > 8589934592 { refuse = 1; }   /* C-ARENA-LIFT: cap = 3968MB (fbase, keeper-approved 2026-09-12 raise from 3900MB=4089446400; the verified-safe ceiling, prior inside the cf-documented verified-safe zone <3968MB, ~180MB below the 32-bit mov-ebx-fbase hard ceiling ~4080MB). 2026-09-02: the prior ~2560MB cap was OVER-CONSERVATIVE by ~1400MB and its "~2950MB segv" was REFUTED by the cf-arena-ceiling locale -- the real consumer (s1.rz doing the deep //@S5TYPED self-emit, the arena-heaviest op) reaches fn main clean at fbase up to 3968MB (probe-cap limit, no fault). The genuine hard ceiling is the `mov ebx, fbase` load below (32-bit, zero-extends into rbx): correct only for fbase < 4GB (~4080MB). 3GB sits 1GB below that hard limit and ~950MB inside the verified-safe zone. Floor programs (arena_top==0) skip this whole block, so sub-threshold ELFs stay byte-identical; only programs with fbase in (old-cap, 3GB] change (they previously REFUSED). Frame-exhaustion (depth x FRAME_SZ vs p_memsz) is separately guarded by want_b (7168 frames) + the arena-headroom gate. Past 3GB still a LOUD refuse. */
            if fbase < 0 { refuse = 1; }                                          // already wrapped in the align step
            spsave = fbase - 8388608;                                            // 8MB below the frame base, above the arena
            // NOTE (C-EAS, 2026-08-24): this adaptive msz_byte is now DEAD -- it computes fbase/16MB + 16, while the
            // frame-budget clamp below computes fbase/16MB + at least 56 and takes the max, so it can never govern.
            // Retained only because `fbase`/`spsave` above ARE live. cf therefore carried TWO disagreeing ceilings
            // (0x7F000000 here, 255 units below); the guard above now makes 0x7F000000 a REFUSAL, not a silent cap.
            let mut msz: i64 = fbase + 268435456;                                    // want a 256MB frame region above the arenas
            if msz > 68719476736 { msz = 68719476736; }   /* ~4032MB, under p_memsz max (255*16MB) and the 4GB bound */
            msz_byte = ((msz / 16777216) & 255) as i32;                                  // p_memsz byte3 (msz is 16MB-aligned)
        }
        // C-CFML-MEMSZ-REGRESSION: the adaptive branch above REPLACED the default rather than raising it, and
        // `fbase + 256MB` is SMALLER than the 1GB default for any arena_top below ~768MB -- so a program whose
        // arenas crossed ld's `heaptop` trigger got a SMALLER mapped image than one that did not (measured:
        // __str_arena 128MB -> p_memsz 416MB). Asking for more memory yielded less. The adaptive value is meant
        // to be an increase; make that explicit.
        if msz_byte < 64 { msz_byte = 64; }
        // C-CF-FRAME-CAPACITY: scan_frame() sizes FRAME_SZ per program, but p_memsz was a CONSTANT that never
        // consulted it -- so a program's usable recursion depth was an accident of the default: (memsz - fbase)
        // / FRAME_SZ. gen2 (max vreg 39461 -> FRAME_SZ 312KB) got ~2940 frames and then walked off the end of
        // its own LOAD segment, with no check on either side. Give every program the SAME depth budget instead.
        // Arithmetic in 16MB units (p_memsz is byte3-encoded, hence 16MB-quantized) to stay clear of i32
        // overflow: FRAME_SZ*4096 / 16777216 == FRAME_SZ/4096, and FRAME_SZ is already 4096-aligned.
        // The budget is 7168 frames, which is not arbitrary: it is exactly what the 1GB default already gave a
        // program at the FRAME_SZ floor ((1024-128)MB / 128KB = 7168). So this generalises the depth every small
        // program ALREADY had to every program, rather than inventing a number. It also lands a floor program on
        // exactly 8+56 = 64 units, so its ELF stays BYTE-IDENTICAL to the pre-change output (the CFML contract);
        // only programs whose FRAME_SZ is above the floor -- the ones that were silently short of depth -- move.
        // FRAME_SZ is 4096-aligned, so FRAME_SZ*7168/16777216 == (FRAME_SZ/4096)*7/4, which keeps every
        // intermediate small (gen2: 78*7/4 = 136) and clear of i32 overflow.
        let mut want_b = (fbase / 16777216) as i32 + (FRAME_SZ / 4096) * 7 / 4;   // fbase + 7168 frames, in 16MB units
        // C-CF-MEMSZ-CLAMP: the ceiling is 255 units (4080MB), not the 127 (2032MB) this used to clamp at. The old
        // bound was asserted as "the ~2GB 32-bit-SIGNED address ceiling", but nothing here is signed: the frame
        // pointer is `mov ebx,imm32` / `add ebx,imm32` (line 215/230), 32-bit ops that ZERO-extend into rbx, so the
        // real bound is 4GB unsigned. Probed to confirm rather than reasoned only: forced p_memsz of 2304 / 2880 /
        // 3520 / 4000 / 4080 MB all run gen2 correctly (956-line witness, trivial input ok).
        // 255 units is simultaneously the byte-3 encoding maximum and, at a 0x400000 base, a segment ending at
        // 4084MB -- i.e. the encoding ceiling and the ebx ceiling COINCIDE. Widening p_memsz past one byte would
        // therefore buy nothing: it would only describe memory the emitted code cannot address. The byte encoding
        // is adequate; the clamp was the defect. (16MB granularity is likewise not a defect -- p_memsz is a
        // demand-zero reservation, so rounding a reservation up costs address space and nothing else.)
        // C-EAS-C2 lifted: no 255 clamp -- p_memsz is a 3-byte unit field now and rbx is 64-bit. A sanity ceiling
        // remains so a runaway FRAME_SZ cannot ask for a terabyte: 4095 units = 64GB, far above any real program and
        // still an order of magnitude under the 47-bit user address space.
        if want_b > 4095 { want_b = 4095; }
        if want_b > msz_byte { msz_byte = want_b; }
        OUT[0] = 127; OUT[1] = 69; OUT[2] = 76; OUT[3] = 70; OUT[4] = 2; OUT[5] = 1; OUT[6] = 1;
        OUT[16] = 2; OUT[18] = 62; OUT[20] = 1; OUT[24] = 120; OUT[26] = 64; OUT[32] = 64;
        OUT[52] = 64; OUT[54] = 56; OUT[56] = 1; OUT[64] = 1; OUT[68] = 7; OUT[82] = 64; OUT[90] = 64;
        // C-EAS-C2: p_memsz was byte 3 ONLY, capping it at 255 units = 4080MB -- which coincided with the 4GB
        // `ebx` bound, so widening it alone would have been motion without effect. With the frame pointer now
        // 64-bit (above), the field is the binding limit, so write units into bytes 3..5. memsz = units << 24, so
        // byte3 = units & 255, byte4 = (units>>8) & 255, byte5 = (units>>16) & 255 -- no i64 arithmetic needed.
        OUT[107] = (msz_byte & 255) as u8; OUT[108] = ((msz_byte >> 8) & 255) as u8; OUT[109] = ((msz_byte >> 16) & 255) as u8; OUT[113] = 16;   /* p_memsz byte3 (CFML per-program: default 0x40 = 1GB; adaptive = frame_base+256MB >> 24). memsz>filesz is demand-zero. */
        OUTN = 120; BRZ_SP = 0; LOOP_SP = 0; CNT_SP = 0; BRK_SP = 0; NLBL = 0; NGT = 0; NF = 0; NCF = 0;
        let mut inmain = 1; let mut pc = 0;
        if spsave > 2130706432 { eb(184); ed(spsave as i32); eb(72); eb(137); eb(32); } else { eb(72); eb(137); eb(36); eb(37); ed(spsave as i32); }   /* K0: mov [spsave], rsp (>2GB: mov eax,spsave[zx] + mov [rax],rsp) -- save the entry stack pointer (argc/argv/envp at [rsp]) to a free word above the arenas, below the frame base (CFML per-program; default 64MB) */
        if fbase >= 4294967296 { eb(72); eb(187); ed((fbase % 4294967296) as i32); ed((fbase / 4294967296) as i32); } else { eb(187); ed(fbase as i32); }   /* B1-3: fbase<4GB -> mov ebx,imm32 (BYTE-IDENTICAL, zero-extends); fbase>=4GB -> movabs rbx,imm64 (48 BB + imm64) -- the 64-bit frame base for >4GB heaps. Conditional keeps sub-4GB programs unperturbed. */
        let mut p = 0;
        while p < BLEN {
            let mut e = p; while e < BLEN && bufat(e) != 10 { e = e + 1; }
            let mut c = p; while c < e && bufat(c) == 32 { c = c + 1; }
            if c >= e { p = e + 1; }
            else if bufat(c) == 105 && bufat(c + 1) == 114 {
                let mut nmoff = c + 3; while nmoff < e && bufat(nmoff) == 32 { nmoff = nmoff + 1; }
                if NF >= 16384 { refuse = 1; } else { FNAME_OFF[NF as usize] = nmoff; FNAME_LEN[NF as usize] = namelen(nmoff, e); FADDR[NF as usize] = OUTN; NF = NF + 1; }
                inmain = 1; pc = 0; NPUSH = 0; FGEN = FGEN + 1; p = e + 1;   // R2: new fn -> invalidate the @reg map
            }
            else if bufat(c) == 102 && bufat(c + 1) == 110 {
                let mut nmoff = c + 3; while nmoff < e && bufat(nmoff) == 32 { nmoff = nmoff + 1; }
                if NF >= 16384 { refuse = 1; } else { FNAME_OFF[NF as usize] = nmoff; FNAME_LEN[NF as usize] = namelen(nmoff, e); FADDR[NF as usize] = OUTN; NF = NF + 1; }
                inmain = 0; pc = 0; FGEN = FGEN + 1;
                eb(72); eb(129); eb(195); ed(FRAME_SZ);   // add RBX, FRAME_SZ -- REX.W. C-EAS-C1: this was `add ebx`,
                // a 32-bit add that wraps at 4GB, and it was the ONLY thing confining the frame pointer. `mov ebx,fbase`
                // zero-extends and is fine while fbase < 4GB, and slot access is already [rbx + disp32] on the full
                // 64-bit base -- so the frame region, not the addressing mode, was the 4GB bound.
                scan_saves(e + 1);                // R2 micro-opt: push only the callee-saved regs this fn uses
                NPUSH = 0;                        // CL-A1: incoming stack args sit above these pushes
                if USED_R12 == 1 { eb(65); eb(84); NPUSH = NPUSH + 1; }   // push r12
                if USED_R13 == 1 { eb(65); eb(85); NPUSH = NPUSH + 1; }   // push r13
                p = e + 1;
            }
            else if bufat(c) == 101 && bufat(c + 1) == 110 && bufat(c + 2) == 100 && (c + 3 >= e || bufat(c + 3) <= 32) { p = e + 1; }
            else if bufat(c) == 104 && bufat(c + 1) == 101 && bufat(c + 2) == 97 && bufat(c + 3) == 112 { p = e + 1; }   // skip the ld `heaptop` frame-base directive (already consumed by scan_heaptop)
            else if bufat(c) == 114 && bufat(c + 1) == 101 && bufat(c + 2) == 116 {
                let mut sp = c + 3; while sp < e && bufat(sp) != 118 { sp = sp + 1; }
                let mut a = 0; if sp < e { let mut sq = sp + 1; while sq < e && bufat(sq) >= 48 && bufat(sq) <= 57 { a = a * 10 + (bufat(sq) - 48); sq = sq + 1; } }
                if inmain == 1 { let ra = vreg_of(a); if ra == 0 { eb(139); eb(187); ed(a * 8); } else { if rrexb(ra) == 1 { eb(65); } eb(139); eb(248 + rrm(ra)); }   // mov edi, r/m[a] (exit code, reg-aware)
                    eb(184); eb(60); eb(0); eb(0); eb(0); eb(15); eb(5); }
                else { load_rax(a); eb(72); eb(129); eb(235); ed(FRAME_SZ); if USED_R13 == 1 { eb(65); eb(93); } if USED_R12 == 1 { eb(65); eb(92); } eb(195); }   // mov rax,[N]; sub ebx,FRAME_SZ; [pop r13; pop r12]; ret (slot-frame fix)
                p = e + 1;
            } else {
                let mut dv = 0 - 1; let mut o = c;
                if bufat(c) == 118 && bufat(c + 1) >= 48 && bufat(c + 1) <= 57 {
                    let mut q = c + 1; dv = 0; while q < e && bufat(q) >= 48 && bufat(q) <= 57 { dv = dv * 10 + (bufat(q) - 48); q = q + 1; }
                    if q < e && bufat(q) == 58 { let mut eq = q; while eq < e && bufat(eq) != 61 { eq = eq + 1; }
                        let mut ap = q;                                  // R2: scan ':' .. '=' for @rNN and record the value's register
                        while ap < eq {
                            if bufat(ap) == 64 && bufat(ap + 1) == 114 {
                                let d0 = bufat(ap + 2); let mut code = 0;
                                if d0 == 115 { code = 5; } else if d0 == 100 { code = 6; } else if d0 == 56 { code = 7; } else if d0 == 57 { code = 8; }
                                else if d0 == 49 { let d1 = bufat(ap + 3); if d1 == 48 { code = 3; } else if d1 == 49 { code = 4; } else if d1 == 50 { code = 1; } else if d1 == 51 { code = 2; } }
                                if code > 0 && dv >= 0 && dv < 40000 { VREG[dv as usize] = code; VGEN[dv as usize] = FGEN; }
                                ap = eq;
                            } else { ap = ap + 1; }
                        }
                        o = eq + 1; while o < e && bufat(o) == 32 { o = o + 1; } }
                    else { dv = 0 - 1; o = c; }
                }
                let mut oe = o; while oe < e && bufat(oe) > 32 { oe = oe + 1; }
                let olen = oe - o; let o0 = bufat(o); let mut o1 = 0; let mut o2 = 0; if o + 1 < e { o1 = bufat(o + 1); } if o + 2 < e { o2 = bufat(o + 2); }
                let mut a = 0; let mut b2 = 0; let mut c3 = 0; let mut d4 = 0; let mut e5 = 0; let mut f6 = 0; let mut ns = 0; let mut sp = oe;
                // CL-A1: the cap was `ns < 6`, so a 7th operand was never even READ. Now every operand is counted
                // and the overflow is stashed in XARG for the stack-passing path below. Past XARG's capacity the
                // build REFUSES rather than emitting a call that silently drops arguments.
                while sp < e {
                    if bufat(sp) == 118 && bufat(sp + 1) >= 48 && bufat(sp + 1) <= 57 {
                        let mut sq = sp + 1; let mut sv2 = 0; while sq < e && bufat(sq) >= 48 && bufat(sq) <= 57 { sv2 = sv2 * 10 + (bufat(sq) - 48); sq = sq + 1; }
                        if ns == 0 { a = sv2; } else if ns == 1 { b2 = sv2; } else if ns == 2 { c3 = sv2; } else if ns == 3 { d4 = sv2; } else if ns == 4 { e5 = sv2; } else if ns == 5 { f6 = sv2; }
                        else { if ns - 6 < 40 { XARG[(ns - 6) as usize] = sv2; } else { refuse = 1; } }
                        ns = ns + 1; sp = sq;
                    } else { sp = sp + 1; }
                }
                if dv >= 0 {
                  let is64 = bufat(oe - 1) == 56;                                  // op ends in '8' -> i64 variant
                  let isconv = o0 == 99 && o1 == 111 && o2 == 110 && o + 3 < e && bufat(o + 3) == 118;   // conv/convs (4th char 'v') -- NOT const (4th char 's')
                  if is64 == 1 || isconv == 1 {
                    if isconv == 1 { if olen == 5 { eb(72); eb(99); eb(131); ed(a * 8); store_rax(dv); } else { eb(139); eb(131); ed(a * 8); store_rax(dv); } }   // convs: movsxd (sign); conv: mov eax (zero-extend low 32)
                    else if o0 == 97 && o1 == 100 { load_rax(a); op_rax(3, b2); store_rax(dv); }   // add8: add rax,[B]
                    else if o0 == 115 && o1 == 117 { load_rax(a); op_rax(43, b2); store_rax(dv); } // sub8
                    else if o0 == 97 && o1 == 110 { load_rax(a); op_rax(35, b2); store_rax(dv); }  // and8
                    else if o0 == 111 { load_rax(a); op_rax(11, b2); store_rax(dv); }              // or8
                    else if o0 == 120 { load_rax(a); op_rax(51, b2); store_rax(dv); }              // xor8
                    else if o0 == 109 && o1 == 117 { load_rax(a); imul_rax(b2); store_rax(dv); }  // mul8: imul rax,[B]
                    else if o0 == 100 && o1 == 105 { load_rax(a); eb(72); eb(153); idiv_v(b2, 1); store_rax(dv); }  // div8: cqo; idiv qword[B]; rax
                    else if o0 == 109 && o1 == 111 { load_rax(a); eb(72); eb(153); idiv_v(b2, 1); store_rdx(dv); }  // mod8: ...; rdx
                    else if o0 == 115 && o1 == 104 && o2 == 108 { load_rax(a); load_rcx(b2); eb(72); eb(211); eb(224); store_rax(dv); }    // shl8: shl rax,cl
                    else if o0 == 115 && o1 == 104 && o2 == 114 { load_rax(a); load_rcx(b2); eb(72); eb(211); eb(248); store_rax(dv); }    // shr8: sar rax,cl
                    else if o0 == 115 && o1 == 104 && o2 == 117 { load_rax(a); load_rcx(b2); eb(72); eb(211); eb(232); store_rax(dv); }    // shu8: shr rax,cl
                    else if o0 == 108 && o1 == 111 { load_rax(a); eb(72); eb(139); eb(0); store_rax(dv); }              // load8: mov rax,[rax]
                    else {
                        let mut cc = 0;
                        if o0 == 108 && o1 == 116 { if olen == 3 { cc = 156; } else { cc = 146; } }   // lt8 setl / ltu8 setb
                        else if o0 == 108 && o1 == 101 { if olen == 3 { cc = 158; } else { cc = 150; } }
                        else if o0 == 103 && o1 == 116 { if olen == 3 { cc = 159; } else { cc = 151; } }
                        else if o0 == 103 && o1 == 101 { if olen == 3 { cc = 157; } else { cc = 147; } }
                        else if o0 == 101 && o1 == 113 { cc = 148; } else if o0 == 110 && o1 == 101 { cc = 149; }
                        load_rax(a); op_rax(59, b2); eb(15); eb(cc); eb(192); eb(15); eb(182); eb(192); store_eax(dv);   // cmp rax,[B]; setcc al; movzx eax,al
                    }
                  }
                  else if o0 == 99 && o1 == 111 && olen == 5 {
                        let mut vp = oe; while vp < e && bufat(vp) == 32 { vp = vp + 1; }
                        let mut neg = 0; if vp < e && bufat(vp) == 45 { neg = 1; vp = vp + 1; }
                        // C-ARENA-C3E: detect a literal in [2^31, 2^32). Accumulating in i32 it wraps NEGATIVE exactly
                        // once on crossing 2^31 and stays there below 2^32, so `big` is an exact test over that range.
                        // Such a constant must be ZERO-extended, not sign-extended.
                        // C-ARENA-C3E, made WIDTH-AGNOSTIC. The old test relied on i32 accumulation wrapping
                        // negative at 2^31 (`if val < 0`), which is fragile: a compiler that widens `val` to i64 (SEM0)
                        // never wraps and mis-classified [2^31,2^32) consts (took store_imm -> sign-extend -> wrong).
                        // Accumulate in i64 and detect the SAME range by VALUE (>= 2^31), then store the LOW 32 bits
                        // exactly as before -- store_imm_zx (zero-extend) for >=2^31, store_imm (sign-extend) below.
                        // >2^32 stays TRUNCATED to the low 32 bits (unchanged: the frame-below >4GB heap scheme relies
                        // on the low-32 base + the FB=1 64-bit-operand load/store path for the high bits).
                        let mut val: i64 = 0;
                        while vp < e && bufat(vp) >= 48 && bufat(vp) <= 57 { val = val * 10 + ((bufat(vp) - 48) as i64); vp = vp + 1; }
                        if neg == 1 { val = 0 - val; }
                        if val >= 2147483648 { store_imm_zx(dv, (val & 4294967295) as i32); } else { store_imm(dv, val as i32); }
                    }
                    else if o0 == 112 && o1 == 97 {
                        store_param(dv, pc); pc = pc + 1;   // mov r/m[dv], rdi/rsi/rdx/rcx/r8/r9
                    }
                    else if o0 == 99 && o1 == 97 && olen == 4 {                     /* call NAME (not calli, which is olen 5) */
                        let mut nmoff = oe; while nmoff < e && bufat(nmoff) == 32 { nmoff = nmoff + 1; } let nmlen = namelen(nmoff, e);
                        // CL-A1 STACK ARGS. Arguments 7..N are pushed RIGHT TO LEFT so argument 7 ends up at
                        // [rsp+0] when the call executes, which is what System V specifies. The pushes happen
                        // BEFORE the register marshalling so that a pushed argument cannot clobber a register
                        // argument that has already been loaded. A `push` is 8 bytes, so an odd number of them
                        // would leave rsp 8 mod 16 at the call; one extra `sub rsp,8` restores the 16-byte
                        // alignment the ABI requires. The caller cleans the whole area up after the call.
                        let xn = ns - 6;
                        let mut pad = 0;
                        if xn > 0 { if xn % 2 == 1 { pad = 8; sub_rsp(8); } let mut xi = xn - 1; while xi >= 0 { push_slot(XARG[xi as usize]); xi = xi - 1; } }
                        if ns >= 1 { marshal(a, 0, 187, 248); } if ns >= 2 { marshal(b2, 0, 179, 240); } if ns >= 3 { marshal(c3, 0, 147, 208); } if ns >= 4 { marshal(d4, 0, 139, 200); } if ns >= 5 { marshal(e5, 4, 131, 192); } if ns >= 6 { marshal(f6, 4, 139, 200); }   // mov rdi/rsi/rdx/rcx/r8/r9, [arg]
                        eb(232); CF_SITE[NCF as usize] = OUTN; CF_OFF[NCF as usize] = nmoff; CF_LEN[NCF as usize] = nmlen; NCF = NCF + 1; ed(0);
                        if xn > 0 { add_rsp(xn * 8 + pad); }
                        store_eax(dv);
                    }
                    else if o0 == 115 && o1 == 121 {
                        load_eax(a); if ns >= 2 { marshal(b2, 0, 187, 248); } if ns >= 3 { marshal(c3, 0, 179, 240); } if ns >= 4 { marshal(d4, 0, 147, 208); } if ns >= 5 { marshal(e5, 4, 147, 208); } if ns >= 6 { marshal(f6, 4, 131, 192); }   // K1: 64-bit syscall args (mov rdi/rsi/rdx/r10/r8, [arg]) -- r10 is the syscall 4th-arg reg (not rcx); enables fork/execve/wait4/pipe for the rzc build driver. 64-bit is behavior-identical for the existing sub-4GB fd/buf/len users.
                        eb(15); eb(5); store_eax(dv);
                    }
                    else if o0 == 102 && o1 == 97 && o2 == 100 {                   /* faddr NAME -> mov eax, abs-addr (backpatched) */
                        let mut nmoff = oe; while nmoff < e && bufat(nmoff) == 32 { nmoff = nmoff + 1; } let nmlen = namelen(nmoff, e);
                        eb(184); FA_SITE[NFA as usize] = OUTN; FA_OFF[NFA as usize] = nmoff; FA_LEN[NFA as usize] = nmlen; NFA = NFA + 1; ed(0);
                        store_rax(dv);
                    }
                    else if o0 == 99 && o1 == 97 && o2 == 108 {                     /* calli code env args -> indirect call __closure(env, args..) */
                        if ns >= 2 { marshal(b2, 0, 187, 248); }        /* rdi = env */
                        if ns >= 3 { marshal(c3, 0, 179, 240); }        /* rsi = arg0 */
                        if ns >= 4 { marshal(d4, 0, 147, 208); }        /* rdx = arg1 */
                        load_rax(a); eb(255); eb(208);                             /* mov rax,[code]; call rax */
                        store_rax(dv);
                    }
                    else if o0 == 114 && o1 == 101 { eb(141); eb(131); ed(a * 8); store_eax(dv); }
                    else if o0 == 103 && o1 == 101 && o2 == 116 && olen == 3 { if FB == 1 { load_rax(a); } else { load_eax(a); } load_ecx(b2); eb(139); eb(4); eb(200); store_eax(dv); }
                    else if o0 == 103 && o1 == 101 && o2 == 116 && olen == 4 { if FB == 1 { load_rax(a); } else { load_eax(a); } load_ecx(b2); eb(15); eb(182); eb(4); eb(8); store_eax(dv); }
                    else if o0 == 108 && o1 == 111 && olen == 4 { if FB == 1 { load_rax(a); } else { load_eax(a); } eb(139); eb(0); store_eax(dv); }
                    else if o0 == 108 && o1 == 111 && olen == 5 { if FB == 1 { load_rax(a); } else { load_eax(a); } eb(15); eb(182); eb(0); store_eax(dv); }
                    else {
                        load_eax(a);
                        if o0 == 97 && o1 == 100 { op_eax(3, b2); store_eax(dv); }
                        else if o0 == 115 && o1 == 117 { op_eax(43, b2); store_eax(dv); }
                        else if o0 == 97 && o1 == 110 { op_eax(35, b2); store_eax(dv); }
                        else if o0 == 111 { op_eax(11, b2); store_eax(dv); }
                        else if o0 == 120 { op_eax(51, b2); store_eax(dv); }
                        else if o0 == 109 && o1 == 117 { imul_eax(b2); store_eax(dv); }
                        else if o0 == 100 && o1 == 105 { eb(153); idiv_v(b2, 0); store_eax(dv); }
                        else if o0 == 109 && o1 == 111 { eb(153); idiv_v(b2, 0); store_edx(dv); }
                        else if o0 == 100 && o1 == 117 { eb(49); eb(210); div6_v(b2); store_eax(dv); }
                        else if o0 == 117 && o1 == 100 { eb(49); eb(210); div6_v(b2); store_edx(dv); }
                        else if o0 == 115 && o1 == 104 && o2 == 108 { load_ecx(b2); eb(211); eb(224); store_eax(dv); }
                        else if o0 == 115 && o1 == 104 && o2 == 114 { load_ecx(b2); eb(211); eb(248); store_eax(dv); }
                        else if o0 == 115 && o1 == 104 && o2 == 117 { load_ecx(b2); eb(211); eb(232); store_eax(dv); }
                        else {
                            let mut cc = 0;
                            if o0 == 108 && o1 == 116 { if olen == 3 { cc = 146; } else { cc = 156; } }
                            else if o0 == 108 && o1 == 101 { if olen == 3 { cc = 150; } else { cc = 158; } }
                            else if o0 == 103 && o1 == 116 { if olen == 3 { cc = 151; } else { cc = 159; } }
                            else if o0 == 103 && o1 == 101 { if olen == 3 { cc = 147; } else { cc = 157; } }
                            else if o0 == 101 && o1 == 113 { cc = 148; } else if o0 == 110 && o1 == 101 { cc = 149; }
                            op_eax(59, b2); eb(15); eb(cc); eb(192); eb(15); eb(182); eb(192); store_eax(dv);
                        }
                    }
                } else {
                    if o0 == 112 && o1 == 117 && olen == 3 { if FB == 1 { load_rax(a); } else { load_eax(a); } load_ecx(b2); load_edx(c3); eb(137); eb(20); eb(200); }
                    else if o0 == 112 && o1 == 117 && olen == 4 { if FB == 1 { load_rax(a); } else { load_eax(a); } load_ecx(b2); load_edx(c3); eb(136); eb(20); eb(8); }
                    else if o0 == 115 && o1 == 116 && olen == 5 { if FB == 1 { load_rax(a); } else { load_eax(a); } load_edx(b2); eb(137); eb(16); }
                    else if o0 == 115 && o1 == 116 && olen == 6 && bufat(oe - 1) == 56 { load_rax(a); marshal(b2, 0, 147, 208); eb(72); eb(137); eb(16); }   // store8: mov rdx,[b]; mov [rax],rdx (64-bit)
                    else if o0 == 115 && o1 == 116 && olen == 6 { if FB == 1 { load_rax(a); } else { load_eax(a); } load_edx(b2); eb(136); eb(16); }   // storeb: mov [rax],dl
                    else if o0 == 98 && o1 == 114 && o2 == 122 { load_eax(a); eb(133); eb(192); eb(15); eb(132); BRZ_SITE[BRZ_SP as usize] = OUTN; BRZ_SP = BRZ_SP + 1; ed(0); }
                    else if o0 == 98 && o1 == 114 && o2 == 99 {                     /* brc <cmp> vA vB -- fused compare + branch-if-false to pool */
                        let mut cmpo = oe; while cmpo < e && bufat(cmpo) == 32 { cmpo = cmpo + 1; }
                        let cw0 = bufat(cmpo); let cw1 = bufat(cmpo + 1); let mut cwl = 2; if cmpo + 2 < e && bufat(cmpo + 2) == 117 { cwl = 3; }
                        load_eax(a); op_eax(59, b2);
                        let mut jc = 0;
                        if cw0 == 108 && cw1 == 116 { if cwl == 2 { jc = 141; } else { jc = 131; } }
                        else if cw0 == 108 && cw1 == 101 { if cwl == 2 { jc = 143; } else { jc = 135; } }
                        else if cw0 == 103 && cw1 == 116 { if cwl == 2 { jc = 142; } else { jc = 134; } }
                        else if cw0 == 103 && cw1 == 101 { if cwl == 2 { jc = 140; } else { jc = 130; } }
                        else if cw0 == 101 && cw1 == 113 { jc = 133; }
                        else if cw0 == 110 && cw1 == 101 { jc = 132; }
                        eb(15); eb(jc); BRZ_SITE[BRZ_SP as usize] = OUTN; BRZ_SP = BRZ_SP + 1; ed(0);
                    }
                    else if o0 == 112 && o1 == 111 { BRZ_SP = BRZ_SP - 1; patch(BRZ_SITE[BRZ_SP as usize], OUTN); }
                    else if o0 == 108 && o1 == 111 { LOOP_POS[LOOP_SP as usize] = OUTN; LOOP_BD[LOOP_SP as usize] = BRZ_SP; LOOP_CD[LOOP_SP as usize] = CNT_SP; LOOP_KD[LOOP_SP as usize] = BRK_SP; LOOP_SP = LOOP_SP + 1; }
                    else if o0 == 101 && o1 == 110 && o2 == 100 { LOOP_SP = LOOP_SP - 1; while BRZ_SP > LOOP_BD[LOOP_SP as usize] { BRZ_SP = BRZ_SP - 1; patch(BRZ_SITE[BRZ_SP as usize], OUTN); } while BRK_SP > LOOP_KD[LOOP_SP as usize] { BRK_SP = BRK_SP - 1; patch(BRK_SITE[BRK_SP as usize], OUTN); } }
                    else if o0 == 106 && o1 == 109 { eb(233); let rel = LOOP_POS[(LOOP_SP - 1) as usize] - (OUTN + 4); ed(rel); }
                    else if o0 == 98 && o1 == 114 && o2 == 107 { eb(233); BRK_SITE[BRK_SP as usize] = OUTN; BRK_SP = BRK_SP + 1; ed(0); }
                    else if o0 == 99 && o1 == 110 && o2 == 116 { eb(233); CNT_SITE[CNT_SP as usize] = OUTN; CNT_SP = CNT_SP + 1; ed(0); }
                    else if o0 == 99 && o1 == 111 && o2 == 110 { while CNT_SP > LOOP_CD[(LOOP_SP - 1) as usize] { CNT_SP = CNT_SP - 1; patch(CNT_SITE[CNT_SP as usize], OUTN); } }
                    else if o0 == 108 && o1 == 97 { let mut nmoff = oe; while nmoff < e && bufat(nmoff) == 32 { nmoff = nmoff + 1; } LBL_OFF[NLBL as usize] = nmoff; LBL_LEN[NLBL as usize] = namelen(nmoff, e); LBL_POS[NLBL as usize] = OUTN; NLBL = NLBL + 1; }
                    else if o0 == 103 && o1 == 111 && o2 == 116 { let mut nmoff = oe; while nmoff < e && bufat(nmoff) == 32 { nmoff = nmoff + 1; } eb(233); if NGT >= 16000 { refuse = 1; } else { GT_SITE[NGT as usize] = OUTN; GT_OFF[NGT as usize] = nmoff; GT_LEN[NGT as usize] = namelen(nmoff, e); NGT = NGT + 1; } ed(0); }
                }
                p = e + 1;
            }
        }
        let mut i = 0;
        while i < NCF {
            let mut j = 0;
            while j < NF {
                if CF_LEN[i as usize] == FNAME_LEN[j as usize] {
                    let mut m = 1; let mut ko = 0; while ko < CF_LEN[i as usize] { if bufat(CF_OFF[i as usize] + ko) != bufat(FNAME_OFF[j as usize] + ko) { m = 0; } ko = ko + 1; }
                    if m == 1 { patch(CF_SITE[i as usize], FADDR[j as usize]); j = NF; }
                }
                j = j + 1;
            }
            i = i + 1;
        }
        i = 0;
        while i < NGT {
            let mut j = 0;
            while j < NLBL {
                if GT_LEN[i as usize] == LBL_LEN[j as usize] {
                    let mut m = 1; let mut ko = 0; while ko < GT_LEN[i as usize] { if bufat(GT_OFF[i as usize] + ko) != bufat(LBL_OFF[j as usize] + ko) { m = 0; } ko = ko + 1; }
                    if m == 1 { patch(GT_SITE[i as usize], LBL_POS[j as usize]); j = NLBL; }
                }
                j = j + 1;
            }
            i = i + 1;
        }
        i = 0;
        while i < NFA {                                                            /* faddr: absolute address 0x400000 + fn file offset */
            let mut j = 0;
            while j < NF {
                if FA_LEN[i as usize] == FNAME_LEN[j as usize] {
                    let mut m = 1; let mut ko = 0; while ko < FA_LEN[i as usize] { if bufat(FA_OFF[i as usize] + ko) != bufat(FNAME_OFF[j as usize] + ko) { m = 0; } ko = ko + 1; }
                    if m == 1 { let ad = 4194304 + FADDR[j as usize]; let st = FA_SITE[i as usize]; OUT[st as usize] = (ad & 255) as u8; OUT[(st + 1) as usize] = ((ad >> 8) & 255) as u8; OUT[(st + 2) as usize] = ((ad >> 16) & 255) as u8; OUT[(st + 3) as usize] = ((ad >> 24) & 255) as u8; j = NF; }
                }
                j = j + 1;
            }
            i = i + 1;
        }
        let n = OUTN;
        OUT[96] = (n & 255) as u8; OUT[97] = ((n >> 8) & 255) as u8; OUT[98] = ((n >> 16) & 255) as u8; OUT[99] = ((n >> 24) & 255) as u8;
        // C-EAS-FBASE-CAP: refuse loudly (write nothing) rather than emit a silently mis-sized image.
        if refuse == 0 { __syscall(1, 1, &OUT[0], n); }
    }
}
