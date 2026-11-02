fn a(v: &Vec<(String, u32)>) -> u32 { if let Some((_, r)) = v.iter().find(|t| t.1 == 2) { println!("a {}", *r); } 0 }
fn b(v: &Vec<(String, u32)>) -> u32 { if let Some((_, r)) = v.iter().find(|t| t.1 == 2) { return *r; } 0 }
fn c(v: &Vec<(String, u32)>) -> u32 { if let Some((_, r)) = v.iter().find(|t| t.1 == 2) { let x: u32 = *r; return x; } 0 }
fn main() { let v: Vec<(String, u32)> = vec![("a".into(), 1), ("b".into(), 2)]; a(&v); println!("{}", b(&v)); println!("{}", c(&v)); }
