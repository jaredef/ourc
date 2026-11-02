// R2a P2 guard: a struct literal through a named `use` import, and use-tree groups (one binding per leaf).
mod a {
    pub struct P { pub x: i32 }
    pub struct Q { pub p: P }
    pub fn f() -> i32 { 1 }
    pub mod b { pub fn h() -> i32 { 3 } }
}
use a::{P, Q};
use a::{f as ff, b::{h}};
fn main() {
    let q = Q { p: P { x: 6 } };
    println!("{} {}", q.p.x, ff() + h());
}
