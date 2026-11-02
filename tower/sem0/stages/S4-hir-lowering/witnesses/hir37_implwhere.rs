struct S<T> { v: T }
impl<T: Clone> S<T> {
    fn get(&self) -> i32 { 0 }
}
trait Tr { fn t(&self) -> i32; }
impl<T: Clone> Tr for S<T> {
    fn t(&self) -> i32 { 1 }
}

fn main() {}
