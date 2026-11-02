enum E { Lit(i64), Add(Box<E>, Box<E>) }
fn f(e: &E) -> i64 {
    match e {
        E::Add(a, _) => match a.as_ref() { E::Lit(n) => *n, _ => -1 },
        E::Lit(v) => *v,
    }
}
fn main() { println!("{}", f(&E::Add(Box::new(E::Lit(7)), Box::new(E::Lit(9))))); }
