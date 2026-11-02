trait Zero { fn zero() -> Self; }
impl Zero for i8 { fn zero() -> Self { 0 } }
impl Zero for u32 { fn zero() -> Self { 7 } }
fn zeros<T: Zero + std::fmt::Debug>(n: usize) -> Vec<T> { (0..n).map(|_| T::zero()).collect() }
fn main() { println!("{:?} {:?}", zeros::<i8>(2), zeros::<u32>(3)); }
