use std::collections::HashMap;
fn main() {
    let words = ["apple", "app", "apply", "banana", "band"];
    // count words by prefix of length 3
    let mut counts: HashMap<String, i64> = HashMap::new();
    for w in words.iter() {
        if w.len() >= 3 {
            let p = &w[0..3];
            *counts.entry(p.to_string()).or_insert(0) += 1;
        }
    }
    let app = *counts.get("app").unwrap();
    let ban = *counts.get("ban").unwrap();
    println!("{} {}", app, ban);
}
