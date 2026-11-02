struct Wrapper<T> { value: T, tag: i32 }
impl Wrapper<i32> {
    fn new(v: i32, t: i32) -> Wrapper<i32> { Wrapper { value: v, tag: t } }
    fn get(&self) -> i32 { self.value }
    fn combined(&self) -> i32 { self.value + self.tag * 1000 }
}
fn main() {
    let a = Wrapper::new(42, 7);
    let b = Wrapper::new(99, 3);
    println!("{} {}", a.get(), b.get());
    println!("{} {}", a.combined(), b.combined());
}
