fn main() { let mut total = 0; for i in (0..10).rev().step_by(3) { total += i; } println!("{}", total); }
