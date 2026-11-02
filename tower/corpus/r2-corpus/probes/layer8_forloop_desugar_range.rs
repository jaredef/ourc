type P<T> = Box<T>;
type ThinVec<T> = Vec<T>;
#[derive(Clone, Copy)]
enum Res { Local(u32), Path(u8), Err }
// AST
enum AstExpr {
    Num(i64),
    Name(String),
    Add(P<AstExpr>, P<AstExpr>),
    For(String, P<AstExpr>, P<AstExpr>),   // for PAT in IT { BODY }
    Range(P<AstExpr>, P<AstExpr>),          // a..b
}
// HIR (desugared)
enum HirExpr {
    Lit(i64),
    Path(Res),
    Add(P<HirExpr>, P<HirExpr>),
    Loop(P<HirExpr>),                       // loop { BODY }
    Match(P<HirExpr>, ThinVec<HirArm>),     // match SCRUT { arms }
    Let(u32, P<HirExpr>),                   // let _id = EXPR (in a block)
    Block(ThinVec<P<HirExpr>>),
    RangeS(P<HirExpr>, P<HirExpr>),         // Range { start, end }
    Call(String, ThinVec<P<HirExpr>>),      // a synthetic call: into_iter(x) / x.next()
    Break,
}
struct HirArm { name: String, binds: ThinVec<u32>, body: P<HirExpr> }
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
    fn lower(&mut self, e: &AstExpr) -> P<HirExpr> {
        match e {
            AstExpr::Num(n) => Box::new(HirExpr::Lit(*n)),
            AstExpr::Name(s) => Box::new(HirExpr::Path(self.resolve(s))),
            AstExpr::Add(a, b) => { let ha = self.lower(a); let hb = self.lower(b); Box::new(HirExpr::Add(ha, hb)) }
            AstExpr::Range(a, b) => { let ha = self.lower(a); let hb = self.lower(b); Box::new(HirExpr::RangeS(ha, hb)) }
            // L8-15: for PAT in IT { B } -> { let _it = into_iter(IT); loop { match _it.next() { Some(PAT) => B, None => break } } }
            AstExpr::For(pat, iter, body) => {
                let hiter = self.lower(iter);
                // let _it = into_iter(IT)
                self.push();
                let it_id = self.bind("_it");
                let mut ia: ThinVec<P<HirExpr>> = Vec::new(); ia.push(hiter);
                let call_ii = Box::new(HirExpr::Call(String::from("into_iter"), ia));
                let let_it = Box::new(HirExpr::Let(it_id, call_ii));
                // loop { match _it.next() { Some(pat) => body, None => break } }
                let mut na: ThinVec<P<HirExpr>> = Vec::new(); na.push(Box::new(HirExpr::Path(Res::Local(it_id))));
                let next_call = Box::new(HirExpr::Call(String::from("next"), na));
                // Some(pat) arm
                self.push();
                let pid = self.bind(pat);
                let hbody = self.lower(body);
                self.pop();
                let mut pbinds: ThinVec<u32> = Vec::new(); pbinds.push(pid);
                let mut arms: ThinVec<HirArm> = Vec::new();
                arms.push(HirArm { name: String::from("Some"), binds: pbinds, body: hbody });
                arms.push(HirArm { name: String::from("None"), binds: Vec::new(), body: Box::new(HirExpr::Break) });
                let the_match = Box::new(HirExpr::Match(next_call, arms));
                let the_loop = Box::new(HirExpr::Loop(the_match));
                self.pop();
                let mut blk: ThinVec<P<HirExpr>> = Vec::new(); blk.push(let_it); blk.push(the_loop);
                Box::new(HirExpr::Block(blk))
            }
        }
    }
}
fn dump_res(r: &Res) -> String { match r { Res::Local(n) => format!("_{}", n), Res::Path(p) => format!("P{}", p), Res::Err => String::from("?") } }
fn dump(e: &HirExpr) -> String {
    match e {
        HirExpr::Lit(n) => format!("{}", n),
        HirExpr::Path(r) => dump_res(r),
        HirExpr::Add(a, b) => format!("(+ {} {})", dump(a), dump(b)),
        HirExpr::RangeS(a, b) => format!("Range{{{}..{}}}", dump(a), dump(b)),
        HirExpr::Loop(b) => format!("(loop {})", dump(b)),
        HirExpr::Let(id, v) => format!("(let _{} = {})", id, dump(v)),
        HirExpr::Break => String::from("break"),
        HirExpr::Call(f, args) => {
            let mut s = String::new(); s.push_str("("); s.push_str(f);
            for a in args.iter() { s.push_str(" "); s.push_str(&dump(a)); }
            s.push_str(")"); s
        }
        HirExpr::Block(xs) => {
            let mut s = String::new(); s.push_str("{");
            for x in xs.iter() { s.push_str(" "); s.push_str(&dump(x)); }
            s.push_str(" }"); s
        }
        HirExpr::Match(sc, arms) => {
            let mut s = String::new(); s.push_str("(match "); s.push_str(&dump(sc));
            for arm in arms.iter() {
                s.push_str(" ["); s.push_str(&arm.name); s.push_str("(");
                let mut first = true;
                for b in arm.binds.iter() { if !first { s.push_str(","); } first = false; s.push_str(&format!("_{}", b)); }
                s.push_str(")=>"); s.push_str(&dump(&arm.body)); s.push_str("]");
            }
            s.push_str(")"); s
        }
    }
}
fn main() {
    let mut lo = Lowerer { ribs: Vec::new(), next_local: 0 };
    lo.push();
    // for x in (0..3) { x + 1 }
    let f = AstExpr::For(
        String::from("x"),
        Box::new(AstExpr::Range(Box::new(AstExpr::Num(0)), Box::new(AstExpr::Num(3)))),
        Box::new(AstExpr::Add(Box::new(AstExpr::Name(String::from("x"))), Box::new(AstExpr::Num(1)))),
    );
    println!("{}", dump(&lo.lower(&f)));
    lo.pop();
}
