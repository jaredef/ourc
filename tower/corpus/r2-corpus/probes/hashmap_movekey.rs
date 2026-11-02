use std::collections::HashMap;
fn main() {
    let mut m: HashMap<String, i32> = HashMap::new();
    m.insert(String::from("a"), 1);
    *m.entry(String::from("a")).or_insert(0) += 9;
    println!("{}", m["a"]);
}
