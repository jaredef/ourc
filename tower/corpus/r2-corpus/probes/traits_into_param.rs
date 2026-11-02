struct C(i64);
impl From<i64> for C { fn from(x: i64) -> C { C(x) } }
fn take(v: impl Into<C>) -> i64 { v.into().0 }
fn main() { println!("{}", take(99i64)); }
