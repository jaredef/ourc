// R2a P4 guard: `&mut s.f` for an AGGREGATE field (Vec / struct) passes the field's handle -- the callee's writes land in the
// field (the oparse fill_capture_tys(&mut p.clos, ..) shape); a Vec<struct> field mutated through an index.
struct Fun { ptys: Vec<String> }
struct In { k: i64 }
struct P { name: String, clos: Vec<Fun>, v: Vec<i64>, inner: In }
fn set(cl: &mut Vec<Fun>, from: usize) { let mut i = from; while i < cl.len() { cl[i].ptys[0] = String::from("z"); i = i + 1; } }
fn bump(v: &mut Vec<i64>) { v.push(9); }
fn setk(i: &mut In) { i.k = 42; }
fn run(p: &mut P) { set(&mut p.clos, 0); }
fn main() {
    let mut p = P { name: String::from("n"), clos: vec![Fun { ptys: vec![String::new()] }], v: vec![1], inner: In { k: 1 } };
    run(&mut p);
    bump(&mut p.v);
    setk(&mut p.inner);
    println!("{} {} {:?} {}", p.clos[0].ptys[0], p.name, p.v, p.inner.k);
}
