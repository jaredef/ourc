trait Named { fn tag(&self) -> i32; }
struct A; struct B;
impl Named for A { fn tag(&self) -> i32 { 1 } }
impl Named for B { fn tag(&self) -> i32 { 2 } }
fn total(items: &[&dyn Named]) -> i32 { let mut s = 0; let mut i = 0; while i < items.len() { s = s + items[i].tag(); i = i + 1; } s }
fn main() { let a = A; let b = B; let v: Vec<&dyn Named> = vec![&a, &b, &a]; println!("{}", total(&v)); }
