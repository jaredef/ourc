fn call_fn<F: Fn()>(f: F) { f() }
fn call_fn_mut<F: FnMut()>(mut f: F) { f() }
fn call_fn_once<F: FnOnce()>(f: F) { f() }
fn main() {
    let mut x = 0_usize;
    let y = 2_usize;
    call_fn(|| println!("r{}", x));
    call_fn_mut(|| x += y);
    call_fn_once(|| x += y);
    let mut z = 1;
    let r = || z * 10;
    println!("{} {}", x, r());
    let mut w = || z += 5;
    w();
    let m = move || z * 100;
    z += 1;
    println!("{} {}", z, m());
}
