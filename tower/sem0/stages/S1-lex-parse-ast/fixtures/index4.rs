use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

/// A type usable as a typed index into an `IndexVec`.
pub trait Idx: Copy + 'static {
    fn new(idx: usize) -> Self;
    fn index(self) -> usize;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NodeId {
    private: u32,
}

impl Idx for NodeId {
    fn new(idx: usize) -> Self {
        NodeId { private: idx as u32 }
    }
    fn index(self) -> usize {
        self.private as usize
    }
}

/// A vector indexed by a typed newtype index `I` rather than bare `usize`.
pub struct IndexVec<I: Idx, T> {
    pub raw: Vec<T>,
    _marker: PhantomData<fn(I)>,
}

impl<I: Idx, T> IndexVec<I, T> {
    pub fn new() -> Self {
        IndexVec { raw: Vec::new(), _marker: PhantomData }
    }

    pub fn push(&mut self, elem: T) -> I {
        let idx = self.raw.len();
        self.raw.push(elem);
        Idx::new(idx)
    }

    pub fn get(&self, idx: I) -> Option<&T> {
        self.raw.get(idx.index())
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.raw.iter()
    }
}

impl<I: Idx, T> Index<I> for IndexVec<I, T> {
    type Output = T;
    fn index(&self, idx: I) -> &T {
        &self.raw[idx.index()]
    }
}

impl<I: Idx, T> IndexMut<I> for IndexVec<I, T> {
    fn index_mut(&mut self, idx: I) -> &mut T {
        &mut self.raw[idx.index()]
    }
}

/// An unsized indexed slice view, as in rustc_index's `IndexSlice`.
pub struct IndexSlice<I: Idx, T> {
    _marker: PhantomData<fn(I)>,
    pub raw: [T],
}

/// A fixed-capacity bitset over `N` machine words.
pub struct BitSet<const N: usize> {
    words: [u64; N],
}

impl<const N: usize> BitSet<N> {
    pub fn new_empty() -> Self {
        BitSet { words: [0; N] }
    }

    pub fn insert(&mut self, elem: usize) -> bool {
        let word = elem / 64;
        let mask = 1u64 << (elem % 64);
        let old = self.words[word];
        self.words[word] = old | mask;
        (old & mask) == 0
    }

    pub fn count(&self) -> usize {
        let mut total = 0usize;
        for i in 0..N {
            total += self.words[i].count_ones() as usize;
        }
        total
    }
}
