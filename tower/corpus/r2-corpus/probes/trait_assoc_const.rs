// P-TRAIT R3: trait assoc const via a generic -- turbofish monomorph on a USER type
// (m::<A>() -> m_A), a trait-const registry, and T::CONST resolution (T=the substituted type).
trait Bound { const MAX: i64; }
struct A; struct B;
impl Bound for A { const MAX: i64 = 11; }
impl Bound for B { const MAX: i64 = 22; }
fn m<T: Bound>() -> i64 { T::MAX }
fn main() {
    println!("{}", m::<A>() + m::<B>());   // 33
    println!("{}", A::MAX);                 // 11 (direct)
}
