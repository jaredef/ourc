struct P { x: i64, y: i64 } fn main() { let p = P { x: 3, y: 4 }; let P { x, y: yy } = p; match (x, yy) { (3, b) => println!("{}", b), _ => println!("no") } }
