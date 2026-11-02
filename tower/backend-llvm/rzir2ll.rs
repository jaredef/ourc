// rzir -> LLVM IR bridge, in the SEM0-compilable Rust subset (compiled by the tower, NOT python).
// Reads rzir on stdin, writes LLVM IR text on stdout. alloca-everything model: every SSA value vN gets an entry
// alloca %vN.s; defs store, uses load; `ref vN` = the slot address. LLVM mem2reg(-O2) promotes to SSA+phi.
// Structured CF (brz/pool = if; loop/brz/cont/jmp/endloop = loop) via forward-ref labels + pending/loop stacks.
use std::io::Read;

fn streq(a: &str, b: &str) -> bool {
    let ab = a.as_bytes(); let bb = b.as_bytes();
    if ab.len() != bb.len() { return false; }
    let mut i = 0; while i < ab.len() { if ab[i] != bb[i] { return false; } i += 1; } true
}
fn split_ws(s: &str) -> Vec<String> {
    let b = s.as_bytes(); let mut out: Vec<String> = Vec::new(); let mut cur = String::new(); let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if c == 32 || c == 9 || c == 13 { if cur.len() > 0 { out.push(cur.clone()); cur = String::new(); } }
        else { cur.push(c as char); }
        i += 1;
    }
    if cur.len() > 0 { out.push(cur); } out
}
fn split_lines(s: &str) -> Vec<String> {
    let b = s.as_bytes(); let mut out: Vec<String> = Vec::new(); let mut cur = String::new(); let mut i = 0;
    while i < b.len() { let c = b[i]; if c == 10 { out.push(cur.clone()); cur = String::new(); } else { cur.push(c as char); } i += 1; }
    if cur.len() > 0 { out.push(cur); } out
}
// If `s` is a slot token `vN` or `vN:` return `vN`, else "". Used to alloca EVERY referenced slot.
fn vtok(s: &str) -> String {
    let b = s.as_bytes(); let n = b.len();
    if n < 2 { return String::new(); }
    if b[0] != 118 { return String::new(); }
    let mut end = n; if b[n-1] == 58 { end = n - 1; }
    if end <= 1 { return String::new(); }
    let mut o = String::from("v"); let mut i = 1;
    if i < end && b[i] == 45 { o.push('-'); i += 1; }   // v-1: emit_s9's dropped-value sentinel (negative vreg). cf reads a garbage slot; alloca it so the load is valid (LLVM allows `-` in identifiers).
    if i >= end { return String::new(); }
    while i < end { let c = b[i]; if c < 48 || c > 57 { return String::new(); } o.push(c as char); i += 1; }
    o
}
fn strip_last(s: &str) -> String {
    let b = s.as_bytes(); let mut o = String::new(); let mut i = 0; let n = b.len();
    while i + 1 < n { o.push(b[i] as char); i += 1; } o
}
// LLVM opcode for a binary rzir op ("" if not a binary op).
fn binop(op: &str) -> String {
    if streq(op,"add")||streq(op,"add8") { return String::from("add"); }
    if streq(op,"sub")||streq(op,"sub8") { return String::from("sub"); }
    if streq(op,"mul")||streq(op,"mul8") { return String::from("mul"); }
    if streq(op,"udiv") { return String::from("udiv"); }
    if streq(op,"umod") { return String::from("urem"); }
    if streq(op,"div")||streq(op,"div8") { return String::from("sdiv"); }
    if streq(op,"mod")||streq(op,"mod8") { return String::from("srem"); }
    if streq(op,"or")||streq(op,"or8") { return String::from("or"); }
    if streq(op,"and")||streq(op,"and8") { return String::from("and"); }
    if streq(op,"xor")||streq(op,"xor8") { return String::from("xor"); }
    if streq(op,"shl")||streq(op,"shl8") { return String::from("shl"); }
    if streq(op,"shu8") { return String::from("lshr"); }
    if streq(op,"shr8")||streq(op,"shr") { return String::from("ashr"); }   // ARITHMETIC right shift (signed >>, and the sign-extend in s9_wrap_mask's shl8/shr8 pair for i8/i16/i32 wrapping ops)
    String::new()
}
// LLVM icmp predicate for a compare op ("" if not a compare).
fn cmpop(op: &str) -> String {
    if streq(op,"eq") { return String::from("eq"); }
    if streq(op,"ne") { return String::from("ne"); }
    if streq(op,"lt")||streq(op,"lt8") { return String::from("slt"); }
    if streq(op,"gt")||streq(op,"gt8") { return String::from("sgt"); }
    if streq(op,"ge")||streq(op,"ge8") { return String::from("sge"); }
    if streq(op,"le")||streq(op,"le8") { return String::from("sle"); }
    if streq(op,"ltu8") { return String::from("ult"); }
    if streq(op,"gtu8") { return String::from("ugt"); }
    String::new()
}
fn str_to_i64(s: &str) -> i64 { let b = s.as_bytes(); let mut n: i64 = 0; let mut i = 0; while i < b.len() { let c = b[i]; if c >= 48 && c <= 57 { n = n * 10 + (c - 48) as i64; } i += 1; } n }
// a rzir vreg operand is `v` followed by ASCII digits (v0, v34). A call's args are always vregs; the callee is
// every token between `call` and the first vreg -- so a mangled fn name with SPACES (`dyn Trait::method`,
// `&'a str::decode`) is parsed whole, not mis-split by the ws tokenizer. (full-closure: proc_macro str::decode.)
fn is_vreg(s: &str) -> bool { let b = s.as_bytes(); if b.len() < 2 { return false; } if b[0] != 118 { return false; } let mut i = 1; while i < b.len() { if b[i] < 48 || b[i] > 57 { return false; } i += 1; } true }
fn join_ws(t: &Vec<String>, lo: usize, hi: usize) -> String { let mut out = String::new(); let mut i = lo; while i < hi { if i > lo { out.push_str(" "); } out.push_str(&t[i]); i += 1; } out }
fn gtype_of(gnames: &Vec<String>, gsizes: &Vec<i64>, name: &str) -> String {
    let mut i = 0; while i < gnames.len() { if streq(&gnames[i], name) { if gsizes[i] <= 8 { return String::from("i64"); } let mut sz = gsizes[i]; if sz > 3221225472 { sz = 3221225472; } return format!("[{} x i8]", sz); } i += 1; } String::new()
}
fn is_term(out: &str) -> bool {
    // does the last emitted line terminate a basic block? Scan BACKWARDS for the last line's first token --
    // NEVER split the whole `out` (is_term is called per-emitted-instruction; a full split_lines here is O(n) per
    // call x O(n) calls = O(n^2) bump-arena allocation that never frees -> exhausts the arena on a large function.
    // This backward scan allocates only the (tiny) first token. Return value is identical to the old split version.
    let b = out.as_bytes(); let n = b.len();
    if n == 0 { return false; }
    let mut end = n;
    if b[end-1] == 10 { end -= 1; }        // drop one trailing newline
    if end == 0 { return false; }
    let mut start = end;
    while start > 0 && b[start-1] != 10 { start -= 1; }   // start of the last line
    let mut i = start;
    while i < end && (b[i] == 32 || b[i] == 9 || b[i] == 13) { i += 1; }   // skip leading ws
    let mut tok = String::new();
    while i < end && b[i] != 32 && b[i] != 9 && b[i] != 13 { tok.push(b[i] as char); i += 1; }
    if tok.len() == 0 { return false; }
    streq(&tok,"br") || streq(&tok,"ret") || streq(&tok,"unreachable")
}

fn main() {
    let mut whole = String::new();
    std::io::stdin().read_to_string(&mut whole).unwrap();
    let lines = split_lines(&whole);
    // partition into functions: header line `fn NAME` / `ir NAME -> TY`, body until next header/gdef.
    let mut names: Vec<String> = Vec::new();
    let mut bodies: Vec<String> = Vec::new();
    let mut gnames: Vec<String> = Vec::new();
    let mut externs: Vec<String> = Vec::new();   // `extern NAME` directives -> real C-ABI extern calls (P4 libLLVM FFI), vs undefined-callee inert-0
    let mut gsizes: Vec<i64> = Vec::new();
    let mut cur_name = String::new(); let mut cur_body = String::new(); let mut have = false;
    let mut li = 0;
    while li < lines.len() {
        let raw = &lines[li]; let t = split_ws(raw); li += 1;
        if t.len() == 0 { continue; }
        if streq(&t[0],"fn") || streq(&t[0],"ir") {
            if have { names.push(cur_name.clone()); bodies.push(cur_body.clone()); }
            // `fn dyn Trait::method` (SEM0 dyn-dispatch thunk) has a SPACE in the name; the whitespace tokenizer
            // splits it, so join the `dyn` prefix with the next token so the def name matches the call callee
            // (both become "dyn Trait::method", emitted as a quoted LLVM symbol). (RB-2: rustc_trait_selection.)
            let mut _nh = t.len(); { let mut _z = 1; while _z < t.len() { if streq(&t[_z], "->") { _nh = _z; _z = t.len(); } else { _z += 1; } } } cur_name = join_ws(&t, 1, _nh);   // name = tokens after fn/ir UP TO the `-> rettype` marker (may contain spaces: `dyn T::m`, `&'a str::decode`)
            cur_body = String::new(); have = true; continue;
        }
        if streq(&t[0],"gdef") { if t.len() >= 3 { gnames.push(t[1].clone()); gsizes.push(str_to_i64(&t[2])); } continue; }
        if streq(&t[0],"extern") { if t.len() >= 2 { externs.push(t[1].clone()); } continue; }
        if have { cur_body.push_str(raw); cur_body.push('\n'); }
    }
    if have { names.push(cur_name.clone()); bodies.push(cur_body.clone()); }

    let mut extern_abs = false;
    // OPT-IN mmap heap: emit __heap as a runtime mmap region (not a BSS array) ONLY when the input carries the
    // sentinel `gdef __mmapheap 1`. This is set when minting the rzir2ll TRANSLATOR itself (it needs a >4GB arena
    // for the self-image and its code is 64-bit-address-clean). Default (no sentinel) stays BSS -> low addresses,
    // so ordinary programs are unaffected (a high mmap address exposes latent 32-bit-addr truncation, e.g. in
    // closures -- the b1 arc). Keeps the parity board zero-regression while enabling tower self-host purity (L4).
    // mmap __heap is now the DEFAULT (MHD-5): the FR helpers + closure cell are 64-bit-address-clean, so a high
    // mmap heap is safe for every program -- this removes the 3GB BSS -no-pie wall everywhere (not just the mint
    // translator), enabling big map/vec-using programs via the bridge at >3GB. Escape hatch: `gdef __nommapheap 1`
    // forces the old BSS heap. (The former opt-in `__mmapheap` sentinel is now redundant/harmless.)
    let mut mmap_heap = true; { let mut z = 0; while z < gnames.len() { if streq(&gnames[z], "__nommapheap") { mmap_heap = false; } z += 1; } }
    let mut emitted: Vec<String> = Vec::new();   // FIRST-WINS dedup, matching rzir-ld (later same-name blocks dropped)
    let mut fi = 0;
    while fi < names.len() {
        let fname0 = names[fi].clone();
        let body = bodies[fi].clone();
        fi += 1;
        // DEDUP duplicate function names first-wins (rzir-ld drops later same-name blocks; llc rejects a
        // redefinition). e.g. two bare `fn parse` -> keep the first, skip the rest. (locale crate-scale-ir-validity)
        { let mut dup = false; let mut z = 0; while z < emitted.len() { if streq(&emitted[z], &fname0) { dup = true; } z += 1; } if dup { continue; } }
        emitted.push(fname0.clone());
        let blines = split_lines(&body);
        // collect defined value names (token[0] minus ':' where token[2]=="=") and params.
        let mut defs: Vec<String> = Vec::new();
        let mut params: Vec<String> = Vec::new();
        let mut bi = 0;
        while bi < blines.len() {
            let t = split_ws(&blines[bi]); bi += 1;
            if t.len() >= 3 && streq(&t[2],"=") {
                let v = strip_last(&t[0]);
                // dedup
                let mut seen = false; let mut k = 0; while k < defs.len() { if streq(&defs[k],&v) { seen = true; } k += 1; }
                if !seen { defs.push(v.clone()); }
                if t.len() >= 4 && streq(&t[3],"param") { params.push(v.clone()); }
            }
        }
        // ALSO alloca every REFERENCED slot (operands), not only defs: emit_s9 may `ret vN` / use a vN never
        // defined in this fn (cf reads an uninitialized slot -> garbage; llc rejects an undeclared %vN.s). Alloca
        // all referenced vN so the load hits an uninit alloca == cf's behavior. (locale crate-scale-ir-validity)
        bi = 0;
        while bi < blines.len() {
            let t = split_ws(&blines[bi]); bi += 1;
            let mut ti = 0;
            while ti < t.len() {
                let vn = vtok(&t[ti]); ti += 1;
                if vn.len() > 0 {
                    let mut seen = false; let mut k = 0; while k < defs.len() { if streq(&defs[k],&vn) { seen = true; } k += 1; }
                    if !seen { defs.push(vn); }
                }
            }
        }
        let mut fname = fname0.clone();
        if streq(&fname0,"main") { fname = String::from("main"); }
        // signature
        let mut sig = String::new();
        let mut pi = 0; while pi < params.len() { if pi > 0 { sig.push_str(", "); } sig.push_str(&format!("i64 %{}.a", params[pi])); pi += 1; }
        let mut out = String::new();
        if streq(&fname0, "main") { out.push_str("define i64 @\"main\"(i32 %__argc.r, i8** %__argv.r) {\n"); } else { out.push_str(&format!("define i64 @\"{}\"({}) {{\n", fname, sig)); }
        out.push_str("entry:\n");
        // ONEBIN driver D2 rung10 (Phase A): capture the process's real argc/argv into globals so env::args_os()
        // can deliver them (raw_args -> env::args_os().enumerate()); without this the driver gets zero args and no-ops.
        if streq(&fname0, "main") { out.push_str("  %__argc64 = zext i32 %__argc.r to i64\n  store i64 %__argc64, i64* @__argc\n  %__argvi = ptrtoint i8** %__argv.r to i64\n  store i64 %__argvi, i64* @__argv\n"); }
        let mut di = 0; while di < defs.len() { out.push_str(&format!("  %{}.s = alloca i64\n", defs[di])); di += 1; }
        pi = 0; while pi < params.len() { out.push_str(&format!("  store i64 %{}.a, i64* %{}.s\n", params[pi], params[pi])); pi += 1; }
        // main prologue: mmap the __heap bump arena at its full requested size (no BSS -no-pie cap). ANON|PRIVATE
        // (flags 0x22=34), PROT_READ|WRITE (3), fd -1. Lazily committed, so RSS grows only as used. This removes
        // the 3GB BSS wall that made a tower-built rzir2ll crash on the self-image (tower self-host purity, L4).
        // HEAP-64G: the floor was 16 GiB; the bump arena has no end check, so semc SEGV'd at exactly 16 GiB emitting
        // rustc_abi / rustc_attr_ir / rustc_borrowck (2026-10-26). Floor now 64 GiB, plus MAP_NORESERVE (0x4000;
        // flags 0x4022=16418) so the kernel's overcommit heuristic never refuses the reservation. Address space only.
        if mmap_heap && streq(&fname0, "main") { let mut hs: i64 = 0; let mut hz = 0; while hz < gnames.len() { if streq(&gnames[hz], "__heap") { hs = gsizes[hz]; } hz += 1; } if hs > 0 { let ms = if hs < 68719476736 { 68719476736 } else { hs }; out.push_str(&format!("  %hm = call i8* @mmap(i8* null, i64 {}, i32 3, i32 16418, i32 -1, i64 0)\n", ms)); out.push_str("  %hmi = ptrtoint i8* %hm to i64\n"); out.push_str("  store i64 %hmi, i64* @__heap_base\n"); } }
        let mut ctr: i64 = 0;
        let mut pending: Vec<String> = Vec::new();  // brz target labels
        let mut ltop: Vec<String> = Vec::new();
        let mut lexit: Vec<String> = Vec::new();
        let mut lcont: Vec<String> = Vec::new();     // per-loop continue label
        let mut ldepth: Vec<i64> = Vec::new();      // pending len at loop start
        bi = 0;
        while bi < blines.len() {
            let t = split_ws(&blines[bi]); bi += 1;
            if t.len() == 0 { continue; }
            // dead-block guard
            let mut is_def = false; if t.len() >= 3 { if streq(&t[2],"=") { is_def = true; } }
            if is_def {
                let v = strip_last(&t[0]); let op = t[3].clone();
                if streq(&op,"param") { continue; }
                if is_term(&out) { ctr += 1; out.push_str(&format!("dead{}:\n", ctr)); }
                if streq(&op,"const") { ctr += 1; out.push_str(&format!("  %t{} = add i64 0, {}\n", ctr, t[4])); out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue; }
                if streq(&op,"faddr") {
                    // faddr to a name the module never defines (e.g. a __fnthunk emit_s9 referenced but did not
                    // emit in this crate) -> inert 0, same first-wins/undefined discipline as `call` (cf tolerates
                    // an undefined faddr in dead/unreached code; ptrtoint of an undefined global fails llc at parse).
                    let mut fdef = false; { let mut z = 0; while z < names.len() { if streq(&names[z], &t[4]) { fdef = true; } z += 1; } }
                    if fdef { out.push_str(&format!("  store i64 ptrtoint(ptr @\"{}\" to i64), i64* %{}.s\n", t[4], v)); }
                    else { ctr += 1; out.push_str(&format!("  %t{} = add i64 0, 0\n", ctr)); out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); }
                    continue;
                }
                if streq(&op,"gref") {
                    // __heap is the never-freeing bump arena; it is mmap'd at runtime (see main prologue) rather than
                    // a BSS array, so it can exceed the ~3GB -no-pie BSS limit. gref __heap -> load the mmap base.
                    if mmap_heap && streq(&t[4],"__heap") { ctr += 1; out.push_str(&format!("  %t{} = load i64, i64* @__heap_base\n", ctr)); out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue; }
                    if streq(&t[4],"__argc") || streq(&t[4],"__argv") { out.push_str(&format!("  store i64 ptrtoint(i64* @{} to i64), i64* %{}.s\n", t[4], v)); continue; }   // ONEBIN driver D2 rung10: gref the real-argv globals (declared/set in the @main prologue; not in the gdef list)
                    let gt = gtype_of(&gnames, &gsizes, &t[4]);
                    if gt.len() > 0 { out.push_str(&format!("  store i64 ptrtoint({}* @{} to i64), i64* %{}.s\n", gt, t[4], v)); }
                    else { ctr += 1; out.push_str(&format!("  %t{} = add i64 0, 0\n", ctr)); out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); }
                    continue;
                }
                let bo = binop(&op);
                if bo.len() > 0 {
                    ctr += 1; let a = ctr; out.push_str(&format!("  %ld{} = load i64, i64* %{}.s\n", a, t[4]));
                    ctr += 1; let b = ctr; out.push_str(&format!("  %ld{} = load i64, i64* %{}.s\n", b, t[5]));
                    ctr += 1; out.push_str(&format!("  %t{} = {} i64 %ld{}, %ld{}\n", ctr, bo, a, b));
                    out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue;
                }
                let co = cmpop(&op);
                if co.len() > 0 {
                    ctr += 1; let a = ctr; out.push_str(&format!("  %ld{} = load i64, i64* %{}.s\n", a, t[4]));
                    ctr += 1; let b = ctr; out.push_str(&format!("  %ld{} = load i64, i64* %{}.s\n", b, t[5]));
                    ctr += 1; let c = ctr; out.push_str(&format!("  %c{} = icmp {} i64 %ld{}, %ld{}\n", c, co, a, b));
                    ctr += 1; out.push_str(&format!("  %t{} = zext i1 %c{} to i64\n", ctr, c));
                    out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue;
                }
                if streq(&op,"conv") { ctr += 1; let a = ctr; out.push_str(&format!("  %ld{} = load i64, i64* %{}.s\n", a, t[4])); ctr += 1; out.push_str(&format!("  %t{} = and i64 %ld{}, 4294967295\n", ctr, a)); out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue; }
                if streq(&op,"convs") { ctr += 1; let a = ctr; out.push_str(&format!("  %ld{} = load i64, i64* %{}.s\n", a, t[4])); ctr += 1; let x = ctr; out.push_str(&format!("  %x{} = trunc i64 %ld{} to i32\n", x, a)); ctr += 1; out.push_str(&format!("  %t{} = sext i32 %x{} to i64\n", ctr, x)); out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue; }
                if streq(&op,"ref") { ctr += 1; out.push_str(&format!("  %t{} = ptrtoint i64* %{}.s to i64\n", ctr, t[4])); out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue; }
                if streq(&op,"load") || streq(&op,"load8") || streq(&op,"loadb") {
                    ctr += 1; let a = ctr; out.push_str(&format!("  %ld{} = load i64, i64* %{}.s\n", a, t[4]));
                    if streq(&op,"load8") {
                        ctr += 1; let p = ctr; out.push_str(&format!("  %p{} = inttoptr i64 %ld{} to i64*\n", p, a));
                        ctr += 1; out.push_str(&format!("  %t{} = load i64, i64* %p{}\n", ctr, p));
                        out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue;
                    }
                    if streq(&op,"loadb") {
                        ctr += 1; let p = ctr; out.push_str(&format!("  %p{} = inttoptr i64 %ld{} to i8*\n", p, a));
                        ctr += 1; let w = ctr; out.push_str(&format!("  %w{} = load i8, i8* %p{}\n", w, p));
                        ctr += 1; out.push_str(&format!("  %t{} = zext i8 %w{} to i64\n", ctr, w));
                        out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue;
                    }
                    ctr += 1; let p = ctr; out.push_str(&format!("  %p{} = inttoptr i64 %ld{} to i32*\n", p, a));
                    ctr += 1; let w = ctr; out.push_str(&format!("  %w{} = load i32, i32* %p{}\n", w, p));
                    ctr += 1; out.push_str(&format!("  %t{} = zext i32 %w{} to i64\n", ctr, w));
                    out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue;
                }
                if streq(&op,"call") {
                    // EMPTY/MISSING CALLEE: an unresolved call emitted as `call ` (a dangler in the rzir, e.g. a
                    // lifted closure calling an unresolved target). cf tolerates it (reads garbage/inert); the C
                    // ld path leaves it; but t[4] is out-of-bounds here -> SEGV. Route to inert-0 like the
                    // undefined-callee dead path, so the IR is valid and links. (RB-2: rustc_data_structures.)
                    if t.len() < 5 || t[4].len() == 0 {
                        ctr += 1; out.push_str(&format!("  %t{} = add i64 0, 0\n", ctr));
                        out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue;
                    }
                    // `call dyn Trait::method ARGS` -- the dyn-thunk name has a SPACE, so join `dyn`+next token
                    // into the callee and start args one later (mirrors the fn-def join above). (RB-2.)
                    let mut argstart = 4; while argstart < t.len() && is_vreg(&t[argstart]) == false { argstart += 1; }
                    let callee = join_ws(&t, 4, argstart);
                    if streq(&callee,"abs") && argstart < t.len() {   // libc abs is 1-arg; a 0-arg `call abs` is a user/core fn named abs -> normal path (full-closure: core)
                        extern_abs = true;
                        ctr += 1; let a = ctr; out.push_str(&format!("  %ld{} = load i64, i64* %{}.s\n", a, t[5]));
                        ctr += 1; let tr = ctr; out.push_str(&format!("  %tr{} = trunc i64 %ld{} to i32\n", tr, a));
                        ctr += 1; let rr = ctr; out.push_str(&format!("  %rr{} = call i32 @abs(i32 %tr{})\n", rr, tr));
                        ctr += 1; out.push_str(&format!("  %t{} = sext i32 %rr{} to i64\n", ctr, rr));
                        out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue;
                    }
                    // UNDEFINED CALLEE (a dead generic template like `call T::area`, never monomorphized/called):
                    // emit_s9 routes every LIVE call to a $Concrete copy, so a call to a name it never defined is
                    // provably dead. cf tolerates an undefined `call` in dead code; llc REJECTS an undefined symbol
                    // at IR-parse. Route it to an inert 0 so the IR is valid and links (locale: generic-trait-method-mono).
                    let mut cdef = false; { let mut z = 0; while z < names.len() { if streq(&names[z], &callee) { cdef = true; } z += 1; } }
                    let mut cext = false; { let mut z = 0; while z < externs.len() { if streq(&externs[z], &callee) { cext = true; } z += 1; } }
                    if !cdef && cext {
                        // real C-ABI extern call to a declared `extern NAME` (P4 libLLVM/wrapper). Varargs form so any
                        // arity works; args passed as i64 (low 32 bits reach edi/... for i32 C params).
                        let mut argnames: Vec<String> = Vec::new(); let mut ai = argstart;
                        while ai < t.len() { ctr += 1; out.push_str(&format!("  %ld{} = load i64, i64* %{}.s\n", ctr, t[ai])); argnames.push(format!("%ld{}", ctr)); ai += 1; }
                        let mut aa = String::new(); let mut qi = 0; while qi < argnames.len() { if qi > 0 { aa.push_str(", "); } aa.push_str(&format!("i64 {}", argnames[qi])); qi += 1; }
                        ctr += 1; out.push_str(&format!("  %t{} = call i64 (...) @\"{}\"({})\n", ctr, callee, aa));
                        out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue;
                    }
                    if !cdef {
                        ctr += 1; out.push_str(&format!("  %t{} = add i64 0, 0\n", ctr));
                        out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue;
                    }
                    // load args
                    let mut argnames: Vec<String> = Vec::new(); let mut ai = argstart;
                    while ai < t.len() { ctr += 1; out.push_str(&format!("  %ld{} = load i64, i64* %{}.s\n", ctr, t[ai])); argnames.push(format!("%ld{}", ctr)); ai += 1; }
                    let mut aa = String::new(); let mut qi = 0; while qi < argnames.len() { if qi > 0 { aa.push_str(", "); } aa.push_str(&format!("i64 {}", argnames[qi])); qi += 1; }
                    ctr += 1; out.push_str(&format!("  %t{} = call i64 @\"{}\"({})\n", ctr, callee, aa));
                    out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue;
                }
                if streq(&op,"calli") {
                    ctr += 1; let fp = ctr; out.push_str(&format!("  %ld{} = load i64, i64* %{}.s\n", fp, t[4]));
                    let mut argnames: Vec<String> = Vec::new(); let mut ai = 5;
                    while ai < t.len() { ctr += 1; out.push_str(&format!("  %ld{} = load i64, i64* %{}.s\n", ctr, t[ai])); argnames.push(format!("%ld{}", ctr)); ai += 1; }
                    let mut aa = String::new(); let mut qi = 0;
                    while qi < argnames.len() { if qi > 0 { aa.push_str(", "); } aa.push_str(&format!("i64 {}", argnames[qi])); qi += 1; }
                    ctr += 1; let pp = ctr; out.push_str(&format!("  %fp{} = inttoptr i64 %ld{} to ptr\n", pp, fp));
                    ctr += 1; out.push_str(&format!("  %t{} = call i64 %fp{}({})\n", ctr, pp, aa));
                    out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue;
                }
                if streq(&op,"syscall") {
                    let mut argnames: Vec<String> = Vec::new(); let mut ai = 4;
                    while ai < t.len() { ctr += 1; out.push_str(&format!("  %ld{} = load i64, i64* %{}.s\n", ctr, t[ai])); argnames.push(format!("%ld{}", ctr)); ai += 1; }
                    let regs: Vec<String> = vec![String::from("{rax}"),String::from("{rdi}"),String::from("{rsi}"),String::from("{rdx}"),String::from("{r10}"),String::from("{r8}"),String::from("{r9}")];
                    let mut cons = String::from("={rax},"); let mut aa = String::new(); let mut qi = 0;
                    while qi < argnames.len() { if qi > 0 { cons.push_str(","); aa.push_str(", "); } cons.push_str(&regs[qi]); aa.push_str(&format!("i64 {}", argnames[qi])); qi += 1; }
                    ctr += 1; out.push_str(&format!("  %t{} = call i64 asm sideeffect \"syscall\", \"{},~{{rcx}},~{{r11}},~{{memory}}\"({})\n", ctr, cons, aa));
                    out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue;
                }
                // unknown op -> valid placeholder
                ctr += 1; out.push_str(&format!("  %t{} = add i64 0, 0\n", ctr));
                out.push_str(&format!("  store i64 %t{}, i64* %{}.s\n", ctr, v)); continue;
            }
            // statement ops
            let op = t[0].clone();
            if streq(&op,"ret") {
                if is_term(&out) { ctr += 1; out.push_str(&format!("dead{}:\n", ctr)); }
                if t.len() > 1 { ctr += 1; out.push_str(&format!("  %ld{} = load i64, i64* %{}.s\n", ctr, t[1])); out.push_str(&format!("  ret i64 %ld{}\n", ctr)); }
                else { out.push_str("  ret i64 0\n"); }
                continue;
            }
            if streq(&op,"store") || streq(&op,"store8") || streq(&op,"storeb") {
                if is_term(&out) { ctr += 1; out.push_str(&format!("dead{}:\n", ctr)); }
                ctr += 1; let a = ctr; out.push_str(&format!("  %ld{} = load i64, i64* %{}.s\n", a, t[1]));
                ctr += 1; let d = ctr; out.push_str(&format!("  %ld{} = load i64, i64* %{}.s\n", d, t[2]));
                if streq(&op,"store8") {
                    ctr += 1; let p = ctr; out.push_str(&format!("  %p{} = inttoptr i64 %ld{} to i64*\n", p, a));
                    out.push_str(&format!("  store i64 %ld{}, i64* %p{}\n", d, p)); continue;
                }
                if streq(&op,"storeb") {
                    ctr += 1; let p = ctr; out.push_str(&format!("  %p{} = inttoptr i64 %ld{} to i8*\n", p, a));
                    ctr += 1; let x = ctr; out.push_str(&format!("  %x{} = trunc i64 %ld{} to i8\n", x, d));
                    out.push_str(&format!("  store i8 %x{}, i8* %p{}\n", x, p)); continue;
                }
                ctr += 1; let p = ctr; out.push_str(&format!("  %p{} = inttoptr i64 %ld{} to i32*\n", p, a));
                ctr += 1; let x = ctr; out.push_str(&format!("  %x{} = trunc i64 %ld{} to i32\n", x, d));
                out.push_str(&format!("  store i32 %x{}, i32* %p{}\n", x, p)); continue;
            }
            if streq(&op,"brz") {
                if is_term(&out) { ctr += 1; out.push_str(&format!("dead{}:\n", ctr)); }
                ctr += 1; let a = ctr; out.push_str(&format!("  %ld{} = load i64, i64* %{}.s\n", a, t[1]));
                ctr += 1; let bl = format!("B{}", ctr); let thn = format!("T{}", ctr);
                ctr += 1; let c = ctr; out.push_str(&format!("  %bc{} = icmp ne i64 %ld{}, 0\n", c, a));
                out.push_str(&format!("  br i1 %bc{}, label %{}, label %{}\n", c, thn, bl));
                out.push_str(&format!("{}:\n", thn)); pending.push(bl); continue;
            }
            if streq(&op,"pool") {
                if pending.len() == 0 { continue; }
                let bl = pending[pending.len()-1].clone(); pending.pop();
                if !is_term(&out) { out.push_str(&format!("  br label %{}\n", bl)); }
                out.push_str(&format!("{}:\n", bl)); continue;
            }
            if streq(&op,"loop") {
                ctr += 1; let tl = format!("LT{}", ctr); let el = format!("LE{}", ctr); let cl = format!("LC{}", ctr);
                out.push_str(&format!("  br label %{}\n", tl)); out.push_str(&format!("{}:\n", tl));
                ltop.push(tl); lexit.push(el); lcont.push(cl); ldepth.push(pending.len() as i64); continue;
            }
            if streq(&op,"goto") { if !is_term(&out) { out.push_str(&format!("  br label %{}\n", t[1])); } continue; }
            if streq(&op,"label") { if !is_term(&out) { out.push_str(&format!("  br label %{}\n", t[1])); } out.push_str(&format!("{}:\n", t[1])); continue; }
            if streq(&op,"brk") { if lexit.len() > 0 { if !is_term(&out) { out.push_str(&format!("  br label %{}\n", lexit[lexit.len()-1])); } } continue; }
            if streq(&op,"cnt") { if lcont.len() > 0 { if !is_term(&out) { out.push_str(&format!("  br label %{}\n", lcont[lcont.len()-1])); } } continue; }
            if streq(&op,"cont") { if lcont.len() > 0 { let cl = lcont[lcont.len()-1].clone(); if !is_term(&out) { out.push_str(&format!("  br label %{}\n", cl)); } out.push_str(&format!("{}:\n", cl)); } continue; }
            if streq(&op,"jmp") {
                if ltop.len() > 0 { if !is_term(&out) { out.push_str(&format!("  br label %{}\n", ltop[ltop.len()-1])); } }
                continue;
            }
            if streq(&op,"endloop") {
                if ltop.len() == 0 { continue; }
                let el = lexit[lexit.len()-1].clone(); let dep = ldepth[ldepth.len()-1];
                ltop.pop(); lexit.pop(); lcont.pop(); ldepth.pop();
                if !is_term(&out) { out.push_str(&format!("  br label %{}\n", el)); }
                while (pending.len() as i64) > dep { let bl = pending[pending.len()-1].clone(); pending.pop(); out.push_str(&format!("{}:\n", bl)); out.push_str(&format!("  br label %{}\n", el)); }
                out.push_str(&format!("{}:\n", el)); continue;
            }
            // unknown statement: ignore
        }
        if !is_term(&out) { out.push_str("  ret i64 0\n"); }
        // FLUSH any label referenced by an unmatched brz/loop still open at fn end (a structured-CF construct
        // whose pool/endloop the rzir shape did not pair): place each as a dead terminated block so no label is
        // left undefined (llc rejects `use of undefined value %B..`). cf tolerates it; llc does not. (locale crate-scale-ir-validity)
        while pending.len() > 0 { let bl = pending[pending.len()-1].clone(); pending.pop(); out.push_str(&format!("{}:\n  ret i64 0\n", bl)); }
        while ltop.len() > 0 { let e = lexit[lexit.len()-1].clone(); let c = lcont[lcont.len()-1].clone(); ltop.pop(); lexit.pop(); lcont.pop(); ldepth.pop(); out.push_str(&format!("{}:\n  ret i64 0\n", c)); out.push_str(&format!("{}:\n  ret i64 0\n", e)); }
        out.push_str("}\n");
        print!("{}", out);   // STREAM per-function: print each function as it is translated instead of
        // accumulating the whole module in `prog`. The tower bump allocator never frees, so accumulating a 731k-line
        // self-emit exhausts the arena; streaming bounds peak to one function. Globals/declares emit AFTER (llc
        // accepts functions-before-globals). See arc 2026-09-28-tower-rzir2ll-miscompile.
    }
    print!("@__argc = global i64 0\n@__argv = global i64 0\n");   // ONEBIN driver D2 rung10 (Phase A): real argv capture (set in @main prologue)
    let mut gemit: Vec<String> = Vec::new();   // dedup globals first-wins: a multi-crate //@SUPERSET rzir carries
    let mut gi = 0;                             // one gdef __heap/__obuf/... PER crate -> dup @globals fail llc.
    while gi < gnames.len() {
        let mut gdup = false; { let mut z = 0; while z < gemit.len() { if streq(&gemit[z], &gnames[gi]) { gdup = true; } z += 1; } }
        if gdup { gi += 1; continue; }
        gemit.push(gnames[gi].clone());
        // __heap is mmap'd at runtime (main prologue); emit only a pointer slot, not the (BSS-capped) array.
        if mmap_heap && streq(&gnames[gi], "__heap") { print!("@__heap_base = global i64 0\n"); gi += 1; continue; }
        let gt = gtype_of(&gnames, &gsizes, &gnames[gi]);
        if streq(&gt, "i64") { print!("@{} = global i64 0\n", gnames[gi]); }
        else { print!("@{} = global {} zeroinitializer\n", gnames[gi], gt); }
        gi += 1;
    }
    { let mut ei = 0; while ei < externs.len() { let mut d = false; let mut z = 0; while z < names.len() { if streq(&names[z], &externs[ei]) { d = true; } z += 1; } if !d { print!("declare i64 @\"{}\"(...)\n", externs[ei]); } ei += 1; } }
    if extern_abs { print!("declare i32 @abs(i32)\n"); }
    if mmap_heap { print!("declare i8* @mmap(i8*, i64, i32, i32, i32, i64)\n"); }   // __heap runtime allocation (L4 tower self-host purity)
}
