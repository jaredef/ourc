struct Point { x: i32, y: i32 }
fn main() {
    let mut p = Point { x: 3, y: 4 };
    p.x = p.x + 10;
    std::process::exit(p.x + p.y);   // 13 + 4 = 17
}
