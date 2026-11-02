#[derive(Clone)]
enum T { A, B }
fn f(t: T) -> i64 { let r = if let T::A = t { 1 } else { 2 }; r }
fn main() { println!("{} {}", f(T::A), f(T::B)); }
