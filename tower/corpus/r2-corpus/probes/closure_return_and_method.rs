// P-CLOSURE-DYNFIELD rung 2 guard: a fn returning a boxed closure, and calling a
// generic Fn-bounded field through an impl method `(self.f)(x)`. Both hinge on the
// `->`-aware generic scan (return type + impl header) so `Box<dyn Fn(i64)->i64>` /
// `impl<F: Fn(i64)->i64>` don't miscount the inner `->`.
fn make(k: i64) -> Box<dyn Fn(i64) -> i64> { Box::new(move |x| x + k) }
struct Adder<F: Fn(i64) -> i64> { f: F }
impl<F: Fn(i64) -> i64> Adder<F> {
    fn apply(&self, x: i64) -> i64 { (self.f)(x) }
}
fn main() {
    let f = make(100);
    println!("{}", f(14));            // 114
    let a = Adder { f: |x| x + 10 };
    let mut acc = 0;
    for i in 0..5 { acc += a.apply(i); } // (10+11+12+13+14)=60
    println!("{}", acc);              // 60
}
