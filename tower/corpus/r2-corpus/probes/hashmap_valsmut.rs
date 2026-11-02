use std::collections::HashMap;
fn main() {
    let mut m: HashMap<i32, i32> = HashMap::new();
    m.insert(1, 1); m.insert(2, 2);
    for v in m.values_mut() { *v *= 10; }
    println!("{}", m[&1] + m[&2]);
}
