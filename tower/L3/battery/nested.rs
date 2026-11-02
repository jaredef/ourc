fn main() {
    let mut t = 0;
    let mut i = 0;
    while i < 5 { let mut j = 0; while j < 5 { t = t + 1; j = j + 1; } i = i + 1; }
    std::process::exit(t);   // 25
}
