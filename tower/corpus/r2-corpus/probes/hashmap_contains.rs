use std::collections::HashMap;
fn main() {
    let mut m: HashMap<i32, i32> = HashMap::new();
    m.insert(3, 1);
    println!("{}", m.contains_key(&3) as i32 + m.contains_key(&4) as i32);
}
