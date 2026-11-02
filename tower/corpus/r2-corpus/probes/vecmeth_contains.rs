fn main() {
    let v = vec![5, 10, 15];
    println!("{}", v.contains(&10) as i64 + v.contains(&99) as i64);
}
