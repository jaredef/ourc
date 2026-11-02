use std::collections::HashMap;
fn main() {
    let mut m: HashMap<i32, i32> = HashMap::new();
    *m.entry(1).or_insert(0) += 5;
    *m.entry(1).or_insert(0) += 3;
    println!("{}", m[&1]);
}
