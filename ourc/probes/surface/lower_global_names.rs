#![allow(non_upper_case_globals)]
static i: isize = 10;
const n: i64 = 3;
fn g() -> i64 { let mut s = 0; for k in 0..n { s += k; } s + (i as i64) }
fn main(){ let v = vec![5, 6]; let t: i64 = v.iter().sum(); println!("{} {} {} {} {}", i, n, g(), v.len(), t); }
