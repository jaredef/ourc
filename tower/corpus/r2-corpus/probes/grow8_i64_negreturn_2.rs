fn neg() -> i64 { -7 }
fn main() {
    let x = neg();
    let y: i64 = 1_000_000_000_000 + x;
    println!("{} {}", x, y);
}
