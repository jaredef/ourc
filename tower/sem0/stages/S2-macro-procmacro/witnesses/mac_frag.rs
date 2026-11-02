macro_rules! pf { ($p:pat) => { { let x = 1; match x { $p => 1, _ => 0 } } } }
macro_rules! bf { ($b:block) => { $b } }
macro_rules! pathf { ($q:path) => { $q(1) } }
macro_rules! litf { ($l:literal) => { $l } }
fn id(x: i32) -> i32 { x }
fn a() -> i32 { pf!(1) }
fn b() -> i32 { bf!({ 5 }) }
fn c() -> i32 { pathf!(id) }
fn d() -> i32 { litf!(42) }
fn main() {}
