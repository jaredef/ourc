fn main() {
    let a = [5, 10, 15, 20];
    let mut s = 0;
    let mut i = 0;
    while i < 4 { s = s + a[i]; i = i + 1; }
    std::process::exit(s);   // 50
}
