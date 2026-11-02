struct P { x: i64, y: i64 }
fn mk(x: i64, y: i64) -> P { P { x, y } }
fn main() {
    let p = mk(11, 22);
    println!("{}", p.x + p.y);
}
