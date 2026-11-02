use std::collections::HashSet;
fn main() {
    let mut s: HashSet<i32> = HashSet::new();
    s.insert(1); s.insert(2); s.insert(2); s.insert(3);
    println!("{}", s.len() + (s.contains(&2) as usize));
}
