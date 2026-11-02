trait Producer {
    type Item;
    fn produce(&self) -> Self::Item;
}
struct IntProd;
impl Producer for IntProd { type Item = i32; fn produce(&self) -> i32 { 42 } }
fn run<P: Producer<Item = i32>>(p: &P) -> i32 { p.produce() + 1 }
fn main() {
    println!("{}", run(&IntProd));
}
