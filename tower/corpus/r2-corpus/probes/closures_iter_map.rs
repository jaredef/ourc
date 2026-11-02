fn main() {
    let v = vec![1, 2, 3, 4];
    let t: i64 = v.iter().map(|x| x * x).sum();
    println!("{}", t);
}
