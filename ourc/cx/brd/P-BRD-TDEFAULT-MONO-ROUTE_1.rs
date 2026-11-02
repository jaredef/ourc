trait Animal { fn name(&self) -> String; fn sound(&self) -> String { "...".into() } fn intro(&self) -> String { format!("{} says {}", self.name(), self.sound()) } }
struct Dog; struct Fish(u8);
impl Animal for Dog { fn name(&self) -> String { "dog".into() } fn sound(&self) -> String { "woof".into() } }
impl Animal for Fish { fn name(&self) -> String { format!("fish{}", self.0) } }
fn main() { let zoo: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Fish(2))]; for a in zoo.iter() { println!("{}", a.intro()); } }
