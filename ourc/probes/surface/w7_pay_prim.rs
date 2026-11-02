enum V { I(i32), U(u8), C(char), F(f64), B(bool) }
fn show(v: &V) -> String {
    match v {
        V::I(x) => format!("i{}", x),
        V::U(x) => format!("u{}", x),
        V::C(c) => format!("c{}", c),
        V::F(f) => format!("f{}", f),
        V::B(b) => format!("b{}", b),
    }
}
fn main() {
    let vs = [V::I(-3), V::U(200), V::C('q'), V::F(2.5), V::B(true)];
    for v in vs.iter() { print!("{} ", show(v)); }
    match V::C('z') { V::C(c) => println!("{}", c), _ => {} }
}
