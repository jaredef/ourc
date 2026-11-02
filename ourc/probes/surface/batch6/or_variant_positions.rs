enum E { A(i64, i64), B(i64) } fn f(e: &E) -> i64 { match e { E::A(_, x) | E::B(x) => *x } } fn main() { println!("{} {}", f(&E::A(1, 2)), f(&E::B(9))); }
