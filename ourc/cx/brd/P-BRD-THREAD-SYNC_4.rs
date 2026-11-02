use std::sync::{Arc, Mutex};
fn main() { let m = Arc::new(Mutex::new(0)); let m2 = m.clone(); let f = move || { let mut g = m2.lock().unwrap(); *g += 5; }; f(); println!("{}", *m.lock().unwrap()); }
