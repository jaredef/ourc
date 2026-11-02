pub struct AsciiChar;
macro_rules! into_int_impl {
    ($($ty:ty)*) => {
        $(
            #[unstable(feature = "x", issue = "1")]
            impl From<AsciiChar> for $ty {
                fn from(chr: AsciiChar) -> $ty { 0 }
            }
        )*
    }
}
into_int_impl!(u8 u16 u32 u64 u128 char);
impl AsciiChar { pub fn z(self) {} }
fn main() {}
