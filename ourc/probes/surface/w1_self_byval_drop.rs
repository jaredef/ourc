struct S { x: i32 }
impl Drop for S {
    fn drop(&mut self) { println!("bye {}", self.x); }
}
impl S {
    fn take(self) -> i32 { self.x }
    fn pass(self) -> S { self }
    fn peek(&self) -> i32 { self.x }
}
trait T { fn eat(self) -> i32; }
impl T for S { fn eat(self) -> i32 { self.x + 100 } }
fn main() {
    let c = S { x: 20 };
    println!("{}", c.take());
    let d = S { x: 30 };
    let e = d.pass();
    println!("{}", e.peek());
    let f = S { x: 40 };
    println!("{}", f.eat());
    println!("end");
}
