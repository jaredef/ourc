fn main() {
    let mut v: Vec<i64> = Vec::new();
    let a = v.is_empty();
    v.push(1);
    println!("{}", a as i64 + v.is_empty() as i64);
}
