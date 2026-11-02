// RUSTC-FE: a Vec<u8> STRUCT FIELD grown via push (the P-STRUCT-FIELD-VEC-U8-ELEM-READ deferral).
// dispatch_bytes had no push handler -> len stayed 0 -> element reads gave 0. Now push appends at
// 1-byte stride (storeb at buf[len], len++), consistent with the [i] byte reads. rustc byte cursors.
struct Buf { b: Vec<u8> }
fn main() {
    let mut x = Buf { b: Vec::new() };
    let mut i = 0; while i < 5 { x.b.push((65 + i) as u8); i += 1; }
    let mut s = 0; let mut j = 0; while j < x.b.len() { s += x.b[j] as i32; j += 1; }
    println!("{} {}", x.b.len(), s);
}
