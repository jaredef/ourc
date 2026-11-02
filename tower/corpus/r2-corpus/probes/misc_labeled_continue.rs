fn main() { let mut s = 0; 'o: for i in 0..3 { for j in 0..3 { if j == 1 { continue 'o; } s += i * 10 + j; } } println!("{}", s); }
