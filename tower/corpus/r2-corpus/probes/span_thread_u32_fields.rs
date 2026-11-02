struct Node { kind: i64, lo: u32, hi: u32 }
fn span_len(n: &Node) -> u32 { n.hi - n.lo }
fn main() {
    let n = Node { kind: 7, lo: 3, hi: 11 };
    let m = Node { kind: 2, lo: n.lo, hi: n.hi + 5 };
    println!("{} {} {}", span_len(&n), span_len(&m), m.hi);
}
