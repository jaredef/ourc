#[derive(Debug, Default)] struct Opts { opt: u8, name: Option<String>, emit: Vec<String>, ed: u16 }
fn main() { let o = Opts { ed: 2015, ..Default::default() }; println!("{:?}", o); }
