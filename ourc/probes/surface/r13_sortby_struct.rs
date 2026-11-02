#[derive(Debug, Clone)]
struct P { x: i32, y: i32 }
fn main() {
    let mut v = vec![P { x: 2, y: 1 }, P { x: 0, y: 5 }, P { x: 2, y: 0 }];
    v.sort_by(|a, b| if a.x != b.x { a.x.cmp(&b.x) } else { a.y.cmp(&b.y) });
    println!("{:?}", v);
    v.sort_by(|a, b| b.y.cmp(&a.y));
    println!("{:?}", v);
    let mut w = vec![3, 1, 2];
    w.sort_by(|a, b| b.cmp(a));
    println!("{:?}", w);
}
