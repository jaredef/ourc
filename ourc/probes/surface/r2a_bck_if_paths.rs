// R2a P4 guard: move checking treats if/else branches as alternative paths -- a move in a returning branch or in one arm
// does not reach a later use / the other arm (rustc accepts all of these).
enum H { L(i64), C(Box<H>, i64), W(Vec<H>) }
fn ev(h: &H) -> i64 { match h { H::L(x) => *x, H::C(b, k) => ev(b) * k, H::W(v) => v.len() as i64 } }
fn g(h: H) -> H { H::C(Box::new(h), 5) }
fn a(c: bool) -> H { let bh = H::L(3); if c { return H::C(Box::new(bh), 2); } bh }
fn b(c: bool) -> H { let acc = H::L(4); let r = if c { H::C(Box::new(acc), 3) } else { acc }; r }
fn d(c: bool) -> H { let bh = H::L(3); if c { return g(bh); } bh }
fn e(c: bool) -> H { let bh = H::L(3); if c { return H::W(vec![bh]); } bh }
fn f(n: i64) -> H { match n { 0 => H::L(0), _ => { let bh = H::L(n); if n > 5 { return H::C(Box::new(bh), 2); } return bh; } } }
fn main() {
    println!("{} {} {} {} {} {} {} {} {} {}", ev(&a(true)), ev(&a(false)), ev(&b(true)), ev(&b(false)),
        ev(&d(true)), ev(&d(false)), ev(&e(true)), ev(&e(false)), ev(&f(7)), ev(&f(2)));
}
