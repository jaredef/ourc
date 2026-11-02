// P-TRAIT R5: a user `impl Iterator` driven through the adapter chain -- materialize by
// looping next() into a Vec, then dispatch the Vec adapters.
struct Counter { n: i64, max: i64 }
impl Iterator for Counter {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.n < self.max { self.n += 1; Some(self.n) } else { None }
    }
}
fn main() {
    let c = Counter { n: 0, max: 5 };
    println!("{}", c.sum::<i64>());                       // 1+2+3+4+5 = 15
    let d = Counter { n: 0, max: 6 };
    let s: i64 = d.filter(|x| x % 2 == 0).map(|x| x * x).sum();
    println!("{}", s);                                    // 4+16+36 = 56
}
