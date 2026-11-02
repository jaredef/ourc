// Novel demo (2026-07-07): rusteze end-to-end proof on Pi (aarch64 Linux).
// Exercises fn, let mut, if/else, while, arithmetic, recursion, exit-code.
// R0-R6 subset. Operator set constrained to what cf-arm64 currently emits
// (add/sub/mul/div, ge/le/eq), so the rzrs -> chain -> cf-arm64 composition
// covers this program end-to-end on Pi. See demo/FINDINGS.md.
//
// Computes: sum_{i=1..=6} tri(i) - 2*fact(4), returned as the exit code.
// tri(n) = n*(n+1)/2. Sum i=1..6 of tri = 1+3+6+10+15+21 = 56.
// fact(4) = 24; 2*24 = 48. Result = 56 - 48 = 8.

fn tri(n: i32) -> i32 {
    let mut i: i32 = 1;
    let mut s: i32 = 0;
    while i <= n {
        s = s + i;
        i = i + 1;
    }
    return s;
}

fn fact(n: i32) -> i32 {
    if n <= 1 { return 1; }
    return n * fact(n - 1);
}

fn main() {
    let mut total: i32 = 0;
    let mut k: i32 = 1;
    while k <= 6 {
        total = total + tri(k);
        k = k + 1;
    }
    std::process::exit(total - 2 * fact(4));
}
