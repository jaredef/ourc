trait H { fn hi(&self) -> i32; }
struct S<T> { x: T }
impl<T> ::std::ops::Drop for S<T> {
    fn drop(&mut self) { println!("bye"); }
}
struct P<T: Copy> { v: T }
impl<T: Copy> Drop for P<T> {
    fn drop(&mut self) { println!("p bye"); }
}
impl<T> H for S<T> { fn hi(&self) -> i32 { 7 } }
fn main() {
    let a = S { x: 1 };
    {
        let _p = P { v: 2 };
        println!("inner");
    }
    let h = S { x: 0 }.hi();
    println!("{}", h);
    let b = S { x: String::from("s") };
    drop(b);
    println!("end");
}
