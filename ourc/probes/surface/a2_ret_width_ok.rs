fn f(n: u8) -> u8 {
    let a: u8 = n + 1;
    if a > 9 {
        return a;
    }
    a
}
fn main() {
    println!("{}", f(3));
}
