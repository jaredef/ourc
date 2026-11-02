struct R { w: i64, h: i64 }
impl R { fn area(&self) -> i64 { self.w * self.h } }
fn main() {
    let r = R { w: 6, h: 7 };
    println!("{}", r.area());
}
