#[derive(Clone, Copy, PartialEq)]
pub enum TokenKind {
    LineComment,
    Whitespace,
    Ident,
    Literal { kind: u32, suffix: usize },
    OpenParen,
    Eof,
}
pub fn is_trivia(k: TokenKind) -> bool {
    match k {
        TokenKind::LineComment => true,
        TokenKind::Whitespace => true,
        _ => false,
    }
}
