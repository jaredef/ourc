use std::str::Chars;
struct Cur<'a> { n: i64, chars: Chars<'a> }
impl<'a> Cur<'a> { fn rem(&self) -> usize { self.chars.as_str().len() } }
fn main() { let c = Cur { n: 1, chars: "abcd".chars() }; println!("{}", c.rem()); }
