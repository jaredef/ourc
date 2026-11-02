// RUSTC-FE sub-arc 00 (fail-closed) / FC: attributes on enum variants must not hang the parser and must
// parse correctly (real enums attribute variants, e.g. Option::None #[lang="None"]).
enum E { #[deprecated] A, #[allow(dead_code)] B(i32), C }
fn main() {
    let x = E::B(7);
    match x { E::A => println!("A"), E::B(v) => println!("B{}", v), E::C => println!("C") }
}
