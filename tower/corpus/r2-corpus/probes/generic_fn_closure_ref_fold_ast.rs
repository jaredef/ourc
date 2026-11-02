enum E { Lit(i64), Add(Box<E>, Box<E>) }
fn map_lit<F: Fn(i64) -> i64>(e: &E, f: &F) -> E {
    match e { E::Lit(v) => E::Lit(f(*v)), E::Add(a, b) => E::Add(Box::new(map_lit(a, f)), Box::new(map_lit(b, f))) }
}
fn eval(e: &E) -> i64 { match e { E::Lit(v) => *v, E::Add(a,b) => eval(a)+eval(b) } }
fn main() {
    let t = E::Add(Box::new(E::Lit(3)), Box::new(E::Lit(4)));
    let d = map_lit(&t, &|x| x * 10);
    println!("{}", eval(&d));
}
