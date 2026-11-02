enum H { Lit(i64), Call(String, Vec<usize>), Match(usize, Vec<(String, usize)>) }
struct L { nodes: Vec<H> }
impl L { fn show(&self, i: usize) -> String { match &self.nodes[i] { H::Lit(n) => n.to_string(), H::Call(f, a) => format!("{}({})", f, a.iter().map(|x| self.show(*x)).collect::<Vec<_>>().join(",")),
  H::Match(s, arms) => format!("match {} [{}]", self.show(*s), arms.iter().map(|(p, b)| format!("{}=>{}", p, self.show(*b))).collect::<Vec<_>>().join(" | ")) } } }
fn main() { let l = L { nodes: vec![H::Lit(1), H::Lit(2), H::Call("f".into(), vec![0, 1]), H::Match(2, vec![(format!("Some({})", "x"), 0), ("None".into(), 1)])] }; println!("{}", l.show(3)); }
