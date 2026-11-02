// P-BRD-CLOSURE-METHOD-BOUND: a closure passed to a generic METHOD (inherent, and a trait default gated by
// `where Self: Sized`) takes its param type from the method's `F: Fn(&str) -> R` bound.
trait Named { fn name(&self) -> String; fn with<F: Fn(&str) -> String>(&self, f: F) -> String where Self: Sized { f(&self.name()) } }
struct Dog; struct Cat;
impl Named for Dog { fn name(&self) -> String { String::from("dog") } }
impl Cat { fn shout<F: Fn(&str) -> usize>(&self, f: F) -> usize { f("meow") } }
fn main() {
    println!("{} {}", Dog.with(|n| n.repeat(2)), Dog.with(|n| format!("<{}>", n)));
    println!("{}", Cat.shout(|s| s.len() * 10));
}
