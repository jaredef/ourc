fn main() {
    let mut p: i64 = 1;
    let mut i = 0;
    while i < 40 { println!("2^{} = {}", i, p); p = p * 2; i = i + 1; }
}
