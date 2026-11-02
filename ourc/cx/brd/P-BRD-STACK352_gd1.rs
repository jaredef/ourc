#[derive(Debug, Clone, Default, PartialEq)] struct Cfg<T> { name: String, items: Vec<T>, flag: bool, opt: Option<T> }
fn main() { let cfg: Cfg<u8> = Cfg::default(); println!("{:?}", cfg); }
