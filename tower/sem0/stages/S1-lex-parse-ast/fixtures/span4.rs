use std::collections::HashMap;
use std::ops::{Add, Sub};

pub trait Pos {
    fn from_usize(n: usize) -> Self;
    fn to_usize(&self) -> usize;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BytePos(pub u32);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharPos(pub usize);

impl Pos for BytePos {
    fn from_usize(n: usize) -> BytePos {
        BytePos(n as u32)
    }
    fn to_usize(&self) -> usize {
        self.0 as usize
    }
}

impl Add for BytePos {
    type Output = BytePos;
    fn add(self, rhs: BytePos) -> BytePos {
        BytePos(self.0 + rhs.0)
    }
}

impl Sub for BytePos {
    type Output = BytePos;
    fn sub(self, rhs: BytePos) -> BytePos {
        BytePos(self.0 - rhs.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SyntaxContext(u32);

impl SyntaxContext {
    pub fn root() -> SyntaxContext {
        SyntaxContext(0)
    }
    pub fn is_root(&self) -> bool {
        self.0 == 0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub lo: BytePos,
    pub hi: BytePos,
    pub ctxt: SyntaxContext,
}

impl Span {
    pub fn new(lo: BytePos, hi: BytePos) -> Span {
        Span { lo, hi, ctxt: SyntaxContext::root() }
    }

    pub fn with_lo(&self, lo: BytePos) -> Span {
        Span { lo, ..*self }
    }

    pub fn len(&self) -> u32 {
        self.hi.0 - self.lo.0
    }

    pub fn contains(&self, other: Span) -> bool {
        self.lo.0 <= other.lo.0 && other.hi.0 <= self.hi.0
    }

    pub fn to(&self, end: Span) -> Span {
        let lo = if self.lo.0 < end.lo.0 { self.lo } else { end.lo };
        let hi = if self.hi.0 > end.hi.0 { self.hi } else { end.hi };
        Span::new(lo, hi)
    }

    pub fn is_dummy(&self) -> bool {
        match (self.lo.0, self.hi.0) {
            (0, 0) => true,
            _ => false,
        }
    }
}
