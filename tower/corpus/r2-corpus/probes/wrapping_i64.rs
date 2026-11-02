// i64 wrapping_mul/add/sub -- regression guard for TI3e-p (N-wrapping-i64-width):
// the wrapping ops emitted a plain 32-bit op regardless of receiver width, so an
// i64 checksum/hash truncated to 32 bits. Now an i64 receiver routes through the
// 64-bit i64op.
fn main() {
    let a: i64 = 1_000_000_000;
    println!("{}", a.wrapping_mul(1_000_000_000));
    let mut h: i64 = 5381;
    let data = [1i64, 2, 3, 4, 5, 6, 7, 8];
    let mut i = 0;
    while i < data.len() {
        h = h.wrapping_mul(33).wrapping_add(data[i]);
        i += 1;
    }
    println!("{}", h);
    let s: i64 = 5_000_000_000;
    println!("{}", s.wrapping_add(10));
    // u8/u32 wrapping unchanged
    let b: u8 = 250;
    println!("{}", b.wrapping_add(10));
}
