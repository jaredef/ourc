// cx04 HIR lowering: AST -> HIR desugaring (for -> loop+match on next(), `?` -> match, while -> loop+if),
// HirId allocation, arena of nodes addressed by index, owner map.
#[derive(Debug, Clone)]
enum Ast { Int(i64), Var(String), Call(String, Vec<Ast>), For(String, Box<Ast>, Vec<Ast>), While(Box<Ast>, Vec<Ast>), Try(Box<Ast>), Let(String, Box<Ast>) }
#[derive(Debug, Clone, Copy, PartialEq)] struct HirId { owner: u32, local: u32 }
#[derive(Debug)]
enum Hir { Lit(i64), Path(String), Call(String, Vec<usize>), Loop(Vec<usize>), Match(usize, Vec<(String, usize)>), If(usize, usize, usize), Break, Block(Vec<usize>), Let(String, usize) }
struct Lctx { nodes: Vec<(HirId, Hir)>, owner: u32, next_local: u32 }
impl Lctx {
    fn alloc(&mut self, h: Hir) -> usize { let id = HirId { owner: self.owner, local: self.next_local }; self.next_local += 1; self.nodes.push((id, h)); self.nodes.len() - 1 }
    fn lower(&mut self, a: &Ast) -> usize {
        match a {
            Ast::Int(n) => self.alloc(Hir::Lit(*n)),
            Ast::Var(v) => self.alloc(Hir::Path(v.clone())),
            Ast::Call(f, args) => { let a2: Vec<usize> = args.iter().map(|x| self.lower(x)).collect(); self.alloc(Hir::Call(f.clone(), a2)) }
            Ast::Let(n, e) => { let e2 = self.lower(e); self.alloc(Hir::Let(n.clone(), e2)) }
            Ast::For(pat, iter, body) => {
                let it = self.lower(iter); let into = self.alloc(Hir::Call("into_iter".into(), vec![it]));
                let next = self.alloc(Hir::Call("next".into(), vec![into]));
                let b: Vec<usize> = body.iter().map(|x| self.lower(x)).collect(); let blk = self.alloc(Hir::Block(b));
                let brk = self.alloc(Hir::Break);
                let m = self.alloc(Hir::Match(next, vec![(format!("Some({})", pat), blk), ("None".into(), brk)]));
                self.alloc(Hir::Loop(vec![m]))
            }
            Ast::While(c, body) => {
                let c2 = self.lower(c); let b: Vec<usize> = body.iter().map(|x| self.lower(x)).collect();
                let blk = self.alloc(Hir::Block(b)); let brk = self.alloc(Hir::Break); let i = self.alloc(Hir::If(c2, blk, brk));
                self.alloc(Hir::Loop(vec![i]))
            }
            Ast::Try(e) => {
                let e2 = self.lower(e); let ok = self.alloc(Hir::Path("v".into()));
                let ret = self.alloc(Hir::Call("return_from_residual".into(), vec![]));
                self.alloc(Hir::Match(e2, vec![("Continue(v)".into(), ok), ("Break(r)".into(), ret)]))
            }
        }
    }
    fn show(&self, i: usize) -> String {
        match &self.nodes[i].1 {
            Hir::Lit(n) => n.to_string(), Hir::Path(p) => p.clone(), Hir::Break => "break".into(),
            Hir::Call(f, a) => format!("{}({})", f, a.iter().map(|x| self.show(*x)).collect::<Vec<_>>().join(",")),
            Hir::Loop(b) | Hir::Block(b) => format!("{{{}}}", b.iter().map(|x| self.show(*x)).collect::<Vec<_>>().join(";")),
            Hir::Match(s, arms) => format!("match {} [{}]", self.show(*s), arms.iter().map(|(p, b)| format!("{}=>{}", p, self.show(*b))).collect::<Vec<_>>().join(" | ")),
            Hir::If(c, t, e) => format!("if {} {} else {}", self.show(*c), self.show(*t), self.show(*e)),
            Hir::Let(n, e) => format!("let {}={}", n, self.show(*e)),
        }
    }
}
fn main() {
    let p = Ast::Int(5);
    let mut l = Lctx { nodes: vec![], owner: 0, next_local: 0 }; println!("@0");
    let root = l.lower(&p); println!("@1 {}", root);
    println!("{:?}", l.nodes[root].0); println!("@2");
    println!("{}", l.show(root));
    let q = Ast::Call("f".into(), vec![Ast::Int(1), Ast::Var("x".into())]); let r2 = l.lower(&q); println!("{}", l.show(r2));
    let w = Ast::Let("y".into(), Box::new(Ast::Int(2))); let r3 = l.lower(&w); println!("{}", l.show(r3));
}
