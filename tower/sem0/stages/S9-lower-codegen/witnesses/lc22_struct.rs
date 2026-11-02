struct P { x: i32, y: i32 }
fn main() {
    let p: P = P { x: 3, y: 4 };
    let s: i32 = p.x + p.y;
    println!("{}", s);
}
