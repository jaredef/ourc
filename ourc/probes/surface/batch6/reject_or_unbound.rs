enum E { A(i64, i64), B(i64) } fn main() { let e = E::B(1); let n = match e { E::A(x, y) | E::B(x) => x, }; println!("{}", n); }
