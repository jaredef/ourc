fn main() { let v: Vec<i64> = vec![]; if let Some(x) = v.first() { println!("{}", x); } else { println!("empty"); } let o = Some(3); if let Some(y) = o { println!("{}", y + 1); } }
