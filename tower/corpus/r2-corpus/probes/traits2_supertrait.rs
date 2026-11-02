trait A { fn a(&self) -> i32; }
trait B: A { fn b(&self) -> i32; }
struct S { x: i32 }
impl A for S { fn a(&self) -> i32 { self.x } }
impl B for S { fn b(&self) -> i32 { self.a() + 1 } }
fn main() {
    let s = S { x: 10 };
    println!("{}", s.b());
    println!("{}", s.a());
}
