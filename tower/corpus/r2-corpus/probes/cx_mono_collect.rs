// P-CX-MONO witness (ourc construct-generation, 2026-10-25): a rustc_monomorphize-shaped collector -- HashSet<Instance>
// dedup with a Vec<String> field (Vec equality stops on a length mismatch), VecDeque worklist pop typing, a closure
// returning String via get().cloned().unwrap_or_default() (None -> empty String), a match whose last arm is vec![].
use std::collections::{HashSet, VecDeque};
#[derive(Clone, Debug, PartialEq, Eq, Hash)] struct Instance { def: String, args: Vec<String> }
fn callees(i: &Instance) -> Vec<Instance> {
    let t = |k: usize| i.args.get(k).cloned().unwrap_or_default();
    match i.def.as_str() {
        "main" => vec![Instance { def: "total".into(), args: vec!["Sq".into()] }, Instance { def: "total".into(), args: vec!["Rect".into()] }],
        "total" => vec![Instance { def: "Iterator::sum".into(), args: vec!["i64".into()] }, Instance { def: format!("{}::area", t(0)), args: vec![] }],
        _ => vec![],
    }
}
fn main() {
    let mut seen: HashSet<Instance> = HashSet::new(); let mut order = Vec::new(); let mut wl = VecDeque::new();
    wl.push_back(Instance { def: "main".into(), args: vec![] });
    while let Some(i) = wl.pop_front() { if seen.insert(i.clone()) { for c in callees(&i) { wl.push_back(c); } order.push(i); } }
    for i in &order { println!("mono {}<{}>", i.def, i.args.join(",")); }
    let e: Vec<String> = vec![]; let f: Vec<String> = vec!["i".into()]; println!("{} {} [{}]", seen.len(), f == e, e.get(3).cloned().unwrap_or_default());
}
