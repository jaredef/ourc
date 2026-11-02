use std::sync::{Arc, Mutex};
fn main() { let m = Arc::new(Mutex::new(0)); let m2 = m.clone(); *m2.lock().unwrap() += 5; println!("{}", *m.lock().unwrap()); }
