struct Pair<A, B> { a: A, b: B }
impl<A: Copy, B: Copy> Pair<A, B> {
    fn swap(&self) -> Pair<B, A> { Pair { a: self.b, b: self.a } }
    fn same(&self) -> Pair<A, B> { Pair { a: self.a, b: self.b } }
}
fn main() {
    let p = Pair { a: 1, b: 'x' };
    let q = p.swap();
    let r = q.swap();
    let s = p.same();
    println!("{} {} {} {} {} {}", q.a, q.b, r.a, r.b, s.a, s.b);
}
