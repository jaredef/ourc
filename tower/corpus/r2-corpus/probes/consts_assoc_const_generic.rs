trait Bound { const MAX: i64; }
struct A; struct B;
impl Bound for A { const MAX: i64 = 11; }
impl Bound for B { const MAX: i64 = 22; }
fn m<T: Bound>() -> i64 { T::MAX }
fn main() { println!("{}", m::<A>() + m::<B>()); }
