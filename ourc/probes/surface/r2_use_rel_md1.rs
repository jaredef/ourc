mod geo {
    pub mod area { pub fn sq(x: i64) -> i64 { x * x } pub fn cube(x: i64) -> i64 { x * x * x } }
    pub use area::cube;
    pub use self::area::sq;
    pub fn both(x: i64) -> i64 { sq(x) + cube(x) }
}
fn sq(x: i64) -> i64 { x + 1000 }
use geo::cube;
fn main() { println!("{} {} {} {} {}", geo::sq(2), geo::cube(2), cube(3), geo::both(2), sq(2)); }
