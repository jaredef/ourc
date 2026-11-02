fn main() {
    let mut x = 0;
    let mut i = 0;
    while i < 10 { x = x + i; i = i + 1; }
    std::process::exit(x);   // 45
}
