fn main() { let a = vec![10, 20, 30]; let b = vec![1, 2, 3]; let mut s = 0; for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() { s += i as i64 * (x + y); } println!("{}", s); }
