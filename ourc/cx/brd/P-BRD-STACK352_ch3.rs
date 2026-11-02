fn main() { let v = vec![String::from("a")]; let w: Vec<String> = v.iter().cloned().chain(std::iter::once(String::from("z"))).collect(); println!("{:?}", w); }
