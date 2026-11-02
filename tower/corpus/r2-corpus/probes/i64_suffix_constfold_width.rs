// P-TY-LITERAL-DEFAULT: a const-foldable multiply with an i64/u64 suffix (or a >i32 literal) must fold in
// 64-bit, not trip the i32 const-overflow guard. i32 overflow still rejects (rustc parity).
fn main() {
    let a = 1000000i64 * 1000000;   // 1e12, was FE-reject
    let b = 1000000u64 * 1000000;
    let c: i64 = 5_000_000_000;     // > i32::MAX unsuffixed
    println!("{} {} {}", a, b, c);
}
