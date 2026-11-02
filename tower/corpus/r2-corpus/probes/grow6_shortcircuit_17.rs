fn side(n: i64, log: &mut i64) -> bool {
    *log += n;
    n > 0
}
fn main() {
    let mut log = 0;
    let _ = side(1, &mut log) || side(2, &mut log);
    let _ = side(-1, &mut log) && side(4, &mut log);
    println!("{}", log);
}
