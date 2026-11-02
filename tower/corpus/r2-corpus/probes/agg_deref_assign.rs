// `*p = v` through a `&mut AGG` -- regression guard for rung278 (C8 AGG-DEREF-ASSIGN): SEM0 stored v's block
// POINTER over the first word of p's block, so `*s = format!(..)` on `s: &mut String` left s.len() = a heap
// address. An aggregate's value is its block; the fix copies v's block into p's (String/Vec header = 2 words,
// a struct = its field words). This same shape miscompiled SEM0's own s9_uniq_body_scan, which is what made the
// tower-built emitter SEGV on rustc_ast under SUPERSET.
struct Q { n: String, k: i64, b: bool }
fn f(s: &mut String) { *s = format!("x{}", 1); }
fn fc(s: &mut String, t: &str) { if s.len() == 0 { *s = format!("{}", t); } }
fn g(v: &Vec<i32>, s: &mut String, c: &mut i32) { for x in v.iter() { if *x > 1 { *c += 1; *s = format!("n{}", x); } } }
fn r(s: &mut String, d: i32) { if d > 0 { r(s, d - 1); } else { *s = format!("deep"); } }
fn h(v: &mut Vec<String>) { let mut n: Vec<String> = Vec::new(); n.push(String::from("p")); *v = n; }
fn hi(v: &mut Vec<i64>) { *v = vec![7, 8, 9]; }
fn sq(q: &mut Q) { *q = Q { n: String::from("new"), k: 9, b: true }; }
fn si(s: &mut i64) { *s = 42; }
fn main() {
    let mut a = String::new(); f(&mut a); println!("{} {}", a, a.len());
    let mut b = String::new(); fc(&mut b, "hello"); fc(&mut b, "zz"); b.push_str("!"); println!("{} {}", b, b.len());
    let mut c = String::new(); let mut k = 0; g(&vec![1, 2, 3], &mut c, &mut k); println!("{} {} {}", c, c.len(), k);
    let mut d = String::new(); r(&mut d, 3); println!("{} {}", d, d.len());
    let mut e: Vec<String> = Vec::new(); e.push(String::from("a")); e.push(String::from("b")); h(&mut e); e.push(String::from("q")); println!("{} {} {}", e.len(), e[0], e[1]);
    let mut w: Vec<i64> = Vec::new(); hi(&mut w); println!("{} {}", w.len(), w[2]);
    let mut q = Q { n: String::from("old"), k: 1, b: false }; sq(&mut q); println!("{} {} {}", q.n, q.k, q.b);
    let mut o = String::from("orig"); let oc = o.clone(); f(&mut o); println!("{} {}", o, oc);
    let mut i = 1; si(&mut i); println!("{}", i);
    let mut l = String::from("x"); { let rr = &mut l; *rr = String::from("yz"); } println!("{} {}", l, l.len());
    let mut lv: Vec<i64> = Vec::new(); { let wr = &mut lv; *wr = vec![1, 2]; } lv.push(3); println!("{} {}", lv.len(), lv[2]);
}
