#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)] enum P { A, B }
fn main() { let mut v = vec![P::B, P::A, P::B]; v.sort(); println!("{:?}", v); }
