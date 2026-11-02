fn main() { let mut n = 0; 'a: for i in 0..4 { 'b: for j in 0..4 { for k in 0..4 { n += 1; if k == 1 { continue; } if j == 2 { break 'b; } if i == 2 { break 'a; } } } } println!("{}", n); }
