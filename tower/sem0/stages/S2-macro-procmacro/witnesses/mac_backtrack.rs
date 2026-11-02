macro_rules! m { ($a:expr, $b:expr) => { $a + $b }; ($a:expr) => { $a } }
macro_rules! n { (zero) => { 0 }; ($x:expr) => { $x } }
fn f() -> i32 { m!(1, 2) }
fn g() -> i32 { m!(5) }
fn h() -> i32 { n!(zero) }
fn i() -> i32 { n!(9) }
fn main() {}
