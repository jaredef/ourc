fn a(v: &Vec<(String, u32)>) -> bool { v.iter().find(|t| t.1 == 2).is_some() }
fn b(v: &Vec<(String, u32)>) -> bool { v.iter().any(|t| t.1 == 2) }
fn c(v: &Vec<(u32, u32)>) -> bool { v.iter().any(|t| t.1 == 2) }
fn d(v: &Vec<(String, u32)>) -> usize { v.iter().filter(|t| t.1 == 2).count() }
fn main() { let v: Vec<(String, u32)> = vec![("a".into(), 1), ("b".into(), 2)]; println!("{}", c(&vec![(1, 2)])); println!("{}", b(&v)); println!("{}", d(&v)); println!("{}", a(&v)); }
