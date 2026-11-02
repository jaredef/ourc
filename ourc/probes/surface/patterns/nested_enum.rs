enum E { A(Option<i64>), B } fn f(e: E) -> i64 { match e { E::A(Some(v)) => v, E::A(None) => 1, E::B => 2 } } fn main() { println!("{} {} {}", f(E::A(Some(40))), f(E::A(None)), f(E::B)); }
