fn main() { let bb = Box::new(Box::new(vec![1, 2])); println!("{:?}", *bb.clone()); }
