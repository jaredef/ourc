use std::collections::HashMap;
fn main() {
    let words = [1, 2, 2, 3, 3, 3];
    let mut m: HashMap<i32, i32> = HashMap::new();
    for &w in words.iter() { *m.entry(w).or_insert(0) += 1; }
    println!("{}", m[&3]);
}
