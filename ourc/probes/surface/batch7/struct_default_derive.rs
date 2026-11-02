#[derive(Debug, Default, Clone, Copy, PartialEq)] struct Cfg { n: i64, on: bool } fn main() { let a = Cfg::default(); let b = Cfg { n: 5, ..a }; println!("{:?} {:?} {}", a, b, a == b); }
