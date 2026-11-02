struct Pair<T> { a: T, b: T } impl<T: PartialOrd + Copy> Pair<T> { fn max(&self) -> T { if self.a > self.b { self.a } else { self.b } } }
fn main() { let p = Pair { a: 3, b: 9 }; let q = Pair { a: 2.5, b: 1.5 }; println!("{} {}", p.max(), q.max()); }
