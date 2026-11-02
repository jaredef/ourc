fn main() { let v = vec!["a", "b"]; let s: Vec<String> = v.iter().map(|x| x.to_uppercase()).collect(); println!("{} {}", s.join("-"), s.concat()); }
