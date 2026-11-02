fn main() { let x = 1; let y = { let x = x + 10; x * 2 }; let x = "s"; println!("{} {}", x, y); }
