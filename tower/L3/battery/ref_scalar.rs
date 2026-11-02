fn main() { let mut x = 5; let p = &mut x; *p = *p + 20; std::process::exit(x); }   // 25
