// tractable: fn f(p:&P)->&T{ &p.x } returns a scalar/i64 field ref; `*f(&p)` is a no-op
// (f already returns the value; the & is transparent in the value model).
struct P { x: i64, y: i64 }
fn getx(p: &P) -> &i64 { &p.x }
fn gety(p: &P) -> &i64 { &p.y }
fn main() {
    let p = P { x: 42, y: 7 };
    println!("{}", *getx(&p));   // 42
    println!("{}", *gety(&p));   // 7
    println!("{}", *getx(&p) + *gety(&p)); // 49
}
