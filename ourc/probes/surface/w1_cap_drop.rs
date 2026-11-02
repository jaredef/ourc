#[derive(Debug)]
struct D(&'static str);
impl Drop for D {
    fn drop(&mut self) { println!("drop {}", self.0); }
}
#[derive(Debug)]
struct A { x: D, y: D }
struct W { d: D }
fn main() {
    let a = A { x: D("a.x"), y: D("a.y") };
    let c = move || println!("{:?} {:?}", a.y, a.x);
    c();
    let s = D("s");
    let t = D("t");
    let u = move || println!("{} {}", t.0, s.0);
    u();
    let w = W { d: D("w") };
    println!("end");
}
