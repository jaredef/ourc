fn main() { let c = 'q'; let s = format!("<{}{}>", c, 'z'); let t = c.to_string(); println!("{} {} {}", s, s.len(), t); }
