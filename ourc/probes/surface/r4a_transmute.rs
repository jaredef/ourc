use std::mem;
fn main() {
    let bits: u64 = unsafe { mem::transmute(1.5f64) };
    let back: f64 = unsafe { mem::transmute(bits) };
    let a: [u8; 4] = unsafe { mem::transmute(0x01020304u32.to_le()) };
    let n: u32 = unsafe { mem::transmute([1u8, 2, 3, 4]) };
    let x: i32 = unsafe { mem::transmute(4294967295u32) };
    let y: u8 = unsafe { mem::transmute(-1i8) };
    let w: [u8; 8] = unsafe { mem::transmute(-2i64) };
    let t = unsafe { mem::transmute::<f64, i64>(-2.0) };
    let z: u32 = unsafe { mem::zeroed() };
    println!("{} {} {:?} {} {} {} {:?} {} {}", bits, back, a, n, x, y, w, t, z);
}
