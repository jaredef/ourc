struct Doc { title: String, words: Vec<i32> }
impl Doc {
    fn new(t: &str) -> Doc { let mut d = Doc { title: String::new(), words: Vec::new() }; d.title.push_str(t); d }
    fn add(&mut self, n: i32) { self.words.push(n); }
    fn count(&self) -> i32 { self.words.len() as i32 }
    fn total(&self) -> i32 { let mut s = 0; let mut i = 0; while i < self.words.len() { s = s + self.words[i]; i = i + 1; } s }
}
fn main() {
    let mut d = Doc::new("report");
    d.add(10); d.add(20); d.add(30);
    println!("{} words={} total={} titlelen={}", d.title, d.count(), d.total(), d.title.len());
}
