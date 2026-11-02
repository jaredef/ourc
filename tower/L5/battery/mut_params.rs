fn addone(x: &mut i32) { *x = *x + 1; }
fn swap(a: &mut i32, b: &mut i32) { let t = *a; *a = *b; *b = t; }
fn divmod(n: i32, d: i32, q: &mut i32, r: &mut i32) { *q = n / d; *r = n % d; }
fn main() {
    let mut v = 10;
    addone(&mut v); addone(&mut v);
    println!("v = {}", v);
    let mut a = 3; let mut b = 7;
    swap(&mut a, &mut b);
    println!("a={} b={}", a, b);
    let mut q = 0; let mut r = 0;
    divmod(17, 5, &mut q, &mut r);
    println!("17/5 = {} rem {}", q, r);
}
