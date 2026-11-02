// R2a P6 SIB-PATH guard (E0433): a sibling module TYPE path in a signature does not resolve either
mod a { pub struct T { pub q: i32 } }
mod b { pub fn g(t: &a::T) -> i32 { t.q } }
fn main() { println!("{}", b::g(&a::T { q: 3 })); }
