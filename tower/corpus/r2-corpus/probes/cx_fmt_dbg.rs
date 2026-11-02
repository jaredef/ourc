// P-CX-FMT-DBG witness (ourc construct-generation, 2026-10-24): format!("{:?}") of aggregates (enum, struct, Vec,
// &Enum param/local) built a String holding the cell ADDRESS; println! was already right. Both must agree.
#[derive(Debug, Clone, PartialEq)] enum Ty { Int, Var(u32) }
#[derive(Debug)] struct P { x: i64, t: Ty }
fn h(a: &Ty) -> String { format!("{:?}", a) }
fn main() {
    let t = Ty::Var(4); let s = format!("<{:?}>", t); println!("{} {}", s, s.len());
    let p = P { x: 2, t: Ty::Int }; println!("{}", format!("{:?}|{}", p, 7));
    let r = &t; println!("{} {}", format!("{:?}", r), h(&Ty::Int));
    println!("{}", format!("{:?} {:?}", vec![1, 2], Some(Ty::Var(1))));
    println!("{:?} {:?}", t, r);
}
