fn main(){ let mut n = 5; { let mut f = move || { n *= 2; n }; println!("{} {}", f(), f()); } let mut g = || { n += 1; }; g(); println!("{}", n); }
