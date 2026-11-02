fn a(x: i32) -> S { S { f: x, g: 1 } }
fn b(base: S, k: i32) -> S { S { f: k, ..base } }
fn c() -> R { 0..10 }
fn d(x: i32) -> i32 { loop { if x > 0 { break; } } x }
fn e(x: i32) -> i32 { 'l: loop { break 'l; } }
fn f(x: i32) -> i32 { let y = unsafe { x }; y }
fn g() -> f64 { 1.5 }
fn main() {}
