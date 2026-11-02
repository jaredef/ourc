fn main() { let mut n = 0; 'o: for i in 0..5 { for j in 0..5 { if j > i { continue 'o; } n += 1; } } println!("{}", n); }
