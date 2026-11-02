// R2a P4 guard: a range slice of a byte string is a byte string (len / index / iterate) and compares by CONTENT
// (slices vs bindings with distinct buffers, == and !=); a Vec<u8> slice is unaffected.
fn main() {
    let a = String::from("xyz");
    let fb = a.as_bytes();
    let c = String::from("xy");
    let pb = c.as_bytes();
    let s = &fb[1..3];
    let mut n = 0;
    for b in &fb[..2] { n = n + *b as i64; }
    let v: Vec<u8> = vec![1, 2, 3];
    let vs = &v[1..];
    println!("{} {} {} {} {} {} {} {}", &fb[..2] == pb, &fb[..2] != pb, fb == pb, s.len(), s[0], n, vs.len(), vs[0]);
}
