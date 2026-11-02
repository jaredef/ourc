struct Adder<F: Fn(i64) -> i64> { f: F }
impl<F: Fn(i64) -> i64> Adder<F> {
    fn apply(&self, x: i64) -> i64 { (self.f)(x) }
}
fn main() {
    let base = 10;
    let a = Adder { f: |x| x + base };
    let mut acc = 0;
    for i in 0..5 { acc += a.apply(i); }
    println!("{}", acc);
}
