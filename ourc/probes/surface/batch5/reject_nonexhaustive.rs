enum E { A, B } fn main() { let e = E::A; let n = match e { E::A => 1 }; println!("{}", n); }
