fn main() {
    let adder = |x: i64| move |y: i64| x + y;
    let add5 = adder(5);
    println!("{}", add5(10) + add5(20));
}
