#[derive(PartialEq)]
enum T { Id(String), N(i64) }
fn main() {
    let a = T::Id(String::from("a"));
    let b = T::Id(String::from("a"));
    let c = T::Id(String::from("b"));
    println!("{} {}", a == b, a == c);
    println!("{} {}", T::N(3) == T::N(3), T::Id(String::from("x")) == T::N(3));
}
