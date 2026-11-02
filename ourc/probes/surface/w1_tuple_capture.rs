struct Foo(i32);
impl Drop for Foo {
    fn drop(&mut self) { println!("{} dropped", self.0); }
}
fn fn_tup(t: (String, i32)) { println!("<{} {}>", t.0, t.1); }
fn main() {
    { let t = (Foo(0), Foo(1)); }
    let t = (Foo(2), 0i32);
    let c = || { let _t = t.1; };
    c();
    let s = (String::from("a"), String::from("b"));
    let d = || println!("[{} {}]", s.0, s.1);
    d();
    fn_tup((String::from("p"), 3));
    let a = Foo(8);
    { let r = (&a, 1); println!("{}", r.1); }
    let u = (Foo(5), 1);
    let v = u;
    println!("end {}", v.1);
}
