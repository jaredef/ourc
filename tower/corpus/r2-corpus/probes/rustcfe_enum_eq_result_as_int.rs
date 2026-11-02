#[derive(PartialEq)]
enum E { A(i32), B(i32) }
fn main() { let a = E::A(1); let b = E::A(1); println!("{}", (a == b) as i32); }
