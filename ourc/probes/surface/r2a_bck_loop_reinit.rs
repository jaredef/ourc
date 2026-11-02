// R2a P4 guard: a loop-body move followed by an UNCONDITIONAL reassignment in the same body is fresh each iteration
// (while / for / loop, nested loop, fresh assignment first) -- rustc accepts all of these.
enum H { L(i64), C(Box<H>, i64) }
fn ev(h: &H) -> i64 { match h { H::L(x) => *x, H::C(b, k) => ev(b) * k } }
fn main() {
    let mut a = H::L(1);
    let mut i = 0;
    while i < 3 { let arm = H::C(Box::new(a), 2); a = arm; i += 1; }
    let mut b = H::L(1);
    for k in 1..4 { let arm = H::C(Box::new(b), k); let mut w = 0; while w < 1 { w += 1; } b = arm; }
    let mut c2 = H::L(1);
    for k in 1..4 { c2 = H::L(k); let t = H::C(Box::new(c2), 10); c2 = t; }
    let mut d = H::L(1);
    let mut j = 0;
    loop { if j == 2 { break; } let t = H::C(Box::new(d), 3); d = t; j += 1; }
    println!("{} {} {} {}", ev(&a), ev(&b), ev(&c2), ev(&d));
}
