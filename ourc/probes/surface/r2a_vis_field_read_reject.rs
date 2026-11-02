// R2a P6 guard (reject side, rustc E0616): reading a PRIVATE field from outside the struct's module (here a root fn).
mod m { pub struct S { y: i32 } pub fn mk() -> S { S { y: 3 } } }
fn rd(s: &m::S) -> i32 { s.y }
fn main() { println!("{}", rd(&m::mk())); }
