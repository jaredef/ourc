struct P { x: i64, y: i64 } fn main(){ let mut p = P { x: 1, y: 5 }; { let q = &mut p.y; *q = *q * 10; } println!("{} {}", p.y, p.x); }
