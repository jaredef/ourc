use std::sync::atomic::{AtomicUsize, Ordering};
fn counter() -> usize { static C: AtomicUsize = AtomicUsize::new(0); C.fetch_add(1, Ordering::SeqCst) }
static G: AtomicUsize = AtomicUsize::new(5);
fn main() { println!("{} {} {} {}", counter(), counter(), G.fetch_add(1, Ordering::SeqCst), G.load(Ordering::SeqCst)); }
