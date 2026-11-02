fn counter<T>() -> usize { use std::sync::atomic::{AtomicUsize, Ordering}; static C: AtomicUsize = AtomicUsize::new(0); C.fetch_add(1, Ordering::SeqCst) }
fn main() { let a = (counter::<u8>(), counter::<u8>(), counter::<u16>()); println!("{:?}", a); }
