fn main() { let x = 1; let x = x + 1; { let x = x * 10; println!("{}", x); } println!("{}", x); }
