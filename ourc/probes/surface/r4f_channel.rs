use std::sync::mpsc::{self, channel};
use std::thread;
fn main() {
    let (tx, rx) = channel();
    tx.send(5).unwrap();
    tx.send(6).unwrap();
    println!("{} {}", rx.recv().unwrap(), rx.recv().unwrap());
    let (t2, r2) = mpsc::channel();
    let h = thread::spawn(move || {
        for i in 0..3 { t2.send(i * 10).unwrap(); }
        7
    });
    let got: Vec<i32> = r2.iter().collect();
    println!("{:?} {}", got, h.join().unwrap());
    let (t3, r3) = channel();
    t3.send(9u8).unwrap();
    drop(t3);
    println!("{} {}", r3.recv().unwrap(), r3.recv().is_err());
}
