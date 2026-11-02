static DUMMY: u32 = 0;

pub trait Pos {
    const ZERO: usize;
    fn from_usize(n: usize) -> Self;
    fn to_usize(&self) -> usize;
    fn is_zero(&self) -> bool {
        self.to_usize() == 0
    }
}

pub struct Span {
    lo: u32,
    hi: u32,
}

impl Span {
    pub fn classify(&self, n: u32) -> u32 {
        match n {
            0 => 1,
            x if x > 10 => 2,
            _ => 3,
        }
    }

    pub fn first(s: &[u32]) -> u32 {
        match s {
            [a, ..] => *a,
            [] => 0,
        }
    }

    pub fn widen(&self) -> u64 {
        self.hi as u64
    }
}
