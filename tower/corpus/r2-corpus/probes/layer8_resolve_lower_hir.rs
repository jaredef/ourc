type P<T> = Box<T>;
type ThinVec<T> = Vec<T>;

// --- AST (Layer-7-shape input, minimal): a let-in expression language ---
enum AstExpr {
    Num(i64),
    Name(String),
    Bin(u8, P<AstExpr>, P<AstExpr>),
    LetIn(String, P<AstExpr>, P<AstExpr>),
}

// --- N1: the Res / DefId / HirId model ---
#[derive(Clone, Copy)]
enum Res { Local(u32), Def(u32), PrimTy(u8), Err }

// --- H1: the HIR node zoo (resolved: Path carries a Res) ---
enum HirExpr {
    Lit(i64),
    Path(Res),
    Bin(u8, P<HirExpr>, P<HirExpr>),
    LetIn(u32, P<HirExpr>, P<HirExpr>),
}

// --- N3: the rib/scope stack (a Vec<Rib>, each Rib a scope of name->local-id) ---
struct Rib { names: ThinVec<String>, ids: ThinVec<u32> }
struct Resolver { ribs: ThinVec<Rib>, next_local: u32, next_hir: u32 }
impl Resolver {
    fn push_rib(&mut self) { self.ribs.push(Rib { names: Vec::new(), ids: Vec::new() }); }
    fn pop_rib(&mut self) { self.ribs.pop(); }
    fn bind(&mut self, name: &str) -> u32 {
        let id = self.next_local; self.next_local += 1;
        let top = self.ribs.len() - 1;
        self.ribs[top].names.push(String::from(name));
        self.ribs[top].ids.push(id);
        id
    }
    // N4: resolve_path -- search ribs top-down (innermost first), then prims
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
        if name == "PI" { return Res::Def(0); }
        Res::Err
    }
    // H3: structural lowering (AST -> HIR), threading the Res
    fn lower(&mut self, e: &AstExpr) -> P<HirExpr> {
        match e {
            AstExpr::Num(n) => Box::new(HirExpr::Lit(*n)),
            AstExpr::Name(s) => { let r = self.resolve(s); Box::new(HirExpr::Path(r)) }
            AstExpr::Bin(op, a, b) => {
                let ha = self.lower(a);
                let hb = self.lower(b);
                Box::new(HirExpr::Bin(*op, ha, hb))
            }
            AstExpr::LetIn(name, val, body) => {
                let hv = self.lower(val);
                self.push_rib();
                let id = self.bind(name);
                let hb = self.lower(body);
                self.pop_rib();
                Box::new(HirExpr::LetIn(id, hv, hb))
            }
        }
    }
}

// --- O1: the observable -- a canonical HIR dump ---
fn dump_res(r: &Res) -> String {
    match r {
        Res::Local(n) => format!("Local({})", n),
        Res::Def(n) => format!("Def({})", n),
        Res::PrimTy(p) => format!("PrimTy({})", p),
        Res::Err => String::from("Err"),
    }
}
fn dump(e: &HirExpr) -> String {
    match e {
        HirExpr::Lit(n) => format!("{}", n),
        HirExpr::Path(r) => dump_res(r),
        HirExpr::Bin(op, a, b) => format!("({} {} {})", *op as char, dump(a), dump(b)),
        HirExpr::LetIn(id, v, body) => format!("(let _{} = {} in {})", id, dump(v), dump(body)),
    }
}

fn main() {
    // let x = 1 + 2 in x + x   (x resolves to Local; inner x's resolve to the binding)
    let ast = AstExpr::LetIn(
        String::from("x"),
        Box::new(AstExpr::Bin(43, Box::new(AstExpr::Num(1)), Box::new(AstExpr::Num(2)))),
        Box::new(AstExpr::Bin(43, Box::new(AstExpr::Name(String::from("x"))), Box::new(AstExpr::Name(String::from("x"))))),
    );
    let mut r = Resolver { ribs: Vec::new(), next_local: 0, next_hir: 0 };
    r.push_rib();
    let hir = r.lower(&ast);
    r.pop_rib();
    println!("{}", dump(&hir));

    // an unresolved name -> Err (diagnostics/recovery)
    let ast2 = AstExpr::Name(String::from("nope"));
    let mut r2 = Resolver { ribs: Vec::new(), next_local: 0, next_hir: 0 };
    r2.push_rib();
    let hir2 = r2.lower(&ast2);
    println!("{}", dump(&hir2));

    // shadowing: let y = 5 in (let y = 6 in y)  -> inner y is Local(1)
    let ast3 = AstExpr::LetIn(
        String::from("y"),
        Box::new(AstExpr::Num(5)),
        Box::new(AstExpr::LetIn(
            String::from("y"),
            Box::new(AstExpr::Num(6)),
            Box::new(AstExpr::Name(String::from("y"))),
        )),
    );
    let mut r3 = Resolver { ribs: Vec::new(), next_local: 0, next_hir: 0 };
    r3.push_rib();
    let hir3 = r3.lower(&ast3);
    r3.pop_rib();
    println!("{}", dump(&hir3));
}
