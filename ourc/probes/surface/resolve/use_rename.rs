mod m { pub fn f() -> i64 { 42 } } use m::f as g; fn main() { println!("{}", g()); }
