#[derive(Debug, Clone, PartialEq)] enum Tok { Ident(String), Int(i64), Punct(char), Eof }
struct Cursor<'a> { chars: std::iter::Peekable<std::str::Chars<'a>> }
impl<'a> Cursor<'a> { fn next_tok(&mut self) -> Tok {
  while let Some(&c) = self.chars.peek() { if c.is_whitespace() { self.chars.next(); } else { break; } }
  let c = match self.chars.next() { Some(c) => c, None => return Tok::Eof };
  if c.is_ascii_digit() { let mut n = c.to_digit(10).unwrap() as i64; while let Some(&d) = self.chars.peek() { if let Some(v) = d.to_digit(10) { n = n * 10 + v as i64; self.chars.next(); } else { break; } } Tok::Int(n) }
  else if c.is_alphabetic() { let mut s = String::new(); s.push(c); Tok::Ident(s) } else { Tok::Punct(c) } } }
fn main() { let mut c = Cursor { chars: "12 + x".chars().peekable() }; let mut toks = Vec::new();
  loop { let t = c.next_tok(); let end = t == Tok::Eof; toks.push(t); if end { break; } } println!("{:?}", toks); }
