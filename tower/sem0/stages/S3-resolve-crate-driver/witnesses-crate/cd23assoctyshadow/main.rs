pub trait S { type T; fn m(&self); }
impl<T> S for Box<T> { type T = u8; fn m(&self) {} }
impl<U> S for Vec<U> { type T = u16; fn m(&self) {} }
impl S for i32 { type T = u32; fn m(&self) {} }
fn main() {}
