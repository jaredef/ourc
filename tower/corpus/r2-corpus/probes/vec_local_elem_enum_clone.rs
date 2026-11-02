#[derive(Clone)]
enum Tok { Num(i64), Plus }
fn main() { let v = vec![Tok::Num(5), Tok::Plus]; let t = v[0].clone(); println!("{}", match t { Tok::Num(n) => n, _ => 0 }); }
