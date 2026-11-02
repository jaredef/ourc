mod m { pub fn f() -> i64 { 41 } pub mod n { pub fn g() -> i64 { 1 } } } fn main() { println!("{}", m::f() + m::n::g()); }
