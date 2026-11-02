fn main(){ let mut n = 4; { let m = &mut n; *m *= 3; } println!("{}", n); }
