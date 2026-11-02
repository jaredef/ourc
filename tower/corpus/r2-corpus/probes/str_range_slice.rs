use std::collections::HashMap;
fn main() {
    let w = "banana";
    let a = &w[0..3];
    let b = &w[2..5];
    let mut m: HashMap<String, i64> = HashMap::new();
    *m.entry(a.to_string()).or_insert(0) += 1;
    *m.entry(a.to_string()).or_insert(0) += 1;
    println!("{}|{}|{}|{}", a, b, a.len(), *m.get("ban").unwrap());
}
