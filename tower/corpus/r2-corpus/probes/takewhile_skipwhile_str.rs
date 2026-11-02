// R20b: take_while/skip_while predicate over a Vec<&str> must bind the closure param as a str descriptor
// (extends R20 find/filter). The param was a bare scalar so `|s| s.len()` mis-evaluated.
fn main() {
    let v = vec!["a", "bb", "ccc", "d"];
    let t: Vec<&str> = v.iter().take_while(|s| s.len() < 3).cloned().collect();
    let s: Vec<&str> = v.iter().skip_while(|s| s.len() < 2).cloned().collect();
    println!("{:?} {:?}", t, s);
}
