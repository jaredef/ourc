trait A { fn n(&self) -> String; fn s(&self) -> String { String::from("...") } fn intro(&self) -> String { format!("{}/{}", self.n(), self.s()) } }
struct Dog; struct Fish(u8);
impl A for Dog { fn n(&self) -> String { String::from("dog") } fn s(&self) -> String { String::from("woof") } }
impl A for Fish { fn n(&self) -> String { String::from("fish") } }
fn main() { println!("{}", Dog.intro()); println!("{}", Fish(2).intro()); }
