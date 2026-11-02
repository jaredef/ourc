// cx03 name resolution: rustc_resolve-shaped module tree + use imports + rib stack with shadowing,
// paths with crate/super/self segments, glob imports, Res enum with DefKind.
use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq)] enum DefKind { Fn, Struct, Mod, Const }
#[derive(Debug, Clone, Copy, PartialEq)] enum Res { Def(DefKind, u32), Local(u32), Err }
struct Module { parent: Option<usize>, name: String, items: HashMap<String, Res>, globs: Vec<usize> }
struct Resolver { mods: Vec<Module>, ribs: Vec<Vec<(String, Res)>>, next: u32 }
impl Resolver {
    fn add_mod(&mut self, parent: Option<usize>, name: &str) -> usize {
        let id = self.mods.len(); self.mods.push(Module { parent, name: name.to_string(), items: HashMap::new(), globs: vec![] });
        if let Some(p) = parent { let d = self.fresh(); let _ = d; self.mods[p].items.insert(name.to_string(), Res::Def(DefKind::Mod, id as u32)); }
        id
    }
    fn fresh(&mut self) -> u32 { self.next += 1; self.next }
    fn def(&mut self, m: usize, name: &str, k: DefKind) { let id = self.fresh(); self.mods[m].items.insert(name.to_string(), Res::Def(k, id)); }
    fn lookup_in(&self, m: usize, name: &str, seen: &mut Vec<usize>) -> Option<Res> {
        if seen.contains(&m) { return None; } seen.push(m);
        if let Some(r) = self.mods[m].items.get(name) { return Some(*r); }
        for &g in &self.mods[m].globs { if let Some(r) = self.lookup_in(g, name, seen) { return Some(r); } }
        None
    }
    fn resolve_path(&self, cur: usize, path: &str) -> Res {
        let segs: Vec<&str> = path.split("::").collect();
        let mut m = cur; let mut i = 0;
        if segs.len() == 1 {
            for rib in self.ribs.iter().rev() { if let Some((_, r)) = rib.iter().rev().find(|(n, _)| n == segs[0]) { return *r; } }
        }
        while i + 1 < segs.len() {
            m = match segs[i] { "crate" => 0, "self" => m, "super" => match self.mods[m].parent { Some(p) => p, None => return Res::Err },
                s => match self.lookup_in(m, s, &mut vec![]) { Some(Res::Def(DefKind::Mod, id)) => id as usize, _ => return Res::Err } };
            i += 1;
        }
        self.lookup_in(m, segs[i], &mut vec![]).unwrap_or(Res::Err)
    }
}
fn main() {
    let mut r = Resolver { mods: vec![], ribs: vec![], next: 0 };
    let root = r.add_mod(None, "crate"); let a = r.add_mod(Some(root), "a"); let b = r.add_mod(Some(a), "b"); let c = r.add_mod(Some(root), "c");
    r.def(b, "helper", DefKind::Fn); r.def(c, "MAX", DefKind::Const);
    r.mods[b].globs.push(c); r.mods[c].globs.push(b); println!("@1");
    println!("{:?}", r.resolve_path(b, "MAX")); println!("@2");
    let probes = [(b, "helper"), (c, "helper")];
    for (m, p) in probes.iter() { println!("{} {}", m, p); println!("{}::{} -> {:?}", r.mods[*m].name, p, r.resolve_path(*m, p)); }
}
