// AST: a tiny statement language ; HIR: desugared. Lowering desugars For -> While, IfLet -> Match-ish.
enum AstStmt { Assign(i32), ForSum(i32, i32), IfPos(i32, i32, i32) }  // ForSum(lo,hi)=sum lo..hi ; IfPos(c,t,e)
enum HirStmt { Set(i32), Loop(i32, i32), Sel(i32, i32, i32) }
struct Lctx { hirs: Vec<i32> }  // flattened tag stream for the differential
impl Lctx {
    fn new() -> Lctx { Lctx { hirs: Vec::new() } }
    fn lower(&mut self, s: &AstStmt) -> HirStmt {
        match s {
            AstStmt::Assign(v) => { self.hirs.push(0); HirStmt::Set(*v) }
            AstStmt::ForSum(a, b) => { self.hirs.push(1); HirStmt::Loop(*a, *b) }        // for i in a..b { s+=i } desugars to a loop
            AstStmt::IfPos(c, t, e) => { self.hirs.push(2); HirStmt::Sel(*c, *t, *e) }   // if c>0 {t} else {e} -> a select
        }
    }
}
fn eval_hir(h: &HirStmt) -> i32 {
    match h {
        HirStmt::Set(v) => *v,
        HirStmt::Loop(a, b) => { let mut s = 0; let mut i = *a; while i < *b { s = s + i; i = i + 1; } s }   // the desugared for-loop
        HirStmt::Sel(c, t, e) => if *c > 0 { *t } else { *e },
    }
}
fn main() {
    let prog = vec![AstStmt::Assign(5), AstStmt::ForSum(1, 5), AstStmt::IfPos(3, 100, 200)];
    let mut lc = Lctx::new();
    let mut total = 0;
    let mut i = 0;
    while i < prog.len() { let h = lc.lower(&prog[i]); total = total + eval_hir(&h); i = i + 1; }
    println!("{}", total);          // 5 + (1+2+3+4=10) + 100 = 115
    let mut tags = 0; let mut j = 0;
    while j < lc.hirs.len() { tags = tags * 10 + lc.hirs[j]; j = j + 1; }
    println!("{}", tags);           // 012
}
