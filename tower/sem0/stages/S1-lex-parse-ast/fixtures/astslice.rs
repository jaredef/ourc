use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Interner<'a, K: Copy> {
    names: Vec<&'a str>,
    keys: Vec<K>,
    dirty: bool,
}

#[derive(Copy, Clone, Debug)]
pub enum Token<'a> {
    Ident(&'a str),
    Literal { value: u64, suffix: Option<char> },
    Punct(char),
    Eof,
}

pub trait Resolve {
    type Output;
    fn resolve(&self, idx: usize) -> Option<Self::Output>;
    fn is_empty(&self) -> bool {
        false
    }
}

impl<'a, K: Copy + Ord> Interner<'a, K> {
    pub fn new() -> Self {
        Interner { names: Vec::new(), keys: Vec::new(), dirty: false }
    }

    pub fn intern(&mut self, name: &'a str, key: K) -> usize {
        if let Some(pos) = self.names.iter().position(|&n| n == name) {
            return pos;
        }
        self.names.push(name);
        self.keys.push(key);
        self.dirty = true;
        self.names.len() - 1
    }

    pub fn classify(&self, tok: Token<'a>) -> &'static str {
        match tok {
            Token::Ident(s) if s.starts_with(|c: char| c.is_uppercase()) => "Type",
            Token::Ident(_) => "ident",
            Token::Literal { suffix: Some('f'), .. } => "float",
            Token::Literal { value, .. } if value == 0 => "zero",
            Token::Literal { .. } => "int",
            Token::Punct(c) if ('a'..='z').contains(&c) => "alpha-punct",
            Token::Punct(_) => "punct",
            Token::Eof => "eof",
        }
    }

    pub fn summary<F>(&self, f: F) -> String
    where
        F: Fn(&str) -> bool,
    {
        let picked: Vec<String> = self
            .names
            .iter()
            .filter(|n| f(n))
            .map(|n| n.to_uppercase())
            .collect();
        picked.join(", ")
    }

    pub fn first_key(&self) -> Option<K> {
        let k = self.keys.first()?;
        Some(*k)
    }
}

impl<'a, K: Copy + Ord> fmt::Display for Interner<'a, K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Interner({} names)", self.names.len())
    }
}
