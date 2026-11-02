// R2a P3 guard: tuple patterns with nested enum groups (int payloads), guards, wildcards, a 3-tuple.
#[derive(Clone, Copy)]
enum E { A(i32), B }
fn pick(x: Option<i32>, y: Option<i32>) -> i32 {
    match (x, y) {
        (Some(p), Some(q)) if p > q => p * 100 + q,
        (Some(p), Some(q)) => p + q,
        (Some(p), None) => p,
        (None, Some(q)) => q * 10,
        (None, None) => 0,
    }
}
fn ee(x: E, y: E) -> i32 {
    match (x, y) { (E::A(a), E::A(b)) => a - b, (E::A(a), E::B) => a, (E::B, _) => 0 }
}
fn tri(a: Option<i32>, b: Option<i32>, c: Option<i32>) -> i32 {
    match (a, b, c) { (Some(x), _, Some(z)) => x + z, (_, Some(y), _) => y, _ => -1 }
}
fn main() {
    println!("{} {} {} {} {}", pick(Some(9), Some(2)), pick(Some(1), Some(2)), pick(Some(4), None), pick(None, Some(5)), pick(None, None));
    println!("{} {} {} {} {}", ee(E::A(9), E::A(4)), ee(E::A(3), E::B), ee(E::B, E::A(1)), tri(Some(1), None, Some(2)), tri(None, None, None));
}
