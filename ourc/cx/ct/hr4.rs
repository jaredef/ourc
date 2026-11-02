#[derive(Debug, Clone, Copy, PartialEq)] struct Id { a: u32 }
#[derive(Debug)] enum H { Lit(i64), Path(String) }
fn main() { let nodes: Vec<(Id, H)> = vec![(Id { a: 1 }, H::Lit(5))]; println!("{}", matches!(nodes[0].1, H::Lit(_))); let t = &nodes[0]; println!("{:?}", t.1); let h = &t.1; match h { H::Lit(n) => println!("{}", n), _ => println!("o") } println!("{:?}", nodes[0].1); }
