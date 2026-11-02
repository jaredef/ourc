mod a { pub fn f() -> i32 { 1 } }
mod b { pub fn f() -> i32 { 2 } }
use a::f;
use b::f;
fn main() {}
