struct D(&'static str);
impl Drop for D {
    fn drop(&mut self) { println!("drop {}", self.0); }
}
struct R<'a> { c: &'a D, d: &'a D }
struct T<'a>(&'a D, D);
fn main() {
    let a = D("a");
    let r = R { c: &D("r.c"), d: &D("r.d") };
    let s = R { c: &a, d: &D("s.d") };
    let t = T(&a, D("t.own"));
    let u = (&D("u.0"), &D("u.1"));
    println!("end {} {} {}", r.c.0, s.c.0, t.0 .0);
}
