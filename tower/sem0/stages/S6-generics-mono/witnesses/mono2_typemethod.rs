struct W<T> { v: T }
impl<T> W<T> { fn get(&self) -> i32 { 1 } }
fn f() { let a: W<u8> = W { v: 3 }; a.get(); let b: W<i32> = W { v: 5 }; b.get(); }
fn main() { f(); }
