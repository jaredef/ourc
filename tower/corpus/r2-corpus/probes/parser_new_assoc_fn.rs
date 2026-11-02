struct Parser<'a> { toks: &'a [i32], pos: usize }
impl<'a> Parser<'a> {
    fn new(toks: &'a [i32]) -> Parser<'a> { Parser { toks, pos: 0 } }
    fn peek(&self) -> i32 { if self.pos < self.toks.len() { self.toks[self.pos] } else { -1 } }
    fn bump(&mut self) -> i32 { let v = self.toks[self.pos]; self.pos += 1; v }
    fn parse_sum(&mut self) -> i32 { let mut s = 0; while self.pos < self.toks.len() { s += self.peek(); self.bump(); } s }
}
fn main() { let a = [3, 5, 7, 11]; let mut p = Parser::new(&a); println!("{}", p.parse_sum()); }
