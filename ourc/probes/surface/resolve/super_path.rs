mod a { pub fn base() -> i64 { 40 } pub mod b { pub fn f() -> i64 { super::base() + 2 } } } fn main() { println!("{}", a::b::f()); }
