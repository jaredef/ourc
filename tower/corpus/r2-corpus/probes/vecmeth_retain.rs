fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    v.retain(|&x| x % 2 == 0);
    let s: i64 = v.iter().sum();
    println!("{}", s);
}
