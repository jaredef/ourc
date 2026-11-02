use std::collections::HashMap;
fn main() {
    let mut m: HashMap<i32, i32> = HashMap::new();
    m.insert(1, 10);
    if let Some(v) = m.get_mut(&1) { *v += 7; }
    println!("{}", m[&1]);
}
