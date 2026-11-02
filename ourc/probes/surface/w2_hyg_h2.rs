macro_rules! m { ($e:expr) => {{ let mut s = 0; for i in 0..3 { s = s + i + $e; } s }} } fn main(){ let i = 100; println!("{}", m!(i)); }
