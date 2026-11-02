// R2a P4 guard: a byte string (`s.as_bytes()`, or a slice of one) passed to a `&[u8]` parameter -- the olex::cfg_pre shape;
// a Vec<u8> argument still works.
fn cfg_pre(b: &[u8], p: &str) -> bool {
    let pb = p.as_bytes();
    if b.len() < pb.len() { return false; }
    let mut j = 0;
    while j < pb.len() { if b[j] != pb[j] { return false; } j = j + 1; }
    true
}
fn main() {
    let s = String::from("not(x)");
    let t = "xxany(a)";
    let tb = t.as_bytes();
    let v: Vec<u8> = vec![97, 98, 99];
    println!("{} {} {} {}", cfg_pre(s.as_bytes(), "not("), cfg_pre(s.as_bytes(), "all("), cfg_pre(&tb[2..], "any("), cfg_pre(&v, "ab"));
}
