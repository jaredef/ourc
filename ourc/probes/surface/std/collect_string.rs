fn main() { let s: String = vec!["a", "b", "c"].iter().map(|x| x.to_uppercase()).collect(); println!("{}", s); }
