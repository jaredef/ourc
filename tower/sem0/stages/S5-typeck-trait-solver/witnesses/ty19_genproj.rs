trait Ct { type Item; fn get(&self) -> Self::Item; }
fn f<T: Ct>(t: T) -> T::Item { t.get() }
fn main() {}
