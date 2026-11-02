fn main() { let n = 7; let s = match n { 0 => 0, x if x % 2 == 1 => 1, _ => 2 }; println!("{}", s); }
