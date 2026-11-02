pub trait Tr<U> { fn m(&self, u: U) -> U; }
pub fn f<T: Tr<u8>>(t: &T, x: u8) -> u8 {
    <T as Tr<u8>>::m(t, *&x)
}
fn main() {}
