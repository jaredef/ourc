trait Tr { fn v(&self) -> i32; }
struct S;
impl Tr for S { fn v(&self) -> i32 { 1 } }
fn early(x: i32) -> i32 { if x > 0 { return x; } 0 }
fn unit() { return; }
fn main() {
    let a: [i32; 2] = [1, 2];
    let mut n = 0;
    loop { n = n + 1; if n > 2 { break; } }
    unit();
    println!("{} {} {} {}", a[0], early(5), n, S.v());
}
