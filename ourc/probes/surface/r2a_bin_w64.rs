// R2a P4 guard: integer arithmetic with a declared 64-bit operand keeps that width -- wrapping methods on a value derived from
// an i64 param (the omirtrans udiv64 shape), u64 arithmetic still unsigned; a narrow u8 wrap is untouched.
fn ud(a: i64, c: i64) -> i64 {
    let half = (a >> 1) & 9223372036854775807;
    let mut q = (half / c) << 1;
    let r = a.wrapping_sub(q.wrapping_mul(c));
    q = q + r;
    q
}
fn main() {
    let a: u64 = 18446744073709551615;
    let b = a / 2 + 1;
    let mut h: u64 = 5381;
    for c in "ab".chars() { h = h.wrapping_mul(257).wrapping_add(c as u64); }
    let x: u8 = 250;
    println!("{} {} {} {}", ud(100, 7), b, h, x.wrapping_add(10));
}
