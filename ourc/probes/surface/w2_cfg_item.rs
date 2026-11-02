#[cfg(not(foo))]
fn f() -> i32 { 1 }
#[cfg(foo)]
fn f() -> i32 { 2 }
#[cfg(all(unix, debug_assertions))]
macro_rules! m { () => { "on" } }
#[cfg(any(windows, target_os = "macos"))]
macro_rules! m { () => { "off" } }
#[cfg(target_pointer_width = "32")]
struct S { a: u8 }
#[cfg(target_pointer_width = "64")]
struct S { a: i64 }
#[cfg(feature = "x")]
use std::nothing::Here;
fn main() { let s = S { a: 40 }; println!("{} {} {}", f(), m!(), s.a + 2); }
