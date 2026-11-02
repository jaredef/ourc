mod a { pub fn f() -> i32 { 1 } fn secret() -> i32 { 9 } }
mod b { pub fn g() -> i32 { 2 } }
use a::secret;
use b::g;
fn main() {}
