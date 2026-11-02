pub struct Cursor<'a> {
    len: usize,
    chars: Chars<'a>,
}
pub(crate) const EOF: char = 'x';
impl<'a> Cursor<'a> {
    pub fn first(&self) -> char { EOF }
    pub fn is_eof(&self) -> bool { self.len == 0 }
    pub fn bump(&mut self) -> Option<char> { let c: char = self.chars.next()?; Some(c) }
}
