use std::collections::HashMap;
fn main(){ let mut m:HashMap<i32,i32>=HashMap::new(); m.insert(1,10); m.insert(2,20); println!("{} {}",m.len(),m[&2]); }
