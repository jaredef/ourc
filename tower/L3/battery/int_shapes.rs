trait Shape { fn area(&self) -> i32; }
struct Circle { r: i32 } struct Square { s: i32 }
impl Shape for Circle { fn area(&self) -> i32 { 3 * self.r * self.r } }
impl Shape for Square { fn area(&self) -> i32 { self.s * self.s } }
fn describe<T: Shape>(x: &T) -> i32 { let a = x.area(); if a > 20 { a } else { 0 } }
fn main() { let c = Circle { r: 3 }; let s = Square { s: 3 }; std::process::exit(describe(&c) + describe(&s)); }