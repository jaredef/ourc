struct Rib { names: Vec<String> }
struct R { ribs: Vec<Rib> }
fn main() {
    let mut r = R { ribs: vec![Rib{names: Vec::new()}] };
    r.ribs[0].names.push(String::from("x"));
    println!("{}", r.ribs[0].names.len());
}
