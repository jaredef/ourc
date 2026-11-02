mod m { fn hidden() -> i64 { 40 } pub mod c { pub fn f() -> i64 { super::hidden() + 2 } } } fn main() { println!("{}", m::c::f()); }
