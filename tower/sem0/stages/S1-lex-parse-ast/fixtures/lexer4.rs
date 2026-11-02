use std::str::Chars;

pub enum TokenKind {
    Whitespace,
    Ident,
    Literal { kind: LiteralKind, len: u32 },
    Unknown,
}

pub enum LiteralKind {
    Int { base: Base, empty: bool },
    Char { terminated: bool },
    Byte { terminated: bool },
}

pub enum Base {
    Binary,
    Decimal,
    Hexadecimal,
}

pub struct Cursor<'a> {
    chars: Chars<'a>,
}

const EOF_CHAR: char = '\0';
const NL: u8 = b'\n';

impl<'a> Cursor<'a> {
    fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        Some(c)
    }

    fn eat_while(&mut self, mut pred: impl FnMut(char) -> bool) {
        while let Some(c) = self.bump() {
            if !pred(c) {
                break;
            }
        }
    }

    fn char_literal(&mut self) -> LiteralKind {
        let terminated = loop {
            match self.bump() {
                Some('\'') => break true,
                Some('\\') => {
                    self.bump();
                }
                Some(_) => {}
                None => break false,
            }
        };
        LiteralKind::Char { terminated }
    }

    fn number(&mut self) -> LiteralKind {
        self.eat_while(|c| matches!(c, '0' | '1' | '_'));
        LiteralKind::Int { base: Base::Binary, empty: false }
    }
}

pub const fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r')
}

pub fn classify(c: char) -> TokenKind {
    match c {
        '0'..='9' => TokenKind::Literal { kind: LiteralKind::Int { base: Base::Decimal, empty: false }, len: 1 },
        c if is_whitespace(c) => TokenKind::Whitespace,
        c if c == '_' || c.is_alphabetic() => TokenKind::Ident,
        _ => TokenKind::Unknown,
    }
}
