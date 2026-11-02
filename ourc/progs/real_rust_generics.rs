trait T {
    fn m<F: Fn(i64) -> bool>(&self, f: F) -> i64 where Self: Sized;
}
struct S;
impl T for S {
    fn m<F: Fn(i64) -> bool>(&self, f: F) -> i64 where Self: Sized { 42 }
}
fn main() { let s = S; println!("{}", s.m(|x| x > 0)); }
