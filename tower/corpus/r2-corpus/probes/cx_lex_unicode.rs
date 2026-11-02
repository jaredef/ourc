// P-CX-PEEK-COLLIDE / NONASCII / TUPBOOL witness (ourc construct-generation, 2026-10-25): a Peekable<Chars> cursor`s
// peek() must not take a same-named USER method`s return type; String::push UTF-8-encodes; non-ASCII is_alphabetic;
// {:<w} pads by chars; a bool binder of a plain tuple match arm prints true/false.
struct P { toks: Vec<u8>, pos: usize }
impl P { fn peek(&self) -> &u8 { &self.toks[self.pos] } }
struct Cur<'a> { chars: std::iter::Peekable<std::str::Chars<'a>> }
impl<'a> Cur<'a> { fn word(&mut self) -> String { while let Some(&c) = self.chars.peek() { if c.is_whitespace() { self.chars.next(); } else { break; } }
    let mut s = String::new(); while let Some(&d) = self.chars.peek() { if d.is_alphanumeric() { s.push(d); self.chars.next(); } else { break; } } s } }
fn main() {
    let p = P { toks: vec![7, 9], pos: 1 }; println!("{}", p.pos + p.toks.len());
    let mut c = Cur { chars: "  größe λx héllo".chars().peekable() }; let w1 = c.word(); let w2 = c.word(); let w3 = c.word();
    println!("[{:<8}] [{:>6}] {} {} {}", w1, w2, w3, w3.len(), w3.chars().count());
    println!("{} {} {}", 'ö'.is_alphabetic(), '+'.is_alphanumeric(), 'Ж'.is_alphabetic());
    let ps: Vec<(Option<i64>, bool)> = vec![(None, true), (Some(3), false)];
    for q in &ps { let r = match q { (None, b) => format!("none{}", b), (Some(v), b) => format!("some{}{}", v, b) }; println!("{}", r); }
    match (1, true) { (_, b) => println!("x{}", b) }
}
