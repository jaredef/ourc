enum E { A(i32), B }
fn v(e: &E) -> i32 { match e { E::A(n) => *n, E::B => 0 } }
fn main() { println!("{} {}", v(&E::A(7)), v(&E::B)); }
