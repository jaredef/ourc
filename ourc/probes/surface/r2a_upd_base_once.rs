// R2a P5 guard: a struct update from a VARIABLE base moves the base's unwritten Drop field into the new value -- it drops
// ONCE (through the new value); ourc dropped it twice (the base's scope-end drop ran too). Also the no-field `S { ..a }`.
struct D(&'static str);
impl Drop for D {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}
struct W {
    d: D,
    x: i32,
}
fn main() {
    let a = W { d: D("a"), x: 1 };
    let b = W { x: 2, ..a };
    let c = W { d: D("c"), x: 3 };
    let e = W { ..c };
    println!("end {} {}", b.x, e.x);
}
