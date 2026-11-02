// tractable: by-value `mut self` builder + chained `Type::new().add(..).add(..)`.
struct B { n: i64 }
impl B {
    fn new() -> B { B { n: 0 } }
    fn add(mut self, k: i64) -> Self { self.n += k; self }
}
fn main() {
    let b = B::new().add(5).add(10);
    println!("{}", b.n);                 // 15
    println!("{}", B::new().add(3).add(4).n); // 7 (chained inline)
}
