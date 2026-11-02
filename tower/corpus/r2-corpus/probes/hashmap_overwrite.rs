use std::collections::HashMap;
fn main() {
    let mut m: HashMap<i32, i32> = HashMap::new();
    m.insert(1, 10);
    let old = m.insert(1, 99);
    println!("{}", old.unwrap_or(-1) + m[&1]);
}
