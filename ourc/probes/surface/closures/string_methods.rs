fn main() { let s = String::from("Hello, World"); println!("{} {} {} {}", s.to_lowercase(), s.contains("World"), s.replace("l", "L"), s.split(", ").count()); }
