enum E { Lit(i64), Add(Box<E>, Box<E>) }
fn sum_map<F: Fn(i64) -> i64>(e: &E, f: &F) -> i64 {
    match e { E::Lit(v) => f(*v), E::Add(a, b) => sum_map(a, f) + sum_map(b, f) }
}
fn main() {
    let t = E::Add(Box::new(E::Lit(3)), Box::new(E::Lit(4)));
    println!("{}", sum_map(&t, &|x| x * 10));
}
