fn main() {
    let v: Vec<i64> = vec![7, 5, 3];
    let a = *v.first().unwrap();
    let b = v.first().copied().unwrap();
    println!("{}", a + b);
}
