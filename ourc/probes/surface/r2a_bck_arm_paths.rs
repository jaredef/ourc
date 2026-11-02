// R2a P4 guard: move checking treats match arms as alternative paths (a move in one arm does not reach another arm), incl.
// a shadowing `let x = if .. { .. x .. } else { x }` inside an arm and a returning arm (rustc accepts all of these).
enum H { L(i64), C(Box<H>, i64) }
fn ev(h: &H) -> i64 { match h { H::L(x) => *x, H::C(b, k) => ev(b) * k } }
fn fa(o: Option<i64>) -> H { let lit = H::L(2); match o { Some(k) => H::C(Box::new(lit), k), None => lit } }
fn fb(o: Option<i64>, u: bool) -> H {
    let lit = H::L(2);
    match o {
        Some(k) => H::C(Box::new(lit), k),
        None => { let lit = if u { H::C(Box::new(lit), 3) } else { lit }; lit }
    }
}
fn fc(n: i64) -> H { let lit = H::L(2); match n { 0 => return H::C(Box::new(lit), 9), _ => {} } lit }
fn main() {
    println!("{} {} {} {} {} {} {}", ev(&fa(Some(5))), ev(&fa(None)), ev(&fb(Some(5), true)), ev(&fb(None, true)),
        ev(&fb(None, false)), ev(&fc(0)), ev(&fc(1)));
}
