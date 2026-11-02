// P-BRD-GENFIELD: a field typed by a generic struct's type param (`inner: T`) read on a concrete instance takes the
// instance's type arg -- from a struct literal, an annotated `Wrap<&str>`, and an assoc ctor `Wrap::new(..) -> Self`.
use std::fmt::Display;
struct Wrap<T> { inner: T, n: i32 }
impl<T: Display> Wrap<T> {
    fn new(inner: T) -> Self { Wrap { inner, n: 1 } }
    fn show(&self) -> String { format!("<{}:{}>", self.n, self.inner) }
}
fn main() {
    let a = Wrap { inner: String::from("own"), n: 2 };
    println!("lit field: {} {}", a.inner, a.n);
    let b: Wrap<&str> = Wrap { inner: "ann", n: 3 };
    println!("annotated: {}", b.inner);
    let c = Wrap::new("ctor");
    println!("ctor: {} {}", c.show(), c.inner);
}
