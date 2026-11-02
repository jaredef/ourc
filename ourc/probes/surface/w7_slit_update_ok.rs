struct D { a: i32, b: i32 }
impl Drop for D {
    fn drop(&mut self) { println!("drop {} {}", self.a, self.b); }
}
#[derive(Debug)]
struct N { s: String, n: i32 }
fn main() {
    let x = N { s: String::from("hi"), n: 1 };
    let y = N { n: 2, ..x };
    println!("{:?}", y);
    let d0 = D { a: 1, b: 2 };
    let d1 = D { a: 3, ..d0 };
    println!("{} {}", d1.a, d1.b);
}
