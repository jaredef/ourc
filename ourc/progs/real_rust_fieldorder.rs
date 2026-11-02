#[derive(Copy, Clone)]
struct P { a: i64, b: i64 }
fn main() {
    let p = P { b: 2, a: 40 };
    println!("{}", p.a + p.b);
    println!("{}", p.a - p.b);
}
