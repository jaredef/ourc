use std::fmt::Display;
fn show(x: impl Display) -> String { format!("[{}]", x) }
fn main() {
    println!("{}{}", show(7), show("hi"));
}
