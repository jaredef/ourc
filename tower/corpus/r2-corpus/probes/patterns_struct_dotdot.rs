struct S { a: i32, b: i32, c: i32 }
fn main() {
    let s = S { a: 1, b: 2, c: 3 };
    let S { a, .. } = s;
    println!("{}", a);
}
