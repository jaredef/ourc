mod m { pub struct S { pub v: i64 } pub const K: i64 = 100; pub fn tag() -> i64 { 5 } }
use crate::m::S;
use crate::m::K;
use crate::m::tag;
fn main() { let s = S { v: 7 }; println!("{}", s.v + K + tag()); }
