struct Buf { s: String }
impl Buf {
    fn new() -> Buf { Buf { s: String::new() } }
    fn add(&mut self, t: &str) { self.s.push_str(t); }
    fn total(&self) -> i32 { self.s.len() as i32 }
}
fn main() {
    let mut b = Buf::new();
    b.add("hello");
    b.add(", world");
    println!("total {}", b.total());
}
