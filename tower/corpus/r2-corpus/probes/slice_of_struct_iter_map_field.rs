// R31: a `&[Struct]` slice param -> `.iter().map(|p| p.field)` must dispatch (the map closure param bound as
// the element struct). The slice param never recorded its element struct-type (SVELT), so p.field misfired.
struct P { x: i32, y: i32 }
fn sum_x(s: &[P]) -> i32 { s.iter().map(|p| p.x).sum() }
fn max_y(s: &[P]) -> i32 { s.iter().map(|p| p.y).max().unwrap() }
fn main() {
    let v = vec![P { x: 1, y: 5 }, P { x: 2, y: 3 }, P { x: 4, y: 1 }];
    println!("{} {}", sum_x(&v), max_y(&v));
}
