fn main() {
    let mut sum: i64 = 0;
    let mut i: i64 = 1;
    while i <= 1000000 { sum = sum + i; i = i + 1; }
    println!("sum 1..1e6 = {}", sum);
}
