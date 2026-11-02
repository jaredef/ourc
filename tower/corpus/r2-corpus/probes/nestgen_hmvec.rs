use std::collections::HashMap;
fn main() {
    let mut m: HashMap<i32, Vec<i32>> = HashMap::new();
    m.entry(1).or_insert_with(Vec::new).push(5);
    m.entry(1).or_insert_with(Vec::new).push(7);
    let s: i32 = m[&1].iter().sum();
    println!("{}", s);
}
