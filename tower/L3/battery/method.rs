struct Point { x: i32, y: i32 }
impl Point { fn norm(&self) -> i32 { self.x + self.y } fn scaled(&self, k: i32) -> i32 { (self.x + self.y) * k } }
fn main() { let p = Point { x: 3, y: 4 }; std::process::exit(p.norm() + p.scaled(10)); }   // 7 + 70 = 77
