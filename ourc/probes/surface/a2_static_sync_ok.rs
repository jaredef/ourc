use std::sync::Mutex;
static N: i32 = 3;
static M: Mutex<i32> = Mutex::new(5);
fn main() { println!("{} {}", N, *M.lock().unwrap()); }
