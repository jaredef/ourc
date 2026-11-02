fn main() {
    let mut m = [0i64; 30];
    m[0] = 0; m[1] = 1;
    let mut i = 2;
    while i < 30 { m[i] = m[i-1] + m[i-2]; i += 1; }
    println!("{}", m[29]);
}
