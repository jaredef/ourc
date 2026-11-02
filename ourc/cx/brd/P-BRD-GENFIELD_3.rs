struct Wrap<T> { inner: T, n: i32 }
fn main() { let b = Wrap { inner: String::from("own"), n: 1 }; println!("<{}> {}", b.inner, b.n); let c = Wrap { inner: 2.5f64, n: 3 }; println!("{}", c.inner); }
