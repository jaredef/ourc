use std::fmt::Debug;
fn describe<T>(x: T) -> String where T: Debug + Clone {
    let y = x.clone();
    format!("{:?}/{:?}", x, y)
}
fn main() {
    println!("{}", describe(5));
    println!("{}", describe("hi"));
}
