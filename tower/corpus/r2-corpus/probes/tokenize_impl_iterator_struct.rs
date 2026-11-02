use std::str::Chars;
#[derive(PartialEq)]
enum TokKind { Semi, Eq, Other, Eof }
struct Token { kind: TokKind, len: u32 }
impl Token { fn new(kind: TokKind, len: u32) -> Token { Token { kind, len } } }
struct Cursor<'a> { len_remaining: usize, chars: Chars<'a> }
impl<'a> Cursor<'a> {
    fn new(input: &'a str) -> Cursor<'a> { Cursor { len_remaining: input.len(), chars: input.chars() } }
    fn bump(&mut self) -> Option<char> { self.chars.next() }
    fn pos_within_token(&self) -> u32 { (self.len_remaining - self.chars.as_str().len()) as u32 }
    fn reset_pos_within_token(&mut self) { self.len_remaining = self.chars.as_str().len(); }
    fn advance_token(&mut self) -> Token {
        let first_char = match self.bump() { Some(c) => c, None => return Token::new(TokKind::Eof, 0) };
        let kind = match first_char { ';' => TokKind::Semi, '=' => TokKind::Eq, _ => TokKind::Other };
        let res = Token::new(kind, self.pos_within_token());
        self.reset_pos_within_token();
        res
    }
}
fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind != TokKind::Eof { Some(token) } else { None }
    })
}
fn main() { let mut n: i64 = 0; for t in tokenize("a=1;") { n += t.len as i64; } println!("{}", n); }  // rustc: a(1)=(1)1(1);(1)=4
