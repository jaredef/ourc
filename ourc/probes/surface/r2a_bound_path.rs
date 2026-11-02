// R2a P5 guard: a generic bound written as a PATH with an associated-type argument (`std::ops::Add<Output = T>`), alone and
// after `+` -- only the path's first segment was read, so `<Output = T>`'s `>` closed the parameter list ("expected `(`")
fn g<T: Copy + std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
fn h<T: std::ops::Mul<Output = T> + Copy>(a: T) -> T {
    a * a
}
fn main() {
    println!("{} {} {}", g(2u8, 3), g(1.5, 2.0), h(7i64));
}
