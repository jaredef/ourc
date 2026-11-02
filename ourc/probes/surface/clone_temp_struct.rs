#[derive(Clone)]
struct S<T>(T, i64);
#[derive(Clone)]
struct P { x: i64, y: i64 }
fn main(){ let a = S(4, 5).clone(); let mut b = P { x: 1, y: 2 }.clone(); b.x = 9; let c = b.clone(); b.y = 7; println!("{} {} {} {} {}", a.0, a.1, b.x, b.y, c.y); }
