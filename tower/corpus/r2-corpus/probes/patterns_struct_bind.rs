struct P { x: i32, y: i32 }
fn main() {
    let p = P { x: 4, y: 6 };
    let P { x: a, y: b } = p;
    println!("{}", a * b);
}
