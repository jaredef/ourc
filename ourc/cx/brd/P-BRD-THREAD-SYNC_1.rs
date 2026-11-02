use std::thread;
fn main() { let h = thread::spawn(move || 7 * 6); println!("{}", h.join().unwrap()); }
