pub trait Enc {}
pub trait Dec {}
pub struct A;
pub struct B;
macro_rules! two_impls {
    ($($t:ident),*) => {
        $(
            impl Enc for $t {
                fn e(&self) {}
            }
            impl Dec for $t {
                fn d(&self) {}
            }
        )*
    }
}
two_impls!(A, B);
