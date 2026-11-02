struct C { n: i32 }
impl C {
    fn get(&self) -> i32 { self.n }
    fn add(&self, k: i32) -> i32 { self.n + k }
}
fn main() {
    let c: C = C { n: 5 };
    let x: i32 = c.get() + c.add(3);
    println!("{}", x);
}
