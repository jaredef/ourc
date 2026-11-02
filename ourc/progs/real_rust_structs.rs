#[derive(Copy, Clone)]
struct Pt { x: i64, y: i64 }
struct Wrap(i64, i64);
fn take(p: Pt) -> i64 { p.x + p.y }
fn main() {
    let w = Wrap(20, 22);
    println!("{}", w.0 + w.1);
    let p = Pt { x: 1, y: 2 };
    println!("{}", (take(p) + take(p)) * 7);
}
