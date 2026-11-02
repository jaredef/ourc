use std::collections::HashMap;
fn main() {
    let mut m: HashMap<i32, i32> = HashMap::new();
    for i in 0..5 { m.insert(i, i * i); }
    let mut s = 0;
    for (_k, v) in &m { s += v; }
    println!("{}", s);
}
