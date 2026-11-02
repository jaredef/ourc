#![feature(decl_macro)]
pub macro m() {}
pub macro n($x: expr) { $x }
pub fn after() {}
fn main() {}
