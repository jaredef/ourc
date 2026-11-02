trait Foo { fn v(&self) -> i64; }
struct B;
impl Foo for B { fn v(&self) -> i64 { 21 } }
fn twice<T: Foo>(x: T) -> i64 { x.v() * 2 }
fn main() {
    let mut i = 0;
    let mut s = 0;
    loop { if i >= 3 { break; } s += 14; i += 1; }
    println!("{}", s);
    println!("{}", twice(B));
}
