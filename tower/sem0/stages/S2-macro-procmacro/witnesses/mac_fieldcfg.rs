struct S { a: u32, #[cfg(any())] b: u8, c: u16 }
enum E { V, #[cfg(any())] Gone, W }
fn main() {}
