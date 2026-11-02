#[derive(Debug, Clone, Copy, PartialEq)] struct Id { a: u32 }
enum H { Lit(i64), Path(String) }
struct L { nodes: Vec<(Id, H)> }
impl L { fn show(&self, i: usize) -> String { match &self.nodes[i].1 { H::Lit(n) => n.to_string(), H::Path(p) => p.clone() } } }
fn main() { let l = L { nodes: vec![(Id { a: 1 }, H::Lit(5)), (Id { a: 2 }, H::Path("x".into()))] }; println!("{} {}", l.show(0), l.show(1)); let t = &l.nodes[1]; match &t.1 { H::Path(p) => println!("{}", p), _ => {} } }
