struct P {
    x: i32,
    y: i32,
}
impl Clone for P {
    fn clone(&self) -> P {
        P { x: self.x, y: self.y }
    }
}
impl Copy for P {}
fn sum(p: P) -> i32 {
    p.x + p.y
}
fn main() {
    let p = P { x: 2, y: 3 };
    let a = sum(p);
    let b = sum(p);
    println!("{} {} {}", a, b, p.x);
}
