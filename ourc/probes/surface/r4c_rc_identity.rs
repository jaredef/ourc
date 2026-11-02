use std::cell::RefCell;
use std::rc::Rc;
struct D(i32);
impl Drop for D {
    fn drop(&mut self) { println!("drop {}", self.0); }
}
fn take(r: Rc<D>) { println!("in {} {}", r.0, Rc::strong_count(&r)); }
fn main() {
    let a = Rc::new(D(1));
    let b = Rc::clone(&a);
    println!("{} {}", Rc::strong_count(&a), b.0);
    drop(b);
    println!("{}", Rc::strong_count(&a));
    {
        let c = a.clone();
        let d = c.clone();
        println!("{}", Rc::strong_count(&d));
    }
    take(a.clone());
    let s = Rc::new(RefCell::new(vec![1]));
    let s2 = s.clone();
    s2.borrow_mut().push(2);
    println!("{:?} {}", s.borrow(), Rc::strong_count(&s));
    let k = Rc::new(42);
    let k2 = k.clone();
    println!("{} {}", k, k2);
    for i in 0..3 {
        let e = a.clone();
        if i == 1 { continue; }
    }
    println!("{}", Rc::strong_count(&a));
}
