struct Inner { v: i64 }
impl Inner { fn dbl(&self) -> i64 { self.v * 2 } }
struct Outer { inner: Inner }
impl Outer { fn total(&self) -> i64 { self.inner.dbl() + 1 } }
fn main() {
    let o = Outer { inner: Inner { v: 20 } };
    println!("{}", o.total());
}
