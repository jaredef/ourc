fn main() { let mut s = 0; { let mut a = || s += 10; a(); } { let mut b = || s += 5; b(); } println!("{}", s); }
