macro_rules! sum { ($($x:expr),*) => { 0 $(+ $x)* }; }
fn main() { let s = sum!(1, 2, 3); }
