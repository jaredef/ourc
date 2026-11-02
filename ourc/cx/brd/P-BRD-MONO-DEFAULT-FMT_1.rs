fn a<T: Default + std::fmt::Display>(_x: T) -> String { format!("{}", T::default()) }
fn b<T: Default + std::fmt::Debug>(_x: T) -> String { let d = T::default(); format!("{:?}", d) }
fn main() { println!("{} {}", a(true), b(true)); }
