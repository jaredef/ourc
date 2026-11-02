use std::cmp::Ordering;

pub struct Span {
    lo: u32,
    hi: u32,
}

impl Span {
    pub fn new(lo: u32, hi: u32) -> Span {
        if lo > hi {
            Span { lo: hi, hi: lo }
        } else {
            Span { lo, hi }
        }
    }

    pub fn contains(&self, other: Span) -> bool {
        self.lo <= other.lo && other.hi <= self.hi
    }

    pub fn len(&self) -> u32 {
        self.hi - self.lo
    }
}

impl Ord for Span {
    fn cmp(&self, other: &Span) -> Ordering {
        (self.lo, self.hi).cmp(&(other.lo, other.hi))
    }
}

pub struct SourceMap {
    files: Vec<String>,
}

impl SourceMap {
    pub fn lookup(&self, idx: usize) -> Option<&String> {
        self.files.get(idx)
    }

    pub fn add(&mut self, name: String) -> usize {
        let id = self.files.len();
        self.files.push(name);
        id
    }
}
