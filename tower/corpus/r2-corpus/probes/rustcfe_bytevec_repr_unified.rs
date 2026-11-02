// RUSTC-FE byte-vec-repr LIFT: an owned `Vec<u8>` LOCAL now shares the ONE 1-byte representation with a
// Vec<u8> FIELD / &[u8] / as_bytes().to_vec() (SK=6/dispatch_bytes). Closes the P-STRUCT-FIELD-VEC-U8-ELEM
// -READ deferral for the primary patterns: local push/index/iter + moved-into-a-field read via a method
// (rustc_lexer's Cursor{ chars/src }). Pin-Art: pinart-bytevec-repr-2026-08-04.md.
struct Cur { src: Vec<u8>, pos: usize }
impl Cur { fn at(&self, i: usize) -> u8 { self.src[i] } fn len(&self) -> usize { self.src.len() } }
fn main() {
    let mut v: Vec<u8> = Vec::new();
    let mut i = 0; while i < 4 { v.push((65 + i) as u8); i += 1; }
    let mut s = 0; for x in v.iter() { s += *x as i32; }         // local iter
    let c = Cur { src: v, pos: 0 };                              // move into a field
    let mut t = 0; let mut j = 0; while j < c.len() { t += c.at(j) as i32; j += 1; }   // read via method
    println!("{} {}", s, t);
}
