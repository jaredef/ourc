struct P { x: i32 }
impl P { fn get(&self) -> i32 { self.x } }
const LIM: u8 = 200 + 1;
const TAB: i32 = [10, 20, 30][2];
fn id<T>(v: T) -> T { v }
fn work() -> i32 {
    let mut n = 5;
    let r = &mut n;
    *r = 7;
    let s = &n;
    let _ = *s;
    let p = P { x: 3 };
    id(p.get())
}
fn main() { let _ = work(); }
