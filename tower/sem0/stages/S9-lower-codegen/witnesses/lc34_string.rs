fn main() {
    let mut s: String = String::new();
    s.push_str("ab");
    s.push_str("cde");
    let n: i32 = s.len() as i32;
    println!("{}", n);
}
