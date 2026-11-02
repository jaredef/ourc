// G1 G-P3 guard: methods of a GENERIC impl over a std container (a blanket `impl<T: Clone> Tw for Vec<T>`, an unbounded
// `impl<T> N for Vec<T>`) are instanced per receiver -- the call named the generic method itself (`Vec$T$tw`), never emitted;
// a `Self` return is the receiver's type
trait Tw {
    fn tw(&self) -> Self;
}
impl<T: Clone> Tw for Vec<T> {
    fn tw(&self) -> Self {
        let mut v = self.clone();
        v.extend(self.clone());
        v
    }
}
trait N {
    fn n(&self) -> usize;
}
impl<T> N for Vec<T> {
    fn n(&self) -> usize {
        self.len() * 2
    }
}
fn main() {
    println!("{:?} {}", vec![1, 2].tw(), vec![1, 2, 3].n());
}
