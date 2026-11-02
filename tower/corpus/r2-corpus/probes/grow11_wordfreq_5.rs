use std::collections::HashMap;
fn main() {
    let text = "the cat sat the mat the cat";
    let mut freq: HashMap<String, i64> = HashMap::new();
    for w in text.split_whitespace() {
        *freq.entry(w.to_string()).or_insert(0) += 1;
    }
    let the = *freq.get("the").unwrap();
    let cat = *freq.get("cat").unwrap();
    println!("{}", the * 100 + cat);
}
