fn t(x: i64) -> bool { println!("t{}", x); true } fn f(x: i64) -> bool { println!("f{}", x); false } fn main() { if f(1) && t(2) { } if t(3) || f(4) { } }
