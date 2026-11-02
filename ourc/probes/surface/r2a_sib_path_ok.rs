// R2a P6 SIB-PATH guard: the ways a sibling module IS in scope -- `use crate::a;` (two modules importing the same name
// is not an import cycle), `crate::` / `super::`, a crate-root glob, and the root itself.
mod a {
    pub fn f() -> i32 {
        1
    }
}
mod b {
    use crate::a;
    pub fn g() -> i32 {
        a::f() + crate::a::f() + super::a::f()
    }
}
mod c {
    use crate::*;
    use crate::a;
    pub fn h() -> i32 {
        a::f() + b::g()
    }
}
fn main() {
    println!("{} {}", c::h(), a::f());
}
