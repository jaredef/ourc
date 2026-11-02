trait T { fn v(&self) -> i32; }
struct S;
impl T for S { fn v(&self) -> i32 { 7 } }
fn f(x: &dyn T, a: [i32; 2], s: &[i32]) -> i32 { x.v() + a[1] + s[0] }
fn main() {
    let b: Box<dyn T> = Box::new(S);
    let arr = [1, 2];
    println!("{} {}", f(&S, arr, &arr[..]), b.v());
}
