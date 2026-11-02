trait Val { fn val(&self) -> i32; }
struct A { n: i32 }
struct B { n: i32 }
impl Val for A { fn val(&self) -> i32 { self.n + 1 } }
impl Val for B { fn val(&self) -> i32 { self.n * 2 } }
fn twice<T: Val>(x: &T) -> i32 { x.val() + x.val() }
fn maxi<T>(a: T, b: T, c: bool) -> T { if c { a } else { b } }
fn main() {
    let a = A { n: 10 };
    let b = B { n: 10 };
    let m = maxi(100, 7, false);
    std::process::exit(twice(&a) + twice(&b) + m);   // 22 + 40 + 7 = 69
}
