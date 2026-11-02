fn main() { let n = 7; let s = match n { x @ 1..=9 => x * 10, _ => 0 }; println!("{}", s); }
