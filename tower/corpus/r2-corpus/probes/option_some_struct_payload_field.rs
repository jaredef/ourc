struct P { x: i64, y: i64 }
fn mk(n: i64) -> Option<P> { if n > 0 { Some(P { x: n, y: n * 2 }) } else { None } }
fn main() {
    match mk(5) { Some(p) => println!("{}", p.x + p.y), None => println!("none") }
    match mk(0) { Some(p) => println!("{}", p.x), None => println!("none") }
}
