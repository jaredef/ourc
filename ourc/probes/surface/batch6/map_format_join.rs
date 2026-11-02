fn main() { let v = vec![4, 1, 3]; let s: Vec<String> = v.iter().map(|x| format!("<{}>", x)).collect(); println!("{}", s.join("")); }
