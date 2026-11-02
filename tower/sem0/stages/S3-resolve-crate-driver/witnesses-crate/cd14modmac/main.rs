pub mod dep {
    pub trait DepHash { fn hash(&self); }
    macro_rules! impl_dep_hash {
        ($($t:ty),+ $(,)?) => {$(
            impl DepHash for $t {
                fn hash(&self) {}
            }
        )+};
    }
    impl DepHash for Option<u8> { fn hash(&self) {} }
    impl_dep_hash!(bool, usize, u64);
    impl DepHash for u32 { fn hash(&self) {} }
}
fn main() {}
