fn pair<T: Copy>(x: T) -> (T, T) { (x, x) }
fn main() {
    let (a, b) = pair(21);
    println!("{}", a + b);
}
