// RUSTC-TARGET S1 (P-TRAIT-DEFAULT-METHOD + String/&str method-return kind, census L1): a method
// (inherent, trait, or DEFAULT) returning String/&str must tag its result so it Displays as text, not a ptr.
trait Greet {
    fn name(&self) -> String;
    fn hello(&self) -> String { format!("hi {}", self.name()) }   // default method returning String
    fn shout(&self) -> String { self.name() }                     // default, forwards a String
}
struct A;
impl Greet for A { fn name(&self) -> String { "a".to_string() } }
struct B { s: String }
impl B { fn get(&self) -> &str { &self.s } }                       // inherent -> &str
fn main() {
    let b = B { s: "yo".to_string() };
    println!("{}|{}|{}|{}", A.hello(), A.shout(), A.name(), b.get());
}
