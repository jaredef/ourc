const K: i32 = 3;
const S: &str = "hi";
fn main() {
    let x = 3;
    let a = match x { K => 1, _ => 0 };
    let s = "hi";
    let b = match &s[..] { S => 1, _ => 0 };
    let c = match &x { &K => 1, _ => 0 };
    println!("{} {} {}", a, b, c);
}
