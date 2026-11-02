// R19b: a find()/filter() predicate over a Vec<&str>/Vec<String> must bind the closure param as a str
// descriptor (SK=1/3) so `|s| s.len()`/`.starts_with()` dispatch. The param was a bare scalar, so the
// predicate mis-evaluated (wrong find element / count). Same class as R14 (map), for the predicate binding.
fn main() {
    let v = vec!["ab", "cde", "f"];
    let n = v.iter().filter(|s| s.len() > 1).count();
    let r: Vec<&str> = v.iter().filter(|s| s.len() == 1).cloned().collect();
    println!("{:?} {} {:?}", v.iter().find(|s| s.len() == 3), n, r);
}
