trait D { fn d(&self) -> i64; } impl D for i64 { fn d(&self) -> i64 { *self * 2 } } fn f<T>(x: T) -> i64 where T: D { x.d() } fn main() { println!("{}", f(21i64)); }
