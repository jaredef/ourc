enum E { Lit(i64), Add(Box<E>, Box<E>) }
fn eval(e: &E) -> i64 { match e { E::Lit(v) => *v, E::Add(a,b) => eval(a)+eval(b) } }
fn mk(n: i64) -> E { E::Lit(n) }
fn main() { println!("{}", eval(&mk(5))); }
