pub const K: u32 = 5;
pub struct S { pub a: u32, b: bool }
pub enum E { A, B(u32) }
pub fn f(x: u32) -> u32 { let y: u32 = x + 1; y }
impl S { pub fn m(&self) -> u32 { 0 } }
