fn main() {
    let c: i64 = 3;
    let x = 2 * 2 * 3 * 3 * 5 * 5 * 7 * 7 * 11 * 11 * c;
    println!("{}", x.wrapping_mul(2));
}
