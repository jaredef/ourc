fn main() {
    let n = 21;
    let s = format!("n={} twice={}", n, n * 2);
    println!("{}", s);
    println!("{}", s.len());
    let t = 42.to_string();
    println!("{}", t);
    let u = "x".to_string();
    println!("{}", u);
}
