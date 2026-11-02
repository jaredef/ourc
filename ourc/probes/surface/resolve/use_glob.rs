mod m { pub fn f() -> i64 { 40 } pub fn h() -> i64 { 2 } } use m::*; fn main() { println!("{}", f() + h()); }
