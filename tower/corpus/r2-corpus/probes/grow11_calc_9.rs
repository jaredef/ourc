struct P { s: Vec<u8>, i: usize }
impl P {
    fn peek(&self) -> u8 { if self.i < self.s.len() { self.s[self.i] } else { 0 } }
    fn expr(&mut self) -> i64 {
        let mut v = self.term();
        while self.peek() == b'+' || self.peek() == b'-' {
            let op = self.peek(); self.i += 1;
            let t = self.term();
            if op == b'+' { v += t; } else { v -= t; }
        }
        v
    }
    fn term(&mut self) -> i64 {
        let mut v = self.num();
        while self.peek() == b'*' {
            self.i += 1;
            v *= self.num();
        }
        v
    }
    fn num(&mut self) -> i64 {
        let mut v = 0i64;
        while self.peek() >= b'0' && self.peek() <= b'9' {
            v = v * 10 + (self.peek() - b'0') as i64;
            self.i += 1;
        }
        v
    }
}
fn main() {
    let mut p = P { s: "2+3*4-1".as_bytes().to_vec(), i: 0 };
    println!("{}", p.expr());
}
