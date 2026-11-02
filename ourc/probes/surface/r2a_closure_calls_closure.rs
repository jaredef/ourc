// R2a P4 guard: closures calling captured closures (chains, enum builders, move, inside if), with fn/closure name hygiene.
enum E { I(i64), B(Box<E>, Box<E>) }
fn ev(e: &E) -> i64 { match e { E::I(x) => *x, E::B(a, b) => ev(a) * ev(b) } }
fn g(x: i64) -> i64 { x + 1000 }
fn a() -> i64 { let g = |x: i64| x * 2; let h = || g(3); h() }
fn b() -> i64 { let h = || g(3); h() }
fn main() {
    let int = |x: i64| E::I(x);
    let bin = |p: E, q: E| E::B(Box::new(p), Box::new(q));
    let kon = || bin(int(3), int(4));
    let k = 10;
    let f = |x: i64| x + k;
    let gg = |y: i64| if y > 0 { f(y) * 2 } else { 0 };
    let h = move || gg(1) + f(0);
    let fv = |x: i64| x + 1;
    let hv = |fv: i64| fv * 3;
    println!("{} {} {} {} {} {}", ev(&kon()), h(), gg(-1), a(), b(), hv(2) + fv(2));
}
