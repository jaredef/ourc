struct Wrap<T> { inner: T }
impl<T: std::fmt::Display> Wrap<T> { fn new(inner: T) -> Self { Wrap { inner } } fn show(&self) -> String { format!("<{}>", self.inner) } }
fn main() { let b = Wrap::new("s"); println!("{}", b.show()); }
