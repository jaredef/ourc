// P-MONO: a bounded generic calling ANOTHER bounded generic. The demand for `inner$Z` only exists
// inside `outer`'s body, which the demand pre-pass must skip (its parameter types are not concrete),
// so the instance was never materialized and the call emitted an undefined name.
trait Mark { fn m(&self) -> isize; }
struct Z(isize);
impl Mark for Z { fn m(&self) -> isize { self.0 } }
fn inner<T: Mark>(a: T) -> isize { a.m() + 1 }
fn outer<T: Mark>(a: T) -> isize { inner(a) * 2 }
fn main() { println!("{}", outer(Z(4))); }
