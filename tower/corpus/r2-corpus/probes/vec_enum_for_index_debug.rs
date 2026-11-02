// R24b (extends R24): `for e in &vec_of_enum { {:?} e }` and `{:?} v[i]` must tag the element as an enum.
// The element enum-type now threads via EVELENUM (&vec -> for-loop var SENUM) and DVELENUM (v[i] read ->
// EISENUM/EIENUMTY). Was: element cell pointers.
#[derive(Debug)]
enum E { A(i32), B { x: i32 }, C }
fn main() {
    let v = vec![E::A(1), E::B { x: 2 }, E::C];
    for e in &v { print!("{:?} ", e); }
    println!();
    println!("{:?} {:?}", v[0], v[2]);
}
