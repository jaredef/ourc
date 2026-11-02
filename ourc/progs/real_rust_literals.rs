// P-LEX: radix integer literals with digit separators and type suffixes, and unary minus.
fn main() {
    let a = 0xBEEF_isize;
    let b = 0o755_isize;
    let c = 0b10101_isize;
    let d = -0xBEEF_isize;
    let e = -5;
    assert_eq!(a, 48879);
    assert_eq!(b, 493);
    assert_eq!(c, 21);
    assert_eq!(d, -48879);
    println!("{} {} {} {} {}", a, b, c, d, e);
}
