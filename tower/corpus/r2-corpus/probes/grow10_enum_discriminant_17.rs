#[derive(Clone, Copy)]
enum Color { Red = 1, Green = 5, Blue = 9 }
fn main() {
    let c = Color::Green;
    let n = c as i32;
    println!("{}", n + Color::Blue as i32);
}
