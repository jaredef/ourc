struct Cur<'a> { chars: std::str::Chars<'a> }
impl<'a> Cur<'a> { fn new(s:&'a str)->Cur<'a>{Cur{chars:s.chars()}} fn bump(&mut self) -> Option<char> { self.chars.next() } }
fn main() { let mut c = Cur::new("abc"); let mut n = 0i64; while let Some(ch) = c.bump() { n += ch as i64; } println!("{}", n); }
