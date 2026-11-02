fn main(){ let x = (1, 2, 3, 4); let (p, .., r) = x; let (.., s) = x; let (a, b, ..) = x; println!("{} {} {} {} {}", p, r, s, a, b); }
