// LAYER4 module-resolution: qualified-path struct/tuple literals `m::S{..}` / `a::b::S(..)` / `crate::m::S{..}` on
// a module-nested struct. The module-path handler stripped `a::b::` to the last segment for CALLS but not for
// struct literals (fell to ec(0) -> NOELF). Now: if the last segment is a struct type, emit the struct/tuple
// literal (+ dispatch a trailing .method()). rzrs has one namespace, so the stripped name resolves globally.
mod geom {
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }
    pub struct Pair(pub i32, pub i32);
    impl Point {
        pub fn sum(&self) -> i32 {
            self.x + self.y
        }
    }
}
fn main() {
    let p = geom::Point { x: 3, y: 4 };
    println!("{}", p.sum());
    let q = crate::geom::Point { x: 10, y: 20 };
    println!("{}", q.x + q.y);
    let pr = geom::Pair(7, 8);
    println!("{}", pr.0 + pr.1);
    println!("{}", geom::Point { x: 1, y: 2 }.sum());
}
