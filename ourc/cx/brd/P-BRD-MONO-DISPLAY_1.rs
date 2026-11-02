use std::fmt::Display;
fn f<T: Display + ?Sized>(x: &T) -> String { format!("{}!", x) }
fn main() { println!("{} {} {}", f("hi"), f(&42), f(&'x')); }
