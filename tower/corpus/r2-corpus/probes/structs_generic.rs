struct W<T> { v: T }
impl W<i64> { fn get(&self) -> i64 { self.v } }
fn main() {
    let w = W { v: 55 };
    println!("{}", w.get());
}
