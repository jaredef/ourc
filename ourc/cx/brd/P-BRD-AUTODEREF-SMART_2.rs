use std::rc::Rc;
struct S(i32);
impl S { fn val(&self) -> i32 { self.0 } }
fn main() { let b = Box::new(Rc::new(S(7))); println!("{}", b.val()); }
