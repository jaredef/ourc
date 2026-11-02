// SPAN-6 differential: a rustc_span-style composite -- Symbol interner + Span/BytePos + SourceMap
#[derive(Clone, Copy, PartialEq)]
struct BytePos(u32);
#[derive(Clone, Copy)]
struct Span { lo: BytePos, hi: BytePos }
impl Span {
    fn new(lo: u32, hi: u32) -> Span { Span { lo: BytePos(lo), hi: BytePos(hi) } }
    fn len(&self) -> u32 { self.hi.0 - self.lo.0 }
    fn is_empty(&self) -> bool { self.lo.0 == self.hi.0 }
}
struct Interner { strs: Vec<String> }
impl Interner {
    fn new() -> Interner { Interner { strs: Vec::new() } }
    fn intern(&mut self, s: &str) -> u32 {
        let mut i = 0;
        while i < self.strs.len() { if self.strs[i] == s { return i as u32; } i = i + 1; }
        self.strs.push(s.to_string());
        (self.strs.len() - 1) as u32
    }
    fn get(&self, id: u32) -> Option<&str> { if (id as usize) < self.strs.len() { Some(&self.strs[id as usize]) } else { None } }
}
struct SourceMap { lines: Vec<u32> }
impl SourceMap {
    fn new() -> SourceMap { SourceMap { lines: Vec::new() } }
    fn add(&mut self, off: u32) { self.lines.push(off); }
    fn line(&self, pos: u32) -> u32 { let mut ln = 0; let mut i = 0; while i < self.lines.len() { if self.lines[i] <= pos { ln = i + 1; } i = i + 1; } ln as u32 }
}
fn main() {
    let mut it = Interner::new();
    let a = it.intern("fn"); let b = it.intern("let"); let c = it.intern("fn");
    println!("{} {} {}", a, b, c);
    match it.get(b) { Some(s) => println!("sym {}", s), None => println!("?") }
    match it.get(9) { Some(s) => println!("sym {}", s), None => println!("no-sym") }
    let sp = Span::new(4, 12);
    println!("len {} empty {}", sp.len(), sp.is_empty() as i32);
    let mut sm = SourceMap::new(); sm.add(0); sm.add(10); sm.add(20);
    println!("line {}", sm.line(15));
}
