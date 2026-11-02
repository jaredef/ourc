struct Rib { names: Vec<String> }
fn main() { let r = Rib { names: vec![String::from("hi")] }; println!("{}", r.names[0]); }
