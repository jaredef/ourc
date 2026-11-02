// match on Option<&T> from .first()/.last()/.get() -- P-MATCH-PATTERN rung 4:
// the match path left the by-value payload untagged, so `Some(n) => *n` did a real
// load of the value -> segfault. Now EOPTREF threads into the match's byref (payload
// is a ref, `*n` a no-op), and `Some(&n)` strips one ref level.
fn main() {
    let v = vec![10, 20, 30];
    match v.first() { Some(n) => println!("{}", *n), None => println!("none") }
    match v.first() { Some(&n) => println!("{}", n + 1), None => {} }
    match v.last() { Some(&n) => println!("{}", n), None => {} }
    // plain (constructed) Option still works
    let o: Option<i32> = Some(7);
    match o { Some(n) => println!("{}", n * 2), None => {} }
    // empty vec -> None arm
    let e: Vec<i32> = Vec::new();
    match e.first() { Some(&n) => println!("{}", n), None => println!("empty") }
}
