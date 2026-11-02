fn main() {
    let mut s = String::new(); s.push_str("hello world");
    let a = &s[0..5];
    let b = &s[6..];
    println!("{} {} {}", a.len(), b.len(), b);
}
