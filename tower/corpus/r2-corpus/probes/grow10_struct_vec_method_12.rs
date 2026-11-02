struct Bag { items: Vec<i32> }
impl Bag {
    fn new() -> Bag { Bag { items: Vec::new() } }
    fn add(&mut self, x: i32) { self.items.push(x); }
    fn total(&self) -> i32 { self.items.iter().sum() }
}
fn main() {
    let mut b = Bag::new();
    b.add(3); b.add(4); b.add(5);
    println!("{} {}", b.items.len(), b.total());
}
