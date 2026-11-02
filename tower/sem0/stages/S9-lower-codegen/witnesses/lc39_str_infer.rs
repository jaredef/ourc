fn main() {
    let mut s = String::new();
    s.push_str("abc");
    println!("{}", s);
    let mut t = String::new();
    t.push_str("X");
    println!("s={} t={}", s, t);
    let n = 7;
    println!("n={} s={}", n, s);
}
