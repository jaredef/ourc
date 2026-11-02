use std::collections::HashMap;
fn main() {
    let mut m: HashMap<i32, i32> = HashMap::new();
    m.insert(1, 0); m.insert(5, 0); m.insert(4, 0);
    let mut s = 0;
    for k in m.keys() { s += k; }
    println!("{}", s);
}
