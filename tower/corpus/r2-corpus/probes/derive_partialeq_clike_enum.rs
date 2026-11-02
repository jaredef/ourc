#[derive(PartialEq)]
enum Color { Red, Green, Blue }
fn main() {
    let a = Color::Green; let b = Color::Green; let c = Color::Blue;
    println!("{} {}", a == b, a == c);
}
