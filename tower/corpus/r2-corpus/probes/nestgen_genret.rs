fn wrap<T>(x: T) -> Option<T> { Some(x) }
fn main() {
    let v = wrap(vec![1, 2, 3]);
    println!("{}", v.unwrap().len());
}
