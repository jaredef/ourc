pub enum Transparency {
    Transparent,
    SemiTransparent,
    Opaque,
}

pub struct ExpnId(pub u32);

pub struct MacroCall<'a> {
    pub name: &'a str,
    pub args: Vec<TokenTree>,
    pub depth: usize,
}

pub enum TokenTree {
    Token(String),
    Delimited(Delimiter, Vec<TokenTree>),
}

pub enum Delimiter {
    Parenthesis,
    Brace,
    Bracket,
}

pub struct Expander {
    pub depth_limit: usize,
    pub calls: Vec<ExpnId>,
}

impl Expander {
    pub fn new(depth_limit: usize) -> Self {
        Expander { depth_limit, calls: Vec::new() }
    }

    pub fn count_tokens(&self, tt: &TokenTree) -> usize {
        match tt {
            TokenTree::Token(_) => 1,
            TokenTree::Delimited(_, inner) => {
                inner.iter().map(|t| self.count_tokens(t)).sum::<usize>() + 2
            }
        }
    }

    pub fn can_expand(&self, call: &MacroCall<'_>) -> bool {
        call.depth < self.depth_limit && !call.args.is_empty()
    }
}

macro_rules! delim {
    ($d:ident, $($t:tt)*) => {
        TokenTree::Delimited(Delimiter::$d, vec![$($t)*])
    };
}
