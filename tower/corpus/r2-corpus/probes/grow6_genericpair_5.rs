fn pair<A, B>(a: A, b: B) -> (A, B) { (a, b) }
fn main() {
    let (x, y) = pair(3i64, 4i64);
    let (p, q) = pair(10i64, 20i64);
    println!("{}", x + y + p + q);
}
