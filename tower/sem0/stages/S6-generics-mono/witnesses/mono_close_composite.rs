struct W<T> { v: T }
impl<T> W<T> { fn get(&self) -> i32 { 1 } }
fn id<T>(x: T) -> T { x }
fn wrap<T>(x: T) -> T { id(x) }
fn arr<const N: usize>() -> usize { N }
fn compute() {
    let m: u8 = 3;
    let a = id(5);
    let b = wrap(m);
    let w: W<u8> = W { v: m };
    w.get();
    arr::<4>();
}
fn main() { compute(); }
