// CONSTEVAL R2 witnesses: provable const i32 overflow REJECTS; but in-range i32 arithmetic,
// i64-typed arithmetic that exceeds i32 (via `as i64` or i64 operands), and runtime/mut/wrapping
// operands must ALL still compile+run. rustc accepts all the non-rejected forms.
fn main() {
    let a: i32 = 2000000000;
    let b: i32 = 100;
    let x: i64 = 3000000000;   // fits i64
    let c: i64 = (a as i64) * (b as i64);   // 2e11, legal i64
    let mut m: i32 = 2147483647; m = m - 1;  // mut -> not const-tracked
    println!("{}|{}|{}|{}", a - b, x + 2, c, m);
}
