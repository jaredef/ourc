struct C { n: i64 } impl C { fn inc(&mut self) -> &mut Self { self.n += 1; self } fn get(&self) -> i64 { self.n } } fn main() { let mut c = C { n: 0 }; c.inc().inc(); println!("{}", c.get()); }
