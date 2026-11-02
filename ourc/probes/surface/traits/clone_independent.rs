#[derive(Clone)] struct P { x: i64, y: i64 } fn main() { let mut a = P { x: 1, y: 2 }; let b = a.clone(); a.x = 40; println!("{} {} {}", a.x, b.x, a.y + b.y); }
