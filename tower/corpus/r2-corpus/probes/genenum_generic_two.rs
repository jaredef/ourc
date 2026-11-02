enum Pair<A, B> { First(A), Second(B) }
fn main() {
    let p: Pair<i32, i32> = Pair::Second(20);
    match p { Pair::First(a) => println!("{}", a), Pair::Second(b) => println!("{}", b) }
}
