fn main() {
    let mut a = [0; 5];
    let mut i: usize = 0;
    while i < 5 { a[i] = (i * i) as i32; i = i + 1; }
    std::process::exit((a[2] + a[3] + a[4]) as i32);   // 4 + 9 + 16 = 29
}
