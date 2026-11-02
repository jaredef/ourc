macro_rules! sum { ($($x:expr),*) => { 0 $(+ $x)* }; } fn main() { println!("{}", sum!(1, 2, 3, 4)); }
