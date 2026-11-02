pub trait Idx: Copy {
    fn new(idx: usize) -> Self;
    fn index(self) -> usize;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NodeId(u32);

impl Idx for NodeId {
    fn new(idx: usize) -> Self {
        NodeId(idx as u32)
    }
    fn index(self) -> usize {
        self.0 as usize
    }
}

pub struct IndexVec<I, T> {
    raw: Vec<T>,
    _marker: std::marker::PhantomData<fn(I)>,
}

impl<I: Idx, T> IndexVec<I, T> {
    pub fn push(&mut self, t: T) -> I {
        let idx = I::new(self.raw.len());
        self.raw.push(t);
        idx
    }

    pub fn get(&self, i: I) -> Option<&T> {
        self.raw.get(i.index())
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.raw.iter()
    }
}
