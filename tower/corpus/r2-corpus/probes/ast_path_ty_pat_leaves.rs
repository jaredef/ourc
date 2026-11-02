type P<T> = Box<T>;
type ThinVec<T> = Vec<T>;
struct PathSegment { name: String }
struct Path { segments: ThinVec<PathSegment> }
enum TyKind { PathTy(Path), Ref(P<Ty>), Tup(ThinVec<P<Ty>>) }
struct Ty { kind: TyKind }
enum PatKind { Wild, Ident(String), Tuple(ThinVec<P<Pat>>) }
struct Pat { kind: PatKind }
fn dump_path(p: &Path) -> String {
    let mut s = String::new(); let mut first = true;
    for seg in p.segments.iter() {
        if !first { s.push_str("::"); }
        first = false;
        s.push_str(&seg.name);
    }
    s
}
fn dump_ty(t: &Ty) -> String {
    match &t.kind {
        TyKind::PathTy(p) => dump_path(p),
        TyKind::Ref(inner) => { let mut s = String::from("&"); s.push_str(&dump_ty(inner)); s }
        TyKind::Tup(elems) => {
            let mut s = String::from("(");
            let mut first = true;
            for e in elems.iter() { if !first { s.push_str(", "); } first = false; s.push_str(&dump_ty(e)); }
            s.push_str(")"); s
        }
    }
}
fn dump_pat(p: &Pat) -> String {
    match &p.kind {
        PatKind::Wild => String::from("_"),
        PatKind::Ident(n) => { let mut s = String::new(); s.push_str(n); s }
        PatKind::Tuple(elems) => {
            let mut s = String::from("(");
            let mut first = true;
            for e in elems.iter() { if !first { s.push_str(", "); } first = false; s.push_str(&dump_pat(e)); }
            s.push_str(")"); s
        }
    }
}
fn seg(n: &str) -> PathSegment { PathSegment { name: String::from(n) } }
fn main() {
    let mut segs: ThinVec<PathSegment> = Vec::new();
    segs.push(seg("std")); segs.push(seg("vec")); segs.push(seg("Vec"));
    let path = Path { segments: segs };
    println!("{}", dump_path(&path));
    let mut tsegs: ThinVec<PathSegment> = Vec::new(); tsegs.push(seg("i32"));
    let ty = Ty { kind: TyKind::Ref(Box::new(Ty { kind: TyKind::PathTy(Path { segments: tsegs }) })) };
    println!("{}", dump_ty(&ty));
    let mut pels: ThinVec<P<Pat>> = Vec::new();
    pels.push(Box::new(Pat { kind: PatKind::Ident(String::from("a")) }));
    pels.push(Box::new(Pat { kind: PatKind::Wild }));
    let pat = Pat { kind: PatKind::Tuple(pels) };
    println!("{}", dump_pat(&pat));
}
