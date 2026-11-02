// P-BRD-AUTODEREF-SMART: a method call on a Box<S> / Box<Rc<S>> / Rc<S> local autoderefs to S's method.
use std::rc::Rc;
struct S { v: i32 }
impl S { fn val(&self) -> i32 { self.v } fn twice(&self) -> i32 { self.v * 2 } }
struct T(i32);
impl T { fn get(&self) -> i32 { self.0 + 1 } }
fn main() {
    let b = Box::new(S { v: 7 });
    let c: Box<T> = Box::new(T(8));
    let d = Box::new(Rc::new(S { v: 3 }));
    let e = Rc::new(S { v: 5 });
    println!("{} {} {} {} {}", b.val(), b.twice(), c.get(), d.val(), e.twice());
}
