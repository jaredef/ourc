#[derive(Clone)]
enum T { A, B }
fn f(t: T) -> Option<i64> { let r = if let T::A = t { None } else { Some(7) }; r }
fn main() { println!("{:?} {:?}", f(T::A), f(T::B)); }
