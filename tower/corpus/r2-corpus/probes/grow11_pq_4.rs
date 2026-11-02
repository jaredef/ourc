struct PQ { data: Vec<i64> }
impl PQ {
    fn new() -> PQ { PQ { data: Vec::new() } }
    fn push(&mut self, x: i64) {
        self.data.push(x);
        self.data.sort();
    }
    fn pop_max(&mut self) -> i64 { self.data.pop().unwrap() }
}
fn main() {
    let mut pq = PQ::new();
    pq.push(5); pq.push(1); pq.push(9); pq.push(3);
    let a = pq.pop_max();
    let b = pq.pop_max();
    println!("{}", a * 10 + b);
}
