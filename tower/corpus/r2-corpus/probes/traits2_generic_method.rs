trait T { fn f<U>(&self, u: U) -> i32; }
struct S { x: i32 }
impl T for S { fn f<U>(&self, _u: U) -> i32 { self.x } }
fn main() {
    let s = S { x: 7 };
    println!("{}", s.f(99i32));
}
