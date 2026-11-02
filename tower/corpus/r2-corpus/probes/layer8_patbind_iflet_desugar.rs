type P<T> = Box<T>;
type ThinVec<T> = Vec<T>;
#[derive(Clone, Copy)]
enum Res { Local(u32), Err }
// AST: a tiny expr language with a match (pattern binds idents) + if-let (desugared)
enum Pat { Wild, Bind(String), TupleTwo(String, String) }
enum AstExpr {
    Num(i64),
    Name(String),
    Add(P<AstExpr>, P<AstExpr>),
    Match(P<AstExpr>, ThinVec<Arm>),
    IfLet(String, P<AstExpr>, P<AstExpr>, P<AstExpr>),  // if let Bind(x)=E {A} else {B}
}
struct Arm { pat: Pat, body: P<AstExpr> }
// HIR (desugared: IfLet -> Match)
enum HirExpr {
    Lit(i64),
    Path(Res),
    Add(P<HirExpr>, P<HirExpr>),
    Match(P<HirExpr>, ThinVec<HirArm>),
}
struct HirArm { binds: ThinVec<u32>, body: P<HirExpr> }
struct Rib { names: ThinVec<String>, ids: ThinVec<u32> }
struct Lowerer { ribs: ThinVec<Rib>, next_local: u32 }
impl Lowerer {
    fn push(&mut self) { self.ribs.push(Rib { names: Vec::new(), ids: Vec::new() }); }
    fn pop(&mut self) { self.ribs.pop(); }
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
        Res::Err
    }
    // L8-10: bind a pattern's idents as Locals in the current rib, returning their ids
    fn bind_pat(&mut self, p: &Pat) -> ThinVec<u32> {
        let mut out: ThinVec<u32> = Vec::new();
        match p {
            Pat::Wild => {}
            Pat::Bind(n) => { out.push(self.bind(n)); }
            Pat::TupleTwo(a, b) => { out.push(self.bind(a)); out.push(self.bind(b)); }
        }
        out
    }
    fn lower(&mut self, e: &AstExpr) -> P<HirExpr> {
        match e {
            AstExpr::Num(n) => Box::new(HirExpr::Lit(*n)),
            AstExpr::Name(s) => Box::new(HirExpr::Path(self.resolve(s))),
            AstExpr::Add(a, b) => { let ha = self.lower(a); let hb = self.lower(b); Box::new(HirExpr::Add(ha, hb)) }
            AstExpr::Match(scrut, arms) => {
                let hs = self.lower(scrut);
                let mut harms: ThinVec<HirArm> = Vec::new();
                for arm in arms.iter() {
                    self.push();
                    let binds = self.bind_pat(&arm.pat);
                    let hb = self.lower(&arm.body);
                    self.pop();
                    harms.push(HirArm { binds: binds, body: hb });
                }
                Box::new(HirExpr::Match(hs, harms))
            }
            // L8-16: if let Bind(x) = E { A } else { B }  ->  match E { x => A, _ => B }
            AstExpr::IfLet(name, scrut, then, els) => {
                let hs = self.lower(scrut);
                let mut harms: ThinVec<HirArm> = Vec::new();
                self.push();
                let id = self.bind(name);
                let ha = self.lower(then);
                self.pop();
                let mut b1: ThinVec<u32> = Vec::new(); b1.push(id);
                harms.push(HirArm { binds: b1, body: ha });
                self.push();
                let hb = self.lower(els);
                self.pop();
                harms.push(HirArm { binds: Vec::new(), body: hb });
                Box::new(HirExpr::Match(hs, harms))
            }
        }
    }
}
fn dump_res(r: &Res) -> String {
    match r { Res::Local(n) => format!("_{}", n), Res::Err => String::from("?") }
}
fn dump(e: &HirExpr) -> String {
    match e {
        HirExpr::Lit(n) => format!("{}", n),
        HirExpr::Path(r) => dump_res(r),
        HirExpr::Add(a, b) => format!("(+ {} {})", dump(a), dump(b)),
        HirExpr::Match(s, arms) => {
            let mut out = String::new();
            out.push_str("(match ");
            out.push_str(&dump(s));
            for arm in arms.iter() {
                out.push_str(" [");
                let mut first = true;
                for b in arm.binds.iter() { if !first { out.push_str(","); } first = false; out.push_str(&format!("_{}", b)); }
                out.push_str("=>");
                out.push_str(&dump(&arm.body));
                out.push_str("]");
            }
            out.push_str(")");
            out
        }
    }
}
fn main() {
    let mut lo = Lowerer { ribs: Vec::new(), next_local: 0 };
    lo.push();
    // match (x=1) { Bind(y) => y + y, _ => 0 }  where the scrutinee is a name x (Err, no outer binding)
    let m = AstExpr::Match(
        Box::new(AstExpr::Num(5)),
        vec![
            Arm { pat: Pat::Bind(String::from("y")), body: Box::new(AstExpr::Add(Box::new(AstExpr::Name(String::from("y"))), Box::new(AstExpr::Name(String::from("y"))))) },
            Arm { pat: Pat::Wild, body: Box::new(AstExpr::Num(0)) },
        ],
    );
    println!("{}", dump(&lo.lower(&m)));
    // if let x = 7 { x + 1 } else { 9 }
    let il = AstExpr::IfLet(String::from("x"), Box::new(AstExpr::Num(7)), Box::new(AstExpr::Add(Box::new(AstExpr::Name(String::from("x"))), Box::new(AstExpr::Num(1)))), Box::new(AstExpr::Num(9)));
    println!("{}", dump(&lo.lower(&il)));
    // tuple pattern: match 3 { (a,b) => a + b, _ => 0 }
    let tm = AstExpr::Match(
        Box::new(AstExpr::Num(3)),
        vec![
            Arm { pat: Pat::TupleTwo(String::from("a"), String::from("b")), body: Box::new(AstExpr::Add(Box::new(AstExpr::Name(String::from("a"))), Box::new(AstExpr::Name(String::from("b"))))) },
            Arm { pat: Pat::Wild, body: Box::new(AstExpr::Num(0)) },
        ],
    );
    println!("{}", dump(&lo.lower(&tm)));
    lo.pop();
}
