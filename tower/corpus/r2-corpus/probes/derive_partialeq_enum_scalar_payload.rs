#[derive(PartialEq)]
enum V { P(i64, i64), U }
fn main() {
    let a = V::P(1, 2); let b = V::P(1, 2); let c = V::P(1, 9); let u = V::U;
    println!("{} {} {} {}", a == b, a == c, a == u, u == V::U);
}
