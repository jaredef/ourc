fn main() { let x = 5; let s = match x { 0..=3 => 1, 4..=7 => 2, _ => 3 }; println!("{}", s); }
