fn main() { let mut bb = Box::new(Box::new(vec![1, 2])); bb.push(3); println!("{}", bb.len()); }
