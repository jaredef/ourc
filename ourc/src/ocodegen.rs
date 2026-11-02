// the same i64::MIN hazard as oast::i64s: negating MIN to build digits overflows and emitted a bare "-",
// which is invalid IR. Build from the negative side.
// SYM-LIBM: a USER fn named like a libm symbol that an LLVM float intrinsic lowers to (`fn exp`) is renamed -- llc emits llvm.exp.f64 as a call to `exp`, which bound to the user's i64 fn (rustc mangles, so it never collides)
pub fn csym(n: &str) -> String {
    if n == "exp"
        || n == "exp2"
        || n == "log"
        || n == "log10"
        || n == "log2"
        || n == "pow"
        || n == "sin"
        || n == "cos"
        || n == "exit"
        || n == "fclose"
        || n == "fopen"
        || n == "fread"
        || n == "fseek"
        || n == "ftell"
        || n == "malloc"
        || n == "memcpy"
        || n == "printf"
        || n == "puts"
        || n == "realloc"
        || n == "snprintf"
        || n == "strcmp"
        || n == "strlen"
        || n == "strtod"
        || n == "fwrite"
        || n == "memcmp"
        || n == "system"
        || n == "write"
        || n == "signal"
        || n == "alarm"
        || n == "_exit"
        || n == "setrlimit"
        || n == "sigaltstack"
        || n == "sigaction"
    {
        /* SYM-RUNTIME: and every C symbol the runtime declares -- a user `fn malloc` was refused as a duplicate definition */
        let mut s = String::from("uf$");
        s.push_str(n);
        return s;
    }
    String::from(n)
}
fn n64(n: i64) -> String {
    if n == 0 {
        return String::from("0");
    }
    let neg = n < 0;
    if neg {
        let mut v = n;
        let mut s = String::from("");
        while v < 0 {
            let d = (0 - (v % 10)) as u32;
            let c = (48 + d) as u8 as char;
            let mut t = String::from("");
            t.push(c);
            t.push_str(&s);
            s = t;
            v = v / 10;
        }
        let mut t = String::from("-");
        t.push_str(&s);
        return t;
    }
    let mut u = if neg { 0 - n } else { n };
    let mut s = String::from("");
    while u > 0 {
        let d = (u % 10) as u32;
        let c = (48 + d) as u8 as char;
        let mut t = String::from("");
        t.push(c);
        t.push_str(&s);
        s = t;
        u = u / 10;
    }
    if neg {
        let mut t = String::from("-");
        t.push_str(&s);
        s = t;
    }
    s
}
fn n32(n: u32) -> String {
    if n == 0 {
        return String::from("0");
    }
    let mut v = n;
    let mut s = String::from("");
    while v > 0 {
        let d = (v % 10) as u32;
        let c = (48 + d) as u8 as char;
        let mut t = String::from("");
        t.push(c);
        t.push_str(&s);
        s = t;
        v = v / 10;
    }
    s
}
fn tmp(t: u32) -> String {
    let mut s = String::from("%t");
    s.push_str(&n32(t));
    s
}
fn loc(s: u32) -> String {
    let mut r = String::from("%L");
    r.push_str(&n32(s));
    r
}
fn is_fop(o: &str) -> bool {
    let b = o.as_bytes();
    b.len() > 1 && b[0] == 102
}
fn fllop(o: &str) -> String {
    if o == "f+" {
        String::from("fadd")
    } else if o == "f-" {
        String::from("fsub")
    } else if o == "f*" {
        String::from("fmul")
    } else if o == "f/" {
        String::from("fdiv")
    } else if o == "f%" {
        String::from("frem")
    } else if o == "f==" {
        String::from("oeq")
    } else if o == "f!=" {
        String::from("une")
    } else if o == "f<" {
        String::from("olt")
    } else if o == "f<=" {
        String::from("ole")
    } else if o == "f>" {
        String::from("ogt")
    } else {
        String::from("oge")
    }
}
fn fop_is_cmp(o: &str) -> bool {
    o == "f==" || o == "f!=" || o == "f<" || o == "f<=" || o == "f>" || o == "f>="
}
fn llop(o: &str) -> String {
    if o == "u>>" {
        return String::from("lshr");
    }
    if o == "u/" {
        return String::from("udiv");
    }
    if o == "u%" {
        return String::from("urem");
    } // U-DIV   // U-SHR: logical shift for u64/usize
    if o == "&" {
        String::from("and")
    } else if o == "|" {
        String::from("or")
    } else if o == "^" {
        String::from("xor")
    } else if o == "<<" {
        String::from("shl")
    } else if o == ">>" {
        String::from("ashr")
    } else if o == "+" {
        String::from("add")
    } else if o == "-" {
        String::from("sub")
    } else if o == "*" {
        String::from("mul")
    } else if o == "/" {
        String::from("sdiv")
    } else if o == "%" {
        String::from("srem")
    } else {
        String::from("icmp")
    }
}
fn match_slots(ops: &Vec<Op>) -> String {
    let mut s = String::from("");
    let mut k = 0;
    while k < ops.len() {
        match &ops[k] {
            Op::MatchOp(d, _st, arms) => {
                s.push_str("  ");
                s.push_str(&tmp(*d));
                s.push_str(".slot = alloca i64\n");
                let mut q = 0;
                while q < arms.len() {
                    s.push_str(&match_slots(&arms[q].ops));
                    q = q + 1;
                }
            }
            Op::IfOp(_t, _c, t2, _r, e2, _r2) => {
                s.push_str(&match_slots(t2));
                s.push_str(&match_slots(e2));
            }
            Op::LoopOp(c2, _cr, b2) => {
                s.push_str(&match_slots(c2));
                s.push_str(&match_slots(b2));
            }
            _ => {}
        }
        k = k + 1;
    }
    s
}
fn prologue(b: &Body) -> String {
    let lt = oabi::ll_ty(&oabi::layout_int());
    let mut s = String::from("");
    s.push_str(&match_slots(&b.ops));
    let mut i = 0;
    while i < b.nloc {
        s.push_str("  ");
        s.push_str(&loc(i));
        s.push_str(" = alloca ");
        s.push_str(&lt);
        s.push_str("\n");
        s.push_str("  store ");
        s.push_str(&lt);
        s.push_str(" 0, ");
        s.push_str(&lt);
        s.push_str("* ");
        s.push_str(&loc(i));
        s.push_str("\n");
        i = i + 1;
    }
    s
}
pub fn emit(b: &Body) -> String {
    let lt = oabi::ll_ty(&oabi::layout_int());
    // C-ARGV: main takes argc/argv and stashes them in globals, so the argv primitives can read them.
    // Our own driver reads its options from std::env::args, so a compiler meant to compile that driver
    // needs them, and a signature main() cannot supply them. Every emitted program gets the wider
    // signature; a program that never touches argv is unaffected (cc links either form).
    let mut s = String::from("define ");
    s.push_str(&lt);
    s.push_str(" @main(i32 %argc, i8** %argv) {\nentry:\n");
    s.push_str("  %acw = sext i32 %argc to i64\n  store i64 %acw, i64* @__argc\n");
    s.push_str("  %avw = ptrtoint i8** %argv to i64\n  store i64 %avw, i64* @__argv\n");
    s.push_str(&prologue(b));
    s.push_str(&body_ops_l(&b.ops));
    s.push_str("}\n");
    s
}
pub fn emit_named(b: &Body, name: &str, nparams: u32) -> String {
    let lt = oabi::ll_ty(&oabi::layout_int());
    let mut s = String::from("define ");
    s.push_str(&lt);
    s.push_str(" @");
    s.push_str(&csym(name));
    s.push_str("(");
    let mut p = 0;
    while p < nparams {
        if p > 0 {
            s.push_str(", ");
        }
        s.push_str("i64 %p");
        s.push_str(&n32(p));
        p = p + 1;
    }
    s.push_str(") {\nentry:\n");
    s.push_str(&prologue(b));
    s.push_str(&body_ops_l(&b.ops));
    s.push_str("}\n");
    s
}
// Each distinct string literal becomes a private constant array; StrC yields its ADDRESS as an i64, so a
// string is just a pointer-sized value and needs no new MIR type. The globals and the puts/strlen declarations
// are emitted once in the module preamble (collect_strs walks the whole op tree, nested blocks included).
// C-STRGID: a string constant's global NAME is derived from its content, so the derivation must be
// injective enough to never name two literals the same. It was not, twice over. The step was *33, which
// for a two-character literal reduces to c1*33+c2 and collides whenever c1 differs by one and c2 by 33 --
// ".." and "-O" are exactly that pair, and they are what the composed unit tripped over. Worse, the
// name was the DECIMAL of the hash printed by n64u, whose loop runs while v>0: with one scalar width a
// hash with the high bit set is a negative i64, so n64u answered the empty string and 134 of the unit's
// 761 literals were all named `.s`. Now the step is *257 (injective over byte pairs) and the name is all
// 64 bits in fixed-width hex, which cannot be empty. Simulated over the composed unit's 761 literals
// before building: *33 with hex names still collides twice, *131 and *257 collide zero times.
pub fn str_gid(lit: &str) -> String {
    let mut h: u64 = 5381;
    for c in lit.chars() {
        h = h.wrapping_mul(257).wrapping_add(c as u64);
    }
    let d = "0123456789abcdef";
    let db = d.as_bytes();
    let mut r = String::from(".s");
    let mut i = 60;
    while i >= 0 {
        r.push(db[((h >> i) & 15) as usize] as char);
        if i == 0 {
            i = -1;
        } else {
            i = i - 4;
        }
    }
    r
}
fn esc_ll(lit: &str) -> String {
    let mut r = String::from("");
    let b = lit.as_bytes();
    let mut i = 0;
    while i < b.len() {
        let x = b[i] as u32;
        if x >= 32 && x < 127 && x != 34 && x != 92 {
            r.push(b[i] as char);
        } else {
            r.push_str("\\");
            r.push_str(&hex2(x));
        }
        i = i + 1;
    }
    r
} // UTF8-STR: escape BYTES (a char >255 was cut to one byte)
fn hex2(x: u32) -> String {
    let d = "0123456789ABCDEF";
    let db: Vec<char> = {
        let mut v: Vec<char> = Vec::new();
        for c in d.chars() {
            v.push(c);
        }
        v
    };
    let mut r = String::from("");
    r.push(db[((x >> 4) & 15) as usize]);
    r.push(db[(x & 15) as usize]);
    r
}
fn n64u(x: u64) -> String {
    if x == 0 {
        return String::from("0");
    }
    let mut v = x;
    let mut t: Vec<char> = Vec::new();
    while v > 0 {
        t.push((48 + (v % 10) as u32) as u8 as char);
        v = v / 10;
    }
    let mut r = String::from("");
    let mut i = t.len();
    while i > 0 {
        r.push(t[i - 1]);
        i = i - 1;
    }
    r
}
pub fn collect_globals(ops: &Vec<Op>, out: &mut Vec<String>) {
    let mut k = 0;
    while k < ops.len() {
        match &ops[k] {
            Op::CallOp(_t, n, _av) if olower::is_gname(n) => {
                let g = olower::gname_of(n);
                let mut seen = false;
                let mut j = 0;
                while j < out.len() {
                    if out[j].as_str() == g.as_str() {
                        seen = true;
                    }
                    j = j + 1;
                }
                if !seen {
                    out.push(g);
                }
            }
            Op::IfOp(_t, _c, t2, _r, e2, _r2) => {
                collect_globals(t2, out);
                collect_globals(e2, out);
            }
            Op::LoopOp(c2, _cr, b2) => {
                collect_globals(c2, out);
                collect_globals(b2, out);
            }
            Op::MatchOp(_d, _st, arms) => {
                let mut q = 0;
                while q < arms.len() {
                    collect_globals(&arms[q].ops, out);
                    q = q + 1;
                }
            }
            _ => {}
        }
        k = k + 1;
    }
}
pub fn global_defs(gs: &Vec<String>) -> String {
    let mut s = String::from("");
    let mut i = 0;
    while i < gs.len() {
        s.push_str("@g.");
        s.push_str(gs[i].as_str());
        s.push_str(" = global i64 0\n");
        i = i + 1;
    }
    s
}
pub fn collect_strs(ops: &Vec<Op>, out: &mut Vec<String>) {
    let mut k = 0;
    while k < ops.len() {
        match &ops[k] {
            Op::StrC(_t, v) => {
                let mut seen = false;
                let mut j = 0;
                while j < out.len() {
                    if out[j].as_str() == v.as_str() {
                        seen = true;
                    }
                    j = j + 1;
                }
                if !seen {
                    out.push(String::from(v.as_str()));
                }
            }
            Op::IfOp(_r, _c, th, _tv, el, _ev) => {
                collect_strs(th, out);
                collect_strs(el, out);
            }
            Op::LoopOp(cc, _c, bd) => {
                collect_strs(cc, out);
                collect_strs(bd, out);
            }
            Op::MatchOp(_r, _s, arms) => {
                let mut a = 0;
                while a < arms.len() {
                    collect_strs(&arms[a].ops, out);
                    a = a + 1;
                }
            }
            _ => {}
        }
        k = k + 1;
    }
}
pub fn str_globals(lits: &Vec<String>) -> String {
    let mut s = String::from("");
    let mut i = 0;
    while i < lits.len() {
        let n = lits[i].len() + 1; // UTF8-STR: the array is BYTES
        s.push_str("@");
        s.push_str(&str_gid(lits[i].as_str()));
        // V1a LEN-STRING: a literal carries its byte length in a header just before its bytes
        s.push_str(" = private unnamed_addr constant <{ i64, [");
        s.push_str(&n32(n as u32));
        s.push_str(" x i8] }> <{ i64 ");
        s.push_str(&n32((n - 1) as u32));
        s.push_str(", [");
        s.push_str(&n32(n as u32));
        s.push_str(" x i8] c\"");
        s.push_str(&esc_ll(lits[i].as_str()));
        s.push_str("\\00\" }>\n");
        i = i + 1;
    }
    // the libc surface the string primitives need, plus printf's integer format; declared unconditionally so
    // print_int works in a program with no string literal of its own
    s.push_str("@.fmtd = private unnamed_addr constant [5 x i8] c\"%ld\\0A\\00\"\n");
    s.push_str("@.fmts = private unnamed_addr constant [3 x i8] c\"%s\\00\"\n");
    s.push_str("@.fmtdr = private unnamed_addr constant [4 x i8] c\"%ld\\00\"\n");
    s.push_str("@.fmtur = private unnamed_addr constant [4 x i8] c\"%lu\\00\"\n");
    s.push_str("@.fmtnl = private unnamed_addr constant [2 x i8] c\"\\0A\\00\"\n");
    s.push_str("@.fmtc = private unnamed_addr constant [3 x i8] c\"%c\\00\"\n");
    s.push_str("@.fmtfp = private unnamed_addr constant [5 x i8] c\"%.*f\\00\"\n@.fmtfe = private unnamed_addr constant [5 x i8] c\"%.*e\\00\"\n");
    s.push_str("declare double @llvm.sin.f64(double)\ndeclare double @llvm.cos.f64(double)\ndeclare double @llvm.exp.f64(double)\ndeclare double @llvm.exp2.f64(double)\ndeclare double @llvm.log.f64(double)\ndeclare double @llvm.log10.f64(double)\ndeclare double @llvm.log2.f64(double)\ndeclare double @llvm.pow.f64(double, double)\n"); // F64-LIBM: LLVM intrinsics only -- a libm `declare @exp` would clash with a user `fn exp` (names are unmangled)
    s.push_str("declare double @llvm.sqrt.f64(double)\n");
    s.push_str("declare i8 @llvm.fptosi.sat.i8.f64(double)\ndeclare i16 @llvm.fptosi.sat.i16.f64(double)\ndeclare i32 @llvm.fptosi.sat.i32.f64(double)\ndeclare i64 @llvm.fptosi.sat.i64.f64(double)\ndeclare i8 @llvm.fptoui.sat.i8.f64(double)\ndeclare i16 @llvm.fptoui.sat.i16.f64(double)\ndeclare i32 @llvm.fptoui.sat.i32.f64(double)\ndeclare i64 @llvm.fptoui.sat.i64.f64(double)\n");
    s.push_str("declare void @exit(i32)\n");
    s.push_str("declare i32 @puts(i8*)\ndeclare i64 @strlen(i8*)\ndeclare i32 @strcmp(i8*, i8*)\ndeclare i32 @printf(i8*, ...)\n");
    s.push_str(&vec_runtime());
    s.push_str(&str_runtime());
    s
}
// A GROWABLE VECTOR as real heap memory: header [len, cap, data] from malloc, data doubled with realloc.
// Emitted as LLVM functions in the preamble rather than inlined at each call site, so the branchy grow path
// lives in its own function and `vec_push` stays a plain call in the caller's straight-line block.
// String primitives a tokenizer needs. A str is an i64 address of NUL-terminated bytes (rung 8o), so a
// SUBSTRING or a CONCATENATION must allocate: both malloc a fresh buffer, memcpy into it, and NUL-terminate,
// which makes a computed string indistinguishable from a literal to every consumer (len/print/==).
pub fn str_runtime() -> String {
    let mut s = String::from("");
    s.push_str("declare i8* @memcpy(i8*, i8*, i64)\n");
    // V1a LEN-STRING: a str is the address of its BYTES, with its byte length in the i64 just before them and a NUL just
    // after (kept for libc: fopen, strtod). Length comes from the header, never strlen, so a str may hold `\0`.
    s.push_str("declare i32 @memcmp(i8*, i8*, i64)\ndeclare i64 @fwrite(i8*, i64, i64, i8*)\n@stdout = external global i8*\n");
    s.push_str("define i8* @__smalloc(i64 %sz) {\nentry:\n  %t = add i64 %sz, 8\n  %b = call i8* @__xmalloc(i64 %t)\n  %h = bitcast i8* %b to i64*\n  %n = sub i64 %sz, 1\n  store i64 %n, i64* %h\n  %p = getelementptr i8, i8* %b, i64 8\n  ret i8* %p\n}\n");
    s.push_str("define void @__sset(i8* %p, i64 %n) {\nentry:\n  %b = getelementptr i8, i8* %p, i64 -8\n  %h = bitcast i8* %b to i64*\n  store i64 %n, i64* %h\n  %e = getelementptr i8, i8* %p, i64 %n\n  store i8 0, i8* %e\n  ret void\n}\n");
    s.push_str("define i64 @__slenp(i8* %p) {\nentry:\n  %b = getelementptr i8, i8* %p, i64 -8\n  %h = bitcast i8* %b to i64*\n  %n = load i64, i64* %h\n  ret i64 %n\n}\n");
    s.push_str("define i64 @__slen(i64 %s) {\nentry:\n  %p = inttoptr i64 %s to i8*\n  %n = call i64 @__slenp(i8* %p)\n  ret i64 %n\n}\n");
    s.push_str("define i64 @__sfromc(i8* %c) {\nentry:\n  %n = call i64 @strlen(i8* %c)\n  %n1 = add i64 %n, 1\n  %d = call i8* @__smalloc(i64 %n1)\n  %x = call i8* @memcpy(i8* %d, i8* %c, i64 %n1)\n  %r = ptrtoint i8* %d to i64\n  ret i64 %r\n}\n");
    s.push_str("define void @__sput(i64 %s, i64 %nl) {\nentry:\n  %p = inttoptr i64 %s to i8*\n  %n = call i64 @__slenp(i8* %p)\n  %o = load i8*, i8** @stdout\n  %w = call i64 @fwrite(i8* %p, i64 1, i64 %n, i8* %o)\n  %y = icmp ne i64 %nl, 0\n  br i1 %y, label %nl1, label %fin\nnl1:\n  %pc = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.fmtnl, i64 0, i64 0))\n  br label %fin\nfin:\n  ret void\n}\n");
    s.push_str("define i64 @__str_at(i64 %s, i64 %i) {\nentry:\n  %p = inttoptr i64 %s to i8*\n  %q = getelementptr i8, i8* %p, i64 %i\n  %c = load i8, i8* %q\n  %r = zext i8 %c to i64\n  ret i64 %r\n}\n");
    // C-CELL: a push that answers the VECTOR, which is what a one-slot Cell constructor needs
    s.push_str("define i64 @__vec_push1(i64 %v, i64 %x) {\nentry:\n  %n = call i64 @__vec_push(i64 %v, i64 %x)\n  ret i64 %v\n}\n");
    s.push_str("define i64 @__str_copy(i64 %a) {\nentry:\n  %ap = inttoptr i64 %a to i8*\n  %la = call i64 @__slenp(i8* %ap)\n  %n1 = add i64 %la, 1\n  %d = call i8* @__smalloc(i64 %n1)\n  %x = call i8* @memcpy(i8* %d, i8* %ap, i64 %n1)\n  %r = ptrtoint i8* %d to i64\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__str_cat(i64 %a, i64 %b) {\nentry:\n  %ap = inttoptr i64 %a to i8*\n  %bp = inttoptr i64 %b to i8*\n  %la = call i64 @__slenp(i8* %ap)\n  %lb = call i64 @__slenp(i8* %bp)\n  %t1 = add i64 %la, %lb\n  %t2 = add i64 %t1, 1\n  %d = call i8* @__smalloc(i64 %t2)\n  %c1 = call i8* @memcpy(i8* %d, i8* %ap, i64 %la)\n  %dh = getelementptr i8, i8* %d, i64 %la\n  %lb1 = add i64 %lb, 1\n  %c2 = call i8* @memcpy(i8* %dh, i8* %bp, i64 %lb1)\n  %r = ptrtoint i8* %d to i64\n  ret i64 %r\n}\n");
    // C-STRGROW: an in-place append with a ONE-ENTRY capacity cache. A buffer grows in place only
    // when the handle IS the last buffer this helper produced AND its length is still what we recorded,
    // so no other live handle can observe the change; everything else falls back to a fresh copy.
    // C-GROWSET: the capacity cache holds SIXTEEN buffers, not one (64 was measured too: 10% less memory
    // for 8% more time, because the miss scan is linear -- 16 is the better balance). With one entry, appends that
    // interleave between several strings -- which is what building a function's IR while also building
    // its names does -- missed every time and took the doubling COPY path: measured at 5.74 GB of the
    // self-host's 6.35 GB total, 90% of all bytes allocated, in 3.4 M copies averaging 1.7 KB each.
    s.push_str("@__gp = global [16 x i64] zeroinitializer\n@__gl = global [16 x i64] zeroinitializer\n@__gc = global [16 x i64] zeroinitializer\n@__gi = global i64 0\n");
    s.push_str("define i64 @__str_grow(i64 %a, i64 %b) {\nentry:\n  %ap = inttoptr i64 %a to i8*\n  %bp = inttoptr i64 %b to i8*\n  %lb = call i64 @__slenp(i8* %bp)\n  br label %scan\nscan:\n  %i = phi i64 [ 0, %entry ], [ %i2, %next ]\n  %pp = getelementptr [16 x i64], [16 x i64]* @__gp, i64 0, i64 %i\n  %pv = load i64, i64* %pp\n  %eq = icmp eq i64 %pv, %a\n  br i1 %eq, label %fnd, label %next\nnext:\n  %i2 = add i64 %i, 1\n  %more = icmp ult i64 %i2, 16\n  br i1 %more, label %scan, label %noca\nfnd:\n  %lpp = getelementptr [16 x i64], [16 x i64]* @__gl, i64 0, i64 %i\n  %cpp = getelementptr [16 x i64], [16 x i64]* @__gc, i64 0, i64 %i\n  %gl = load i64, i64* %lpp\n  %gc = load i64, i64* %cpp\n  %nl1 = add i64 %gl, %lb\n  %nd1 = add i64 %nl1, 1\n  %fits = icmp ule i64 %nd1, %gc\n  br i1 %fits, label %inpl, label %fr2\ninpl:\n  %dh0 = getelementptr i8, i8* %ap, i64 %gl\n  %lb1 = add i64 %lb, 1\n  %x0 = call i8* @memcpy(i8* %dh0, i8* %bp, i64 %lb1)\n  call void @__sset(i8* %ap, i64 %nl1)\n  store i64 %nl1, i64* %lpp\n  ret i64 %a\nnoca:\n  %la2 = call i64 @__slenp(i8* %ap)\n  br label %frsh\nfr2:\n  br label %frsh\nfrsh:\n  %la = phi i64 [ %gl, %fr2 ], [ %la2, %noca ]\n  %nl = add i64 %la, %lb\n  %need = add i64 %nl, 1\n  %cap0 = mul i64 %need, 2\n  %cap = add i64 %cap0, 64\n  %d = call i8* @__smalloc(i64 %cap)\n  %x1 = call i8* @memcpy(i8* %d, i8* %ap, i64 %la)\n  %dh = getelementptr i8, i8* %d, i64 %la\n  %lb2 = add i64 %lb, 1\n  %x2 = call i8* @memcpy(i8* %dh, i8* %bp, i64 %lb2)\n  call void @__sset(i8* %d, i64 %nl)\n  %r = ptrtoint i8* %d to i64\n  %sl = load i64, i64* @__gi\n  %sl2 = add i64 %sl, 1\n  %sl3 = urem i64 %sl2, 16\n  store i64 %sl3, i64* @__gi\n  %spp = getelementptr [16 x i64], [16 x i64]* @__gp, i64 0, i64 %sl3\n  %slp = getelementptr [16 x i64], [16 x i64]* @__gl, i64 0, i64 %sl3\n  %scp = getelementptr [16 x i64], [16 x i64]* @__gc, i64 0, i64 %sl3\n  store i64 %r, i64* %spp\n  store i64 %nl, i64* %slp\n  store i64 %cap, i64* %scp\n  ret i64 %r\n}\n");
    // C-READFILE: our own driver reads its input with fs::read_to_string, so a compiler that is to
    // compile that driver needs the primitive. It is a runtime helper over libc (fopen, fseek, ftell,
    // fread, fclose) answering a NUL-terminated buffer, i.e. our str, and the empty string when the file
    // cannot be opened -- which is exactly what the driver's Err arm already does.
    s.push_str("declare i8* @fopen(i8*, i8*)\ndeclare i64 @fseek(i8*, i64, i32)\ndeclare i64 @ftell(i8*)\ndeclare i64 @fread(i8*, i64, i64, i8*)\ndeclare i32 @fclose(i8*)\n");
    s.push_str("@.rdmode = private unnamed_addr constant [2 x i8] c\"r\\00\"\n");
    // EPRINT (R2a P4): eprint!/eprintln! print through the ordinary print path with stdout pointed at fd 2 --
    // stdio_swap(1) flushes and dup2(2, 1) (saving fd 1), stdio_swap(0) flushes and restores it, so ordering holds
    s.push_str("declare i32 @fflush(i8*)\ndeclare i32 @dup(i32)\ndeclare i32 @dup2(i32, i32)\ndeclare i32 @close(i32)\n@__errsv = global i32 -1\n");
    // G1 FMT-AGG-DEBUG: `format!("{:?}", aggregate)` -- the printing Debug builder runs with C's `stdout` pointed at an
    // open_memstream buffer (__cap_on), and __cap_off closes it, restores stdout and answers the buffer (a C string, as str is)
    s.push_str("declare i8* @open_memstream(i8**, i64*)\n@__capsv = global i8* null\n@__capbuf = global i8* null\n@__capsz = global i64 0\n");
    s.push_str("define i64 @__cap_on() {\nentry:\n  %o = load i8*, i8** @stdout\n  store i8* %o, i8** @__capsv\n  %m = call i8* @open_memstream(i8** @__capbuf, i64* @__capsz)\n  store i8* %m, i8** @stdout\n  ret i64 0\n}\n");
    // (V1a strings carry an 8-byte length header: the buffer is COPIED into an __smalloc string, NUL included)
    s.push_str("define i64 @__cap_off() {\nentry:\n  %m = load i8*, i8** @stdout\n  %c = call i32 @fclose(i8* %m)\n  %o = load i8*, i8** @__capsv\n  store i8* %o, i8** @stdout\n  %b = load i8*, i8** @__capbuf\n  %n = load i64, i64* @__capsz\n  %n1 = add i64 %n, 1\n  %d = call i8* @__smalloc(i64 %n1)\n  %cp = call i8* @memcpy(i8* %d, i8* %b, i64 %n1)\n  %r = ptrtoint i8* %d to i64\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__stdio_swap(i64 %on) {\nentry:\n  %f0 = call i32 @fflush(i8* null)\n  %is = icmp ne i64 %on, 0\n  br i1 %is, label %to2, label %back\nto2:\n  %sv = call i32 @dup(i32 1)\n  store i32 %sv, i32* @__errsv\n  %d1 = call i32 @dup2(i32 2, i32 1)\n  ret i64 0\nback:\n  %o = load i32, i32* @__errsv\n  %ok = icmp sge i32 %o, 0\n  br i1 %ok, label %rs, label %done\nrs:\n  %d2 = call i32 @dup2(i32 %o, i32 1)\n  %c = call i32 @close(i32 %o)\n  store i32 -1, i32* @__errsv\n  br label %done\ndone:\n  ret i64 0\n}\n");
    s.push_str("@__argc = global i64 0\n@__argv = global i64 0\n");
    s.push_str("@__rcp = global [65536 x i64] zeroinitializer\n@__rcc = global [65536 x i64] zeroinitializer\n@__rcn = global i64 0\n"); // RC-IDENTITY
    s.push_str(
        "define i64 @__argc_get() {\nentry:\n  %v = load i64, i64* @__argc\n  ret i64 %v\n}\n",
    );
    s.push_str("define i64 @__argv_at(i64 %i) {\nentry:\n  %b = load i64, i64* @__argv\n  %p = inttoptr i64 %b to i8**\n  %q = getelementptr i8*, i8** %p, i64 %i\n  %c = load i8*, i8** %q\n  %r = call i64 @__sfromc(i8* %c)\n  ret i64 %r\n}\n");
    // C-STRGUARD: a str comparison whose operand is not a pointer is always a defect -- an int handle
    // reached a str parameter. It used to SEGFAULT inside strcmp; now it says so and exits 101, which
    // is the same discipline the IR self-check and the index check already follow. 4096 is below any
    // mappable address, so a real str can never be mistaken for one.
    s.push_str("@.strg = private unnamed_addr constant [59 x i8] c\"internal: a str comparison received a non-pointer operand\\0A\\00\"\n");
    // CHR-STR: a char as a one-character C string (malloc 2: the byte, then NUL) -- str_of(char) printed the CODE
    // UTF8-STR: a char >127 is its UTF-8 ENCODING (1-4 bytes); one truncated byte printed garbage
    s.push_str("define i64 @__chrstr(i64 %c) {\nentry:\n  %p = call i8* @__smalloc(i64 5)\n  %p1 = getelementptr i8, i8* %p, i64 1\n  %p2 = getelementptr i8, i8* %p, i64 2\n  %p3 = getelementptr i8, i8* %p, i64 3\n  %p4 = getelementptr i8, i8* %p, i64 4\n  %lo6 = and i64 %c, 63\n  %s6 = lshr i64 %c, 6\n  %mid6 = and i64 %s6, 63\n  %s12 = lshr i64 %c, 12\n  %hi6 = and i64 %s12, 63\n  %s18 = lshr i64 %c, 18\n  %clo = or i64 %lo6, 128\n  %cmid = or i64 %mid6, 128\n  %chi = or i64 %hi6, 128\n  %tlo = trunc i64 %clo to i8\n  %tmid = trunc i64 %cmid to i8\n  %thi = trunc i64 %chi to i8\n  %a1 = icmp ult i64 %c, 128\n  br i1 %a1, label %one, label %m2\none:\n  %b = trunc i64 %c to i8\n  store i8 %b, i8* %p\n  store i8 0, i8* %p1\n  call void @__sset(i8* %p, i64 1)\n  br label %done\nm2:\n  %a2 = icmp ult i64 %c, 2048\n  br i1 %a2, label %two, label %m3\ntwo:\n  %l2 = or i64 %s6, 192\n  %t2 = trunc i64 %l2 to i8\n  store i8 %t2, i8* %p\n  store i8 %tlo, i8* %p1\n  store i8 0, i8* %p2\n  call void @__sset(i8* %p, i64 2)\n  br label %done\nm3:\n  %a3 = icmp ult i64 %c, 65536\n  br i1 %a3, label %three, label %four\nthree:\n  %l3 = or i64 %s12, 224\n  %t3 = trunc i64 %l3 to i8\n  store i8 %t3, i8* %p\n  store i8 %tmid, i8* %p1\n  store i8 %tlo, i8* %p2\n  store i8 0, i8* %p3\n  call void @__sset(i8* %p, i64 3)\n  br label %done\nfour:\n  %l4 = or i64 %s18, 240\n  %t4 = trunc i64 %l4 to i8\n  store i8 %t4, i8* %p\n  store i8 %thi, i8* %p1\n  store i8 %tmid, i8* %p2\n  store i8 %tlo, i8* %p3\n  store i8 0, i8* %p4\n  call void @__sset(i8* %p, i64 4)\n  br label %done\ndone:\n  %r = ptrtoint i8* %p to i64\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__streq(i64 %a, i64 %b) {\nentry:\n  %la = icmp ult i64 %a, 4096\n  %lb = icmp ult i64 %b, 4096\n  %bad = or i1 %la, %lb\n  br i1 %bad, label %oops, label %ok\noops:\n  %m = getelementptr [59 x i8], [59 x i8]* @.strg, i64 0, i64 0\n  %pr = call i32 @printf(i8* %m)\n  call void @exit(i32 101)\n  unreachable\nok:\n  %pa = inttoptr i64 %a to i8*\n  %pb = inttoptr i64 %b to i8*\n  %na = call i64 @__slenp(i8* %pa)\n  %nb = call i64 @__slenp(i8* %pb)\n  %sn = icmp eq i64 %na, %nb\n  br i1 %sn, label %cmp, label %ne\nne:\n  ret i64 0\ncmp:\n  %c = call i32 @memcmp(i8* %pa, i8* %pb, i64 %na)\n  %z = icmp eq i32 %c, 0\n  %r = zext i1 %z to i64\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__fileok(i64 %pth) {\nentry:\n  %p = inttoptr i64 %pth to i8*\n  %m = getelementptr [2 x i8], [2 x i8]* @.rdmode, i64 0, i64 0\n  %f = call i8* @fopen(i8* %p, i8* %m)\n  %isn = icmp eq i8* %f, null\n  br i1 %isn, label %bad, label %ok\nbad:\n  ret i64 0\nok:\n  %cl = call i32 @fclose(i8* %f)\n  ret i64 1\n}\n");
    // SELF-DIFF: a file with a NUL byte (UTF-16 source) would be TRUNCATED at it by our NUL-terminated str -- ourc' then
    // compiled the empty prefix and ACCEPTED parser/utf16-be-without-bom; a short strlen vs the bytes read answers "" (the
    // driver's Err arm: unreadable input), as rustc rejects a non-UTF-8 source
    // V1.2 DRIVE-LINK: run a shell command, answer its exit code ((status >> 8) & 255; a failed launch is non-zero)
    s.push_str("declare i32 @system(i8*)\ndefine i64 @__syscmd(i64 %c) {\nentry:\n  %p = inttoptr i64 %c to i8*\n  %r = call i32 @system(i8* %p)\n  %w = sext i32 %r to i64\n  %x = ashr i64 %w, 8\n  %e = and i64 %x, 255\n  ret i64 %e\n}\n");
    // V5.3 BUDGET: SIGALRM after `secs` -> write the diagnostic to stderr, _exit(1) (async-signal-safe: write + _exit only)
    s.push_str("@.budmsg = private unnamed_addr constant [86 x i8] c\"ourc: compile time limit exceeded -- refused (raise it with --time-limit N, 0 = none)\\0A\"\ndeclare i8* @signal(i32, i8*)\ndeclare i32 @alarm(i32)\ndeclare i32 @setrlimit(i32, i64*)\ndeclare i32 @sigaltstack(i8*, i8*)\ndeclare i32 @sigaction(i32, i8*, i8*)\ndeclare void @llvm.memset.p0i8.i64(i8*, i8, i64, i1)\n@.segvmsg = private unnamed_addr constant [83 x i8] c\"ourc: internal crash (most likely recursion too deep) -- refused; please report it\\0A\"\ndefine void @__on_segv(i32 %sig) {\nentry:\n  %m = getelementptr [83 x i8], [83 x i8]* @.segvmsg, i64 0, i64 0\n  %w = call i64 @write(i32 2, i8* %m, i64 83)\n  call void @_exit(i32 1)\n  unreachable\n}\ndefine void @__on_alarm(i32 %sig) {\nentry:\n  %m = getelementptr [86 x i8], [86 x i8]* @.budmsg, i64 0, i64 0\n  %w = call i64 @write(i32 2, i8* %m, i64 86)\n  call void @_exit(i32 1)\n  unreachable\n}\ndefine i64 @__budget(i64 %s, i64 %gb) {\nentry:\n  store i64 1, i64* @__budget_on\n  %ass = call i8* @malloc(i64 262144)\n  %st = alloca [24 x i8]\n  %stp = getelementptr [24 x i8], [24 x i8]* %st, i64 0, i64 0\n  %st0 = bitcast i8* %stp to i8**\n  store i8* %ass, i8** %st0\n  %st8b = getelementptr i8, i8* %stp, i64 8\n  %st8 = bitcast i8* %st8b to i32*\n  store i32 0, i32* %st8\n  %st16b = getelementptr i8, i8* %stp, i64 16\n  %st16 = bitcast i8* %st16b to i64*\n  store i64 262144, i64* %st16\n  %sa1 = call i32 @sigaltstack(i8* %stp, i8* null)\n  %sa = alloca [152 x i8]\n  %sap = getelementptr [152 x i8], [152 x i8]* %sa, i64 0, i64 0\n  call void @llvm.memset.p0i8.i64(i8* %sap, i8 0, i64 152, i1 false)\n  %sah = bitcast i8* %sap to void (i32)**\n  store void (i32)* @__on_segv, void (i32)** %sah\n  %saf8 = getelementptr i8, i8* %sap, i64 136\n  %saf = bitcast i8* %saf8 to i32*\n  store i32 134217728, i32* %saf\n  %sg1 = call i32 @sigaction(i32 11, i8* %sap, i8* null)\n  %sg2 = call i32 @sigaction(i32 7, i8* %sap, i8* null)\n  %hasm = icmp sgt i64 %gb, 0\n  br i1 %hasm, label %mem, label %tim\nmem:\n  %rl = alloca [2 x i64]\n  %r0 = getelementptr [2 x i64], [2 x i64]* %rl, i64 0, i64 0\n  %r1 = getelementptr [2 x i64], [2 x i64]* %rl, i64 0, i64 1\n  %by = shl i64 %gb, 30\n  store i64 %by, i64* %r0\n  store i64 %by, i64* %r1\n  %sr = call i32 @setrlimit(i32 9, i64* %r0)\n  br label %tim\ntim:\n  %hast = icmp sgt i64 %s, 0\n  br i1 %hast, label %arm, label %done\narm:\n  %h = bitcast void (i32)* @__on_alarm to i8*\n  %o = call i8* @signal(i32 14, i8* %h)\n  %t = trunc i64 %s to i32\n  %r = call i32 @alarm(i32 %t)\n  br label %done\ndone:\n  ret i64 0\n}\n");
    // V1.2: write a str to a file (truncating); 0 = written, 1 = could not open / short write
    s.push_str("@.wrmode = private unnamed_addr constant [2 x i8] c\"w\\00\"\ndefine i64 @__writefile(i64 %pth, i64 %txt) {\nentry:\n  %p = inttoptr i64 %pth to i8*\n  %m = getelementptr [2 x i8], [2 x i8]* @.wrmode, i64 0, i64 0\n  %f = call i8* @fopen(i8* %p, i8* %m)\n  %isn = icmp eq i8* %f, null\n  br i1 %isn, label %bad, label %ok\nbad:\n  ret i64 1\nok:\n  %d = inttoptr i64 %txt to i8*\n  %n = call i64 @__slen(i64 %txt)\n  %w = call i64 @fwrite(i8* %d, i64 1, i64 %n, i8* %f)\n  %cl = call i32 @fclose(i8* %f)\n  %short = icmp ne i64 %w, %n\n  br i1 %short, label %bad, label %fin\nfin:\n  ret i64 0\n}\n");
    s.push_str("define i64 @__readfile(i64 %pth) {\nentry:\n  %p = inttoptr i64 %pth to i8*\n  %m = getelementptr [2 x i8], [2 x i8]* @.rdmode, i64 0, i64 0\n  %f = call i8* @fopen(i8* %p, i8* %m)\n  %isn = icmp eq i8* %f, null\n  br i1 %isn, label %bad, label %ok\nbad:\n  %z = call i8* @__smalloc(i64 1)\n  store i8 0, i8* %z\n  %zr = ptrtoint i8* %z to i64\n  ret i64 %zr\nok:\n  %s0 = call i64 @fseek(i8* %f, i64 0, i32 2)\n  %n = call i64 @ftell(i8* %f)\n  %s1 = call i64 @fseek(i8* %f, i64 0, i32 0)\n  %n1 = add i64 %n, 1\n  %d = call i8* @__smalloc(i64 %n1)\n  %got = call i64 @fread(i8* %d, i64 1, i64 %n, i8* %f)\n  call void @__sset(i8* %d, i64 %got)\n  %e = getelementptr i8, i8* %d, i64 %got\n  store i8 0, i8* %e\n  %cl = call i32 @fclose(i8* %f)\n  %sl = call i64 @strlen(i8* %d)\n  %hasnul = icmp ne i64 %sl, %got\n  br i1 %hasnul, label %bad, label %fin\nfin:\n  %r = ptrtoint i8* %d to i64\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__chr1(i64 %c) {\nentry:\n  %r = call i64 @__chrstr(i64 %c)\n  ret i64 %r\n}\n"); // UTF8-STR: push(char) appends the UTF-8 encoding (was one truncated byte)
                                                                                                              // V1a: clamp to the bytes there are -- a NUL-terminated str made an over-long sub stop at the terminator (fmt padding asked
                                                                                                              // for more than it had); with a length header the extra bytes would be copied as `\0`s
    s.push_str("define i64 @__str_sub(i64 %s, i64 %o, i64 %n0) {\nentry:\n  %p = inttoptr i64 %s to i8*\n  %sl = call i64 @__slenp(i8* %p)\n  %av0 = sub i64 %sl, %o\n  %avn = icmp slt i64 %av0, 0\n  %av = select i1 %avn, i64 0, i64 %av0\n  %big = icmp sgt i64 %n0, %av\n  %n = select i1 %big, i64 %av, i64 %n0\n  %q = getelementptr i8, i8* %p, i64 %o\n  %n1 = add i64 %n, 1\n  %d = call i8* @__smalloc(i64 %n1)\n  %c1 = call i8* @memcpy(i8* %d, i8* %q, i64 %n)\n  %e = getelementptr i8, i8* %d, i64 %n\n  store i8 0, i8* %e\n  %r = ptrtoint i8* %d to i64\n  ret i64 %r\n}\n");
    s
}
pub fn vec_runtime() -> String {
    let mut s = String::from("");
    s.push_str("declare i8* @malloc(i64)\ndeclare i8* @realloc(i8*, i64)\n");
    // V5.3 BUDGET: every allocation is checked -- a failed one (the --mem-limit cap, or real exhaustion) ends with a message
    // and exit 1 instead of a NULL dereference. @__budget_on: ourc's own driver armed the limits (its message names the flag)
    s.push_str("@__budget_on = global i64 0\n@.oom_ourc = private unnamed_addr constant [80 x i8] c\"ourc: memory limit exceeded -- refused (raise it with --mem-limit GB, 0 = none)\\0A\"\n@.oom_prog = private unnamed_addr constant [25 x i8] c\"memory allocation failed\\0A\"\ndeclare i64 @write(i32, i8*, i64)\ndeclare void @_exit(i32)\ndefine void @__oom() {\nentry:\n  %on = load i64, i64* @__budget_on\n  %o = icmp ne i64 %on, 0\n  br i1 %o, label %ourc, label %prog\nourc:\n  %m = getelementptr [80 x i8], [80 x i8]* @.oom_ourc, i64 0, i64 0\n  %w = call i64 @write(i32 2, i8* %m, i64 80)\n  call void @_exit(i32 1)\n  unreachable\nprog:\n  %m2 = getelementptr [25 x i8], [25 x i8]* @.oom_prog, i64 0, i64 0\n  %w2 = call i64 @write(i32 2, i8* %m2, i64 25)\n  call void @_exit(i32 1)\n  unreachable\n}\ndefine i8* @__xmalloc(i64 %n) {\nentry:\n  %p = call i8* @malloc(i64 %n)\n  %z = icmp eq i8* %p, null\n  br i1 %z, label %bad, label %ok\nbad:\n  call void @__oom()\n  unreachable\nok:\n  ret i8* %p\n}\ndefine i8* @__xrealloc(i8* %q, i64 %n) {\nentry:\n  %p = call i8* @realloc(i8* %q, i64 %n)\n  %z = icmp eq i8* %p, null\n  br i1 %z, label %bad, label %ok\nbad:\n  call void @__oom()\n  unreachable\nok:\n  ret i8* %p\n}\n");
    s.push_str("declare double @strtod(i8*, i8**)\ndeclare i32 @snprintf(i8*, i64, i8*, ...)\n");
    // a zero divisor exits 101, the way rustc's panic does
    s.push_str("define i64 @__divchk(i64 %d) {\nentry:\n  %z = icmp eq i64 %d, 0\n  br i1 %z, label %bad, label %ok\nbad:\n  call void @exit(i32 101)\n  unreachable\nok:\n  ret i64 %d\n}\n");
    // N2: an out-of-range index exits 101 too -- our reads used to run off the end and print garbage
    s.push_str("define i64 @__ixchk(i64 %i, i64 %n) {\nentry:\n  %lo = icmp slt i64 %i, 0\n  %hi = icmp sge i64 %i, %n\n  %bad = or i1 %lo, %hi\n  br i1 %bad, label %oob, label %ok\noob:\n  call void @exit(i32 101)\n  unreachable\nok:\n  ret i64 %i\n}\n");
    s.push_str("define i64 @__vec_new() {\nentry:\n  %p = call i8* @__xmalloc(i64 24)\n  %h = bitcast i8* %p to i64*\n  store i64 0, i64* %h\n  %cp = getelementptr i64, i64* %h, i64 1\n  store i64 4, i64* %cp\n  %d = call i8* @__xmalloc(i64 32)\n  %di = ptrtoint i8* %d to i64\n  %dp = getelementptr i64, i64* %h, i64 2\n  store i64 %di, i64* %dp\n  %r = ptrtoint i64* %h to i64\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__vec_push(i64 %v, i64 %x) {\nentry:\n  %h = inttoptr i64 %v to i64*\n  %len = load i64, i64* %h\n  %cp = getelementptr i64, i64* %h, i64 1\n  %cap = load i64, i64* %cp\n  %full = icmp sge i64 %len, %cap\n  br i1 %full, label %grow, label %put\ngrow:\n  %nc = mul i64 %cap, 2\n  %dpg = getelementptr i64, i64* %h, i64 2\n  %old = load i64, i64* %dpg\n  %oldp = inttoptr i64 %old to i8*\n  %bytes = mul i64 %nc, 8\n  %new = call i8* @__xrealloc(i8* %oldp, i64 %bytes)\n  %newi = ptrtoint i8* %new to i64\n  store i64 %newi, i64* %dpg\n  store i64 %nc, i64* %cp\n  br label %put\nput:\n  %dp2 = getelementptr i64, i64* %h, i64 2\n  %d = load i64, i64* %dp2\n  %dpp = inttoptr i64 %d to i64*\n  %slot = getelementptr i64, i64* %dpp, i64 %len\n  store i64 %x, i64* %slot\n  %nl = add i64 %len, 1\n  store i64 %nl, i64* %h\n  ret i64 %nl\n}\n");
    s.push_str("define i64 @__vec_get(i64 %v, i64 %i) {\nentry:\n  %h = inttoptr i64 %v to i64*\n  %ln = load i64, i64* %h\n  %ck = call i64 @__ixchk(i64 %i, i64 %ln)\n  %dp = getelementptr i64, i64* %h, i64 2\n  %d = load i64, i64* %dp\n  %dpp = inttoptr i64 %d to i64*\n  %slot = getelementptr i64, i64* %dpp, i64 %ck\n  %r = load i64, i64* %slot\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__vec_elem_addr(i64 %v, i64 %i) {\nentry:\n  %h = inttoptr i64 %v to i64*\n  %dp = getelementptr i64, i64* %h, i64 2\n  %d = load i64, i64* %dp\n  %dpp = inttoptr i64 %d to i64*\n  %slot = getelementptr i64, i64* %dpp, i64 %i\n  %r = ptrtoint i64* %slot to i64\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__fld_addr(i64 %b, i64 %i) {\nentry:\n  %p = inttoptr i64 %b to i64*\n  %q = getelementptr i64, i64* %p, i64 %i\n  %r = ptrtoint i64* %q to i64\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__store_at(i64 %a, i64 %x) {\nentry:\n  %p = inttoptr i64 %a to i64*\n  store i64 %x, i64* %p\n  ret i64 0\n}\n");
    s.push_str("define i64 @__load_at(i64 %a) {\nentry:\n  %p = inttoptr i64 %a to i64*\n  %r = load i64, i64* %p\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__vec_set(i64 %v, i64 %i, i64 %x) {\nentry:\n  %h = inttoptr i64 %v to i64*\n  %dp = getelementptr i64, i64* %h, i64 2\n  %d = load i64, i64* %dp\n  %dpp = inttoptr i64 %d to i64*\n  %slot = getelementptr i64, i64* %dpp, i64 %i\n  store i64 %x, i64* %slot\n  ret i64 %x\n}\n");
    s.push_str("define i64 @__vec_popraw(i64 %v) {\nentry:\n  %h = inttoptr i64 %v to i64*\n  %n = load i64, i64* %h\n  %n1 = sub i64 %n, 1\n  store i64 %n1, i64* %h\n  %dp = getelementptr i64, i64* %h, i64 2\n  %d = load i64, i64* %dp\n  %dpp = inttoptr i64 %d to i64*\n  %slot = getelementptr i64, i64* %dpp, i64 %n1\n  %r = load i64, i64* %slot\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__arr_get(i64 %a, i64 %i) {\nentry:\n  %p = inttoptr i64 %a to i64*\n  %n = load i64, i64* %p\n  %ck = call i64 @__ixchk(i64 %i, i64 %n)\n  %ix = add i64 1, %ck\n  %ep = getelementptr i64, i64* %p, i64 %ix\n  %r = load i64, i64* %ep\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__arr_set(i64 %a, i64 %i, i64 %v) {\nentry:\n  %p = inttoptr i64 %a to i64*\n  %n = load i64, i64* %p\n  %ck = call i64 @__ixchk(i64 %i, i64 %n)\n  %ix = add i64 1, %ck\n  %ep = getelementptr i64, i64* %p, i64 %ix\n  store i64 %v, i64* %ep\n  ret i64 %v\n}\n");
    s.push_str("define i64 @__arr_len(i64 %a) {\nentry:\n  %p = inttoptr i64 %a to i64*\n  %n = load i64, i64* %p\n  ret i64 %n\n}\n");
    // two enum values are equal when their tag and every payload WORD agree (the static side refuses a
    // payload whose type is not word-comparable, so this never compares a pointer as a value)
    s.push_str("define i64 @__enumeq(i64 %a, i64 %b, i64 %n) {\nentry:\n  %ap = inttoptr i64 %a to i64*\n  %bp = inttoptr i64 %b to i64*\n  br label %loop\nloop:\n  %i = phi i64 [ 0, %entry ], [ %i2, %body ]\n  %c = icmp slt i64 %i, %n\n  br i1 %c, label %body, label %same\nbody:\n  %aep = getelementptr i64, i64* %ap, i64 %i\n  %bep = getelementptr i64, i64* %bp, i64 %i\n  %av = load i64, i64* %aep\n  %bv = load i64, i64* %bep\n  %eq = icmp eq i64 %av, %bv\n  %i2 = add i64 %i, 1\n  br i1 %eq, label %loop, label %diff\ndiff:\n  ret i64 0\nsame:\n  ret i64 1\n}\n");
    // SLICING copies: `v[a..b]` / `arr[a..b]` produce a fresh VECTOR, so one representation answers
    // every later read (len, index, equality) regardless of what was sliced
    s.push_str("define i64 @__vec_slice(i64 %v, i64 %a, i64 %b) {\nentry:\n  %n = call i64 @__vec_new()\n  br label %loop\nloop:\n  %i = phi i64 [ %a, %entry ], [ %i2, %body ]\n  %c = icmp slt i64 %i, %b\n  br i1 %c, label %body, label %done\nbody:\n  %x = call i64 @__vec_get(i64 %v, i64 %i)\n  %p = call i64 @__vec_push(i64 %n, i64 %x)\n  %i2 = add i64 %i, 1\n  br label %loop\ndone:\n  ret i64 %n\n}\n");
    s.push_str("define i64 @__arr_slice(i64 %v, i64 %a, i64 %b) {\nentry:\n  %n = call i64 @__vec_new()\n  %bp = inttoptr i64 %v to i64*\n  br label %loop\nloop:\n  %i = phi i64 [ %a, %entry ], [ %i2, %body ]\n  %c = icmp slt i64 %i, %b\n  br i1 %c, label %body, label %done\nbody:\n  %ix = add i64 1, %i\n  %ep = getelementptr i64, i64* %bp, i64 %ix\n  %x = load i64, i64* %ep\n  %p = call i64 @__vec_push(i64 %n, i64 %x)\n  %i2 = add i64 %i, 1\n  br label %loop\ndone:\n  ret i64 %n\n}\n");
    s.push_str("define i64 @__arr_fill(i64 %n, i64 %x) {\nentry:\n  %n1 = add i64 %n, 1\n  %sz = mul i64 %n1, 8\n  %p = call i8* @__xmalloc(i64 %sz)\n  %h = bitcast i8* %p to i64*\n  store i64 %n, i64* %h\n  br label %lc\nlc:\n  %i = phi i64 [ 1, %entry ], [ %i2, %lb ]\n  %more = icmp sle i64 %i, %n\n  br i1 %more, label %lb, label %done\nlb:\n  %ep = getelementptr i64, i64* %h, i64 %i\n  store i64 %x, i64* %ep\n  %i2 = add i64 %i, 1\n  br label %lc\ndone:\n  %r = ptrtoint i8* %p to i64\n  ret i64 %r\n}\n"); // ARR-REP-LOOP: [x; n] for a scalar x -- header n at slot 0, x in slots 1..n (the ArrLit layout)
    s.push_str("define i64 @__arreq(i64 %a, i64 %b) {\nentry:\n  %pa = inttoptr i64 %a to i64*\n  %pb = inttoptr i64 %b to i64*\n  %na = load i64, i64* %pa\n  %nb = load i64, i64* %pb\n  %sl = icmp eq i64 %na, %nb\n  br i1 %sl, label %go, label %ne\nne:\n  ret i64 0\ngo:\n  br label %lc\nlc:\n  %i = phi i64 [ 1, %go ], [ %i2, %lb2 ]\n  %more = icmp sle i64 %i, %na\n  br i1 %more, label %lb2, label %eq\nlb2:\n  %ga = getelementptr i64, i64* %pa, i64 %i\n  %gb = getelementptr i64, i64* %pb, i64 %i\n  %va = load i64, i64* %ga\n  %vb = load i64, i64* %gb\n  %same = icmp eq i64 %va, %vb\n  %i2 = add i64 %i, 1\n  br i1 %same, label %lc, label %ne\neq:\n  ret i64 1\n}\n");
    s.push_str("define i64 @__veceq(i64 %a, i64 %b) {\nentry:\n  %ha = inttoptr i64 %a to i64*\n  %hb = inttoptr i64 %b to i64*\n  %la = load i64, i64* %ha\n  %lb = load i64, i64* %hb\n  %sl = icmp eq i64 %la, %lb\n  br i1 %sl, label %go, label %ne\nne:\n  ret i64 0\ngo:\n  %dpa = getelementptr i64, i64* %ha, i64 2\n  %dpb = getelementptr i64, i64* %hb, i64 2\n  %da = load i64, i64* %dpa\n  %db = load i64, i64* %dpb\n  %pa = inttoptr i64 %da to i64*\n  %pb = inttoptr i64 %db to i64*\n  br label %lc\nlc:\n  %i = phi i64 [ 0, %go ], [ %i2, %lb2 ]\n  %more = icmp slt i64 %i, %la\n  br i1 %more, label %lb2, label %eq\nlb2:\n  %ga = getelementptr i64, i64* %pa, i64 %i\n  %gb = getelementptr i64, i64* %pb, i64 %i\n  %va = load i64, i64* %ga\n  %vb = load i64, i64* %gb\n  %same = icmp eq i64 %va, %vb\n  %i2 = add i64 %i, 1\n  br i1 %same, label %lc, label %ne\neq:\n  ret i64 1\n}\n");
    // RC-IDENTITY (R4c): the Rc strong-count side table, keyed by the referent's own heap pointer (the Rc handle IS that pointer)
    s.push_str("define i64 @__rc_slot(i64 %p) {\nentry:\n  %n = load i64, i64* @__rcn\n  br label %lc\nlc:\n  %i = phi i64 [ 0, %entry ], [ %i2, %nx ]\n  %more = icmp slt i64 %i, %n\n  br i1 %more, label %lb, label %miss\nlb:\n  %ep = getelementptr [65536 x i64], [65536 x i64]* @__rcp, i64 0, i64 %i\n  %e = load i64, i64* %ep\n  %hit = icmp eq i64 %e, %p\n  br i1 %hit, label %found, label %nx\nnx:\n  %i2 = add i64 %i, 1\n  br label %lc\nfound:\n  ret i64 %i\nmiss:\n  ret i64 -1\n}\n");
    s.push_str("define i64 @__rc_new(i64 %p) {\nentry:\n  %k = call i64 @__rc_slot(i64 %p)\n  %have = icmp sge i64 %k, 0\n  br i1 %have, label %set, label %add\nadd:\n  %n = load i64, i64* @__rcn\n  %pp = getelementptr [65536 x i64], [65536 x i64]* @__rcp, i64 0, i64 %n\n  store i64 %p, i64* %pp\n  %n2 = add i64 %n, 1\n  store i64 %n2, i64* @__rcn\n  br label %set\nset:\n  %s = phi i64 [ %k, %entry ], [ %n, %add ]\n  %cp = getelementptr [65536 x i64], [65536 x i64]* @__rcc, i64 0, i64 %s\n  store i64 1, i64* %cp\n  ret i64 %p\n}\n");
    s.push_str("define i64 @__rc_inc(i64 %p) {\nentry:\n  %k = call i64 @__rc_slot(i64 %p)\n  %cp = getelementptr [65536 x i64], [65536 x i64]* @__rcc, i64 0, i64 %k\n  %c = load i64, i64* %cp\n  %c2 = add i64 %c, 1\n  store i64 %c2, i64* %cp\n  ret i64 %p\n}\n");
    s.push_str("define i64 @__rc_dec(i64 %p) {\nentry:\n  %k = call i64 @__rc_slot(i64 %p)\n  %cp = getelementptr [65536 x i64], [65536 x i64]* @__rcc, i64 0, i64 %k\n  %c = load i64, i64* %cp\n  %c2 = sub i64 %c, 1\n  store i64 %c2, i64* %cp\n  ret i64 %c2\n}\n");
    s.push_str("define i64 @__rc_count(i64 %p) {\nentry:\n  %k = call i64 @__rc_slot(i64 %p)\n  %cp = getelementptr [65536 x i64], [65536 x i64]* @__rcc, i64 0, i64 %k\n  %c = load i64, i64* %cp\n  ret i64 %c\n}\n");
    s.push_str("define i64 @__vec_len(i64 %v) {\nentry:\n  %h = inttoptr i64 %v to i64*\n  %r = load i64, i64* %h\n  ret i64 %r\n}\n");
    s
}
pub fn ops_return(ops: &Vec<Op>) -> bool {
    let mut k = 0;
    while k < ops.len() {
        match &ops[k] {
            Op::RetEarly(_t) => {
                return true;
            }
            _ => {}
        }
        k = k + 1;
    }
    false
}
fn body_ops_l(ops: &Vec<Op>) -> String {
    let no: Vec<i64> = Vec::new();
    body_ops_in(ops, 0 - 1, &no)
}
// `lid` is the ENCLOSING loop's label id (-1 = none): what `break`/`continue` need to branch to
// outs: the ids of ALL enclosing loops, innermost last (LABEL: `break 'l` = BrkN(n) leaves n loops beyond the innermost)
fn body_ops_in(ops: &Vec<Op>, lid0: i64, outs: &Vec<i64>) -> String {
    let lid = lid0;
    let lt = oabi::ll_ty(&oabi::layout_int());
    let mut s = String::from("");
    let mut k = 0;
    while k < ops.len() {
        match &ops[k] {
            Op::ParamOp(t, p) => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = add i64 0, %p");
                s.push_str(&n32(*p));
                s.push_str("\n");
            }
            Op::CastOp(t, sx, ty) => {
                if ty.as_str() == "$i2f" {
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str("f = sitofp i64 ");
                    s.push_str(&tmp(*sx));
                    s.push_str(
                        " to double
  ",
                    );
                    s.push_str(&tmp(*t));
                    s.push_str(" = bitcast double ");
                    s.push_str(&tmp(*t));
                    s.push_str(
                        "f to i64
",
                    );
                } else
                // D1.1 int->f64
                if ty.len() >= 5 && &ty[0..5] == "$f2i$" {
                    let w = &ty[5..];
                    let bits = oast::width_bits(w);
                    let sg = oast::width_signed(w);
                    let bs = n32(bits);
                    let intr = if sg {
                        String::from("llvm.fptosi.sat.i")
                    } else {
                        String::from("llvm.fptoui.sat.i")
                    };
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str("d = bitcast i64 ");
                    s.push_str(&tmp(*sx));
                    s.push_str(" to double\n");
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str("n = call i");
                    s.push_str(&bs);
                    s.push_str(" @");
                    s.push_str(&intr);
                    s.push_str(&bs);
                    s.push_str(".f64(double ");
                    s.push_str(&tmp(*t));
                    s.push_str("d)\n");
                    if bits >= 64 {
                        s.push_str("  ");
                        s.push_str(&tmp(*t));
                        s.push_str(" = add i64 0, ");
                        s.push_str(&tmp(*t));
                        s.push_str("n\n");
                    } else {
                        s.push_str("  ");
                        s.push_str(&tmp(*t));
                        s.push_str(" = ");
                        if sg {
                            s.push_str("sext i");
                        } else {
                            s.push_str("zext i");
                        }
                        s.push_str(&bs);
                        s.push_str(" ");
                        s.push_str(&tmp(*t));
                        s.push_str("n to i64\n");
                    }
                } else if ty.as_str() == "$f2i" {
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str("d = bitcast i64 ");
                    s.push_str(&tmp(*sx));
                    s.push_str(
                        " to double
  ",
                    );
                    s.push_str(&tmp(*t));
                    s.push_str(" = fptosi double ");
                    s.push_str(&tmp(*t));
                    s.push_str(
                        "d to i64
",
                    );
                } else
                // D1.1 f64->int
                if ty.as_str() == "$idcast" {
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str(" = add i64 0, ");
                    s.push_str(&tmp(*sx));
                    s.push_str(
                        "
",
                    );
                } else {
                    // a width cast really truncates: trunc to the width, then zext (unsigned) or sext (signed).
                    // 64-bit targets are identity.
                    let bits = oast::width_bits(ty);
                    let sg = oast::width_signed(ty);
                    if bits >= 64 {
                        s.push_str("  ");
                        s.push_str(&tmp(*t));
                        s.push_str(" = add i64 0, ");
                        s.push_str(&tmp(*sx));
                        s.push_str("\n");
                    } else {
                        s.push_str("  ");
                        s.push_str(&tmp(*t));
                        s.push_str("w = trunc i64 ");
                        s.push_str(&tmp(*sx));
                        s.push_str(" to i");
                        s.push_str(&n32(bits));
                        s.push_str("\n  ");
                        s.push_str(&tmp(*t));
                        s.push_str(" = ");
                        if sg {
                            s.push_str("sext i");
                        } else {
                            s.push_str("zext i");
                        }
                        s.push_str(&n32(bits));
                        s.push_str(" ");
                        s.push_str(&tmp(*t));
                        s.push_str("w to i64\n");
                    }
                }
            }
            Op::Brk(bid) => {
                if lid >= 0 {
                    s.push_str("  br label %lend");
                    s.push_str(&n32(lid as u32));
                    s.push_str("\nbk");
                    s.push_str(&n32(*bid));
                    s.push_str("_");
                    s.push_str(&n32(k as u32));
                    s.push_str(":\n");
                }
            }
            Op::BrkN(bid, n) => {
                let nn = *n as usize;
                if nn < outs.len() {
                    let tgt = outs[outs.len() - 1 - nn];
                    s.push_str("  br label %lend");
                    s.push_str(&n32(tgt as u32));
                    s.push_str("\nbk");
                    s.push_str(&n32(*bid));
                    s.push_str("_");
                    s.push_str(&n32(k as u32));
                    s.push_str(":\n");
                }
            }
            Op::ContN(cid, n) => {
                let nn = *n as usize;
                if nn < outs.len() {
                    let tgt = outs[outs.len() - 1 - nn];
                    s.push_str("  br label %lcond");
                    s.push_str(&n32(tgt as u32));
                    s.push_str("\nct");
                    s.push_str(&n32(*cid));
                    s.push_str("_");
                    s.push_str(&n32(k as u32));
                    s.push_str(":\n");
                }
            }
            Op::Cont(cid) => {
                if lid >= 0 {
                    s.push_str("  br label %lcond");
                    s.push_str(&n32(lid as u32));
                    s.push_str("\nct");
                    s.push_str(&n32(*cid));
                    s.push_str("_");
                    s.push_str(&n32(k as u32));
                    s.push_str(":\n");
                }
            }
            Op::RetEarly(t) => {
                s.push_str("  ret i64 ");
                s.push_str(&tmp(*t));
                s.push_str("\n");
                s.push_str("ar");
                s.push_str(&n32(*t));
                s.push_str(":\n");
            }
            Op::StrC(t, v) => {
                let n = v.len() + 1;
                s.push_str("  ");
                s.push_str(&tmp(*t));
                // V1a: the value is the address of the BYTES (field 1), past the length header
                let ty = format!("<{{ i64, [{} x i8] }}>", n32(n as u32));
                s.push_str(" = ptrtoint i8* getelementptr inbounds (");
                s.push_str(&ty);
                s.push_str(", ");
                s.push_str(&ty);
                s.push_str("* @");
                s.push_str(&str_gid(v.as_str()));
                s.push_str(", i32 0, i32 1, i64 0) to i64\n");
            }
            // print/len are the two string PRIMITIVES, lowered to libc on the i8* recovered from the i64 value
            Op::CallOp(t, n, av) if n.as_str() == "print" && av.len() == 1 => {
                // V1a: print by LENGTH (fwrite), so a `\0` inside the string is printed, not the end of it
                s.push_str("  call void @__sput(i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(", i64 1)\n  ");
                s.push_str(&tmp(*t));
                s.push_str(" = add i64 0, 0\n");
            }
            Op::CallOp(t, n, av)
                if n.as_str() == "str_at"
                    || n.as_str() == "str_cat"
                    || n.as_str() == "str_sub"
                    || n.as_str() == "chr1"
                    || n.as_str() == "str_grow"
                    || n.as_str() == "str_copy" =>
            {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__");
                s.push_str(n);
                s.push_str("(");
                let mut j = 0;
                while j < av.len() {
                    if j > 0 {
                        s.push_str(", ");
                    }
                    s.push_str("i64 ");
                    s.push_str(&tmp(av[j]));
                    j = j + 1;
                }
                s.push_str(")\n");
            }
            Op::CallOp(t, n, av)
                if n.as_str() == "vec_new"
                    || n.as_str() == "vec_push"
                    || n.as_str() == "vec_push1"
                    || n.as_str() == "vec_get"
                    || n.as_str() == "vec_len"
                    || n.as_str() == "vec_set"
                    || n.as_str() == "vec_elem_addr"
                    || n.as_str() == "fld_addr"
                    || n.as_str() == "store_at"
                    || n.as_str() == "load_at" =>
            {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__");
                s.push_str(n);
                s.push_str("(");
                let mut j = 0;
                while j < av.len() {
                    if j > 0 {
                        s.push_str(", ");
                    }
                    s.push_str("i64 ");
                    s.push_str(&tmp(av[j]));
                    j = j + 1;
                }
                s.push_str(")\n");
            }
            Op::CallOp(t, n, av)
                if (n.as_str() == "arreq" && av.len() == 2)
                    || (n.as_str() == "arr_len" && av.len() == 1)
                    || ((n.as_str() == "vec_slice" || n.as_str() == "arr_slice")
                        && av.len() == 3)
                    || (n.as_str() == "enumeq" && av.len() == 3)
                    || (n.as_str() == "vec_popraw" && av.len() == 1)
                    || (n.as_str() == "arr_fill" && av.len() == 2)
                    || ((n.as_str() == "rc_new"
                        || n.as_str() == "rc_inc"
                        || n.as_str() == "rc_dec"
                        || n.as_str() == "rc_count")
                        && av.len() == 1) =>
            {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__");
                s.push_str(n);
                s.push_str("(");
                let mut j = 0;
                while j < av.len() {
                    if j > 0 {
                        s.push_str(", ");
                    }
                    s.push_str("i64 ");
                    s.push_str(&tmp(av[j]));
                    j = j + 1;
                }
                s.push_str(")\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "veceq" && av.len() == 2 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__veceq(i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(", i64 ");
                s.push_str(&tmp(av[1]));
                s.push_str(")\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "chr_str" && av.len() == 1 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__chrstr(i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(")\n");
            } // CHR-STR
            Op::CallOp(t, n, av) if n.as_str() == "streq" && av.len() == 2 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__streq(i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(", i64 ");
                s.push_str(&tmp(av[1]));
                s.push_str(")\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "print_raw" && av.len() == 1 => {
                // V1a: by length, no newline
                s.push_str("  call void @__sput(i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(", i64 0)\n  ");
                s.push_str(&tmp(*t));
                s.push_str(" = add i64 0, 0\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "print_uint_raw" && av.len() == 1 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str("c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.fmtur, i64 0, i64 0), i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(")\n  ");
                s.push_str(&tmp(*t));
                s.push_str(" = sext i32 ");
                s.push_str(&tmp(*t));
                s.push_str("c to i64\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "print_int_raw" && av.len() == 1 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str("c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.fmtdr, i64 0, i64 0), i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(")\n  ");
                s.push_str(&tmp(*t));
                s.push_str(" = sext i32 ");
                s.push_str(&tmp(*t));
                s.push_str("c to i64\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "proc_exit" && av.len() == 1 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str("w = trunc i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(" to i32\n  call void @exit(i32 ");
                s.push_str(&tmp(*t));
                s.push_str("w)\n  ");
                s.push_str(&tmp(*t));
                s.push_str(" = add i64 0, 0\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "panic_exit" && av.len() == 0 => {
                s.push_str("  call void @exit(i32 101)\n  ");
                s.push_str(&tmp(*t));
                s.push_str(" = add i64 0, 0\n");
            }
            Op::CallOp(t, n, av) if olower::is_gname(n) => {
                let g = olower::gname_of(n);
                if av.len() == 0 {
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str(" = load i64, i64* @g.");
                    s.push_str(&g);
                    s.push_str("\n");
                } else {
                    s.push_str("  store i64 ");
                    s.push_str(&tmp(av[0]));
                    s.push_str(", i64* @g.");
                    s.push_str(&g);
                    s.push_str("\n  ");
                    s.push_str(&tmp(*t));
                    s.push_str(" = add i64 0, ");
                    s.push_str(&tmp(av[0]));
                    s.push_str("\n");
                }
            }
            Op::CallOp(t, n, av) if n.as_str() == "f64_parse" && av.len() == 1 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str("p = inttoptr i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(" to i8*\n  ");
                s.push_str(&tmp(*t));
                s.push_str("d = call double @strtod(i8* ");
                s.push_str(&tmp(*t));
                s.push_str("p, i8** null)\n  ");
                s.push_str(&tmp(*t));
                s.push_str(" = bitcast double ");
                s.push_str(&tmp(*t));
                s.push_str("d to i64\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "f64_snprintf" && av.len() == 2 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str("b = call i8* @__smalloc(i64 1400)\n  ");
                s.push_str(&tmp(*t));
                s.push_str("d = bitcast i64 ");
                s.push_str(&tmp(av[1]));
                s.push_str(" to double\n  ");
                s.push_str(&tmp(*t));
                s.push_str("w = trunc i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(" to i32\n  ");
                s.push_str(&tmp(*t));
                s.push_str("c = call i32 (i8*, i64, i8*, ...) @snprintf(i8* ");
                s.push_str(&tmp(*t));
                s.push_str("b, i64 1400, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmtfp, i64 0, i64 0), i32 ");
                s.push_str(&tmp(*t));
                s.push_str("w, double ");
                s.push_str(&tmp(*t));
                s.push_str("d)\n  ");
                // V1a: the header length is what snprintf wrote
                s.push_str(&tmp(*t));
                s.push_str("cl = sext i32 ");
                s.push_str(&tmp(*t));
                s.push_str("c to i64\n  call void @__sset(i8* ");
                s.push_str(&tmp(*t));
                s.push_str("b, i64 ");
                s.push_str(&tmp(*t));
                s.push_str("cl)\n  ");
                s.push_str(&tmp(*t));
                s.push_str(" = ptrtoint i8* ");
                s.push_str(&tmp(*t));
                s.push_str("b to i64\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "f64_snprintf_e" && av.len() == 2 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str("b = call i8* @__smalloc(i64 1400)\n  ");
                s.push_str(&tmp(*t));
                s.push_str("d = bitcast i64 ");
                s.push_str(&tmp(av[1]));
                s.push_str(" to double\n  ");
                s.push_str(&tmp(*t));
                s.push_str("w = trunc i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(" to i32\n  ");
                s.push_str(&tmp(*t));
                s.push_str("c = call i32 (i8*, i64, i8*, ...) @snprintf(i8* ");
                s.push_str(&tmp(*t));
                s.push_str("b, i64 1400, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmtfe, i64 0, i64 0), i32 ");
                s.push_str(&tmp(*t));
                s.push_str("w, double ");
                s.push_str(&tmp(*t));
                s.push_str("d)\n  ");
                // V1a: the header length is what snprintf wrote
                s.push_str(&tmp(*t));
                s.push_str("cl = sext i32 ");
                s.push_str(&tmp(*t));
                s.push_str("c to i64\n  call void @__sset(i8* ");
                s.push_str(&tmp(*t));
                s.push_str("b, i64 ");
                s.push_str(&tmp(*t));
                s.push_str("cl)\n  ");
                s.push_str(&tmp(*t));
                s.push_str(" = ptrtoint i8* ");
                s.push_str(&tmp(*t));
                s.push_str("b to i64\n");
            }
            Op::CallOp(t, n, av)
                if (oast::str_starts(n.as_str(), "f64m_") && av.len() == 1)
                    || (oast::str_starts(n.as_str(), "f64m2_") && av.len() == 2) =>
            {
                // F64-LIBM: bitcast in, call libm, bitcast out
                let cn = {
                    let b = if oast::str_starts(n.as_str(), "f64m2_") {
                        String::from(&n[6..])
                    } else {
                        String::from(&n[5..])
                    };
                    let mut c = String::from("llvm.");
                    c.push_str(b.as_str());
                    c.push_str(".f64");
                    c
                };
                let mut k = 0;
                while k < av.len() {
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str("d");
                    s.push_str(&oast::u32s2(k as u32));
                    s.push_str(" = bitcast i64 ");
                    s.push_str(&tmp(av[k]));
                    s.push_str(" to double\n");
                    k = k + 1;
                }
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str("r = call double @");
                s.push_str(&cn);
                s.push('(');
                k = 0;
                while k < av.len() {
                    if k > 0 {
                        s.push_str(", ");
                    }
                    s.push_str("double ");
                    s.push_str(&tmp(*t));
                    s.push_str("d");
                    s.push_str(&oast::u32s2(k as u32));
                    k = k + 1;
                }
                s.push_str(")\n");
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = bitcast double ");
                s.push_str(&tmp(*t));
                s.push_str("r to i64\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "f64_sqrt" && av.len() == 1 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str("d = bitcast i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(" to double\n");
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str("r = call double @llvm.sqrt.f64(double ");
                s.push_str(&tmp(*t));
                s.push_str("d)\n");
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = bitcast double ");
                s.push_str(&tmp(*t));
                s.push_str("r to i64\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "f64_isnan" && av.len() == 1 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str("d = bitcast i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(" to double\n  ");
                s.push_str(&tmp(*t));
                s.push_str("c = fcmp uno double ");
                s.push_str(&tmp(*t));
                s.push_str("d, ");
                s.push_str(&tmp(*t));
                s.push_str("d\n  ");
                s.push_str(&tmp(*t));
                s.push_str(" = zext i1 ");
                s.push_str(&tmp(*t));
                s.push_str("c to i64\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "print_char" && av.len() == 1 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str("w = trunc i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(" to i32\n  ");
                s.push_str(&tmp(*t));
                s.push_str("c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 ");
                s.push_str(&tmp(*t));
                s.push_str("w)\n  ");
                s.push_str(&tmp(*t));
                s.push_str(" = sext i32 ");
                s.push_str(&tmp(*t));
                s.push_str("c to i64\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "print_nl" && av.len() == 0 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str("c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.fmtnl, i64 0, i64 0))\n  ");
                s.push_str(&tmp(*t));
                s.push_str(" = sext i32 ");
                s.push_str(&tmp(*t));
                s.push_str("c to i64\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "print_int" && av.len() == 1 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str("c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmtd, i64 0, i64 0), i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(")\n  ");
                s.push_str(&tmp(*t));
                s.push_str(" = sext i32 ");
                s.push_str(&tmp(*t));
                s.push_str("c to i64\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "argc" && av.len() == 0 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__argc_get()\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "argv_at" && av.len() == 1 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__argv_at(i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(")\n");
            }
            Op::CallOp(t, n, av)
                if (n.as_str() == "cap_on" || n.as_str() == "cap_off") && av.len() == 0 =>
            {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__");
                s.push_str(n.as_str());
                s.push_str("()\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "stdio_swap" && av.len() == 1 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__stdio_swap(i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(")\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "fileok" && av.len() == 1 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__fileok(i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(")\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "budget" && av.len() == 2 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__budget(i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(", i64 ");
                s.push_str(&tmp(av[1]));
                s.push_str(")\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "writefile" && av.len() == 2 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__writefile(i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(", i64 ");
                s.push_str(&tmp(av[1]));
                s.push_str(")\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "syscmd" && av.len() == 1 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__syscmd(i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(")\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "readfile" && av.len() == 1 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__readfile(i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(")\n");
            }
            Op::CallOp(t, n, av) if n.as_str() == "len" && av.len() == 1 => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                // V1a: the byte length is the header, not a scan to the first NUL
                s.push_str(" = call i64 @__slen(i64 ");
                s.push_str(&tmp(av[0]));
                s.push_str(")\n");
            }
            Op::CallOp(t, n, av) => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = call i64 @");
                s.push_str(&csym(n));
                s.push_str("(");
                let mut j = 0;
                while j < av.len() {
                    if j > 0 {
                        s.push_str(", ");
                    }
                    s.push_str("i64 ");
                    s.push_str(&tmp(av[j]));
                    j = j + 1;
                }
                s.push_str(")\n");
            }
            Op::Const(t, v) => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = add i64 0, ");
                s.push_str(&n64(*v));
                s.push_str("\n");
            }
            Op::Bin(t, o, x, y) if is_fop(o) => {
                // an f64 rides our 64-bit word, so a float op is a bitcast pair around the real opcode
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str("a = bitcast i64 ");
                s.push_str(&tmp(*x));
                s.push_str(" to double\n");
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str("b = bitcast i64 ");
                s.push_str(&tmp(*y));
                s.push_str(" to double\n");
                if fop_is_cmp(o) {
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str("c = fcmp ");
                    s.push_str(&fllop(o));
                    s.push_str(" double ");
                    s.push_str(&tmp(*t));
                    s.push_str("a, ");
                    s.push_str(&tmp(*t));
                    s.push_str("b\n");
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str(" = zext i1 ");
                    s.push_str(&tmp(*t));
                    s.push_str("c to i64\n");
                } else {
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str("r = ");
                    s.push_str(&fllop(o));
                    s.push_str(" double ");
                    s.push_str(&tmp(*t));
                    s.push_str("a, ");
                    s.push_str(&tmp(*t));
                    s.push_str("b\n");
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str(" = bitcast double ");
                    s.push_str(&tmp(*t));
                    s.push_str("r to i64\n");
                }
            }
            Op::Bin(t, o, x, y) => {
                if o.as_str() == "<"
                    || o.as_str() == "=="
                    || o.as_str() == "<="
                    || o.as_str() == ">"
                    || o.as_str() == ">="
                    || o.as_str() == "!="
                    || o.as_str() == "u<"
                    || o.as_str() == "u<="
                    || o.as_str() == "u>"
                    || o.as_str() == "u>="
                {
                    let pred = if o.as_str() == "u<" {
                        "ult"
                    } else if o.as_str() == "u<=" {
                        "ule"
                    } else if o.as_str() == "u>" {
                        "ugt"
                    } else if o.as_str() == "u>=" {
                        "uge"
                    } else if o.as_str() == "<" {
                        "slt"
                    } else if o.as_str() == "<=" {
                        "sle"
                    } else if o.as_str() == ">" {
                        "sgt"
                    } else if o.as_str() == ">=" {
                        "sge"
                    } else if o.as_str() == "!=" {
                        "ne"
                    } else {
                        "eq"
                    };
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str(".c = icmp ");
                    s.push_str(pred);
                    s.push_str(" i64 ");
                    s.push_str(&tmp(*x));
                    s.push_str(", ");
                    s.push_str(&tmp(*y));
                    s.push_str("\n");
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str(" = zext i1 ");
                    s.push_str(&tmp(*t));
                    s.push_str(".c to i64\n");
                } else if o.as_str() == "/"
                    || o.as_str() == "%"
                    || o.as_str() == "u/"
                    || o.as_str() == "u%"
                {
                    // U-DIV: the unsigned forms keep the zero-divisor check
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str("d = call i64 @__divchk(i64 ");
                    s.push_str(&tmp(*y));
                    s.push_str(")\n");
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str(" = ");
                    s.push_str(&llop(o));
                    s.push_str(" i64 ");
                    s.push_str(&tmp(*x));
                    s.push_str(", ");
                    s.push_str(&tmp(*t));
                    s.push_str("d\n");
                } else {
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str(" = ");
                    s.push_str(&llop(o));
                    s.push_str(" i64 ");
                    s.push_str(&tmp(*x));
                    s.push_str(", ");
                    s.push_str(&tmp(*y));
                    s.push_str("\n");
                }
            }
            Op::LoadLoc(t, sl) => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = load ");
                s.push_str(&lt);
                s.push_str(", ");
                s.push_str(&lt);
                s.push_str("* ");
                s.push_str(&loc(*sl));
                s.push_str("\n");
            }
            Op::StoreLoc(sl, t) => {
                s.push_str("  store ");
                s.push_str(&lt);
                s.push_str(" ");
                s.push_str(&tmp(*t));
                s.push_str(", ");
                s.push_str(&lt);
                s.push_str("* ");
                s.push_str(&loc(*sl));
                s.push_str("\n");
            }
            Op::IfOp(t, c0, tops, tr, eops, er) => {
                let lid = *t;
                let lid2 = lid0;
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(".b = icmp ne i64 ");
                s.push_str(&tmp(*c0));
                s.push_str(", 0\n");
                s.push_str("  br i1 ");
                s.push_str(&tmp(*t));
                s.push_str(".b, label %then");
                s.push_str(&n32(lid));
                s.push_str(", label %else");
                s.push_str(&n32(lid));
                s.push_str("\n");
                let tret = ops_return(tops);
                let eret = ops_return(eops);
                s.push_str("then");
                s.push_str(&n32(lid));
                s.push_str(":\n");
                s.push_str(&body_ops_in(tops, lid2, outs));
                s.push_str("  br label %thx");
                s.push_str(&n32(lid));
                s.push_str("\n");
                s.push_str("thx");
                s.push_str(&n32(lid));
                s.push_str(":\n");
                s.push_str("  br label %join");
                s.push_str(&n32(lid));
                s.push_str("\n");
                s.push_str("else");
                s.push_str(&n32(lid));
                s.push_str(":\n");
                s.push_str(&body_ops_in(eops, lid2, outs));
                s.push_str("  br label %elx");
                s.push_str(&n32(lid));
                s.push_str("\n");
                s.push_str("elx");
                s.push_str(&n32(lid));
                s.push_str(":\n");
                s.push_str("  br label %join");
                s.push_str(&n32(lid));
                s.push_str("\n");
                s.push_str("join");
                s.push_str(&n32(lid));
                s.push_str(":\n");
                // a branch whose body RETURNS reaches `thx`/`elx` only through the unreachable label the
                // early return left behind, and LLVM would still demand a value for it; with one live arm the
                // join takes that arm's value directly instead of a two-incoming phi.
                if tret && !eret {
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str(" = add i64 0, ");
                    s.push_str(&tmp(*er));
                    s.push_str("\n");
                } else if eret && !tret {
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str(" = add i64 0, ");
                    s.push_str(&tmp(*tr));
                    s.push_str("\n");
                } else {
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str(" = phi i64 [ ");
                    s.push_str(&tmp(*tr));
                    s.push_str(", %thx");
                    s.push_str(&n32(lid));
                    s.push_str(" ], [ ");
                    s.push_str(&tmp(*er));
                    s.push_str(", %elx");
                    s.push_str(&n32(lid));
                    s.push_str(" ]\n");
                } // C11: explicit join-PREDECESSOR blocks (thx/elx) -- with a NESTED if the control arriving at join comes from an inner join, so naming then/else in the phi is invalid IR
            }
            Op::LoopOp(cops, cr, bops) => {
                let lid = *cr;
                s.push_str("  br label %lcond");
                s.push_str(&n32(lid));
                s.push_str("\n");
                s.push_str("lcond");
                s.push_str(&n32(lid));
                s.push_str(":\n");
                s.push_str(&body_ops_in(cops, 0 - 1, outs));
                s.push_str("  %lc");
                s.push_str(&n32(lid));
                s.push_str(" = icmp ne i64 ");
                s.push_str(&tmp(*cr));
                s.push_str(", 0\n");
                s.push_str("  br i1 %lc");
                s.push_str(&n32(lid));
                s.push_str(", label %lbody");
                s.push_str(&n32(lid));
                s.push_str(", label %lend");
                s.push_str(&n32(lid));
                s.push_str("\n");
                s.push_str("lbody");
                s.push_str(&n32(lid));
                s.push_str(":\n");
                s.push_str(&{
                    let mut o2: Vec<i64> = Vec::new();
                    let mut q = 0;
                    while q < outs.len() {
                        o2.push(outs[q]);
                        q = q + 1;
                    }
                    o2.push(lid as i64);
                    body_ops_in(bops, lid as i64, &o2)
                });
                s.push_str("  br label %lcond");
                s.push_str(&n32(lid));
                s.push_str("\n");
                s.push_str("lend");
                s.push_str(&n32(lid));
                s.push_str(":\n");
            }
            Op::MakeStruct(t, nf, av) => {
                let sz = oabi::struct_size(*nf);
                // HEAP, not alloca: an aggregate built in a callee's frame is RETURNED as a handle, and a
                // stack slot dies with the frame (`Pt::origin()` then read a clobbered 2 instead of 0). Our
                // value model is "aggregates are heap handles", so the allocation has to outlive the frame.
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(".mb = call i8* @__xmalloc(i64 ");
                s.push_str(&n32(sz));
                s.push_str(")\n");
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(".m = bitcast i8* ");
                s.push_str(&tmp(*t));
                s.push_str(".mb to [");
                s.push_str(&n32(*nf));
                s.push_str(" x i64]*\n");
                let mut k = 0;
                while k < av.len() {
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str(".f");
                    s.push_str(&n32(k as u32));
                    s.push_str(" = getelementptr [");
                    s.push_str(&n32(*nf));
                    s.push_str(" x i64], [");
                    s.push_str(&n32(*nf));
                    s.push_str(" x i64]* ");
                    s.push_str(&tmp(*t));
                    s.push_str(".m, i64 0, i64 ");
                    s.push_str(&n32(k as u32));
                    s.push_str("\n");
                    s.push_str("  store i64 ");
                    s.push_str(&tmp(av[k]));
                    s.push_str(", i64* ");
                    s.push_str(&tmp(*t));
                    s.push_str(".f");
                    s.push_str(&n32(k as u32));
                    s.push_str("\n");
                    k = k + 1;
                }
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(".p = getelementptr [");
                s.push_str(&n32(*nf));
                s.push_str(" x i64], [");
                s.push_str(&n32(*nf));
                s.push_str(" x i64]* ");
                s.push_str(&tmp(*t));
                s.push_str(".m, i64 0, i64 0\n");
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = ptrtoint i64* ");
                s.push_str(&tmp(*t));
                s.push_str(".p to i64   ; struct size=");
                s.push_str(&n32(sz));
                s.push_str("\n");
            }
            Op::LoadField(t, bt, ix) => {
                let off = oabi::field_offset(*ix);
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(".bp = inttoptr i64 ");
                s.push_str(&tmp(*bt));
                s.push_str(" to i8*\n");
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(".fp8 = getelementptr i8, i8* ");
                s.push_str(&tmp(*t));
                s.push_str(".bp, i64 ");
                s.push_str(&n32(off));
                s.push_str("\n");
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(".fp = bitcast i8* ");
                s.push_str(&tmp(*t));
                s.push_str(".fp8 to i64*\n");
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = load i64, i64* ");
                s.push_str(&tmp(*t));
                s.push_str(".fp\n");
            }
            Op::StoreIdx(bt, it, vt) => {
                s.push_str("  %si");
                s.push_str(&n32(*vt));
                s.push_str(" = call i64 @__arr_set(i64 ");
                s.push_str(&tmp(*bt));
                s.push_str(", i64 ");
                s.push_str(&tmp(*it));
                s.push_str(", i64 ");
                s.push_str(&tmp(*vt));
                s.push_str(")\n");
            }
            Op::StoreField(bt, fx, vt) => {
                s.push_str("  %sf");
                s.push_str(&n32(*vt));
                s.push_str(".bp = inttoptr i64 ");
                s.push_str(&tmp(*bt));
                s.push_str(" to i64*\n");
                s.push_str("  %sf");
                s.push_str(&n32(*vt));
                s.push_str(".ep = getelementptr i64, i64* %sf");
                s.push_str(&n32(*vt));
                s.push_str(".bp, i64 ");
                s.push_str(&n32(*fx));
                s.push_str("\n");
                s.push_str("  store i64 ");
                s.push_str(&tmp(*vt));
                s.push_str(", i64* %sf");
                s.push_str(&n32(*vt));
                s.push_str(".ep\n");
            }
            Op::MakeArr(t, n, av) => {
                let np = (*n) + 1;
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(".ab = call i8* @__xmalloc(i64 ");
                s.push_str(&n32(oabi::struct_size(np)));
                s.push_str(")\n");
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(".a = bitcast i8* ");
                s.push_str(&tmp(*t));
                s.push_str(".ab to [");
                s.push_str(&n32(np));
                s.push_str(" x i64]*\n");
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(".lp = getelementptr [");
                s.push_str(&n32(np));
                s.push_str(" x i64], [");
                s.push_str(&n32(np));
                s.push_str(" x i64]* ");
                s.push_str(&tmp(*t));
                s.push_str(".a, i64 0, i64 0\n");
                s.push_str("  store i64 ");
                s.push_str(&n32(*n));
                s.push_str(", i64* ");
                s.push_str(&tmp(*t));
                s.push_str(".lp\n");
                let mut k = 0;
                while k < av.len() {
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str(".e");
                    s.push_str(&n32(k as u32));
                    s.push_str(" = getelementptr [");
                    s.push_str(&n32(np));
                    s.push_str(" x i64], [");
                    s.push_str(&n32(np));
                    s.push_str(" x i64]* ");
                    s.push_str(&tmp(*t));
                    s.push_str(".a, i64 0, i64 ");
                    s.push_str(&n32((k as u32) + 1));
                    s.push_str("\n");
                    s.push_str("  store i64 ");
                    s.push_str(&tmp(av[k]));
                    s.push_str(", i64* ");
                    s.push_str(&tmp(*t));
                    s.push_str(".e");
                    s.push_str(&n32(k as u32));
                    s.push_str("\n");
                    k = k + 1;
                }
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = ptrtoint [");
                s.push_str(&n32(np));
                s.push_str(" x i64]* ");
                s.push_str(&tmp(*t));
                s.push_str(".a to i64\n");
            }
            Op::LoadIdx(t, bt, it) => {
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__arr_get(i64 ");
                s.push_str(&tmp(*bt));
                s.push_str(", i64 ");
                s.push_str(&tmp(*it));
                s.push_str(")\n");
            }
            Op::MakeEnum(t, tag, av) => {
                let n = (av.len() as u32) + 1;
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(".mb = call i8* @__xmalloc(i64 ");
                s.push_str(&n32(oabi::struct_size(n)));
                s.push_str(")\n");
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(".m = bitcast i8* ");
                s.push_str(&tmp(*t));
                s.push_str(".mb to [");
                s.push_str(&n32(n));
                s.push_str(" x i64]*\n");
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(".t = getelementptr [");
                s.push_str(&n32(n));
                s.push_str(" x i64], [");
                s.push_str(&n32(n));
                s.push_str(" x i64]* ");
                s.push_str(&tmp(*t));
                s.push_str(".m, i64 0, i64 0\n");
                s.push_str("  store i64 ");
                s.push_str(&n32(*tag));
                s.push_str(", i64* ");
                s.push_str(&tmp(*t));
                s.push_str(".t\n");
                let mut k = 0;
                while k < av.len() {
                    s.push_str("  ");
                    s.push_str(&tmp(*t));
                    s.push_str(".p");
                    s.push_str(&n32(k as u32));
                    s.push_str(" = getelementptr [");
                    s.push_str(&n32(n));
                    s.push_str(" x i64], [");
                    s.push_str(&n32(n));
                    s.push_str(" x i64]* ");
                    s.push_str(&tmp(*t));
                    s.push_str(".m, i64 0, i64 ");
                    s.push_str(&n32((k as u32) + 1));
                    s.push_str("\n");
                    s.push_str("  store i64 ");
                    s.push_str(&tmp(av[k]));
                    s.push_str(", i64* ");
                    s.push_str(&tmp(*t));
                    s.push_str(".p");
                    s.push_str(&n32(k as u32));
                    s.push_str("\n");
                    k = k + 1;
                }
                s.push_str("  ");
                s.push_str(&tmp(*t));
                s.push_str(" = ptrtoint [");
                s.push_str(&n32(n));
                s.push_str(" x i64]* ");
                s.push_str(&tmp(*t));
                s.push_str(".m to i64\n");
            }
            Op::MatchOp(dest, st, arms) => {
                let mid = *dest;
                s.push_str("  ");
                s.push_str(&tmp(mid));
                s.push_str(".bp = inttoptr i64 ");
                s.push_str(&tmp(*st));
                s.push_str(" to i64*\n");
                s.push_str("  ");
                s.push_str(&tmp(mid));
                s.push_str(".tg = load i64, i64* ");
                s.push_str(&tmp(mid));
                s.push_str(".bp\n");
                s.push_str("  br label %m");
                s.push_str(&n32(mid));
                s.push_str("a0\n"); // enter the first arm TEST block
                let mut k = 0;
                while k < arms.len() {
                    let a = &arms[k];
                    let armlbl = format!("m{}a{}", n32(mid), n32(k as u32));
                    let nextlbl = if k + 1 < arms.len() {
                        format!("m{}a{}", n32(mid), n32((k as u32) + 1))
                    } else {
                        format!("m{}dflt", n32(mid))
                    };
                    s.push_str(&armlbl);
                    s.push_str(":\n"); // C11: every arm TEST needs its own basic block -- emitting the next test after a terminator with no label produced an undefined branch target
                    if a.tag >= 0 {
                        s.push_str("  ");
                        s.push_str(&tmp(mid));
                        s.push_str(".c");
                        s.push_str(&n32(k as u32));
                        s.push_str(" = icmp eq i64 ");
                        s.push_str(&tmp(mid));
                        s.push_str(".tg, ");
                        s.push_str(&n32(a.tag as u32));
                        s.push_str("\n");
                        s.push_str("  br i1 ");
                        s.push_str(&tmp(mid));
                        s.push_str(".c");
                        s.push_str(&n32(k as u32));
                        s.push_str(", label %");
                        s.push_str(&armlbl);
                        s.push_str("b, label %");
                        s.push_str(&nextlbl);
                        s.push_str("\n");
                    } else {
                        s.push_str("  br label %");
                        s.push_str(&armlbl);
                        s.push_str("b\n");
                    }
                    s.push_str(&armlbl);
                    s.push_str("b:\n");
                    // bind payload slots from the cell
                    let mut q = 0;
                    while q < a.slots.len() {
                        s.push_str("  ");
                        s.push_str(&tmp(mid));
                        s.push_str(".g");
                        s.push_str(&n32(k as u32));
                        s.push_str("_");
                        s.push_str(&n32(q as u32));
                        s.push_str(" = getelementptr i64, i64* ");
                        s.push_str(&tmp(mid));
                        s.push_str(".bp, i64 ");
                        s.push_str(&n32((q as u32) + 1));
                        s.push_str("\n");
                        s.push_str("  ");
                        s.push_str(&tmp(mid));
                        s.push_str(".v");
                        s.push_str(&n32(k as u32));
                        s.push_str("_");
                        s.push_str(&n32(q as u32));
                        s.push_str(" = load i64, i64* ");
                        s.push_str(&tmp(mid));
                        s.push_str(".g");
                        s.push_str(&n32(k as u32));
                        s.push_str("_");
                        s.push_str(&n32(q as u32));
                        s.push_str("\n");
                        s.push_str("  store i64 ");
                        s.push_str(&tmp(mid));
                        s.push_str(".v");
                        s.push_str(&n32(k as u32));
                        s.push_str("_");
                        s.push_str(&n32(q as u32));
                        s.push_str(", i64* ");
                        s.push_str(&loc(a.slots[q]));
                        s.push_str("\n");
                        q = q + 1;
                    }
                    s.push_str(&body_ops_in(&a.ops, lid0, outs));
                    s.push_str("  store i64 ");
                    s.push_str(&tmp(a.res));
                    s.push_str(", i64* ");
                    s.push_str(&tmp(mid));
                    s.push_str(".slot\n");
                    s.push_str("  br label %m");
                    s.push_str(&n32(mid));
                    s.push_str("end\n");
                    k = k + 1;
                }
                s.push_str("m");
                s.push_str(&n32(mid));
                s.push_str("dflt:\n");
                s.push_str("  store i64 0, i64* ");
                s.push_str(&tmp(mid));
                s.push_str(".slot\n");
                s.push_str("  br label %m");
                s.push_str(&n32(mid));
                s.push_str("end\n");
                s.push_str("m");
                s.push_str(&n32(mid));
                s.push_str("end:\n");
                s.push_str("  ");
                s.push_str(&tmp(mid));
                s.push_str(" = load i64, i64* ");
                s.push_str(&tmp(mid));
                s.push_str(".slot\n");
            }
            Op::Ret(src) => {
                s.push_str("  ret i64 ");
                s.push_str(&tmp(*src));
                s.push_str("\n");
            }
        }
        k = k + 1;
    }
    s
}
