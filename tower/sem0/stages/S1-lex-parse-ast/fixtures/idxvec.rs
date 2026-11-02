use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

pub struct IndexVec<I, T> {
    raw: Vec<T>,
    _marker: PhantomData<fn(I)>,
}

impl<I: Idx, T> IndexVec<I, T> {
    pub fn new() -> Self {
        IndexVec { raw: Vec::new(), _marker: PhantomData }
    }

    pub fn push(&mut self, d: T) -> I {
        let idx = I::new(self.raw.len());
        self.raw.push(d);
        idx
    }

    pub fn len(&self) -> usize {
        self.raw.len()
    }

    pub fn get(&self, i: I) -> Option<&T> {
        self.raw.get(i.index())
    }
}

impl<I: Idx, T> Index<I> for IndexVec<I, T> {
    type Output = T;

    fn index(&self, i: I) -> &T {
        &self.raw[i.index()]
    }
}
