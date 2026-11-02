use std::collections::HashMap;
fn main() {
    let m: HashMap<i32, i32> = HashMap::new();
    println!("{}", m.get(&99).copied().unwrap_or(-1));
}
