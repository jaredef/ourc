use std::ptr;
fn main() {
    let a = 0;
    let b = 0;
    let pa = &a as *const i32;
    let pb = &b as *const i32;
    let z: *const u8 = ptr::null();
    let x = 1;
    let p = ptr::from_ref(&x);
    let mut y = 1;
    let w: *mut i32 = &mut y;
    unsafe { *w += 41; };
    let mut u = 1;
    let r: &mut i32 = &mut u;
    *r += 1;
    let mut v = vec![1];
    let wv: &mut Vec<i32> = &mut v;
    wv.push(2);
    println!("{} {} {} {} {} {} {} {} {:?}", ptr::eq(pa, pb), ptr::eq(pa, &a), pa.is_null(), z.is_null(), p.is_null(), unsafe { *p }, y, u, v);
}
