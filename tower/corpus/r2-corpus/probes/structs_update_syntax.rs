#[derive(Default)]
struct S { a: i64, b: i64, c: i64 }
fn main() {
    let s = S { a: 5, ..Default::default() };
    println!("{}", s.a + s.b + s.c);
}
