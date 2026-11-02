fn main(){ let a = [1, 2, 3, 4]; let [p, .., q] = a; let [.., r] = a; let [s, t, ..] = a; println!("{} {} {} {} {}", p, q, r, s, t); }
