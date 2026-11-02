// R32b: `v[i].method()` / `v[i].field` on a Vec<Struct> element must route the chained access through struct
// dispatch (was: returned the raw struct pointer, dropping the trailing method/field -> a pointer). Fixes the
// last census blocker: a default trait method returning String called on an indexed element.
trait Sh { fn area(&self) -> i32; fn describe(&self) -> String { format!("area={}", self.area()) } }
struct R { w: i32, h: i32 }
impl Sh for R { fn area(&self) -> i32 { self.w * self.h } }
fn main() {
    let v = vec![R { w: 2, h: 3 }, R { w: 4, h: 5 }];
    println!("{} {} {}", v[0].describe(), v[1].area(), v[0].w + v[1].h);
}
