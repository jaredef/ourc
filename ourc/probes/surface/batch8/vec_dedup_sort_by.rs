fn main() { let mut v = vec![5, 1, 4, 1, 5, 9, 2, 6]; v.sort_by(|a, b| b.cmp(a)); v.dedup(); let s: Vec<String> = v.iter().map(|x| x.to_string()).collect(); println!("{}", s.join("-")); }
