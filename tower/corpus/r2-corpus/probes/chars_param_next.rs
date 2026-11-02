use std::str::Chars;
fn f(chars: &mut Chars<'_>) -> i64 {
    if chars.next() != Some('{') { return 1; }
    0
}
fn main() { let mut c = "x".chars(); println!("{}", f(&mut c)); }
