enum T { A(u32), B(u32), C }
fn f(a: &T) -> u32 { match a { T::A(x) | T::B(x) => *x, T::C => 0 } }
fn g(a: T) -> u32 { match a { T::A(x) | T::B(x) => x + 100, T::C => 0 } }
fn main() { println!("{} {} {} {} {}", f(&T::A(1)), f(&T::B(2)), f(&T::C), g(T::B(7)), g(T::A(8))); }
