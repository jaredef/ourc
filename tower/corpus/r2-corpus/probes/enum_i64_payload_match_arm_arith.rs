enum N { Lit(i64), Two(i64) }
fn ev(n: &N) -> i64 { match n { N::Lit(v) => *v, N::Two(v) => *v + 1000000000000 } }
fn main() { let e = N::Two(5); println!("{}", ev(&e)); }
