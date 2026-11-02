//! Small data-structures module in the style of rustc_data_structures.
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct SortedMap<K, V = u32> {
    data: Vec<(K, V)>,
}

impl<K: Ord + Clone, V: Clone> SortedMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) {
        match self.data.binary_search_by(|(k, _)| k.cmp(&key)) {
            Ok(i) => self.data[i].1 = value,
            Err(i) => self.data.insert(i, (key, value)),
        }
    }
    pub fn retain<F>(&mut self, mut pred: F)
    where
        F: FnMut(&K) -> bool,
    {
        self.data.retain(|(k, _)| pred(k));
    }
}

pub struct Interned<T>(pub T);

impl<T> Deref for Interned<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

pub struct SyncBuckets {
    inner: Arc<Mutex<HashMap<String, Vec<u32>>>>,
}

impl SyncBuckets {
    pub fn total(&self) -> u32 {
        let guard = self.inner.lock().unwrap();
        guard.values().flat_map(|v| v.iter()).fold(0, |acc, x| acc + x)
    }
}

#[derive(Clone, Debug)]
pub enum Node {
    Leaf(u32),
    Branch { left: u32, right: u32 },
}

pub fn classify(n: &Node) -> &'static str {
    match n {
        Node::Leaf(x) if *x == 0 => "zero",
        Node::Branch { left, right } if left == right => "balanced",
        _ => "other",
    }
}

pub fn is_leaf(n: &Node) -> bool {
    matches!(n, Node::Leaf(_))
}

pub fn ends(xs: &[u32]) -> Option<(u32, u32)> {
    match xs {
        [] => None,
        [only] => Some((*only, *only)),
        [first, .., last] => Some((*first, *last)),
    }
}

pub fn transform(xs: &[u32]) -> Vec<u32> {
    xs.iter().map(|x| x + 1).filter(|x| *x % 2 == 0).collect()
}
