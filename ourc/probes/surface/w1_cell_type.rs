use std::cell::RefCell;
use std::sync::Mutex;
struct S { v: RefCell<Vec<i32>> }
fn f(x: RefCell<String>) -> String { x.borrow().clone() }
fn bump(c: &RefCell<i32>) { *c.borrow_mut() += 5; }
fn g(m: &Mutex<i32>) -> i32 { *m.lock().unwrap() + 1 }
fn main() {
    let c = RefCell::new(1);
    bump(&c);
    let s = S { v: RefCell::new(vec![1]) };
    s.v.borrow_mut().push(2);
    let m = Mutex::new(4);
    println!("{} {} {:?} {}", f(RefCell::new(String::from("data"))), c.borrow(), s.v.borrow(), g(&m));
}
