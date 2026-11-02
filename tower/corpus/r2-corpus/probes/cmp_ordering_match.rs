// match on std::cmp::Ordering from .cmp() -- regression guard for TI3e-g:
// .cmp() yields a bare i32 (-1/0/1), so the Ordering match must compare the
// scrutinee directly (no eload) and map Less/Equal/Greater to -1/0/1.
use std::cmp::Ordering;
fn sign(a: i32, b: i32) -> i32 {
    match a.cmp(&b) { Ordering::Less => -1, Ordering::Equal => 0, Ordering::Greater => 1 }
}
fn main() {
    let mut out = String::new();
    for (a, b) in [(1, 2), (5, 5), (9, 3)] {
        out.push_str(match a.cmp(&b) { Ordering::Less => "L", Ordering::Equal => "E", Ordering::Greater => "G" });
    }
    println!("{}", out);
    println!("{} {} {}", sign(3, 9), sign(4, 4), sign(7, 2));
}
