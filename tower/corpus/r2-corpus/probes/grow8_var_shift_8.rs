fn main() {
    let mut acc: i64 = 0;
    let mut k = 0;
    while k < 10 {
        acc += 1i64 << k;
        k += 1;
    }
    println!("{}", acc);
}
