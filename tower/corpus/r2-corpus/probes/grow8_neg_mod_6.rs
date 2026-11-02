fn main() {
    let a: i64 = -7;
    let b: i64 = 3;
    println!("{} {}", a % b, (-a) % b);
    println!("{}", a % (-b));
}
