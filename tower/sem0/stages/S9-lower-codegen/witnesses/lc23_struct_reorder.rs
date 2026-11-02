struct Q { a: i32, b: i32, c: i32 }
fn main() {
    let q: Q = Q { c: 100, a: 2, b: 30 };
    let s: i32 = q.a * q.c - q.b;
    println!("{}", s);
}
