struct Dog;
impl Dog { fn name(&self) -> String { String::from("dog") } fn with<F: Fn(&str) -> String>(&self, f: F) -> String { f(&self.name()) } }
fn main() { println!("{}", Dog.with(|n| n.repeat(2))); }
