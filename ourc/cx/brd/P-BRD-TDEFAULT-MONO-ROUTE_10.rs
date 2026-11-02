trait Animal { fn name(&self) -> String; fn sound(&self) -> String { String::from("...") } fn intro(&self) -> String { format!("{} says {}", self.name(), self.sound()) } }
struct Dog; struct Fish(u8);
impl Animal for Dog { fn name(&self) -> String { String::from("dog") } fn sound(&self) -> String { String::from("woof") } }
impl Animal for Fish { fn name(&self) -> String { format!("fish{}", self.0) } }
fn main() { println!("{} | {}", Dog.intro(), Fish(2).intro()); }
