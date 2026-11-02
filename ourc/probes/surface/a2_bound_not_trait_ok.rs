use std::fmt::Debug;
struct S { a: u32, b: u8 }
fn show<T: Debug, U: Clone>(x: T, _u: U) -> String { format!("{:?}", x) }
fn add(a: i32, b: u8) -> i32 { a + b as i32 }
fn main() { let s = S { a: 1, b: 2 }; println!("{} {} {}", show(s.a, 1), add(1, s.b), s.a); }
