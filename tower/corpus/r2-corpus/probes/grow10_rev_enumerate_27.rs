fn main() {
    let v = vec![10,20,30];
    let mut out = String::new();
    for (i, x) in v.iter().rev().enumerate() {
        out.push_str(&format!("{}:{} ", i, x));
    }
    println!("{}", out.trim());
}
