#[derive(Clone)]
pub enum TokenKind {
    Ident(String),
    Literal { value: String },
    Newline,
}

pub struct Printer<'a> {
    out: &'a mut String,
    depth: usize,
}

impl<'a> Printer<'a> {
    pub fn new(out: &'a mut String) -> Printer<'a> {
        Printer { out: out, depth: 0 }
    }

    pub fn strip(name: &str) -> &str {
        if name.starts_with("r#") {
            &name[2..]
        } else {
            name
        }
    }

    pub fn emit(&mut self, tok: TokenKind) {
        match tok {
            TokenKind::Ident(s) => self.out.push_str(&s),
            TokenKind::Literal { value, .. } => self.out.push_str(&value),
            TokenKind::Newline => self.out.push('\n'),
        }
    }

    pub fn run<I>(&mut self, toks: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = TokenKind>,
    {
        for t in toks {
            self.emit(t);
        }
        Ok(())
    }
}
