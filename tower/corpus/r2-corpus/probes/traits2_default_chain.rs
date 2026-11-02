trait Chain {
    fn base(&self) -> i32;
    fn mid(&self) -> i32 { self.base() + 1 }
    fn top(&self) -> i32 { self.mid() + 10 }
}
struct S { v: i32 }
impl Chain for S { fn base(&self) -> i32 { self.v } }
fn main() {
    let s = S { v: 3 };
    println!("{}", s.top());
}
