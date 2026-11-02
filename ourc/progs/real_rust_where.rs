// P-GEN: a `where` clause on a STRUCT item (both inline and on its own lines) is skipped like the
// ones on fns and impls.
trait Mark { fn m(&self) -> isize; }
struct Z(isize);
impl Mark for Z { fn m(&self) -> isize { self.0 } }
struct G<T> where T: Mark { d: isize, x: T }
struct H<T>
where
    T: Mark,
{ y: isize, z: T }
fn best<T>(a: T) -> isize where T: Mark { a.m() }
fn main() {
    let g = G { d: 5, x: Z(2) };
    let h = H { y: 6, z: Z(3) };
    println!("{} {} {}", g.d, h.y, best(Z(9)));
}
