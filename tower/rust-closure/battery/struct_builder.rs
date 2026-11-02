struct Counter { n: i32 }
impl Counter {
    fn new() -> Counter { Counter { n: 0 } }
    fn bump(&self, k: i32) -> Counter { Counter { n: self.n + k } }
}
fn main() {
    let a = Counter::new();
    let b = a.bump(5);
    let c = b.bump(10);
    println!("{}", c.n);
}
