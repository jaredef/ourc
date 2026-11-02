// RUSTC-TARGET S1 R1 (P-TRAIT-GENERIC-DISPATCH): a trait method called through a generic param
// `fn f<T: Tr>(x: &T) { x.m() }` must monomorph-dispatch to T's impl. Pervasive in rustc source.
trait Sh { fn area(&self) -> i32; }
struct Sq(i32);
struct Rect { w: i32, h: i32 }
impl Sh for Sq { fn area(&self) -> i32 { self.0 * self.0 } }
impl Sh for Rect { fn area(&self) -> i32 { self.w * self.h } }
fn total<T: Sh>(x: &T) -> i32 { x.area() }
fn main() {
    let r = Rect { w: 3, h: 4 };
    println!("{} {} {}", total(&Sq(3)), total(&Rect { w: 2, h: 5 }), total(&r));
}
