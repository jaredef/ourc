macro_rules! first_then { ($($a:tt)* ; $b:expr) => { $b * 2 }; } fn main() { println!("{}", first_then!(x y z ; 21)); }
