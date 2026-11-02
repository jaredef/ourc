// DBG: derive(Debug) on a struct-shaped enum variant must print `V { f: v, .. }` (was `V(first_field)`).
// The variant registry now records VSTRUCT + the field-name spans; emit_enum_debug prints all fields, named
// for struct variants and positional for tuple variants (tuple variants also printed only the 1st field before).
#[derive(Debug)]
enum E { A(i32), B { x: i32, y: i32 }, C(i32, i32) }
fn main() {
    println!("{:?} {:?} {:?}", E::A(5), E::B { x: 1, y: 2 }, E::C(3, 4));
}
