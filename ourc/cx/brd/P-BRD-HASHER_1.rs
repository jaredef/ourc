use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
#[derive(Debug, Clone, Eq)]
struct Ver { major: u32, minor: u32, tag: String }
impl PartialEq for Ver { fn eq(&self, o: &Self) -> bool { self.major == o.major && self.minor == o.minor } }
impl PartialOrd for Ver { fn partial_cmp(&self, o: &Self) -> Option<Ordering> { Some(self.cmp(o)) } }
impl Ord for Ver { fn cmp(&self, o: &Self) -> Ordering { self.major.cmp(&o.major).then_with(|| self.minor.cmp(&o.minor)) } }
impl Hash for Ver { fn hash<H: Hasher>(&self, h: &mut H) { self.major.hash(h); self.minor.hash(h); } }
fn h<T: Hash>(t: &T) -> u64 { let mut s = DefaultHasher::new(); t.hash(&mut s); s.finish() }
fn main() { let a = Ver { major: 1, minor: 2, tag: "a".into() }; let b = Ver { major: 1, minor: 2, tag: "b".into() }; let c = Ver { major: 0, minor: 9, tag: "c".into() };
println!("{:?}", (h(&a) == h(&c), h(&a) == h(&b), h(&c) == h(&c))); let _ = (&a,&b,&c); }
