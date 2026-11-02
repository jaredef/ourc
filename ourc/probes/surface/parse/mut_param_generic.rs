trait It { fn nx(&self) -> i64; } struct C { n: i64 } impl It for C { fn nx(&self) -> i64 { self.n } } fn g<T: It>(mut it: T) -> i64 { it.nx() + 35 } fn main() { println!("{}", g(C { n: 7 })); }
