// RUSTC-TARGET S3 R1 (P-MONO-GENERIC-STRUCT-ENUM, census L2): a generic inherent impl method
// `impl<T: Display> Wrap<T> { fn show(&self) { .. self.v .. } }` must be monomorphized per type-arg so a
// field of type T resolves the concrete kind (i32 / &str / String / struct), not print a pointer.
use std::fmt::Display;
struct Wrap<T> { v: T }
impl<T: Display> Wrap<T> {
    fn show(&self) -> String { format!("<{}>", self.v) }
}
fn main() {
    let a = Wrap { v: 42 };
    let b = Wrap { v: "hi" };
    let c = Wrap { v: String::from("yo") };
    println!("{}|{}|{}", a.show(), b.show(), c.show());
}
