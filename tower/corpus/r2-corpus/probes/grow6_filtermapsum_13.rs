fn main() {
    let s: i64 = (1..=10).filter(|x| x % 2 == 0).map(|x| x * x).sum();
    println!("{}", s);
}
