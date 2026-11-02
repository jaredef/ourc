struct S;
impl S {
    fn n(&self) -> i32 { 1 }
    fn c() -> i32 { 2 }
}
trait Tr { fn t(&self) -> i32; }
impl Tr for S { fn t(&self) -> i32 { 3 } }

fn main() {}
