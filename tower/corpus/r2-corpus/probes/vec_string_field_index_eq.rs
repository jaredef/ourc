struct Rib { names: Vec<String> }
fn main() { let r = Rib { names: vec![String::from("hi")] }; if r.names[0] == "hi" { println!("y"); } else { println!("n"); } }
