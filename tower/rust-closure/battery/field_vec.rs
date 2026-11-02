struct Stack { data: Vec<i32> }
impl Stack {
    fn new() -> Stack { Stack { data: Vec::new() } }
    fn push(&mut self, v: i32) { self.data.push(v); }
    fn len(&self) -> i32 { self.data.len() as i32 }
    fn sum(&self) -> i32 { let mut s = 0; let mut i = 0; while i < self.data.len() { s = s + self.data[i]; i = i + 1; } s }
}
fn main() {
    let mut s = Stack::new();
    let mut i = 1;
    while i <= 6 { s.push(i * i); i = i + 1; }
    println!("len {} sum {}", s.len(), s.sum());
}
