#!/usr/bin/env bash
# build-gate.sh -- the rzc driver gate. Assembles a self-contained toolchain directory (rzc + its 5 stage
# binaries as siblings) and exercises `rzc build|run|test` with NO stage paths -- rzc self-locates them via
# readlink(/proc/self/exe). Proves: a good program builds + runs; a rejected program makes rzc FAIL (nonzero,
# error passed through) and WITHHOLD the ELF; run propagates exit; test runs #[test] files with fork-per-test
# isolation. (Unlike the bare pipe, which emits a broken ELF at exit 0.)
#
#   SP=<scratch containing the mainline frontend rzrs.self> bash build-gate.sh
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; A="$(cd "$HERE/../.." && pwd)"; CC="$A/L2"
SP="${SP:?set SP (must contain the mainline frontend rzrs.self)}"
MC="$A/L0"
"$MC/m0/m0.bin" < "$MC/emitter/rzir-to-elf-cf.m0" > "$SP/cf.m0" 2>/dev/null; chmod +x "$SP/cf.m0"
chain(){ "$CC/link/rzir-ld.bin" | "$CC/opt/rzir-opt.bin" | "$CC/ra/rzir-ra.bin" | "$1"; }

# the extended cf.rs backend (K1 syscall ABI) + rzc itself
"$SP/rzrs.self" < "$A/backend-cf/cf.rs" 2>/dev/null | chain "$SP/cf.m0" > "$SP/cf.rs.bin" 2>/dev/null; chmod +x "$SP/cf.rs.bin"
"$SP/rzrs.self" < "$HERE/rzc.rs"                       2>/dev/null | chain "$SP/cf.rs.bin" > "$SP/rzc" 2>/dev/null; chmod +x "$SP/rzc"
[ -s "$SP/rzc" ] || { echo "FAIL: rzc did not build"; exit 1; }

# assemble the self-contained toolchain directory: rzc + its 5 stage siblings
TC="$SP/tc"; rm -rf "$TC"; mkdir -p "$TC"
cp "$SP/rzc" "$TC/rzc"; cp "$SP/rzrs.self" "$TC/rzrs.self"; cp "$SP/cf.rs.bin" "$TC/cf"
cp "$CC/link/rzir-ld.bin" "$TC/rzir-ld"; cp "$CC/opt/rzir-opt.bin" "$TC/rzir-opt"; cp "$CC/ra/rzir-ra.bin" "$TC/rzir-ra"
chmod +x "$TC"/*
rzc(){ "$TC/rzc" "$@"; }   # invoked with NO stage paths -- rzc finds its siblings itself

# 1) `build`: a good program builds + runs
printf 'fn main()->i32{ let mut s=0; let mut i=1; while i<=10 { s=s+i; i=i+1; } s }' > "$TC/g.rs"
rzc build "$TC/g.rs" "$TC/g.elf" >/dev/null 2>&1; gc=$?
chmod +x "$TC/g.elf" 2>/dev/null; "$TC/g.elf"; rr=$?
[ "$gc" = 0 ] && [ "$rr" = 55 ] || { echo "FAIL: good build (rzc rc=$gc, run=$rr, want 0/55)"; exit 1; }
echo "  PASS build: rzc (self-located stages) exit 0, output ELF runs == 55"

# 2) `build`: a frontend-rejected program -> rzc fails nonzero AND withholds the ELF
printf 'struct P{x:i32} fn main()->i32{ let a=P{x:5}; let b=a; return a.x; }' > "$TC/b.rs"
rm -f "$TC/b.elf"
berr="$(rzc build "$TC/b.rs" "$TC/b.elf" 2>&1 >/dev/null)"; bc=$?
[ "$bc" != 0 ] || { echo "FAIL: rzc returned 0 on a rejected program"; exit 1; }
[ ! -s "$TC/b.elf" ] || { echo "FAIL: rzc emitted an ELF for a rejected program"; exit 1; }
echo "$berr" | grep -q 'use of moved value' || { echo "FAIL: frontend error not surfaced"; exit 1; }
echo "  PASS build: rzc exit $bc on a reject, error surfaced, ELF withheld"

# 3) `run`: build-then-exec propagates the program's exit code
rzc run "$TC/g.rs" >/dev/null 2>&1; xc=$?
[ "$xc" = 55 ] || { echo "FAIL: run propagated $xc, want 55"; exit 1; }
echo "  PASS run: build-then-exec propagated program exit == 55"

# 4) `run`: a rejected program fails without exec
rzc run "$TC/b.rs" >/dev/null 2>&1; yc=$?
[ "$yc" != 0 ] || { echo "FAIL: run returned 0 on a reject"; exit 1; }
echo "  PASS run: reject fails ($yc), no exec"

# 5) `test`: assert-based test passes -> "ok", exit 0
printf 'fn main()->i32{ assert_eq!(2+2,4); assert!(10>3); assert_ne!(1,2); return 0; }' > "$TC/tp.rs"
out="$(rzc test "$TC/tp.rs" 2>&1)"; tc=$?
[ "$tc" = 0 ] && echo "$out" | grep -q 'ok' || { echo "FAIL: passing test (rc=$tc, out=$out)"; exit 1; }
echo "  PASS test: assert-based test -> ok, exit 0"

# 6) `test`: a failing assert -> "FAILED", exit 1
printf 'fn main()->i32{ assert_eq!(2+2,5); return 0; }' > "$TC/tf.rs"
out="$(rzc test "$TC/tf.rs" 2>&1)"; fc=$?
[ "$fc" != 0 ] && echo "$out" | grep -q 'FAILED' || { echo "FAIL: failing test not caught (rc=$fc, out=$out)"; exit 1; }
echo "  PASS test: failing assert -> FAILED, exit $fc"

# 7) `test`: a #[test] file with NO explicit main (K5: the frontend synthesizes the runner)
printf '#[test]\nfn test_a() { assert_eq!(2+2,4); }\n#[test]\nfn test_b() { assert!(5>1); }\n' > "$TC/mt.rs"
out="$(rzc test "$TC/mt.rs" 2>&1)"; mc=$?
[ "$mc" = 0 ] && echo "$out" | grep -q 'ok' || { echo "FAIL: #[test] file (no main) not run (rc=$mc, out=$out)"; exit 1; }
echo "  PASS test: #[test] file with no main -> synthesized runner -> ok"

# 8) `test`: a #[test] file with a failing test -> FAILED
printf '#[test]\nfn test_a() { assert_eq!(1,1); }\n#[test]\nfn test_b() { assert_eq!(1,2); }\n' > "$TC/mtf.rs"
out="$(rzc test "$TC/mtf.rs" 2>&1)"; nc=$?
[ "$nc" != 0 ] && echo "$out" | grep -q 'FAILED' || { echo "FAIL: failing #[test] not caught (rc=$nc, out=$out)"; exit 1; }
echo "  PASS test: #[test] file with a failing test -> FAILED, exit $nc"

# 9) `test`: fork-per-test ISOLATION -- 2 of 3 tests fail; all run, exactly 2 counted, cargo-style per-test lines
printf '#[test]\nfn t_a(){ assert_eq!(1,2); }\n#[test]\nfn t_b(){ assert!(1>0); }\n#[test]\nfn t_c(){ assert_eq!(3,4); }\n' > "$TC/iso.rs"
out="$(rzc test "$TC/iso.rs" 2>&1)"; ic=$?
echo "$out" | grep -q '2 failed' && [ "$ic" != 0 ] || { echo "FAIL: isolation count wrong (rc=$ic, out=$out)"; exit 1; }
echo "$out" | grep -q 'test t_a \.\.\. FAILED' && echo "$out" | grep -q 'test t_b \.\.\. ok' || { echo "FAIL: cargo-style per-test lines missing (out=$out)"; exit 1; }
echo "  PASS test: fork-per-test isolation -- per-test 'test NAME ... ok|FAILED', '2 failed', FAILED"

# 10) `build`: multi-file -- `mod NAME;` inlines sibling files
mkdir -p "$TC/proj"
printf 'fn square(x:i32)->i32{ x*x }\nfn cube(x:i32)->i32{ x*x*x }\n' > "$TC/proj/util.rs"
printf 'mod util;\nfn main()->i32{ return square(4)+cube(3); }\n' > "$TC/proj/main.rs"   # 16+27 = 43
rzc build "$TC/proj/main.rs" "$TC/proj/main.elf" >/dev/null 2>&1; pc=$?
chmod +x "$TC/proj/main.elf" 2>/dev/null; "$TC/proj/main.elf"; pr=$?
[ "$pc" = 0 ] && [ "$pr" = 43 ] || { echo "FAIL: multi-file build (rc=$pc, run=$pr, want 0/43)"; exit 1; }
echo "  PASS build: mod resolution -- 'mod util;' inlined, output runs == 43"

# 11) `build`: a `mod NAME;` whose file is missing -> rzc fails, error, ELF withheld
printf 'mod gone;\nfn main()->i32{ return 0; }\n' > "$TC/proj/bad.rs"
rm -f "$TC/proj/bad.elf"
merr="$(rzc build "$TC/proj/bad.rs" "$TC/proj/bad.elf" 2>&1 >/dev/null)"; mrc=$?
[ "$mrc" != 0 ] && [ ! -s "$TC/proj/bad.elf" ] && echo "$merr" | grep -q 'cannot open module' || { echo "FAIL: missing module not caught (rc=$mrc, out=$merr)"; exit 1; }
echo "  PASS build: missing module -> rc $mrc, 'cannot open module', ELF withheld"

# 12) `build`: invalid code is REJECTED (not silently mis-accepted) and rzc propagates it
#     (guards the "reject, don't mis-lower/accept-invalid" property AND catches a stale front that fails to reject)
#     NOTE: every formerly-"unsupported" construct (macro_rules!/operators/->impl/const generics) is now SUPPORTED
#     (arc A1-A6), so die_unsupported has no callers. The standing honest reject is the borrow checker.
printf 'fn main(){ let mut x=5; let a=&mut x; let b=&mut x; *a+=1; *b+=1; println!("{}",x); }\n' > "$TC/mr.rs"
rm -f "$TC/mr.elf"
uerr="$(rzc build "$TC/mr.rs" "$TC/mr.elf" 2>&1 >/dev/null)"; urc=$?
[ "$urc" != 0 ] && [ ! -s "$TC/mr.elf" ] && echo "$uerr" | grep -q 'borrow' || { echo "FAIL: invalid code not rejected (rc=$urc, out=$uerr)"; exit 1; }
echo "  PASS build: invalid (double &mut) -> rejected 'cannot borrow', rc $urc, ELF withheld"

echo "PASS: rzc build|run|test self-locate their stages; mod resolution; #[test] + isolation; reject-not-mis-lower; per-stage exit-code checking"
