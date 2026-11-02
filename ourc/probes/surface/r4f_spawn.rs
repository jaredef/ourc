use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    let h = thread::spawn(|| 40 + 2);
    println!("{}", h.join().unwrap());
    let c = Arc::new(Mutex::new(0));
    let mut hs = Vec::new();
    for _ in 0..3 {
        let c2 = c.clone();
        hs.push(thread::spawn(move || { *c2.lock().unwrap() += 1; }));
    }
    for h in hs { h.join().unwrap(); }
    println!("{}", *c.lock().unwrap());
    thread::spawn(|| println!("in")).join().unwrap();
    let ws: Vec<_> = (0..3).map(|i| thread::spawn(move || i * 10)).collect();
    let s: i32 = ws.into_iter().map(|w| w.join().unwrap()).sum();
    println!("{}", s);
}
