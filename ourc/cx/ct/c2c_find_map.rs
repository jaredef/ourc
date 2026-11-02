fn main() { let v: Vec<(String, u32)> = vec![("a".into(), 1), ("b".into(), 2)]; let r = v.iter().find(|t| t.0 == "b"); println!("{:?}", r.map(|(_, x)| *x)); println!("{:?}", r.map(|t| t.1)); }
