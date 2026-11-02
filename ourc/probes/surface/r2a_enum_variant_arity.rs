// R2a P4 guard: a variant NAME shared by enums in two modules (`b::E::If(..)` vs `a::T::If`) -- if-let and match on the
// payload-carrying one dispatch on the tag (they ran the arm unconditionally); the unit-variant enum is unaffected.
mod a { pub enum T { If, Num(i64) } }
mod b { pub enum E { I(i64), If(Box<E>, Box<E>, Box<E>, u32, u32) } }
use a::*;
use b::*;
fn walk(el: &E) -> i64 { let mut n = 1; if let E::If(_c2, _t2, e2, _a, _b) = el { n = n + walk(e2); } n }
fn g(t: &T) -> i64 { match t { T::If => 100, T::Num(n) => *n } }
fn main() {
    let x = E::If(Box::new(E::I(1)), Box::new(E::I(2)), Box::new(E::If(Box::new(E::I(0)), Box::new(E::I(0)), Box::new(E::I(3)), 0, 0)), 0, 0);
    println!("{} {} {} {}", walk(&x), walk(&E::I(5)), g(&T::If), g(&T::Num(7)));
}
