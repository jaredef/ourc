struct Pair<T> { a: T, b: T }
impl Pair<i32> {
    fn new(a: i32, b: i32) -> Pair<i32> { Pair { a: a, b: b } }
    fn sum(&self) -> i32 { self.a + self.b }
    fn max(&self) -> i32 { if self.a > self.b { self.a } else { self.b } }
}
fn main() {
    let p = Pair::new(3, 7);
    println!("sum {} max {}", p.sum(), p.max());
    let q = Pair { a: 100, b: 20 };
    println!("q sum {}", q.sum());
}
