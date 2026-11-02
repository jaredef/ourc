// RUSTC-TARGET S8 R1 (P-STD-RC-ARC, census L4): Rc/Arc reference-counted pointers. Modelled as
// TRANSPARENT value semantics (correct for immutable Rc, the common case; shared-mutation Rc<RefCell>
// is a separate pipeline). new / clone / from / .clone() over scalar / String / struct / Vec.
use std::rc::Rc;
use std::sync::Arc;
struct P { x: i32 }
fn main() {
    let a = Rc::new(5);
    let b = Rc::clone(&a);
    let s = Rc::new(String::from("hi"));
    let t = s.clone();
    let p = Rc::new(P { x: 7 });
    let v = Arc::new(vec![1, 2, 3]);
    println!("{} {} {} {} {} {}", a, b, s, t, p.x, v.len());
}
