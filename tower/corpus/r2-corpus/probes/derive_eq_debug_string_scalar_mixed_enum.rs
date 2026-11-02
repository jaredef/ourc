#[derive(PartialEq, Debug)]
enum T { Id(String), N(i64) }
fn main() {
    let a = T::Id(String::from("hi"));
    let b = T::N(7);
    println!("{:?} {:?}", a, b);
    let c = T::Id(String::from("hi"));
    println!("{} {}", a == c, a != b);
}
