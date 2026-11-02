struct Ctr { n: i64 }
impl Ctr { fn inc(&mut self) { self.n += 1; } }
fn main() {
    let mut c = Ctr { n: 0 };
    c.inc(); c.inc(); c.inc();
    println!("{}", c.n);
}
