fn main() { let words = vec!["apple", "bob", "kayak"]; let p: Vec<&&str> = words.iter().filter(|w| w.chars().rev().collect::<String>() == **w).collect(); println!("{:?}", p); }
