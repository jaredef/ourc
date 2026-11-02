macro_rules! twice { ($e: expr) => { 'x: for _ in 0..2 { $e; if true { continue 'x; } } }; }
fn main() { let mut n = 0; twice!(n += 1); println!("{}", n); }
