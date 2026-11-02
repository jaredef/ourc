// cx06 generics/mono: real generic code (trait bounds, assoc types, generic structs/impls) AND a
// rustc_monomorphize-shaped collector: worklist of Instance{def, args}, substitution, dedup.
use std::collections::{HashSet, VecDeque};
trait Shape { type Unit; fn area(&self) -> i64; fn name(&self) -> String { "shape".to_string() } }
struct Sq(i64); struct Rect { w: i64, h: i64 }
impl Shape for Sq { type Unit = u8; fn area(&self) -> i64 { self.0 * self.0 } fn name(&self) -> String { format!("sq{}", self.0) } }
impl Shape for Rect { type Unit = u16; fn area(&self) -> i64 { self.w * self.h } }
struct Stack<T> { items: Vec<T> }
impl<T: Clone + std::fmt::Debug> Stack<T> { fn new() -> Self { Stack { items: Vec::new() } } fn push(&mut self, t: T) -> &mut Self { self.items.push(t); self } fn top2(&self) -> Option<(T, T)> { let n = self.items.len(); if n < 2 { None } else { Some((self.items[n - 1].clone(), self.items[n - 2].clone())) } } }
fn total<S: Shape>(xs: &[S]) -> i64 { xs.iter().map(|s| s.area()).sum() }
fn largest<T: PartialOrd + Copy>(xs: &[T]) -> T { let mut m = xs[0]; for &x in xs { if x > m { m = x; } } m }
fn describe<S: Shape + ?Sized>(s: &S) -> String { format!("{}={}", s.name(), s.area()) }
#[derive(Clone, Debug, PartialEq, Eq, Hash)] struct Instance { def: String, args: Vec<String> }
fn callees(i: &Instance) -> Vec<Instance> {
    let t = |k: usize| i.args.get(k).cloned().unwrap_or_default();
    match i.def.as_str() {
        "main" => vec![Instance { def: "total".into(), args: vec!["Sq".into()] }, Instance { def: "total".into(), args: vec!["Rect".into()] }, Instance { def: "largest".into(), args: vec!["i64".into()] }, Instance { def: "Stack::push".into(), args: vec!["String".into()] }],
        "total" => vec![Instance { def: "Iterator::sum".into(), args: vec!["i64".into()] }, Instance { def: format!("{}::area", t(0)), args: vec![] }],
        "Stack::push" => vec![Instance { def: "Vec::push".into(), args: vec![t(0)] }],
        "Vec::push" => vec![Instance { def: "RawVec::grow".into(), args: vec![t(0)] }],
        "largest" => vec![Instance { def: "PartialOrd::gt".into(), args: vec![t(0)] }],
        _ => vec![],
    }
}
fn main() {
    println!("@0");
    println!("{} {}", total(&[Sq(2), Sq(3)]), total(&[Rect { w: 2, h: 5 }]));
    println!("@1");
    println!("{} {:.1} {}", largest(&[3, 9, 4]), largest(&[1.5, 0.5]), largest(&['a', 'z', 'q']));
    println!("@2");
    println!("{} {}", describe(&Sq(4)), describe(&Rect { w: 1, h: 7 }));
    println!("@3");
    let mut s: Stack<String> = Stack::new(); s.push("a".into()).push("b".into()).push("c".into());
    println!("@4");
    println!("{:?} {:?}", s.top2(), Stack::<i32>::new().top2());
    println!("@5");
    let dynv: Vec<Box<dyn Shape<Unit = u8>>> = vec![Box::new(Sq(5)), Box::new(Sq(1))]; println!("dyn {}", dynv.iter().map(|d| d.area()).sum::<i64>());
    println!("@6");
    let mut seen: HashSet<Instance> = HashSet::new(); let mut order = Vec::new(); let mut wl = VecDeque::new();
    println!("@7");
    wl.push_back(Instance { def: "main".into(), args: vec![] });
    println!("@8");
    while let Some(i) = wl.pop_front() { if seen.insert(i.clone()) { for c in callees(&i) { wl.push_back(c); } order.push(i); } }
    println!("@9");
    for i in &order { println!("mono {}<{}>", i.def, i.args.join(",")); }
    println!("@10");
    println!("instances={}", seen.len());
}
