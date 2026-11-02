use std::collections::HashSet;
fn main() {
    let mut s: HashSet<i32> = HashSet::new();
    s.insert(10); s.insert(20); s.insert(10); s.insert(30);
    let mut t: HashSet<String> = HashSet::new();
    t.insert("x".to_string()); t.insert("x".to_string());
    println!("{}|{}|{}|{}", s.len(), s.contains(&20), s.contains(&99), t.len());
}
