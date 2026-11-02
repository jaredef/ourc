trait A { fn name(&self) -> String; fn with<F: Fn(&str) -> String>(&self, f: F) -> String where Self: Sized { f(&self.name()) } }
struct Dog;
impl A for Dog { fn name(&self) -> String { String::from("dog") } }
fn main() { println!("{}", Dog.with(|n| n.repeat(2))); }
