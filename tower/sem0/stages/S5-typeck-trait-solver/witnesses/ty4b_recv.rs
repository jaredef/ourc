struct A { v: i32 }
impl A { fn r(&self) -> i32 { self.v } fn b(self) -> i32 { self.v } fn m(&mut self) -> i32 { self.v } }
fn main() {}
