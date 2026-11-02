fn main() {
    let mut n = 0;
    loop { n = n + 3; if n >= 20 { break; } }
    std::process::exit(n);   // 21
}
