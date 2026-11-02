struct Point { x: i32, y: i32 }
fn dist(p: &Point) -> i32 { p.x + p.y }
fn main() { let p = Point { x: 8, y: 9 }; std::process::exit(dist(&p)); }   // 17
