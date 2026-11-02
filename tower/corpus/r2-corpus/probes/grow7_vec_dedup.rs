fn main() {
    let mut v = vec![1, 1, 2, 3, 3, 3, 4, 4, 5];
    v.dedup();
    let mut s = String::new();
    for x in &v { s.push_str(&x.to_string()); }
    println!("{} {}", v.len(), s);
}
