trait K { const N: i64; } struct A; impl K for A { const N: i64 = 42; } fn main() { println!("{}", A::N); }
