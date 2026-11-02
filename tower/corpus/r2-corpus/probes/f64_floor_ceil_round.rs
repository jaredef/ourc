// LAYER2: f64 floor()/ceil()/round() on the softfloat fixed-point (scale 1e9) representation.
// Sign-correct: floor toward -inf, ceil toward +inf, round half-away-from-zero.
fn main() {
    let a = 3.7f64;
    let b = 3.2f64;
    let c = 2.5f64;
    let d = -3.7f64;
    let e = -2.5f64;
    println!("{} {} {}", a.floor(), a.ceil(), a.round());
    println!("{} {} {}", b.floor(), b.ceil(), b.round());
    println!("{} {}", c.round(), e.round());
    println!("{} {}", d.floor(), d.ceil());
}
