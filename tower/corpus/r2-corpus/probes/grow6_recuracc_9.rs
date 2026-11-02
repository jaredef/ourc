fn fact(n: i64, acc: i64) -> i64 {
    if n <= 1 { acc } else { fact(n - 1, acc * n) }
}
fn main() {
    println!("{}", fact(6, 1));
}
