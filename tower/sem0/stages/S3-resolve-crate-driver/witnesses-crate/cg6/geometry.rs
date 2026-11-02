pub struct Pt { pub x: i32, pub y: i32 }
impl Pt { pub fn norm(&self) -> i32 { self.x * self.x + self.y * self.y } }
pub fn area(w: i32, h: i32) -> i32 { w * h }
