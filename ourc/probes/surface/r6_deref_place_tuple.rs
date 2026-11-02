fn main(){ let mut t = (3, 2); { let r = &mut t.1; *r -= 2; } println!("{}", t.1); }
