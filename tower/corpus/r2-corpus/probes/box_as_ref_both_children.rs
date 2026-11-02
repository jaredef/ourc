enum E { Lit(i64), Add(Box<E>, Box<E>) }
fn f(e: &E) -> i64 { match e { E::Add(a, b) => { let x = match a.as_ref() { E::Lit(n)=>*n, _=>0 }; let y = match b.as_ref() { E::Lit(n)=>*n, _=>0 }; x + y } E::Lit(v) => *v } }
fn main() { println!("{}", f(&E::Add(Box::new(E::Lit(3)), Box::new(E::Lit(9))))); }
