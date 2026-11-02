fn main() {
    let mut v = vec![1, 2];
    v.extend(vec![3, 4, 5]);
    let s: i64 = v.iter().sum();
    println!("{}", s);
}
