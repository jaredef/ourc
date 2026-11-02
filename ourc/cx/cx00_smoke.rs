// cx00: harness smoke -- struct + Vec + String + match through the ourc semc path.
struct P { x: i64, y: i64 }
fn main() {
    let v: Vec<P> = vec![P { x: 1, y: 2 }, P { x: 3, y: 4 }];
    let mut s = 0; for p in &v { s += p.x * p.y; }
    let name = String::from("cx");
    match s { 14 => println!("{} ok {}", name, s), _ => println!("{} bad {}", name, s) }
}
