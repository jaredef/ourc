fn pair() -> (i32, i32) {
    (3, 4)
}
fn main() {
    let (a, mut b) = pair();
    b = b * 10;
    let [x, mut y] = [1, 2];
    y += x;
    println!("{} {} {}", a, b, y);
}
