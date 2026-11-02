trait G { fn name(&self) -> i64; fn twice(&self) -> i64 { self.name() * 2 } } struct A; impl G for A { fn name(&self) -> i64 { 21 } } fn main() { println!("{}", A.twice()); }
