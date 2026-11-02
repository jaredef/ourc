// R2a P4 guard: a match whose arms return their own payload bindings (and whose fallback arm is `return`) has the payload's
// type -- the ext_any shape; a `break` arm in a value match; a panic arm.
enum E { S(i64, Vec<i64>), T(Vec<i64>), N }
fn f(e: &E) -> usize { let xs = match e { E::S(_s, xs) => xs, E::T(xs) => xs, _ => return 0 }; xs.len() }
fn g(e: &E) -> i64 { match e { E::T(v) => v.len() as i64, _ => panic!() } }
fn main() {
    let mut t = 0;
    for i in 0..5 { let k = match i { 3 => break, x => x * 2 }; t = t + k; }
    println!("{} {} {} {} {}", f(&E::S(1, vec![1, 2])), f(&E::T(vec![5])), f(&E::N), g(&E::T(vec![1, 2, 3])), t);
}
