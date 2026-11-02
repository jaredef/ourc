mod m { fn hidden() -> i64 { 41 } pub fn open() -> i64 { hidden() + 1 } } fn main() { println!("{}", m::open()); }
