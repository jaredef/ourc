#[derive(Debug, Clone, PartialEq, Default)] struct W<T> { inner: Vec<T>, tag: Option<T>, n: i32 }
fn main() { let w: W<u8> = W { inner: vec![1, 2], tag: Some(3), n: 1 }; let c = w.clone(); let d: W<u8> = W { inner: vec![1], tag: None, n: 1 }; println!("{} {} {}", w == c, w == d, w != d); }
