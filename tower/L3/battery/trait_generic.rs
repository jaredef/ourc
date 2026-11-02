trait Area { fn area(&self) -> i32; }
struct Sq { s: i32 }
struct Rect { w: i32, h: i32 }
impl Area for Sq { fn area(&self) -> i32 { self.s * self.s } }
impl Area for Rect { fn area(&self) -> i32 { self.w * self.h } }
fn total<T: Area>(a: &T) -> i32 { a.area() }
fn main() { let s = Sq { s: 5 }; let r = Rect { w: 3, h: 4 }; std::process::exit(total(&s) + total(&r)); }  // 25 + 12 = 37
