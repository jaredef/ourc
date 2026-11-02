enum T { A(u32), B(u32), C }
fn f(a: &T, b: &T) -> String { match (a, b) { (T::A(x), T::A(y)) | (T::B(x), T::B(y)) => format!("same{}{}", x, y), (T::A(x), _) | (_, T::A(x)) => format!("one{}", x), _ => "o".into() } }
fn main() { println!("{} {} {} {} {}", f(&T::A(1), &T::A(2)), f(&T::B(3), &T::B(4)), f(&T::C, &T::A(5)), f(&T::A(6), &T::C), f(&T::C, &T::C)); }
