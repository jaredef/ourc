struct Wrap<T> { inner: T }
fn main() { let b = Wrap { inner: "s" }; println!("<{}>", b.inner); }
