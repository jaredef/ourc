struct W<T: ?Sized> { v: Box<T> }
fn main() {
    let w = W { v: Box::new(4) };
    println!("{}", w.v);
}
