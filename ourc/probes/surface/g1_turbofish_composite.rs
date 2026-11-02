// G1 G-P2 guard: turbofish-only instances of COMPOSITE types are keyed losslessly -- ourc's spelling merged `(u8, u32)` and
// `(u64, u64)` into one `tup2$int$int` instance (a wrong size for one of them) and `[u16; 3]` into `arr`
fn sz<T>() -> usize {
    std::mem::size_of::<T>()
}
fn main() {
    println!("{} {} {} {}", sz::<(u8, u32)>(), sz::<(u64, u64)>(), sz::<[u16; 3]>(), sz::<char>());
}
