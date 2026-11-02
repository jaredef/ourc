trait Make {
    type Out<X>;
    fn make(&self, x: i32) -> Self::Out<i32>;
}
struct Doubler;
impl Make for Doubler {
    type Out<X> = i32;
    fn make(&self, x: i32) -> Self::Out<i32> {
        x * 2
    }
}
fn main() {
    println!("{}", Doubler.make(21));
}
