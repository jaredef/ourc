// R2a P6 ORPHAN guard: the impls the orphan rule ALLOWS -- a std trait for a local type, a local trait for std types
use std::fmt;
struct W;
impl fmt::Display for W {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "w")
    }
}
trait T {
    fn t(&self) -> i32;
}
impl T for Vec<i32> {
    fn t(&self) -> i32 {
        self.len() as i32
    }
}
fn main() {
    println!("{} {}", W, vec![1, 2].t());
}
