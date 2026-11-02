// R2a P6 guard (accept side of E0451): public fields (`pub`, `pub(crate)`) are constructible from outside their module; a
// private field is fine inside its module and its descendants, and readable through the module's own fns.
mod m {
    pub struct S { pub x: i32, y: i32 }
    pub struct T { pub(crate) a: i32, pub b: i32 }
    pub fn mk() -> S { S { x: 1, y: 2 } }
    pub fn gety(s: &S) -> i32 { s.y }
    pub mod inner { pub fn mk2() -> super::S { super::S { x: 3, y: 4 } } }
}
fn main() {
    let s = m::mk();
    let t = m::T { a: 5, b: 6 };
    let u = m::inner::mk2();
    println!("{} {} {} {} {}", s.x, m::gety(&s), t.a + t.b, u.x, m::gety(&u));
}
// (read side, accepted: a method of the struct and a child module read the private field -- see rd2 / get below)
mod n {
    pub struct T { y: i32 }
    impl T { pub fn new() -> T { T { y: 9 } } pub fn get(&self) -> i32 { self.y } }
    pub mod sub { pub fn rd2(t: &super::T) -> i32 { t.y } }
}
#[allow(dead_code)]
fn read_side() -> i32 { let t = n::T::new(); t.get() + n::sub::rd2(&t) }
