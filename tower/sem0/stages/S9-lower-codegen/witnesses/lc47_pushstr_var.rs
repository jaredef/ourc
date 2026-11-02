fn main() {
    let mut a = String::new();
    a.push_str("hello");
    let mut b = String::new();
    b.push_str("world");
    a.push_str(&b);
    println!("{}", a);
}
