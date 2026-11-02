trait Ct { type Item; fn get(&self) -> Self::Item; }
trait Mk {}
fn f<T>(t: T) -> T::Item where T: Ct, T: Mk { t.get() }
fn main() {}
