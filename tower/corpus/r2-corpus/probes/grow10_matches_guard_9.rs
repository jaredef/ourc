fn main() {
    let x = 7;
    let a = matches!(x, n if n > 5);
    let b = matches!(x, 0..=5);
    println!("{} {}", a, b);
}
