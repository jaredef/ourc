enum E { Lit(i64), Add(Box<E>, Box<E>) }
fn eval(e: &E) -> i64 { match e { E::Lit(v) => *v, E::Add(a,b) => eval(a)+eval(b) } }
fn f(e: &E) -> i64 { match e { E::Add(a, b) if eval(a) == eval(b) => 1, _ => 0 } }
fn main() { println!("{} {}", f(&E::Add(Box::new(E::Lit(2)),Box::new(E::Lit(2)))), f(&E::Add(Box::new(E::Lit(2)),Box::new(E::Lit(3))))); }
