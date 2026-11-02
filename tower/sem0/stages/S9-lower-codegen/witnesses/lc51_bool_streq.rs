fn streq(a: &str, b: &str) -> bool {
    let ab = a.as_bytes(); let bb = b.as_bytes();
    if ab.len() != bb.len() { return false; }
    let mut i = 0;
    while i < ab.len() { if ab[i] != bb[i] { return false; } i = i + 1; }
    true
}
fn main() {
    let mut a = String::new(); a.push_str("abc");
    let mut b = String::new(); b.push_str("abc");
    let mut c = String::new(); c.push_str("xyz");
    let e1 = if streq(&a, &b) { 1 } else { 0 };
    let e2 = if streq(&a, &c) { 1 } else { 0 };
    println!("{}", e1 * 10 + e2);
}
