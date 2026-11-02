use std::sync::{Arc, Mutex, RwLock};
fn main() {
    let m = Arc::new(Mutex::new(5));
    *m.lock().unwrap() += 1;
    let m2 = m.clone();
    *m2.lock().unwrap() += 10;
    println!("{} {}", *m.lock().unwrap(), Arc::strong_count(&m));
    let v = Mutex::new(vec![1]);
    v.lock().unwrap().push(2);
    let g = v.lock().unwrap();
    println!("{:?} {}", *g, g.len());
    let l = RwLock::new(3);
    *l.write().unwrap() = 9;
    println!("{}", *l.read().unwrap());
}
