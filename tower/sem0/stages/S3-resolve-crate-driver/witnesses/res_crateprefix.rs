mod a { pub fn f() -> i32 { 1 } fn g() -> i32 { 2 } }
use crate::a::g;
fn main() {}
