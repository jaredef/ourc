fn main() {
    let s: String = String::new();
    let mut t: String = s;
    t.push_str("hi");
    t.push_str("!");
    println!("{}", t);
    println!("msg = {} end", t);
    let n: i32 = 5;
    println!("n={} t={}", n, t);
}
