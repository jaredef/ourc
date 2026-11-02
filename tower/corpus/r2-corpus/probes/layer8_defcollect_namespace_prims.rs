type ThinVec<T> = Vec<T>;
#[derive(Clone, Copy)]
enum DefKind { Fn, Struct }
#[derive(Clone, Copy)]
enum Res { Local(u32), Def(DefKind, u32), PrimTy(u8), Err }
struct DefData { name: String, kind: DefKind }
struct Rib { names: ThinVec<String>, ids: ThinVec<u32> }
struct Resolver { defs: ThinVec<DefData>, ribs: ThinVec<Rib>, next_local: u32 }
impl Resolver {
    fn collect(&mut self, name: &str, k: DefKind) -> u32 {
        let id = self.defs.len() as u32;
        self.defs.push(DefData { name: String::from(name), kind: k });
        id
    }
    fn def_of(&self, name: &str) -> Res {
        let mut i = 0;
        while i < self.defs.len() {
            if self.defs[i].name.as_str() == name { let k = self.defs[i].kind; return Res::Def(k, i as u32); }
            i += 1;
        }
        Res::Err
    }
    fn push_rib(&mut self) { self.ribs.push(Rib { names: Vec::new(), ids: Vec::new() }); }
    fn pop_rib(&mut self) { self.ribs.pop(); }
    fn bind(&mut self, name: &str) -> u32 {
        let id = self.next_local; self.next_local += 1;
        let top = self.ribs.len() - 1;
        self.ribs[top].names.push(String::from(name));
        self.ribs[top].ids.push(id);
        id
    }
    fn resolve(&self, name: &str) -> Res {
        let mut ri = self.ribs.len();
        while ri > 0 {
            ri -= 1;
            let rib = &self.ribs[ri as usize];
            let mut i = 0;
            while i < rib.names.len() {
                if rib.names[i].as_str() == name { return Res::Local(rib.ids[i]); }
                i += 1;
            }
        }
        if name == "i32" { return Res::PrimTy(1); }
        if name == "bool" { return Res::PrimTy(2); }
        self.def_of(name)
    }
}
fn dump(r: &Res) -> String {
    match r {
        Res::Local(n) => format!("Local({})", n),
        Res::Def(k, id) => { let ks = match k { DefKind::Fn => "Fn", DefKind::Struct => "Struct" }; format!("Def({},{})", ks, id) }
        Res::PrimTy(p) => format!("PrimTy({})", p),
        Res::Err => String::from("Err"),
    }
}
fn main() {
    let mut r = Resolver { defs: Vec::new(), ribs: Vec::new(), next_local: 0 };
    r.collect("a", DefKind::Fn);
    r.collect("b", DefKind::Fn);
    r.collect("S", DefKind::Struct);
    r.push_rib();
    r.bind("x");
    println!("{}", dump(&r.resolve("x")));
    println!("{}", dump(&r.resolve("b")));
    println!("{}", dump(&r.resolve("a")));
    println!("{}", dump(&r.resolve("S")));
    println!("{}", dump(&r.resolve("i32")));
    println!("{}", dump(&r.resolve("bool")));
    println!("{}", dump(&r.resolve("nope")));
    r.pop_rib();
}
