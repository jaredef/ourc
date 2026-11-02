fn adder(n: i64) -> impl Fn(i64) -> i64 { move |x| x + n }
fn main() {
    let add5 = adder(5);
    println!("{}", add5(37));
}
