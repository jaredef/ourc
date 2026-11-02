macro_rules! impl_zeroable {
    ($($Inner:ident ( $prim:ty )),+ $(,)?) => {
        mod private {
            pub trait Sealed {}
            $(
                pub struct $Inner($prim);
            )+
        }
    };
}
impl_zeroable!(U8Inner(u8), U16Inner(u16), I8Inner(i8),);
fn main() {}
