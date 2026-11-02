fn main(){ let mut a = [1, 2, 3]; { let e = &mut a[2]; *e += 7; } println!("{}", a[2]); }
