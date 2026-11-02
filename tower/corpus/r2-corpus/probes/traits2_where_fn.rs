use std::fmt::Display;
fn show<T>(x: T) where T: Display { println!("{}", x); }
fn main() {
    show(42);
    show(7);
}
