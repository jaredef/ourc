// R23b: {:?} on a Vec<Enum> must print [A(1), B, ..] -- the element enum-type is now tracked (VELENUM ->
// SVELENUM -> DVELENUM -> AVELENUM) and the Vec-Debug printer has an enum branch (emit_enum_debug per element).
// Was: element cell pointers.
#[derive(Debug)]
enum E { A(i32), B { x: i32, y: i32 }, C }
fn main() {
    let v = vec![E::A(1), E::B { x: 2, y: 3 }, E::C];
    println!("{:?}", v);
    println!("{:?}", vec![E::C, E::A(9)]);
}
