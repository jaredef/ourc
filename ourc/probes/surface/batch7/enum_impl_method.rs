#[derive(Debug)] enum Dir { N, E, S, W } impl Dir { fn turn(&self) -> Dir { match self { Dir::N => Dir::E, Dir::E => Dir::S, Dir::S => Dir::W, Dir::W => Dir::N } } }
fn main() { let mut d = Dir::N; for _ in 0..5 { d = d.turn(); } println!("{:?}", d); }
