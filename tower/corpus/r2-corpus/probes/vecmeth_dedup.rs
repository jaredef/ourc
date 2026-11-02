fn main() {
    let mut v = vec![1, 1, 2, 3, 3, 3, 4];
    v.dedup();
    println!("{}", v.len());
}
