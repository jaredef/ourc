// R2a P5 guard: a STRUCT variant of an enum declared in a module, built and matched through the module path and
// through a `use` -- vpayload did not resolve the bare owner `E` to `l$E` ("no variant `B` on enum `E`")
mod l {
    pub enum E {
        A(i32),
        B { n: i32 },
        C,
    }
}
use l::E;
fn main() {
    for e in [l::E::A(1), l::E::B { n: 2 }, E::B { n: 5 }, l::E::C] {
        match e {
            l::E::A(v) => print!("a{} ", v),
            E::B { n } => print!("b{} ", n),
            l::E::C => print!("c"),
        }
    }
    println!();
}
