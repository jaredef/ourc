use std::cell::Cell;
struct R<'a> { i: &'a Cell<bool> }
impl<'a> Drop for R<'a> {
    fn drop(&mut self) { self.i.set(false); println!("drop r"); }
}
enum Op<T> { No, So(T) }
struct P(i32);
impl Drop for P {
    fn drop(&mut self) { println!("dp{}", self.0); }
}
fn sink(_r: Op<P>) { }
fn sink_r(_r: Op<R>) { }
fn main() {
    sink(Op::So(P(1)));
    sink(Op::No);
    let c = &Cell::new(true);
    sink_r(Op::So(R { i: c }));
    println!("{}", c.get());
}
