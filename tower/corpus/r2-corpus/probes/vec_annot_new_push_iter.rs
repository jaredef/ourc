// let v: Vec<&str>/Vec<String> = Vec::new(); push; iterate -- regression guard
// for TI3e-d: the element Ty must come from the type ANNOTATION (the RHS
// Vec::new() carries no elements), threaded into SVELTY so `for s in &v`
// dispatches s.len() correctly.
fn main() {
    let mut a: Vec<&str> = Vec::new();
    a.push("foo"); a.push("quux");
    let mut ta = 0;
    for s in &a { ta += s.len(); }
    println!("{} {}", a.len(), ta);
    let mut b: Vec<String> = Vec::new();
    b.push("ab".to_string()); b.push("cde".to_string());
    let mut tb = 0;
    for s in &b { tb += s.len(); }
    println!("{} {}", b.len(), tb);
}
