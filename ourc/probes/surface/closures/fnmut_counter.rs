fn main() { let mut c = 0; let mut inc = || { c += 1; }; inc(); inc(); inc(); println!("{}", c); }
