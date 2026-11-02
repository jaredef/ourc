use std::thread;
fn main() { let h: Vec<_> = (0..3).map(|i| thread::spawn(move || i * 10)).collect(); let s: u32 = h.into_iter().map(|j| j.join().unwrap()).sum(); println!("{}", s); }
