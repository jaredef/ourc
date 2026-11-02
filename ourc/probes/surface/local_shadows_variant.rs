#![allow(non_camel_case_types)]
enum clam { a(i64), b }
fn get(c: clam) -> i64 { match c { clam::a(v) => v, clam::b => 0 } }
fn main(){ let a = 5; let b = a + 1; let x = get(clam::a(a * 2)) + get(clam::b); println!("{} {} {}", a, b, x); }
