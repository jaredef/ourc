struct Rect { w: i32, h: i32 }
impl Rect {
    fn square(s: i32) -> Rect { Rect { w: s, h: s } }
    fn new(w: i32, h: i32) -> Rect { Rect { w: w, h: h } }
    fn area(&self) -> i32 { self.w * self.h }
    fn perimeter(&self) -> i32 { 2 * (self.w + self.h) }
}
fn main() {
    let r = Rect::new(3, 4);
    let sq = Rect::square(5);
    println!("area {} perim {}", r.area(), r.perimeter());
    println!("sq area {}", sq.area());
    let mut total = 0;
    for i in 1..=4 { let s = Rect::square(i); total = total + s.area(); }
    println!("sum of square areas 1..4 = {}", total);
}
