use std::collections::HashMap;
fn main() {
    let mut m: HashMap<i32, i32> = HashMap::new();
    m.insert(1, 10); m.insert(2, 20); m.insert(3, 30);
    let mut s = 0;
    for v in m.values() { s += v; }
    println!("{}", s);
}
