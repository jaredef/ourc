enum Ast { Int(i64), Call(Vec<Ast>) }
struct L { n: Vec<i64> }
impl L { fn lower(&mut self, a: &Ast) -> usize { match a { Ast::Int(k) => { self.n.push(*k); self.n.len() - 1 } Ast::Call(xs) => { let ids: Vec<usize> = xs.iter().map(|x| self.lower(x)).collect(); self.n.push(ids.len() as i64); ids[0] } } } }
fn main() { let mut l = L { n: vec![] }; let r = l.lower(&Ast::Call(vec![Ast::Int(5), Ast::Int(6)])); println!("{} {:?}", r, l.n); }
