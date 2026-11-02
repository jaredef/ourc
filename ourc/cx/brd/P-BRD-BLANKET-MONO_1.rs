use std::fmt::Display;
trait Loud { fn loud(&self) -> String; }
impl<T: Display> Loud for T { fn loud(&self) -> String { format!("{}!", self) } }
fn main() { println!("{} {} {}", "hi".loud(), 42.loud(), 'x'.loud()); }
