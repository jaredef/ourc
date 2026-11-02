use std::str::Chars;
struct Cur<'a> { chars: Chars<'a> }
impl<'a> Cur<'a> { fn new(s:&'a str)->Cur<'a>{Cur{chars:s.chars()}} fn first(&self)->char{self.chars.clone().next().unwrap_or('\0')} fn bump(&mut self)->Option<char>{self.chars.next()} }
fn main() { let mut c = Cur::new("xyz"); let a = c.first(); let b = c.bump().unwrap_or('?'); let d = c.first(); println!("{} {} {}", a, b, d); }
