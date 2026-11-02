use std::fmt::Display;
struct Wrap<T> { inner: T, n: i32 }
impl<T: Display> Wrap<T> { fn show(&self) -> String { format!("<{}:{}>", self.n, self.inner) } }
fn main() { let d = Wrap { inner: 2.5f64, n: 1 }; println!("{}", d.show()); }
