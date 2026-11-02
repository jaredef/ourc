enum E { Lit(i64), Add(Box<E>, Box<E>) }
fn eval(e: &E) -> i64 { match e { E::Lit(v) => *v, E::Add(a,b) => eval(a)+eval(b) } }
fn f(e: &E) -> i64 { match e { E::Add(a, b) if eval(a) == eval(b) => 1, E::Add(_, _) => 2, E::Lit(n) if *n < 0 => 3, E::Lit(_) => 4 } }
fn main() { println!("{}", f(&E::Lit(5))); }
