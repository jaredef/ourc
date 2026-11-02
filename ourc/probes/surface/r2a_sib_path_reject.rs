// R2a P6 SIB-PATH guard (E0433): a sibling module named without crate::/super:: inside another module does not resolve
mod a { pub fn f() -> i32 { 1 } }
mod b { pub fn g() -> i32 { a::f() } }
fn main() { println!("{}", b::g()); }
