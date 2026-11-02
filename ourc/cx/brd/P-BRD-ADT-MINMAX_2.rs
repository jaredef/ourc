#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)] struct P { x: i32, y: i32 }
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)] enum L { Low, Mid(u8) }
fn main() { let v = vec![P { x: 1, y: 9 }, P { x: 3, y: 0 }, P { x: 3, y: 0 }]; println!("{:?} {:?}", v.iter().max(), v.iter().min()); let e: Vec<L> = vec![]; println!("{:?}", e.iter().max()); let w = vec![L::Mid(2), L::Low, L::Mid(7)]; println!("{:?} {:?}", w.iter().max(), w.iter().min()); }
