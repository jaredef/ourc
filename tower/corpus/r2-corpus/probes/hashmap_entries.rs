use std::collections::HashMap;
fn main() {
    let mut m: HashMap<i32, i32> = HashMap::new();
    m.insert(1, 10); m.insert(2, 20);
    let mut s = 0;
    for (k, v) in &m { s += k * v; }
    println!("{}", s);
}
