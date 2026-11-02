// P-TRAIT R2: `fn f(x: impl Display)` monomorphizes per arg type (like <X: Display>),
// so an i32 arg prints as int and a &str arg prints as the string.
use std::fmt::Display;
fn show(x: impl Display) -> String { format!("[{}]", x) }
fn main() {
    println!("{}{}", show(7), show("hi"));   // [7][hi]
    println!("{}", show(42));                 // [42]
}
