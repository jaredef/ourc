mod a { pub fn f() -> i32 { 1 } mod b { pub fn g() -> i32 { 2 } } }
use a::{f, b::g};
fn main() {}
