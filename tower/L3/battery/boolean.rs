fn main() {
    let a = 5;
    let b = 3;
    let mut r = 0;
    if a > b && b > 0 { r = 1; }
    if a < b || b == 3 { r = r + 10; }
    std::process::exit(r);   // 11
}
