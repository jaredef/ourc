struct Cur<'a> { t: &'a [i32], pos: usize }
impl<'a> Cur<'a> {
    fn peek(&self) -> i32 { if self.pos < self.t.len() { self.t[self.pos] } else { -1 } }
    fn bump(&mut self) -> i32 { let v = self.t[self.pos]; self.pos += 1; v }
}
fn drive(toks: &[i32]) -> i32 { let mut c = Cur { t: toks, pos: 0 }; let mut sum = 0; while c.pos < c.t.len() { sum += c.peek(); c.bump(); } sum }
fn main() { let a = [10, 20, 30, 40]; println!("{}", drive(&a)); }
