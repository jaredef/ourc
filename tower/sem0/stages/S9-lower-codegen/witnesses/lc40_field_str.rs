struct N { s: String, k: i32 }
fn main() {
    let mut a = N { s: String::new(), k: 9 };
    a.s.push_str("hey");
    println!("{}", a.s);
    println!("s={} k={}", a.s, a.k);
}
