type P<T> = Box<T>;
type ThinVec<T> = Vec<T>;
#[derive(Clone, Copy)]
enum DefKind { Fn, Struct }
#[derive(Clone, Copy)]
struct HirId { owner: u32, local: u32 }
#[derive(Clone, Copy)]
enum Res { Local(u32), Def(DefKind, u32) }
// AST fragments to desugar
enum AstExpr {
    Num(i64),
    Name(String),
    WhileLet(String, P<AstExpr>, P<AstExpr>),  // while let Bind(x)=E { B }
    Try(P<AstExpr>),                            // E?
    MethodCall(P<AstExpr>, String, ThinVec<P<AstExpr>>),  // recv.m(args)
}
// HIR (desugared) + HirId on every node
enum HirKind {
    Lit(i64),
    Path(Res),
    Loop(P<HirExpr>),
    Match(P<HirExpr>, ThinVec<HirArm>),
    Ret(P<HirExpr>),
    Break,
    MethodCall(String, P<HirExpr>, ThinVec<P<HirExpr>>),
}
struct HirExpr { id: HirId, kind: HirKind }
struct HirArm { name: String, bind: u32, body: P<HirExpr> }
struct Rib { names: ThinVec<String>, ids: ThinVec<u32> }
struct Lowerer { ribs: ThinVec<Rib>, next_local: u32, owner: u32, local_ctr: u32 }
impl Lowerer {
    fn mk_id(&mut self) -> HirId { let i = HirId { owner: self.owner, local: self.local_ctr }; self.local_ctr += 1; i }
    fn node(&mut self, k: HirKind) -> P<HirExpr> { let id = self.mk_id(); Box::new(HirExpr { id: id, kind: k }) }
    fn push(&mut self) { self.ribs.push(Rib { names: Vec::new(), ids: Vec::new() }); }
    fn pop(&mut self) { self.ribs.pop(); }
    fn bind(&mut self, n: &str) -> u32 {
        let id = self.next_local; self.next_local += 1;
        let t = self.ribs.len() - 1;
        self.ribs[t].names.push(String::from(n)); self.ribs[t].ids.push(id);
        id
    }
    fn resolve(&self, n: &str) -> Res {
        let mut ri = self.ribs.len();
        while ri > 0 { ri -= 1;
            let rib = &self.ribs[ri as usize];
            let mut i = 0;
            while i < rib.names.len() { if rib.names[i].as_str() == n { return Res::Local(rib.ids[i]); } i += 1; }
        }
        Res::Def(DefKind::Fn, 0)
    }
    fn lower(&mut self, e: &AstExpr) -> P<HirExpr> {
        match e {
            AstExpr::Num(n) => self.node(HirKind::Lit(*n)),
            AstExpr::Name(s) => { let r = self.resolve(s); self.node(HirKind::Path(r)) }
            // while let Bind(x)=E { B } -> loop { match E { Some(x) => B, None => break } }
            AstExpr::WhileLet(name, scrut, body) => {
                let hs = self.lower(scrut);
                self.push();
                let bid = self.bind(name);
                let hb = self.lower(body);
                self.pop();
                let brk = self.node(HirKind::Break);
                let mut arms: ThinVec<HirArm> = Vec::new();
                arms.push(HirArm { name: String::from("Some"), bind: bid, body: hb });
                arms.push(HirArm { name: String::from("None"), bind: 0, body: brk });
                let m = self.node(HirKind::Match(hs, arms));
                self.node(HirKind::Loop(m))
            }
            // E? -> match E { Ok(v)=>v, Err(e)=>return Err(e) }
            AstExpr::Try(inner) => {
                let hi = self.lower(inner);
                self.push();
                let vid = self.bind("v");
                let vpath = self.node(HirKind::Path(Res::Local(vid)));
                self.pop();
                self.push();
                let eid = self.bind("e");
                let epath = self.node(HirKind::Path(Res::Local(eid)));
                let ret = self.node(HirKind::Ret(epath));
                self.pop();
                let mut arms: ThinVec<HirArm> = Vec::new();
                arms.push(HirArm { name: String::from("Ok"), bind: vid, body: vpath });
                arms.push(HirArm { name: String::from("Err"), bind: eid, body: ret });
                self.node(HirKind::Match(hi, arms))
            }
            AstExpr::MethodCall(recv, m, args) => {
                let hr = self.lower(recv);
                let mut hargs: ThinVec<P<HirExpr>> = Vec::new();
                for a in args.iter() { hargs.push(self.lower(a)); }
                self.node(HirKind::MethodCall(String::from(m.as_str()), hr, hargs))
            }
        }
    }
}
fn dump_res(r: &Res) -> String {
    match r { Res::Local(n) => format!("_{}", n), Res::Def(k, id) => { let ks = match k { DefKind::Fn => "Fn", DefKind::Struct => "Struct" }; format!("{}{}", ks, id) } }
}
fn dump(e: &HirExpr) -> String {
    let idp = format!("#{}.{}", e.id.owner, e.id.local);
    match &e.kind {
        HirKind::Lit(n) => format!("{}{}", n, idp),
        HirKind::Path(r) => format!("{}{}", dump_res(r), idp),
        HirKind::Break => format!("break{}", idp),
        HirKind::Loop(b) => format!("(loop {}){}", dump(b), idp),
        HirKind::Ret(v) => format!("(ret {}){}", dump(v), idp),
        HirKind::MethodCall(m, r, args) => {
            let mut s = String::new(); s.push_str("(."); s.push_str(m); s.push_str(" "); s.push_str(&dump(r));
            for a in args.iter() { s.push_str(" "); s.push_str(&dump(a)); }
            s.push_str(")"); s.push_str(&idp); s
        }
        HirKind::Match(sc, arms) => {
            let mut s = String::new(); s.push_str("(match "); s.push_str(&dump(sc));
            for arm in arms.iter() { s.push_str(" ["); s.push_str(&arm.name); s.push_str("(_"); s.push_str(&format!("{}", arm.bind)); s.push_str(")=>"); s.push_str(&dump(&arm.body)); s.push_str("]"); }
            s.push_str(")"); s.push_str(&idp); s
        }
    }
}
fn main() {
    let mut lo = Lowerer { ribs: Vec::new(), next_local: 0, owner: 0, local_ctr: 0 };
    lo.push();
    // while let x = 1 { x }
    let wl = AstExpr::WhileLet(String::from("x"), Box::new(AstExpr::Num(1)), Box::new(AstExpr::Name(String::from("x"))));
    println!("{}", dump(&lo.lower(&wl)));
    // (5)?
    let tr = AstExpr::Try(Box::new(AstExpr::Num(5)));
    println!("{}", dump(&lo.lower(&tr)));
    // 3.foo(4, 5)
    let mc = AstExpr::MethodCall(Box::new(AstExpr::Num(3)), String::from("foo"), vec![Box::new(AstExpr::Num(4)), Box::new(AstExpr::Num(5))]);
    println!("{}", dump(&lo.lower(&mc)));
    lo.pop();
}
