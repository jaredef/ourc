struct Cursor<'a> { chars: std::iter::Peekable<std::str::Chars<'a>> }
fn main() { let mut c = Cursor { chars: "ab 1".chars().peekable() }; let mut n = 0;
  while let Some(&ch) = c.chars.peek() { if ch == ' ' { break; } c.chars.next(); n += 1; } println!("{} {:?}", n, c.chars.next()); }
