fn main() {
    let mut f: i64 = 1;
    let mut n: i64 = 1;
    while n <= 20 {
        f *= n;
        n += 1;
    }
    println!("{}", f);
}
