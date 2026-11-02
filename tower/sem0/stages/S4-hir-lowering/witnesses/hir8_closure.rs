fn a() -> i32 { let f = |x| x; f(1) }
fn b(base: i32) -> i32 { let g = move |y| y + base; g(2) }
fn c() -> i32 { let k = const { 2 }; k }
fn d() -> i32 { let h = |x: i32| -> i32 { x + 1 }; h(3) }
fn main() {}
