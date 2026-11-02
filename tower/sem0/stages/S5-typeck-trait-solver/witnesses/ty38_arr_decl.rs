fn param(a: [u8; 3]) -> u8 { a[0] }
fn ret() -> [u8; 2] { [1, 2] }
fn letann() -> u8 { let x: [u8; 2] = [1, 2]; x[0] }
fn cg<const N: usize>(a: [u8; N]) -> u8 { 0 }
fn main() {}
