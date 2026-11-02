mod a { pub fn f() -> i32 { 1 } }
use self::a::f;
fn main() { let x = f(); }
