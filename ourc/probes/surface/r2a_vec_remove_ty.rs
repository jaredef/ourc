// R2a P4 guard: Vec::remove / swap_remove answer the ELEMENT type (String, tuple, enum through a nested Box match).
enum E { I(i64), M(Box<E>, Vec<E>) }
fn ev(e: &E) -> i64 { match e { E::I(x) => *x, E::M(_, v) => v.len() as i64 } }
fn f(e: E) -> E { match e { E::M(inner, _v) => match *inner { E::M(_x, mut a0v) => a0v.remove(0), _ => E::I(0) }, _ => E::I(0) } }
fn g(mut v: Vec<String>) -> String { v.remove(0) }
fn main() {
    let mut w: Vec<(i64, String)> = vec![(1, String::from("p")), (2, String::from("q"))];
    let t = w.remove(0);
    let mut n = vec![10, 20, 30];
    let a = n.remove(1);
    let b = n.swap_remove(0);
    println!("{} {} {} {} {} {} {:?}", ev(&f(E::M(Box::new(E::M(Box::new(E::I(1)), vec![E::I(9)])), vec![]))),
        g(vec![String::from("a")]), t.0, t.1, a, b, n);
}
