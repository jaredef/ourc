struct S { x: i32, y: i32 }
fn getx(s: &S) -> i32 { s.x }
fn suml(a: &S, b: &S) -> i32 { getx(a) + getx(b) + getx(a) }
fn main() {
    let p = S { x: 5, y: 9 };
    let q = S { x: 20, y: 1 };
    println!("{}", getx(&p));
    println!("{}", suml(&p, &q));
}
