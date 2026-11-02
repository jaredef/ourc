fn main() { let mut c = 0; 'outer: for i in 0..5 { for j in 0..5 { if i * j > 6 { break 'outer; } c += 1; } } println!("{}", c); }
