fn gcd(a: i64, b: i64) -> i64 { if b == 0 { a } else { gcd(b, a % b) } }
struct Rat { n: i64, d: i64 }
impl Rat {
    fn new(n: i64, d: i64) -> Rat {
        let g = gcd(if n < 0 { -n } else { n }, d);
        Rat { n: n / g, d: d / g }
    }
    fn add(&self, o: &Rat) -> Rat { Rat::new(self.n * o.d + o.n * self.d, self.d * o.d) }
    fn mul(&self, o: &Rat) -> Rat { Rat::new(self.n * o.n, self.d * o.d) }
}
fn main() {
    let a = Rat::new(1, 2);
    let b = Rat::new(1, 3);
    let s = a.add(&b);
    let m = a.mul(&b);
    println!("{}/{} {}/{}", s.n, s.d, m.n, m.d);
}
