fn stop() -> ! { panic!("x"); }
fn main() {
    let a;
    if std::env::args().count() < 9 { a = 1; } else { stop(); }
    let z;
    let ok = true && { z = 3; true } && z == 3;
    let w;
    let no = false || { w = 4; false } || w == 4;
    let v: Vec<Box<i32>> = Vec::new();
    println!("{} {} {} {}", a, ok, no, v.len());
}
