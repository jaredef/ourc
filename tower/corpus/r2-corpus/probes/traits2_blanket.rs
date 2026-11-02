trait Greet { fn greet(&self) -> i32; }
trait Named { fn id(&self) -> i32; }
impl<T: Named> Greet for T { fn greet(&self) -> i32 { self.id() + 100 } }
struct S { n: i32 }
impl Named for S { fn id(&self) -> i32 { self.n } }
fn main() {
    let s = S { n: 5 };
    println!("{}", s.greet());
}
