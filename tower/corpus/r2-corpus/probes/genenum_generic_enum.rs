enum Maybe<T> { Just(T), Nothing }
fn main() {
    let m: Maybe<i32> = Maybe::Just(11);
    match m { Maybe::Just(v) => println!("{}", v), Maybe::Nothing => println!("0") }
}
