struct S { v: i32 }
impl S { fn val(&self) -> i32 { self.v } }
fn main() { let b = Box::new(S { v: 7 }); println!("{}", b.val()); let c: Box<S> = Box::new(S { v: 8 }); println!("{}", c.val()); }
