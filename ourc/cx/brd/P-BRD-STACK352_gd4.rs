#[derive(Debug, Clone, PartialEq)] struct W<T> { v: T, n: i32 }
fn main() { let a: W<u8> = W { v: 3, n: 1 }; println!("{:?}", a); let b = W { v: 4u8, n: 2 }; println!("{:?}", b); }
