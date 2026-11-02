struct P { s: Vec<u8>, i: usize }
impl P {
    fn peek(&self) -> u8 { if self.i < self.s.len() { self.s[self.i] } else { 0 } }
}
fn main() {
    let mut p = P { s: "hello".as_bytes().to_vec(), i: 0 };
    let mut sum = 0i64;
    for x in &p.s { sum += *x as i64; }
    p.i = 1;
    println!("{}|{}|{}", sum, p.s.len(), p.peek());
}
