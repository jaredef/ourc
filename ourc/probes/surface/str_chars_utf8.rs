fn main() { let mut n = 0; for c in "héy".chars() { print!("[{}]", c); if c == 'é' { n += 1; } } println!(" {}", n);
  for (i, c) in "héy".chars().enumerate() { print!("{}{} ", i, c); }
  let s: String = "héllo".chars().rev().collect(); println!("{} {:?} {}", s, "héy".chars().nth(1), "héllo".chars().count()); }
