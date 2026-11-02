use std::collections::HashMap;
fn main() {
    let mut m: HashMap<&str, Vec<i32>> = HashMap::new();
    m.insert("k", vec![2, 4, 6]);
    let mut t = 0;
    for v in m.values() { for x in v { t += x; } }
    println!("{}", t);
}
