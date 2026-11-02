use std::collections::HashMap;
fn main() { let m: HashMap<i64, i64> = (1..4).map(|x| (x, x * x)).collect(); println!("{}", m.len()); }
