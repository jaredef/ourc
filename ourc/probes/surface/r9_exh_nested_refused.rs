enum T { A(i32), B }
fn main() {
    let o: Option<T> = Some(T::B);
    let x = match o {
        Some(T::A(_)) => 1,
        None => 2,
    };
    println!("{}", x);
}
