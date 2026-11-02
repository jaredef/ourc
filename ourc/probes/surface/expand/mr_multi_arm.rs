macro_rules! m { () => { 0 }; ($a:expr) => { $a }; ($a:expr, $b:expr) => { $a + $b }; } fn main() { println!("{} {} {}", m!(), m!(5), m!(2, 3)); }
