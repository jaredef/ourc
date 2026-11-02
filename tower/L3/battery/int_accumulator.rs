struct Acc { total: i32 }
impl Acc { fn add(&mut self, x: i32) { self.total = self.total + x; } fn get(&self) -> i32 { self.total } }
fn main() { let mut a = Acc { total: 0 }; a.add(10); a.add(20); a.add(12); std::process::exit(a.get()); }