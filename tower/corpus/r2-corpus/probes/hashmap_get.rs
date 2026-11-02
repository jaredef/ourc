use std::collections::HashMap;
fn main() {
    let mut m: HashMap<i32, i32> = HashMap::new();
    m.insert(2, 20);
    let v = m.get(&2).copied().unwrap_or(-1);
    println!("{}", v);
}
