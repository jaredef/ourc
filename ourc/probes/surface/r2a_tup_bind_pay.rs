// R2a P4 guard: a match arm whose body is a TUPLE of its own payload bindings, destructured by `let`, with a `return` fallback
// (the olower cap_drops shape) -- the names keep their payload types (a Vec indexes as a Vec, a String's len is exact).
enum E { C(String, Vec<String>, u32, u32), I(i64) }
fn f(v: &E) -> String {
    let (cn, caps) = match v { E::C(cn, caps, _l, _h) => (cn, caps), _ => return String::from("-") };
    let mut mk = String::from(cn.as_str());
    mk.push('|');
    mk.push_str(caps[0].as_str());
    mk
}
fn g(v: &E) -> usize { let (cn, caps) = match v { E::C(cn, caps, _l, _h) => (cn, caps), _ => return 0 }; cn.len() + caps.len() }
fn main() {
    let c = E::C(String::from("abc"), vec![String::from("k")], 0, 0);
    println!("{} {} {}", f(&c), f(&E::I(1)), g(&c));
}
