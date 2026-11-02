pub mod impls {
    pub trait Hash { fn hash(&self); }
    macro_rules! impl_write {
        ($(($ty:ident, $meth:ident),)*) => {$(
            #[stable(feature = "x", since = "1.0.0")]
            impl Hash for $ty {
                fn hash(&self) {}
                fn hash_slice(data: &[$ty]) {}
            }
        )*}
    }
    impl_write! { (u8, wu8), (u16, wu16), (u32, wu32), }
    impl Hash for bool { fn hash(&self) {} }
}
fn main() {}
