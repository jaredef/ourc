use std::collections::HashMap;
fn main() {
    let words = ["a", "b", "a", "c", "b", "a"];
    let mut m: HashMap<&str, i32> = HashMap::new();
    for w in words.iter() {
        *m.entry(w).or_insert(0) += 1;
    }
    println!("{} {} {}", m["a"], m["b"], m["c"]);
}
