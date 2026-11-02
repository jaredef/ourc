mod m { pub fn f() -> i64 { 42 } } use m::f; fn main() { println!("{}", f()); }
