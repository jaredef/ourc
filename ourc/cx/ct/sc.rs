fn main() { let a = "b"; let b = "a"; println!("{:?} {:?}", a.cmp(b), "x".to_string().cmp(&"y".to_string())); let t = ("q", 1); let u = ("p", 2); println!("{:?}", t.0.cmp(u.0)); }
