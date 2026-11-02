mod a { pub mod b { pub fn f() -> i32 { 1 } } }
use a::b::f;
fn main() { let x = f(); }
