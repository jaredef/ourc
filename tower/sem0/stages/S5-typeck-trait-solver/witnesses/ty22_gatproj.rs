trait Ct { type Item<X>; fn get<X>(&self, x: X) -> Self::Item<X>; }
fn f<T>(t: T, x: i32) -> T::Item<i32> where T: Ct { t.get(x) }
fn main() {}
