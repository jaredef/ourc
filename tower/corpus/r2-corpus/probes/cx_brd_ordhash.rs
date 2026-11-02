// cx_brd_ordhash: derived/manual Ord + Hash surface (P-STD-ORD-HASH, rung359)
use std::cmp::{Ordering, Reverse};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Level { Low, Mid(u8) }
#[derive(Debug, Clone, Eq)]
struct Ver { major: u32, minor: u32, tag: String }
impl PartialEq for Ver { fn eq(&self, o: &Self) -> bool { self.major == o.major && self.minor == o.minor } }
impl PartialOrd for Ver { fn partial_cmp(&self, o: &Self) -> Option<Ordering> { Some(self.cmp(o)) } }
impl Ord for Ver { fn cmp(&self, o: &Self) -> Ordering { self.major.cmp(&o.major).then_with(|| self.minor.cmp(&o.minor)) } }
impl Hash for Ver { fn hash<H: Hasher>(&self, h: &mut H) { self.major.hash(h); self.minor.hash(h); } }
fn h<T: Hash>(t: &T) -> u64 { let mut s = DefaultHasher::new(); t.hash(&mut s); s.finish() }
fn main() {
    let ls = vec![Level::Mid(5), Level::Low, Level::Mid(9)];
    println!("{:?} {:?}", ls.iter().max(), ls.iter().min());
    let a = Ver { major: 1, minor: 2, tag: "a".into() };
    let b = Ver { major: 1, minor: 2, tag: "b".into() };
    let c = Ver { major: 0, minor: 9, tag: "c".into() };
    println!("{} {} {} {}", a == b, h(&a) == h(&b), h(&a) == h(&c), std::cmp::max(a.clone(), c.clone()).tag);
    let mut w = vec!["pear", "fig", "kiwi"]; w.sort_by_key(|x| Reverse(*x));
    println!("{:?} {:?} {} {}", w, 2.5f64.partial_cmp(&2.5), Ordering::Less.is_lt(), [1, 2][..] < [1, 3][..]);
}
