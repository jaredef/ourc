// cx_brd_dropglue: rustc drop glue (P-BORROW-DROP, rung360) -- own Drop then fields in declaration order,
// tuple/Vec/Option<Box>/enum payload glue, drop()/forget()/moves, main + loop-body scope ends, let _ temps, assign.
struct D(&'static str);
impl Drop for D { fn drop(&mut self) { println!("drop {}", self.0); } }
struct Pair { a: D, b: D }
impl Drop for Pair { fn drop(&mut self) { println!("drop Pair"); } }
enum E { One(D), Two(D, D) }
fn make(n: &'static str) -> D { D(n) }
fn main() {
    { let _p = Pair { a: D("fa"), b: D("fb") }; let _t = (D("t0"), D("t1")); }
    { let _v = vec![D("v0"), D("v1")]; let _o = Some(Box::new(D("boxed"))); let _e = E::Two(D("e0"), D("e1")); let _f = E::One(D("e-one")); }
    { let a = D("early"); let b = D("moved"); drop(a); println!("after drop"); let keep = b; std::mem::forget(D("forgotten")); let _ = keep; println!("end scope"); }
    for i in 0..2 { let _l = D("loop"); println!("it {}", i); }
    let _ = make("underscore");
    let mut r = D("old"); r = D("new");
    let _m = D("main-end");
    println!("end main {}", r.0);
}
