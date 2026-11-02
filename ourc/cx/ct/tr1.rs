fn fill(t: &mut Vec<String>, n: u32) { t.push(format!("x{}", n)); }
fn main() { let mut tr = vec![]; fill(&mut tr, 1); fill(&mut tr, 2); for l in tr { println!("  {}", l); } let mut t2 = Vec::new(); fill(&mut t2, 3); println!("{:?}", t2); }
