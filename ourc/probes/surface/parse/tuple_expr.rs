fn main() { let t = (1, (2, 3)); let (a, (b, c)) = t; println!("{} {} {} {}", a, b, c, t.1 .0); }
