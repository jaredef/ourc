fn mk() -> String {
    let mut s = String::new();
    s.push_str("zq");
    s
}
fn main() {
    let d = mk();
    println!("{}", d);
    println!("d={} n={}", d, 7);
}
