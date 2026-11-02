struct A { v: i32 }
impl A { fn get(&self) -> i32 { self.v } fn add(&self, k: i32) -> i32 { k } }
fn f(a: A) -> i32 { let x = a.get(); let y = a.add(3); x + y }
fn main() {}
