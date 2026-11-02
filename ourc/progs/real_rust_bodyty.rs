// C-BODYTY: a function's inferred return is its body's TAIL type. While that question was asked of
// ety_of, an annotated let answered its ANNOTATION and this ordinary program was refused with
// "returns P but body has int". Two attempts to re-interpret the annotation were refuted (rung 95);
// the consumer now has its own rule and the annotation still means what the desugars need.
struct P { a:i64 }
fn mk()->P{ let p: P = P{a:3}; p }
fn one(){ let p: P = P{a:5}; println!("{}",p.a); }
fn main(){ let p: P = P{a:7}; let q=p.a; one(); println!("{} {}", q, mk().a); }
