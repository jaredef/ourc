fn main() {
    let a: i32 = 2_000_000_000;
    let b = a.saturating_add(1_000_000_000);
    let c: u8 = 250u8.saturating_add(20);
    println!("{} {}", b, c);
}
