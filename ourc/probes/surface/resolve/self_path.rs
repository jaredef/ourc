mod a { pub fn x() -> i64 { 2 } pub fn y() -> i64 { self::x() * 21 } } fn main() { println!("{}", a::y()); }
