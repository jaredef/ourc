mod a { pub fn f() -> i32 { 1 } fn g() -> i32 { 2 } }
use a::*;
fn main() { let x = f(); }
