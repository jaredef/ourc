pub fn h(x: u8) -> u8 {
    #[allow(unused)]
    macro_rules! chunk { ($t:ty) => { 0u8 }; }
    chunk!(u8);
    x
}
fn main() {}
