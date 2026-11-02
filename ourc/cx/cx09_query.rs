// cx09 query system: rustc_query_system-shaped memoized queries keyed by (kind,key) with a provider table
// of fn pointers, dep-graph edge recording, cycle detection via an active stack, red/green re-validation.
use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] enum Q { TypeOf, Size, Layout, Cyclic }
type Provider = fn(&mut Tcx, u32) -> Result<i64, String>;
struct Tcx { cache: HashMap<(Q, u32), i64>, active: Vec<(Q, u32)>, edges: Vec<((Q, u32), (Q, u32))>, providers: Vec<(Q, Provider)>, runs: u32, input: Vec<i64> }
impl Tcx {
    fn query(&mut self, q: Q, k: u32) -> Result<i64, String> {
        if let Some(parent) = self.active.last().copied() { self.edges.push((parent, (q, k))); }
        if let Some(v) = self.cache.get(&(q, k)) { return Ok(*v); }
        if self.active.contains(&(q, k)) { let cyc: Vec<String> = self.active.iter().map(|(a, b)| format!("{:?}({})", a, b)).collect(); return Err(format!("cycle: {} -> {:?}({})", cyc.join(" -> "), q, k)); }
        let p = self.providers.iter().find(|(pq, _)| *pq == q).map(|(_, f)| *f).ok_or("no provider")?;
        self.active.push((q, k)); self.runs += 1; let r = p(self, k); self.active.pop();
        if let Ok(v) = r { self.cache.insert((q, k), v); } r
    }
}
fn type_of(t: &mut Tcx, k: u32) -> Result<i64, String> { Ok(t.input[k as usize] * 10 + k as i64) }
fn size(t: &mut Tcx, k: u32) -> Result<i64, String> { let ty = t.query(Q::TypeOf, k)?; Ok(if k > 0 { ty % 7 + t.query(Q::Size, k - 1)? } else { ty % 7 }) }
fn layout(t: &mut Tcx, k: u32) -> Result<i64, String> { let s = t.query(Q::Size, k)?; let a = t.query(Q::Size, k / 2)?; Ok(s * 100 + a) }
fn cyclic(t: &mut Tcx, k: u32) -> Result<i64, String> { t.query(Q::Cyclic, (k + 1) % 3) }
fn green(t: &Tcx, node: (Q, u32), changed: &[u32]) -> bool { if node.0 == Q::TypeOf && changed.contains(&node.1) { return false; } t.edges.iter().filter(|(p, _)| *p == node).all(|(_, c)| green(t, *c, changed)) }
fn main() {
    let mut t = Tcx { cache: HashMap::new(), active: vec![], edges: vec![], providers: vec![(Q::TypeOf, type_of), (Q::Size, size), (Q::Layout, layout), (Q::Cyclic, cyclic)], runs: 0, input: vec![3, 1, 4, 1, 5, 9, 2, 6] };
    for k in [7u32, 5, 7, 3] { println!("layout({}) = {:?} runs={}", k, t.query(Q::Layout, k), t.runs); }
    println!("{:?}", t.query(Q::Cyclic, 0));
    let mut keys: Vec<&(Q, u32)> = t.cache.keys().collect(); keys.sort_by_key(|(q, k)| (*q as u8, *k)); println!("cached={} first={:?} last={:?}", keys.len(), keys[0], keys[keys.len() - 1]);
    let mut e = t.edges.clone(); e.sort_by_key(|((a, b), (c, d))| (*a as u8, *b, *c as u8, *d)); e.dedup(); println!("edges={}", e.len());
    for k in [7u32, 3, 1] { println!("layout({}) green after input[2] changed: {}", k, green(&t, (Q::Layout, k), &[2])); }
}
