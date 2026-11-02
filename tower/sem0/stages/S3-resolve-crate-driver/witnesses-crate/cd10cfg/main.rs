pub struct A;
impl A { pub fn m(&self) {} }
#[cfg(any())]
impl A { pub fn gone(&self) {} }
#[cfg(any())]
pub struct Gone;
pub struct B;
impl B { pub fn n(&self) {} }
fn main() {}
