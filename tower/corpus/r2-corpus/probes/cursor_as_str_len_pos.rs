use std::str::Chars;
struct Cur<'a> { len_remaining: usize, chars: Chars<'a> }
impl<'a> Cur<'a> {
    fn new(s: &'a str) -> Cur<'a> { Cur { len_remaining: s.len(), chars: s.chars() } }
    fn bump(&mut self) -> Option<char> { self.chars.next() }
    fn as_str(&self) -> &str { self.chars.as_str() }
    fn pos(&self) -> u32 { (self.len_remaining - self.chars.as_str().len()) as u32 }
    fn reset(&mut self) { self.len_remaining = self.chars.as_str().len(); }
    fn adv(&mut self) -> u32 {
        let c = match self.bump() { Some(c) => c, None => return 0 };
        let _k = match c { ';' => 1, '=' => 2, _ => 9 };
        let p = self.pos();
        self.reset();
        p
    }
}
fn toks(s: &str) -> impl Iterator<Item = u32> + '_ {
    let mut cur = Cur::new(s);
    std::iter::from_fn(move || { let p = cur.adv(); if p != 0 { Some(p) } else { None } })
}
fn main() { let mut n: u32 = 0; for t in toks("a=1") { n += t; } println!("{}", n); }
