struct Inner { v: i32 }
struct Outer { inner: Inner, n: i64 }
fn f(o: &Outer) -> i64 { o.n }
fn g(o: &Outer) -> i32 { o.inner.v }
fn main() {}
