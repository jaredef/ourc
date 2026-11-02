// LAYER3 M4: `let f: fn(A,..)->R = ..` fn-pointer type annotation on a binding. The let-annotation parser read
// the `fn` name then desynced on `(` (FE-reject); now it consumes `(args)->ret` up to `=`. The RHS (a closure or
// a fn name) supplies the binding kind.
fn dbl(x: i32) -> i32 {
    x * 2
}
fn main() {
    let inc: fn(i32) -> i32 = |x| x + 1;
    println!("{}", inc(9));
    let add: fn(i32, i32) -> i32 = |a, b| a + b;
    println!("{}", add(3, 4));
    let d: fn(i32) -> i32 = dbl;
    println!("{}", d(21));
    let k: fn() -> i32 = || 42;
    println!("{}", k());
}
