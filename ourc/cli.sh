#!/bin/bash
# Build `ourc` (by semc, from our crates) via build-ourc.sh, then compile+run real programs with it.
set -u
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SEM0="${SEM0:?}"; R2L="${R2L:?}"; LLC="${LLC:?}"
W="$(mktemp -d)"; trap 'rm -rf "$W"' EXIT
bash "$(dirname "$0")/build-ourc.sh"
OUT="${OURC_OUT:-/tmp/ourc}"
# use it: compile a real file
echo 'let n = 6; let m = 7; n * m' > /tmp/t1.src
"$OUT" /tmp/t1.src -O --emit-ir > /tmp/t1.out 2>&1; sed -n '1p' /tmp/t1.out
sed -n '/^\(@\|declare\|define\)/,$p' /tmp/t1.out > /tmp/t1.ll
"$LLC" -O2 -filetype=obj /tmp/t1.ll -o /tmp/t1.o 2>/dev/null && cc -no-pie /tmp/t1.o -o /tmp/t1.bin -lm 2>/dev/null
/tmp/t1.bin; echo "  RAN /tmp/t1.bin -> exit=$? (expect 42)"
# and a type error through the CLI
echo 'if 3 { 1 } else { 2 }' > /tmp/t2.src
"$OUT" /tmp/t2.src | head -2
# FUNCTIONS + RECURSION end-to-end
run_src(){ f=$1; want=$2; "$OUT" "$f" --emit-ir > "$f.out" 2>&1; sed -n '/^\(@\|declare\|define\)/,$p' "$f.out" > "$f.ll"
  "$LLC" -O2 -filetype=obj "$f.ll" -o "$f.o" 2>/dev/null && cc -no-pie "$f.o" -o "$f.bin" -lm 2>/dev/null
  if [ -x "$f.bin" ]; then "$f.bin"; rc=$?; [ "$rc" = "$want" ] && echo "  ok   $(basename $f): exit=$rc == $want" || echo "  FAIL $(basename $f): exit=$rc want=$want"; else echo "  FAIL $(basename $f): IR did not build"; sed -n '1,6p' "$f.out"; fi }
echo 'fn dbl(x) { x * 2 } dbl(21)' > /tmp/t3.src;  run_src /tmp/t3.src 42
echo 'fn add3(a, b, c) { a + b + c } add3(10, 20, 12)' > /tmp/t4.src; run_src /tmp/t4.src 42
echo 'fn fact(n) { if n < 2 { 1 } else { n * fact(n - 1) } } fact(5)' > /tmp/t5.src; run_src /tmp/t5.src 120
echo 'fn fib(n) { if n < 2 { n } else { fib(n - 1) + fib(n - 2) } } fib(10)' > /tmp/t6.src; run_src /tmp/t6.src 55
# LOOPS + MUTATION end-to-end
echo 'let mut i = 0; let mut s = 0; while i < 5 { s = s + i; i = i + 1; }; s' > /tmp/t7.src; run_src /tmp/t7.src 10
echo 'let mut n = 1; while n < 100 { n = n * 3; }; n' > /tmp/t8.src; run_src /tmp/t8.src 243
echo 'fn sumto(k) { let mut a = 0; let mut j = 0; while j < k { a = a + j; j = j + 1; }; a } sumto(10)' > /tmp/t9.src; run_src /tmp/t9.src 45
echo 'fn gcd(a, b) { let mut x = a; let mut y = b; while 0 < y { let t = y; y = x - x; x = t; }; x } gcd(12, 0)' > /tmp/ta.src; run_src /tmp/ta.src 12
# STRUCTS end-to-end
echo 'struct P { x, y } let p = P { 3, 4 }; p.x + p.y' > /tmp/tb.src; run_src /tmp/tb.src 7
echo 'struct P { x, y } let p = P { 10, 32 }; p.y' > /tmp/tc.src; run_src /tmp/tc.src 32
echo 'struct V { a, b, c } let v = V { 2, 3, 7 }; v.a * v.b + v.c' > /tmp/td.src; run_src /tmp/td.src 13
echo 'struct P { x, y } fn sx(q) { q } let p = P { 20, 22 }; p.x + p.y' > /tmp/te.src; run_src /tmp/te.src 42
echo 'struct P { x, y } let p = P { 1, 2 }; let q = P { 3, 4 }; p.x + q.y' > /tmp/tf.src; run_src /tmp/tf.src 5
echo 'struct P { x, y } fn addp(a, b) { a + b } let p = P { 15, 27 }; addp(p.x, p.y)' > /tmp/tg.src; run_src /tmp/tg.src 42
echo 'struct C { n } let c = C { 6 }; let mut i = 0; let mut t = 0; while i < c.n { t = t + i; i = i + 1; }; t' > /tmp/th.src; run_src /tmp/th.src 15
# diagnostics through the CLI
echo 'struct P { x, y } let p = P { 1, 2 }; p.z' > /tmp/ti.src; "$OUT" /tmp/ti.src 2>&1 | head -1
# TYPED fields / params / returns end-to-end
echo 'struct P { x: int, y: int } let p = P { 20, 22 }; p.x + p.y' > /tmp/tj.src; run_src /tmp/tj.src 42
echo 'struct F { ok: bool, n: int } let f = F { true, 7 }; if f.ok { f.n } else { 0 }' > /tmp/tk.src; run_src /tmp/tk.src 7
echo 'fn mk(a: int, b: int) -> int { a + b } mk(19, 23)' > /tmp/tl.src; run_src /tmp/tl.src 42
echo 'struct F { ok: bool, n: int } fn pick(c: bool, a: int) -> int { if c { a } else { 0 } } let f = F { true, 42 }; pick(f.ok, f.n)' > /tmp/tm.src; run_src /tmp/tm.src 42
echo 'struct F { ok: bool, n: int } let f = F { 1, 7 }; f.n' > /tmp/tn.src; "$OUT" /tmp/tn.src 2>&1 | head -1
echo 'fn bad(a: int) -> bool { a + 1 } bad(1)' > /tmp/to.src; "$OUT" /tmp/to.src 2>&1 | head -1
# STRUCT-typed params/returns + NESTED structs end-to-end
echo 'struct P { x: int, y: int } fn sum(p: P) -> int { p.x + p.y } let p = P { 19, 23 }; sum(p)' > /tmp/tq.src; run_src /tmp/tq.src 42
echo 'struct P { x: int, y: int } fn mk() -> P { P { 5, 6 } } let q = mk(); q.x + q.y' > /tmp/tr.src; run_src /tmp/tr.src 11
echo 'struct In { a: int } struct Out { i: In, b: int } let o = Out { In { 7 }, 35 }; o.i.a + o.b' > /tmp/ts.src; run_src /tmp/ts.src 42
echo 'fn need(c: bool) -> int { if c { 1 } else { 0 } } need(5)' > /tmp/tt.src; "$OUT" /tmp/tt.src 2>&1 | head -1
# OPERATORS / else-if / else-less if end-to-end
echo 'let a = 3; let b = 9; if a <= 3 && b >= 9 { 42 } else { 0 }' > /tmp/u1.src; run_src /tmp/u1.src 42
echo 'let a = 1; if a > 5 || a != 1 { 1 } else { 7 }' > /tmp/u2.src; run_src /tmp/u2.src 7
echo 'let t = true; if !t { 1 } else { 13 }' > /tmp/u3.src; run_src /tmp/u3.src 13
echo 'let n = 5; if n < 3 { 1 } else if n < 7 { 42 } else { 9 }' > /tmp/u4.src; run_src /tmp/u4.src 42
echo 'let mut i = 0; let mut c = 0; while i <= 4 { if i != 2 { c = c + i; }; i = i + 1; }; c' > /tmp/u5.src; run_src /tmp/u5.src 8
echo 'fn isodd(n: int) -> bool { n != n / 2 * 2 } fn cnt(k: int) -> int { let mut i = 0; let mut c = 0; while i < k { if isodd(i) { c = c + 1; }; i = i + 1; }; c } cnt(10)' > /tmp/u6.src; run_src /tmp/u6.src 5
# ENUMS + MATCH end-to-end
echo 'enum Opt { None, Some(v) } let o = Opt::Some(42); match o { Some(v) => v, None => 0 }' > /tmp/v1.src; run_src /tmp/v1.src 42
echo 'enum Opt { None, Some(v) } let o = Opt::None; match o { Some(v) => v, None => 7 }' > /tmp/v2.src; run_src /tmp/v2.src 7
echo 'enum Sh { Sq(s), Rect(w, h) } let r = Sh::Rect(6, 7); match r { Sq(s) => s * s, Rect(w, h) => w * h }' > /tmp/v3.src; run_src /tmp/v3.src 42
echo 'enum Sh { Sq(s), Rect(w, h) } let q = Sh::Sq(7); match q { Sq(s) => s * s, Rect(w, h) => w * h }' > /tmp/v4.src; run_src /tmp/v4.src 49
echo 'enum Opt { None, Some(v) } fn unwrap_or(o: Opt, d: int) -> int { match o { Some(v) => v, None => d } } unwrap_or(Opt::Some(40), 2) + unwrap_or(Opt::None, 2)' > /tmp/v5.src; run_src /tmp/v5.src 42
echo 'enum Opt { None, Some(v) } let o = Opt::Some(1); match o { Some(v) => v }' > /tmp/v6.src; "$OUT" /tmp/v6.src 2>&1 | head -1
# ARRAYS end-to-end
echo 'let a = [10, 20, 12]; a[0] + a[1] + a[2]' > /tmp/w1.src; run_src /tmp/w1.src 42
echo 'let a = [1, 2, 3, 4, 5]; let mut i = 0; let mut s = 0; while i < 5 { s = s + a[i]; i = i + 1; }; s' > /tmp/w2.src; run_src /tmp/w2.src 15
echo 'fn asum(a: arr, n: int) -> int { let mut i = 0; let mut s = 0; while i < n { s = s + a[i]; i = i + 1; }; s } let xs = [6, 7, 8, 9, 12]; asum(xs, 5)' > /tmp/w3.src; run_src /tmp/w3.src 42
echo 'let a = [1, 2]; a[true]' > /tmp/w4.src; "$OUT" /tmp/w4.src 2>&1 | head -1
# PLACE ASSIGNMENT end-to-end
echo 'let a = [1, 2, 3]; a[1] = 40; a[0] + a[1] + a[2]' > /tmp/x1.src; run_src /tmp/x1.src 44
echo 'struct P { x: int, y: int } let p = P { 1, 2 }; p.x = 40; p.x + p.y' > /tmp/x2.src; run_src /tmp/x2.src 42
echo 'let a = [0, 0, 0, 0, 0]; let mut i = 0; while i < 5 { a[i] = i * i; i = i + 1; }; a[4] + a[3]' > /tmp/x3.src; run_src /tmp/x3.src 25
echo 'fn fill(a: arr, n: int) -> int { let mut i = 0; while i < n { a[i] = i + 1; i = i + 1; }; a[n - 1] } let xs = [0, 0, 0, 0, 0, 0]; fill(xs, 6) * 7' > /tmp/x4.src; run_src /tmp/x4.src 42
# GENERICS end-to-end
echo 'fn id<T>(x: T) -> T { x } id(42)' > /tmp/y1.src; run_src /tmp/y1.src 42
echo 'fn pick<T>(c: bool, a: T, b: T) -> T { if c { a } else { b } } pick(true, 42, 7)' > /tmp/y2.src; run_src /tmp/y2.src 42
echo 'fn first<T>(a: arr) -> int { a[0] } let xs = [42, 1]; first(xs)' > /tmp/y3.src; run_src /tmp/y3.src 42
echo 'fn idg<T>(x: T) -> T { x } enum Opt { None, Some(v) } let o = idg(Opt::Some(42)); match o { Some(v) => v, None => 0 }' > /tmp/y4.src; run_src /tmp/y4.src 42
echo 'struct P { x: int, y: int } fn idg<T>(x: T) -> T { x } let p = idg(P { 20, 22 }); p.x + p.y' > /tmp/y5.src; run_src /tmp/y5.src 42

# rung 8l: impl blocks + method calls
echo 'struct P { x: int, y: int } impl P { fn sum(self) -> int { self.x + self.y } } let p = P { 19, 23 }; p.sum()' > /tmp/z1.src; run_src /tmp/z1.src 42
echo 'struct P { x: int, y: int } impl P { fn scaled(self, k: int) -> int { (self.x + self.y) * k } } let p = P { 3, 4 }; p.scaled(6)' > /tmp/z2.src; run_src /tmp/z2.src 42
echo 'enum Opt { None, Some(v) } impl Opt { fn unwrap_or(self, d: int) -> int { match self { Some(v) => v, None => d } } } let o = Opt::None; o.unwrap_or(42)' > /tmp/z3.src; run_src /tmp/z3.src 42
echo 'struct C { n: int } impl C { fn bump(self) -> C { C { self.n + 1 } } fn get(self) -> int { self.n } } let c = C { 41 }; c.bump().get()' > /tmp/z4.src; run_src /tmp/z4.src 42

# rung 8m: traits, impl-for, default methods (static dispatch, per-type materialization)
echo 'trait Area { fn area(self) -> int; } struct Sq { s: int } impl Area for Sq { fn area(self) -> int { self.s * self.s } } let q = Sq { 7 }; q.area() - 7' > /tmp/t1.src; run_src /tmp/t1.src 42
echo 'trait Shape { fn base(self) -> int; fn twice(self) -> int { self.base() * 2 } } struct R { n: int } impl Shape for R { fn base(self) -> int { self.n } } let r = R { 21 }; r.twice()' > /tmp/t2.src; run_src /tmp/t2.src 42
echo 'trait T { fn v(self) -> int { 10 } } struct A { n: int } struct B { n: int } impl T for A { fn v(self) -> int { self.n } } impl T for B { } let a = A { 32 }; let b = B { 0 }; a.v() + b.v()' > /tmp/t3b.src; run_src /tmp/t3b.src 42
echo 'trait Pair { fn a(self) -> int; fn b(self) -> int; fn sum(self) -> int { self.a() + self.b() } } struct P { x: int, y: int } impl Pair for P { fn a(self) -> int { self.x } fn b(self) -> int { self.y } } let p = P { 19, 23 }; p.sum()' > /tmp/t4.src; run_src /tmp/t4.src 42

# rung 8n: bound generics, monomorphized per concrete type
echo 'trait Area { fn area(self) -> int; } struct Sq { s: int } impl Area for Sq { fn area(self) -> int { self.s * self.s } } fn show<T: Area>(x: T) -> int { x.area() } let q = Sq { 7 }; show(q) - 7' > /tmp/m1.src; run_src /tmp/m1.src 42
echo 'trait Area { fn area(self) -> int; } struct Sq { s: int } struct Rc { w: int, h: int } impl Area for Sq { fn area(self) -> int { self.s * self.s } } impl Area for Rc { fn area(self) -> int { self.w * self.h } } fn tot<T: Area>(x: T) -> int { x.area() } let a = Sq { 5 }; let b = Rc { 3, 6 }; tot(a) - tot(b) + 35' > /tmp/m2.src; run_src /tmp/m2.src 42
echo 'trait N { fn n(self) -> int; fn dbl(self) -> int { self.n() * 2 } } struct W { v: int } impl N for W { fn n(self) -> int { self.v } } fn via<T: N>(x: T) -> int { x.dbl() } let w = W { 21 }; via(w)' > /tmp/m3.src; run_src /tmp/m3.src 42

# rung 8o: strings -- literals, globals, libc puts/strlen
echo 'len("hello") + 37' > /tmp/s1.src; run_src /tmp/s1.src 42
echo 'print("ourc says hello"); 42' > /tmp/s2.src; run_src /tmp/s2.src 42
echo 'fn n(s: str) -> int { len(s) } n("abcdefghij") + 32' > /tmp/s3.src; run_src /tmp/s3.src 42
echo 'if len("abc") == 3 { print("three"); 42 } else { 0 }' > /tmp/s4.src; run_src /tmp/s4.src 42

# rung 8p: print_int (printf) + str content equality (strcmp)
echo 'print_int(42); 42' > /tmp/p1.src; run_src /tmp/p1.src 42
echo 'let a = "abc"; if a == "abc" { 42 } else { 0 }' > /tmp/p2.src; run_src /tmp/p2.src 42
echo 'let a = "abc"; if a != "zzz" { 42 } else { 0 }' > /tmp/p3.src; run_src /tmp/p3.src 42
echo 'fn same(a: str, b: str) -> int { if a == b { 1 } else { 0 } } same("xy", "xy") * 21 + same("xy", "zz") + 21' > /tmp/p4.src; run_src /tmp/p4.src 42
echo 'let mut i = 0; while i < 3 { print_int(i); i = i + 1; }; 42' > /tmp/p5.src; run_src /tmp/p5.src 42

# rung 8q: heap-allocated growable vectors (malloc/realloc runtime)
echo 'let v = vec_new(); vec_push(v, 40); vec_push(v, 2); vec_get(v, 0) + vec_get(v, 1)' > /tmp/ve1.src; run_src /tmp/ve1.src 42
echo 'let v = vec_new(); let mut i = 0; while i < 20 { vec_push(v, i); i = i + 1; }; vec_len(v) + vec_get(v, 19) + 3' > /tmp/ve2.src; run_src /tmp/ve2.src 42
echo 'let v = vec_new(); vec_push(v, 1); vec_set(v, 0, 42); vec_get(v, 0)' > /tmp/ve3.src; run_src /tmp/ve3.src 42
echo 'fn total(v) { let mut s = 0; let mut i = 0; while i < vec_len(v) { s = s + vec_get(v, i); i = i + 1; }; s } let v = vec_new(); let mut k = 0; while k < 9 { vec_push(v, k); k = k + 1; }; total(v) + 6' > /tmp/ve4.src; run_src /tmp/ve4.src 42

# rung 8r: string primitives for a tokenizer -- index, concat, substring (heap)
echo 'str_at("A", 0) - 23' > /tmp/q1.src; run_src /tmp/q1.src 42
echo 'len(str_cat("abc", "de")) + 37' > /tmp/q2.src; run_src /tmp/q2.src 42
echo 'if str_sub("hello world", 6, 5) == "world" { 42 } else { 0 }' > /tmp/q3.src; run_src /tmp/q3.src 42
echo 'fn count(s: str, c) { let mut n = 0; let mut i = 0; while i < len(s) { if str_at(s, i) == c { n = n + 1; }; i = i + 1; }; n } count("abracadabra", 97) * 8 + 2' > /tmp/q4.src; run_src /tmp/q4.src 42
echo 'let mut acc = ""; let mut i = 0; while i < 3 { acc = str_cat(acc, "ab"); i = i + 1; }; len(acc) + 36' > /tmp/q5.src; run_src /tmp/q5.src 42

# rung 8s capstone: a TOKENIZER written in ourc's own language (strings + vectors + nested loops)
run_src "$(dirname "$0")/progs/tokenizer.src" 42

# rung 8t: `return` -- early exit with real CFG handling
echo 'fn f(n) { if n < 0 { return 0; } n * 2 } f(21)' > /tmp/r1.src; run_src /tmp/r1.src 42
echo 'fn g(n) { let mut i = 0; while i < 100 { if i == n { return i * 2; } i = i + 1; } 0 } g(21)' > /tmp/r2.src; run_src /tmp/r2.src 42
echo 'fn h(n) { return n + 1; } h(41)' > /tmp/r3.src; run_src /tmp/r3.src 42
echo 'fn first_sp(s: str) { let mut i = 0; while i < len(s) { if str_at(s, i) == 32 { return i; } i = i + 1; } 0 - 1 } first_sp("abcdefghijkl mno") + 30' > /tmp/r4.src; run_src /tmp/r4.src 42

# rung 8u: tuples -- structural types, struct-shaped runtime
echo 'let p = (40, 2); p.0 + p.1' > /tmp/tu1.src; run_src /tmp/tu1.src 42
echo 'let t = (1, 2, 39); t.0 + t.1 + t.2' > /tmp/tu2.src; run_src /tmp/tu2.src 42
echo 'fn mk(a, b) -> (int, int) { (a, b) } let q = mk(20, 22); q.0 + q.1' > /tmp/tu3.src; run_src /tmp/tu3.src 42
echo 'fn divmod(a, b) -> (int, int) { (a / b, a - (a / b) * b) } let d = divmod(85, 2); d.0 - d.1' > /tmp/tu4.src; run_src /tmp/tu4.src 41

# rung 8v: dyn trait objects -- fat {tag,data} pair + tag-dispatch chain
run_src "$(dirname "$0")/progs/dyn_area.src" 42
run_src "$(dirname "$0")/progs/dyn_default.src" 42
run_src "$(dirname "$0")/progs/dyn_vec.src" 42
run_src "$(dirname "$0")/progs/vec_of_structs.src" 42
run_src "$(dirname "$0")/progs/expr_eval.src" 42

# rung 8y: associated functions (Type::new) + `Self` returns -- and heap aggregates
run_src "$(dirname "$0")/progs/assoc_fns.src" 42
echo 'struct P { x: int, y: int } fn mk(a) { P { a, 2 } } fn f() { let p = mk(40); let q = mk(1); p.x + p.y } f()' > /tmp/hp1.src; run_src /tmp/hp1.src 42

# rung 8z: closures -- parse-time lifting, captures as leading params, Fn params (higher order)
run_src "$(dirname "$0")/progs/closures.src" 42
run_src "$(dirname "$0")/progs/higher_order.src" 42

# rung 9a: `for` over ranges and vectors; lexical scope popping
run_src "$(dirname "$0")/progs/for_loops.src" 42
run_src "$(dirname "$0")/progs/for_nested.src" 42
echo 'let mut x = 1; if 1 == 1 { let x = 7; 0 } else { 0 }; x + 41' > /tmp/shw.src; run_src /tmp/shw.src 42

# capstone 3: a mini INTERPRETER with variables, written in ourc's language
run_src "$(dirname "$0")/progs/vec_of_strings.src" 42
run_src "$(dirname "$0")/progs/mini_interp.src" 42

# rung 9b: the `?` operator -- success variant yields its payload, anything else returns
run_src "$(dirname "$0")/progs/try_operator.src" 42

# rung 9c: operator overloading (inherent impl and via a trait)
run_src "$(dirname "$0")/progs/operators_overload.src" 42
run_src "$(dirname "$0")/progs/operators_trait.src" 42

# rung 9d: modules -- items namespaced, referenced by qualified path
run_src "$(dirname "$0")/progs/modules.src" 42
run_src "$(dirname "$0")/progs/modules_isolation.src" 42

# rung 9e/9f: method sugar on built-in types; PRELUDE written in ourc's own language (map/filter/fold)
run_src "$(dirname "$0")/progs/method_sugar.src" 42
run_src "$(dirname "$0")/progs/prelude_hof.src" 42
run_src "$(dirname "$0")/progs/prelude_generic.src" 42

# rung 9h: generic structs, monomorphized from the type names that appear
run_src "$(dirname "$0")/progs/generic_structs.src" 42

# rung 9i: FILE-BASED modules -- `mod X;` loads X.src, spans map back to the user's file
run_src "$(dirname "$0")/progs/multifile/main.src" 42

# rung 9j: match on integers (literal patterns, `_`) and GUARDS, including guarded enum patterns
run_src "$(dirname "$0")/progs/match_int_guards.src" 42
run_src "$(dirname "$0")/progs/match_enum_guard.src" 42

# capstone 4: a LEXER for ourc's own syntax, written in ourc's language
run_src "$(dirname "$0")/progs/self_lexer.src" 42

# rung 9k: TYPED enum payloads -> recursive ADTs (an AST) + blocks as expressions
run_src "$(dirname "$0")/progs/recursive_ast.src" 42

# capstone 5: a full FRONT-END in ourc's language -- parser builds a recursive AST, two tree-walkers consume it
run_src "$(dirname "$0")/progs/ast_frontend.src" 42

# rung 9l: string building in the PRELUDE (str_from_int, str_find, str_has, say)
run_src "$(dirname "$0")/progs/string_format.src" 42

# rung 9m: compound assignment (+= -= *= /=) and `if let`; match is block-form
run_src "$(dirname "$0")/progs/compound_assign.src" 42
run_src "$(dirname "$0")/progs/if_let.src" 42

# rung 9n: break and continue (loop-label-aware codegen)
run_src "$(dirname "$0")/progs/break_continue.src" 42

# rung 9o: references (&T, &mut T -- transparent in our value model) and const/static items
run_src "$(dirname "$0")/progs/refs_consts.src" 42

# rung 9q: multi-parameter generics -- every type parameter resolved independently
run_src "$(dirname "$0")/progs/multi_generics.src" 42

# rung 9r: `as` casts with real truncation + integer width type names (results verified against rustc)
run_src "$(dirname "$0")/progs/casts_widths.src" 45

# rung 9s: bitwise operators and shifts (results verified against rustc)
run_src "$(dirname "$0")/progs/bitwise.src" 42
run_src "$(dirname "$0")/progs/nested_vec.src" 42
run_src "$(dirname "$0")/progs/comments.src" 42

# rung 9w: REAL RUST source files compiled by ourc (fn main, println!, char literals)
run_src "$(dirname "$0")/progs/real_rust_hello.rs" 0
run_src "$(dirname "$0")/progs/real_rust_chars.rs" 0
run_src "$(dirname "$0")/progs/real_rust_structs.rs" 0
run_src "$(dirname "$0")/progs/real_rust_fieldorder.rs" 0
run_src "$(dirname "$0")/progs/real_rust_option.rs" 0
run_src "$(dirname "$0")/progs/real_rust_closure_pat.rs" 0
run_src "$(dirname "$0")/progs/real_rust_vecmacro.rs" 0
run_src "$(dirname "$0")/progs/real_rust_veceq.rs" 0
run_src "$(dirname "$0")/progs/real_rust_vecindex.rs" 0
run_src "$(dirname "$0")/progs/real_rust_arrays.rs" 0
run_src "$(dirname "$0")/progs/real_rust_static_array.rs" 0
run_src "$(dirname "$0")/progs/real_rust_format.rs" 0
run_src "$(dirname "$0")/progs/real_rust_strmatch.rs" 0
run_src "$(dirname "$0")/progs/real_rust_macros.rs" 0
run_src "$(dirname "$0")/progs/real_rust_macro_rep.rs" 0
run_src "$(dirname "$0")/progs/real_rust_macro_hygiene.rs" 0
run_src "$(dirname "$0")/progs/real_rust_loop_bounds.rs" 0
run_src "$(dirname "$0")/progs/real_rust_for_array.rs" 0
run_src "$(dirname "$0")/progs/real_rust_mutiter.rs" 0
run_src "$(dirname "$0")/progs/real_rust_generics.rs" 0
# rung: SLICE parameters vs array literals (P-TYPE), and a pure-turbofish `None::<T>` (P-PATH)
run_src "$(dirname "$0")/progs/real_rust_slice.rs" 0
# rung: P-PAT stage 1 -- destructuring `let` (tuple, struct, `_` components)
run_src "$(dirname "$0")/progs/real_rust_destructure.rs" 0
# rung: P-PAT stage 2 -- structural match patterns (tuple/struct/slice + literals + rest), ranges, ref, move
run_src "$(dirname "$0")/progs/real_rust_patterns.rs" 0
# rung: P-PAT stage 3 -- tuple-STRUCT patterns in `let` and match arms; nested ctor call arg typing
run_src "$(dirname "$0")/progs/real_rust_tuplestruct.rs" 0
# rung: P-PAT stage 4 -- while let, const/char range endpoints, literal or-patterns
run_src "$(dirname "$0")/progs/real_rust_whilelet.rs" 0
# rung: P-ITEM -- items declared INSIDE a function body (fn, struct, enum, const) are hoisted
run_src "$(dirname "$0")/progs/real_rust_nested_items.rs" 0
# rung: P-LEX -- radix literals (0x/0o/0b, separators, suffixes) and unary minus
run_src "$(dirname "$0")/progs/real_rust_literals.rs" 0
# rung: P-STD -- Vec::new/with_capacity, String::new, integer MAX/MIN
run_src "$(dirname "$0")/progs/real_rust_std.rs" 0
# rung: P-GEN -- `where` clauses on struct items, and where-clause BOUNDS feeding monomorphization
run_src "$(dirname "$0")/progs/real_rust_where.rs" 0
# rung: the driver VERIFIES its own output -- an accepted program must not call a name it never defined
P="$(dirname "$0")/progs/refused_undefined_call.rs.txt"
if "$OUT" "$P" 2>&1 | head -1 | grep -q 'never defined'; then echo "  ok   refused_undefined_call: refused, not silently accepted"
else echo "  FAIL refused_undefined_call: not refused"; "$OUT" "$P" 2>&1 | head -1; fi
# rung: P-MONO -- transitive monomorphization demands (a bounded generic calling a bounded generic)
run_src "$(dirname "$0")/progs/real_rust_mono_chain.rs" 0
# rung: P-PAT -- `&`-prefixed patterns (matching through a reference)
run_src "$(dirname "$0")/progs/real_rust_refpat.rs" 0
# rung: P-STD -- range indexing (slice COPY): v[a..b], v[a..], v[..b], v[..]
run_src "$(dirname "$0")/progs/real_rust_slicing.rs" 0
# rung: `dyn Trait<Assoc = T>` reads; and a trait object from a PRIMITIVE is refused, not segfaulted
run_src "$(dirname "$0")/progs/real_rust_dyn_assoc.rs" 0
P2="$(dirname "$0")/progs/refused_dyn_from_primitive.rs.txt"
if "$OUT" "$P2" 2>&1 | head -1 | grep -q 'trait object'; then echo "  ok   refused_dyn_from_primitive: refused, not segfaulted"
else echo "  FAIL refused_dyn_from_primitive: not refused"; "$OUT" "$P2" 2>&1 | head -1; fi
# rung: P-WIDTH -- arithmetic on a width-typed value wraps to that width (runtime, not just the fold)
run_src "$(dirname "$0")/progs/real_rust_widths.rs" 0
# rung: P-NOM rung 2 -- a char literal keeps its kind (prints as a character; `char` is a known type)
run_src "$(dirname "$0")/progs/real_rust_char_kind.rs" 0
# rung: P-BOX -- a trait object from a PRIMITIVE that implements the trait
run_src "$(dirname "$0")/progs/real_rust_dyn_primitive.rs" 0
# rung: the RC-behavior-topology gate -- every probed point rustc RUNS must agree with us (2026-10-16 arc)
G="$ROOT/apparatus/arcs/2026-10-16-rc-behavior-topology/probes/gate.sh"; if [ -f "$G" ]; then OURS="$OUT" bash "$G"; else echo "  skip RC-behavior gate (not in this tree)"; fi
# rung: P-ORDER -- item names are order-independent (the reader runs twice, pass 1 names-only)
run_src "$(dirname "$0")/progs/real_rust_item_order.rs" 0
# rung: P-ITER -- iterator methods as eager vector pipelines (ranges, map/filter/fold/sum/any/all, *x)
run_src "$(dirname "$0")/progs/real_rust_iter.rs" 0
# rung: P-DEBUG -- {:?} from the static type (sequences, Option, derived structs), plus string escapes
run_src "$(dirname "$0")/progs/real_rust_debug.rs" 0
# rung: P-ITER rung 2 -- for-loop TUPLE patterns and enumerate (a sequence of pairs)
run_src "$(dirname "$0")/progs/real_rust_enumerate.rs" 0
# rung: P-ITER rung 3 -- the Option-returning consumers (max/min/first/last/position/next)
run_src "$(dirname "$0")/progs/real_rust_option_iter.rs" 0
# rung: the UNIT is a zero-tuple (prints `()`), tuple-LITERAL Debug, String::from
run_src "$(dirname "$0")/progs/real_rust_tuple_debug.rs" 0
# rung: enum STRUCT VARIANTS -- declared, constructed by name, matched by name, Debug as `C { x: 2 }`
run_src "$(dirname "$0")/progs/real_rust_enum_struct_variant.rs" 0
# rung: a CALLABLE-bounded type parameter is our `Fn` type (generics list, where clause, for<..>)
run_src "$(dirname "$0")/progs/real_rust_fn_bound.rs" 0
# rung: P-PLACE slice -- `static mut` is a real global; and F-ARGS (format args read at format time)
run_src "$(dirname "$0")/progs/real_rust_static_mut.rs" 0
# rung: Option/Result surface (unwrap/expect/unwrap_or/is_*/map, parse) + STRUCTURAL enum equality
run_src "$(dirname "$0")/progs/real_rust_option_result.rs" 0
# rung: P-PLACE probed -- &mut to aggregates, iter_mut write-back, &mut scalar READ ok / WRITE refused
run_src "$(dirname "$0")/progs/real_rust_places.rs" 0
# rung: N1-N5 panic topology -- div-by-zero and OOB exit 101, process::exit, Result-returning main
run_src "$(dirname "$0")/progs/real_rust_panics.rs" 0
# rung: C3 -- `continue` advances the loop (the step lives in the condition)
run_src "$(dirname "$0")/progs/real_rust_continue.rs" 0
# rung: a `loop` is a value; Vec::pop answers an Option; unique break labels
run_src "$(dirname "$0")/progs/real_rust_loop_value.rs" 0
