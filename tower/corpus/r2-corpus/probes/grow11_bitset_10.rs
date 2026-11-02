struct BitSet { bits: u64 }
impl BitSet {
    fn new() -> BitSet { BitSet { bits: 0 } }
    fn set(&mut self, i: u32) { self.bits |= 1u64 << i; }
    fn has(&self, i: u32) -> bool { (self.bits >> i) & 1 == 1 }
    fn count(&self) -> u32 { self.bits.count_ones() }
}
fn main() {
    let mut b = BitSet::new();
    b.set(1); b.set(3); b.set(63); b.set(3);
    println!("{} {} {}", b.has(3), b.has(2), b.count());
}
