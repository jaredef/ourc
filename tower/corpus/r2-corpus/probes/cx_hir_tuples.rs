// P-CX-RESOLVE/HIR witness (ourc construct-generation, 2026-10-24): an array literal of tuples iterates typed elements;
// a variant-payload binder (`H::Match(s, arms)`) carries its field type into closures; `match &self.nodes[i].1` over a
// Vec<(Id, Hir)> yields the Hir cell (not the field-slot address).
#[derive(Debug, Clone, Copy, PartialEq)] struct Id { owner: u32, local: u32 }
enum H { Lit(i64), Call(String, Vec<usize>), Match(usize, Vec<(String, usize)>) }
struct L { nodes: Vec<(Id, H)> }
impl L { fn show(&self, i: usize) -> String { match &self.nodes[i].1 { H::Lit(n) => n.to_string(), H::Call(f, a) => format!("{}({})", f, a.iter().map(|x| self.show(*x)).collect::<Vec<_>>().join(",")),
  H::Match(s, arms) => format!("match {} [{}]", self.show(*s), arms.iter().map(|(p, b)| format!("{}=>{}", p, self.show(*b))).collect::<Vec<_>>().join(" | ")) } } }
fn main() {
    let probes = [(2usize, "helper"), (3, "x")]; for (m, p) in probes.iter() { println!("{} {} {}", m, p, p.len()); }
    let id = |k: u32| Id { owner: 0, local: k };
    let l = L { nodes: vec![(id(0), H::Lit(1)), (id(1), H::Lit(2)), (id(2), H::Call("f".into(), vec![0, 1])), (id(3), H::Match(2, vec![(format!("Some({})", "x"), 0), ("None".into(), 1)]))] };
    println!("{} {:?}", l.show(3), l.nodes[3].0);
}
