fn main() {
    let x: i32;
    let c = std::env::args().count() > 5;
    if c { x = 1; }
    println!("{}", x);
}
