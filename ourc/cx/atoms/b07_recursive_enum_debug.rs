#[derive(Debug, Clone, PartialEq)] enum T { Named(String, Vec<T>), Param(usize) }
fn main() { let t = T::Named("Vec".into(), vec![T::Named("i32".into(), vec![]), T::Param(0)]); println!("{:?}", t); println!("{:?}", T::Param(3)); }
