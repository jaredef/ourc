struct P { x: i64 } fn main() { let p = P { x: 41 }; let r = &p; let rr = &r; println!("{}", rr.x + 1); }
