macro_rules! atomic_int {
    ($cfg:meta, $stable:meta, $int_type:ident $atomic_type:ident) => {
        #[$cfg]
        pub struct $atomic_type { v: $int_type }
    };
}
atomic_int!(cfg(target_has_atomic = "8"), stable(feature = "x", since = "1.0.0"), i8 AtomicI8);
atomic_int!(cfg(target_has_atomic = "128"), stable(feature = "x", since = "1.0.0"), i128 AtomicI128);
fn main() {}
