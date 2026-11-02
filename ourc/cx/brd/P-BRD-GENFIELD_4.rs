struct Wrap<T> { inner: T }
fn main() { let b: Wrap<&str> = Wrap { inner: "s" }; println!("<{}>", b.inner); }
