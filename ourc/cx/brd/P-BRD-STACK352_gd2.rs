#[derive(Debug, Clone, Default, PartialEq)] struct Cfg<T> { name: String, items: Vec<T>, flag: bool, opt: Option<T> }
fn main() { let cfg: Cfg<u8> = Cfg::default(); let mut c2 = cfg.clone(); c2.items.push(9); c2.opt = Some(1); println!("{:?} {}", c2, cfg == c2); }
