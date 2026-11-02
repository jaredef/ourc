struct S { x: i32 }
impl Drop for S {
    fn drop(&mut self) { println!("bye {}", self.x); }
}
impl S {
    fn hi(&self) -> i32 { self.x * 7 }
    fn bump(&mut self) { self.x += 1; }
    fn take(self) -> i32 { self.x }
}
struct G<T> { v: T }
impl<T> Drop for G<T> {
    fn drop(&mut self) { println!("g bye"); }
}
impl<T> G<T> {
    fn peek(&self) -> i32 { 3 }
}
fn main() {
    let a = S { x: 1 };
    println!("{}", a.hi());
    let mut b = S { x: 10 };
    b.bump();
    println!("{}", b.hi());


    let g = G { v: 5 };
    println!("{}", g.peek());
    println!("end");
}
