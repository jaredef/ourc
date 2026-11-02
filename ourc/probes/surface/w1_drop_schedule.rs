struct D(i64);
impl Drop for D { fn drop(&mut self) { println!("drop {}", self.0); } }
struct W { d: D }
fn main() {
    { let _x: D = D(1); }          // annotated let
    { let _p = Some(D(2)); }       // drop glue: enum payload
    { let _w = W { d: D(3) }; }    // drop glue: struct field
    let _ = D(4);                  // `let _` drops at once
    D(5);                          // temporary drops at the statement end
    let mut x = D(6);
    x = D(7);                      // reassignment drops the old value
    println!("end");
}
