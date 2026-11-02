fn fc() -> (i32, (i32, i32)) { (1, (42, 43)) }
fn fd() -> (i32, i32) { (42, 43) }
fn main() {
    let x = fc();
    let y = fd();
    println!("{} {} {}", x.0, (x.1).1, y == (42, 43));
}
