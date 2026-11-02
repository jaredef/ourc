use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};
static HITS: AtomicUsize = AtomicUsize::new(0);
static N: AtomicI32 = AtomicI32::new(-5);
fn bump() { HITS.fetch_add(1, Ordering::SeqCst); }
fn main() {
    bump();
    bump();
    let old = HITS.fetch_add(5, Ordering::Relaxed);
    N.fetch_sub(10, Ordering::SeqCst);
    let a = AtomicI32::new(10);
    let p = a.fetch_sub(3, Ordering::SeqCst);
    let q = a.swap(100, Ordering::SeqCst);
    let f = AtomicBool::new(false);
    f.store(true, Ordering::SeqCst);
    println!("{} {} {} {} {} {} {}", old, HITS.load(Ordering::SeqCst), N.load(Ordering::SeqCst), p, q, a.load(Ordering::SeqCst), f.load(Ordering::SeqCst));
}
