struct Node { val: i64, next_val: i64 }
impl Node { fn sum(&self) -> i64 { self.val + self.next_val } }
fn main() {
    let n = Node { val: 3, next_val: 4 };
    let m = Node { val: n.sum(), next_val: 10 };
    println!("{}", m.sum());
}
