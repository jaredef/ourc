fn main() {
    let mut t = String::new();
    t.push_str("abc");
    let bb = t.as_bytes();
    let mut sum = 0;
    let mut i = 0;
    while i < bb.len() { sum = sum + (bb[i] as i32); i = i + 1; }
    println!("{}", sum);
}
