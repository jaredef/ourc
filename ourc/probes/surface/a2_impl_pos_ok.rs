fn mk() -> impl Fn(i32) -> i32 { |x| x + 1 }
fn main() {
    let f = mk();
    let g: Box<dyn Fn(i32) -> i32> = Box::new(|y| y * 2);
    println!("{} {}", f(2), g(3));
}
