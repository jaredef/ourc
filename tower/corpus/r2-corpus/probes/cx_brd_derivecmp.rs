// P-BRD-DERIVE-ORD-CMP: derived Ord/PartialOrd `cmp` / `partial_cmp` on structs (field-wise lexicographic -> Ordering).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)] struct P { x: i32, y: i32 }
fn main() {
    let a = P { x: 1, y: 2 }; let b = P { x: 1, y: 5 }; let c = P { x: 0, y: 9 };
    println!("{:?} {:?} {:?} {:?}", a.cmp(&b), a.cmp(&c), a.cmp(&a), a.partial_cmp(&b));
    let mut v = vec![b, c, a]; v.sort(); println!("{:?}", v);
}
