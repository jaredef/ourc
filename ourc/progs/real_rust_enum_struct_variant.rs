// enum STRUCT VARIANTS: declared, constructed with named values in any order, matched by name, and
// Debug-printed as `C { x: 2 }`. The field names are recorded with the variant, which is what makes the
// construction orderable and the printer faithful.
#[derive(Debug)]
enum E { A, B(i32), C { x: i32, y: i32 } }
fn f(e: &E) -> i32 { match e { E::A => 0, E::B(v) => *v, E::C { x, y } => x * 10 + y } }
fn main() {
    println!("{} {} {}", f(&E::A), f(&E::B(7)), f(&E::C { y: 3, x: 4 }));
    println!("{:?} {:?} {:?}", E::A, E::B(1), E::C { x: 2, y: 5 });
}
