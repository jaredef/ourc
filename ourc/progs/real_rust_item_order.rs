// P-ORDER: Rust item names are order-independent. A function may use a struct declared BELOW it, and a
// nested function may use one declared outside and after it. Our reader built its name tables as it
// read, so both failed; it now reads items twice, the first pass names-only.
fn make() -> C { C { x: 7 } }
struct C { x: u8 }
enum E { A, B }
fn pick(e: E) -> u8 { match e { E::A => 1, E::B => 2 } }
fn main() {
    fn nested() -> C { C { x: 5 } }
    println!("{} {} {} {}", make().x, nested().x, pick(E::A), pick(E::B));
    println!("{}", LATER);
}
const LATER: u8 = 9;
