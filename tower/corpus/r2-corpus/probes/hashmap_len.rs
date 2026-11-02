use std::collections::HashMap;
fn main() {
    let mut m: HashMap<i32, i32> = HashMap::new();
    m.insert(1, 1); m.insert(2, 2); m.insert(1, 3);
    println!("{}", m.len());
}
