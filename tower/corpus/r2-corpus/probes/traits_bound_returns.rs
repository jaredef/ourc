trait Doubler { fn d(&self) -> i64; }
struct N(i64);
impl Doubler for N { fn d(&self) -> i64 { self.0 * 2 } }
fn sum<T: Doubler>(a: T, b: T) -> i64 { a.d() + b.d() }
fn main() { println!("{}", sum(N(1), N(2))); }
