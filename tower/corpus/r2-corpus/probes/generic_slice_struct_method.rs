// R32: a generic fn `total<T: Trait>(s: &[T])` / `&Vec<T>` calling `x.method()` on elements must monomorphize
// on the ELEMENT STRUCT type, not the i32 default. The call-site styp inference now uses the arg's element
// struct-type (EVELT) for a &[Struct]/&Vec<Struct> arg -> total_R (was total_i32 -> wrong dispatch -> segfault).
// Also non-generic &[Struct]/&Vec<Struct> map(|x| x.method()/x.field).
trait Shape { fn area(&self) -> i32; }
struct Rect { w: i32, h: i32 }
impl Shape for Rect { fn area(&self) -> i32 { self.w * self.h } }
fn total<T: Shape>(s: &[T]) -> i32 { s.iter().map(|x| x.area()).sum() }
fn total_v<T: Shape>(v: &Vec<T>) -> i32 { v.iter().map(|x| x.area()).sum() }
fn main() {
    let v = vec![Rect { w: 2, h: 3 }, Rect { w: 4, h: 5 }];
    println!("{} {}", total(&v), total_v(&v));
}
