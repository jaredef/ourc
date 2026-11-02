fn main() {
    let o: Option<Vec<i32>> = Some(vec![10, 20, 30]);
    let s: i32 = o.map(|v| v.iter().sum()).unwrap_or(0);
    println!("{}", s);
}
