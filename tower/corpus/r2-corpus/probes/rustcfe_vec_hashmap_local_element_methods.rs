use std::collections::HashMap;
fn main() { let mut v: Vec<HashMap<i32,u32>> = Vec::new(); let mut m = HashMap::new(); m.insert(1,100); v.push(m); v[0].insert(2,200); println!("{}", v[0].len()); }
