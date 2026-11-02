trait Greet { fn base(&self) -> i64; fn twice(&self) -> i64 { self.base() * 2 } }
struct S;
impl Greet for S { fn base(&self) -> i64 { 21 } }
fn main() { let s = S; println!("{}", s.twice()); }
