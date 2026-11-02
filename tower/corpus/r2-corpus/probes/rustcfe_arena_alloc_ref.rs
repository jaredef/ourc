// rustc's real arena shape: alloc returns &T; caller derefs the handle
struct Arena { last: i32 }
impl Arena {
    fn new() -> Arena { Arena { last: 0 } }
    fn alloc(&mut self, v: i32) -> &i32 { self.last = v; &self.last }
}
fn main() {
    let mut a = Arena::new();
    let r = a.alloc(5);
    println!("{}", *r);
}
