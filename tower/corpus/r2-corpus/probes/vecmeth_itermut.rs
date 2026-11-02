fn main() {
    let mut v = vec![1, 2, 3];
    for x in v.iter_mut() { *x *= 10; }
    let s: i64 = v.iter().sum();
    println!("{}", s);
}
