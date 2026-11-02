use std::sync::{Arc, Mutex};
use std::thread;
fn main() { let m = Arc::new(Mutex::new(0)); let m2 = m.clone(); thread::spawn(move || { *m2.lock().unwrap() += 5 }).join().unwrap(); println!("{}", *m.lock().unwrap()); }
