mod a { pub(crate) fn f() -> i32 { 1 } }
use a::f;
fn main() { let x = f(); }
