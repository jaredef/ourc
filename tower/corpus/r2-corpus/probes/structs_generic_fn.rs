struct Pair<T> { a: T, b: T }
impl<T: Copy> Pair<T> { fn first(&self) -> T { self.a } }
fn main() {
    let p = Pair { a: 8i64, b: 9i64 };
    println!("{}", p.first());
}
