#[derive(Default)]
struct S { a: i64, b: i64 }
fn main() {
    let s: S = Default::default();
    println!("{}", s.a + s.b + 99);
}
