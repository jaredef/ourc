// IDX-FIELD-OWNER witness: `&v[i].f` where field name `f` is shared by structs that disagree on boxing
// (CParam.pat: Pat vs FieldPat/Arm.pat: Box<Pat>). The //@S9 emitter lost the indexed receiver struct and took
// the boxed owner, adding a deref -> SIGSEGV. This is the shape that crashed the tower-built SEM0 on rustc_ast.
enum Pat { Wild, Ident(String), Tuple(Vec<Pat>) }
struct CParam { pat: Pat, ty: Vec<u32> }
struct FieldPat { name: String, pat: Box<Pat>, shorthand: bool }
struct Arm { pat: Box<Pat>, guard: Option<Box<E>>, body: Box<E> }
enum E { Lit(i64), Closure(bool, Vec<CParam>, String, Box<E>), Method(Box<E>, String, Vec<u32>, Vec<E>) }
fn ar(p: &Pat) -> i32 { match p { Pat::Tuple(v) => v.len() as i32, Pat::Ident(_s) => -1, _ => -2 } }
fn is_tup(e: &E) -> bool { match e { E::Closure(_m, cps, _rt, _b) => { cps.len() == 1 && ar(&cps[0].pat) >= 0 } _ => false } }
fn arity(e: &E) -> i32 { match e { E::Closure(_m, cps, _rt, _b) => ar(&cps[0].pat), _ => -9 } }
fn walk(e: &E) -> i32 { match e { E::Method(_r, _mn, _t, args) => { let s = if "x(".len() > 0 && is_tup(&args[0]) { 1 } else { 0 }; s * 100 + arity(&args[0]) } _ => -7 } }
fn main() {
    let c = E::Closure(false, vec![CParam { pat: Pat::Ident(String::from("kind")), ty: Vec::new() }], String::new(), Box::new(E::Lit(1)));
    let m = E::Method(Box::new(E::Lit(0)), String::from("map"), Vec::new(), vec![c]);
    let c2 = E::Closure(false, vec![CParam { pat: Pat::Tuple(vec![Pat::Wild, Pat::Wild]), ty: Vec::new() }], String::new(), Box::new(E::Lit(1)));
    let m2 = E::Method(Box::new(E::Lit(0)), String::from("map"), Vec::new(), vec![c2]);
    println!("{} {}", walk(&m), walk(&m2));
}
