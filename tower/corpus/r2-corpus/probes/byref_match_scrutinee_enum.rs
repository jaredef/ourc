// D004-CORE IC-1: `match &<place>` over a heap cell -- the reference is transparent, the arm's
// discriminant test must fire (was: every by-ref enum/Option scrutinee fell through to empty).
enum Shape { Dot, Line(i32), Rect(i32, i32) }
fn area(s: &Shape) -> i32 {
    match s { Shape::Dot => 0, Shape::Line(n) => *n, Shape::Rect(w, h) => *w * *h }
}
fn main() {
    let a = Shape::Rect(3, 4);
    let o: Option<i32> = Some(7);
    // by-ref match on a &Enum field-style place + a by-ref Option scrutinee
    let m = match &o { Some(v) => *v, None => -1 };
    println!("{} {}", area(&a), m);
}
