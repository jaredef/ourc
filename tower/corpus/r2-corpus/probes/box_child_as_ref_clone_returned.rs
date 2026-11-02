#[derive(Clone)]
enum E { Lit(i64), Add(Box<E>, Box<E>) }
fn eval(e: &E) -> i64 { match e { E::Lit(v) => *v, E::Add(a,b) => eval(a)+eval(b) } }
fn left(e: &E) -> E { match e { E::Add(a, _) => a.as_ref().clone(), E::Lit(v) => E::Lit(*v) } }
fn main() { let t = E::Add(Box::new(E::Lit(5)), Box::new(E::Lit(9))); println!("{}", eval(&left(&t))); }
