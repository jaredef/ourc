// R2a P4 guard: a closure binding called inside if branches (statement, value, return, else-if), with fn/closure shadowing.
fn sq(x: i64) -> i64 { x + 100 }
fn f(n: i64) -> i64 {
    let c = |x: i64| x * x;
    if n > 5 { return c(n); }
    if n > 0 { c(n) } else { 0 }
}
fn main() {
    let x = 5;
    let g = |a: i64| a + 1;
    let r = if x > 9 { 0 } else if x > 3 { g(x) } else { g(0) };
    if true { println!("{}", sq(2)); }
    let sq = |x: i64| x * x;
    if true { println!("{} {} {} {}", sq(2), f(3), f(7), r); }
}
