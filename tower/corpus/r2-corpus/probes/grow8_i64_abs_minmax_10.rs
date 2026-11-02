fn main() {
    let a: i64 = -5_000_000_000;
    let b: i64 = 3_000_000_000;
    println!("{} {} {}", a.abs(), a.min(b), a.max(b));
}
