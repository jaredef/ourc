#[derive(Clone, Copy, PartialEq)]
struct HirId(u32);
enum AstExpr { Lit(i32), Add(Box<AstExpr>, Box<AstExpr>), Neg(Box<AstExpr>) }
enum HirExpr { Lit(HirId, i32), Bin(HirId, i32, Box<HirExpr>, Box<HirExpr>) }  // op: 0 add ; Neg lowered to Bin(sub,0,e)
struct Lowerer { next: u32 }
impl Lowerer {
    fn new() -> Lowerer { Lowerer { next: 0 } }
    fn mk_id(&mut self) -> HirId { let i = self.next; self.next = self.next + 1; HirId(i) }
    fn lower(&mut self, e: &AstExpr) -> Box<HirExpr> {
        match e {
            AstExpr::Lit(n) => { let id = self.mk_id(); Box::new(HirExpr::Lit(id, *n)) }
            AstExpr::Add(a, b) => { let l = self.lower(a); let r = self.lower(b); let id = self.mk_id(); Box::new(HirExpr::Bin(id, 0, l, r)) }
            AstExpr::Neg(a) => { let z = self.mk_id(); let zero = Box::new(HirExpr::Lit(z, 0)); let r = self.lower(a); let id = self.mk_id(); Box::new(HirExpr::Bin(id, 1, zero, r)) }
        }
    }
}
fn eval_hir(h: &HirExpr) -> i32 {
    match h { HirExpr::Lit(_, n) => *n, HirExpr::Bin(_, op, a, b) => { let l = eval_hir(a); let r = eval_hir(b); if *op == 0 { l + r } else { l - r } } }
}
fn count_ids(h: &HirExpr) -> u32 {
    match h { HirExpr::Lit(_, _) => 1, HirExpr::Bin(_, _, a, b) => 1 + count_ids(a) + count_ids(b) }
}
fn main() {
    // (2 + -(3))  => 2 + (0 - 3) = -1
    let ast = AstExpr::Add(Box::new(AstExpr::Lit(2)), Box::new(AstExpr::Neg(Box::new(AstExpr::Lit(3)))));
    let mut lo = Lowerer::new();
    let hir = lo.lower(&ast);
    println!("{}", eval_hir(&hir));
    println!("{}", count_ids(&hir));
    println!("{}", lo.next);
}
