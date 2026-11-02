// P-CX-CLOS-CAPTY / TUPLEPAT witness (ourc construct-generation, 2026-10-24): closures comparing a tuple/String element
// against a CAPTURED &str/String local or param; find over Vec<(String, u32)> (a tuple element is an aggregate cell);
// `if let Some((_, r)) = v.iter().find(..)` with v a &Vec param and `*r` (a by-value-held &u32).
struct M { items: Vec<(String, u32)> }
fn look(ms: &[M], n: &str) -> Option<u32> { if let Some((_, r)) = ms[0].items.iter().find(|(k, _)| k == n) { return Some(*r); } None }
fn g(v: &Vec<(String, u32)>, n: &str) -> u32 { if let Some((_, r)) = v.iter().find(|(k, _)| k == n) { let x = *r; return x; } 0 }
fn main() {
    let v: Vec<(String, u32)> = vec![("a".into(), 1), ("b".into(), 2)]; let n = "b"; let ns = String::from("a");
    println!("{:?} {:?} {:?}", v.iter().position(|(k, _)| k == n), v.iter().position(|(k, _)| *k == ns), v.iter().find(|t| t.0 == n).map(|t| t.1));
    let r = v.iter().find(|(k, _)| k.as_str() == "b"); match r { Some(t) => println!("{} {}", t.0, t.1), None => println!("none") }
    let w: Vec<String> = vec!["a".into(), "b".into()]; println!("{:?} {}", w.iter().position(|k| k == n), w.iter().any(|k| *k == ns));
    let ms = vec![M { items: v.clone() }]; println!("{:?} {:?} {}", look(&ms, "b"), look(&ms, "z"), g(&v, "b"));
}
