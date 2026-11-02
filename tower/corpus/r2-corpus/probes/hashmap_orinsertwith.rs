use std::collections::HashMap;
fn main() {
    let mut m: HashMap<i32, i32> = HashMap::new();
    let v = m.entry(7).or_insert_with(|| 42);
    println!("{}", *v);
}
