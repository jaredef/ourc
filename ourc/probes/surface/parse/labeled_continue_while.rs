fn main() { let mut n = 0; let mut i = 0; 'o: while i < 5 { i += 1; let mut j = 0; while j < 5 { j += 1; if j > 2 { continue 'o; } n += 10; } } println!("{}", n); }
