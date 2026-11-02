fn make() -> Box<dyn Fn(i64) -> i64> { Box::new(|x| x * 3) }
fn main() {
    let f = make();
    println!("{}", f(14));
}
