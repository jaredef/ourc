// R2a P5 guard: calling a call's RESULT -- an `impl Fn` returned by a function (root and module path) called directly
// (`adder(5)(6)`, `twice(3)(4)`). It was "expected `}`, found `(`".
mod l {
    pub fn adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }
}
fn twice(k: i32) -> impl Fn(i32) -> i32 {
    move |x| x * k
}
fn main() {
    let s = l::adder(1)(2) + twice(2)(2);
    println!("{} {} {}", l::adder(5)(6), twice(3)(4), s);
}
