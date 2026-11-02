fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let d: i64 = v.drain(1..3).sum();
    println!("{}", d + v.len() as i64);
}
