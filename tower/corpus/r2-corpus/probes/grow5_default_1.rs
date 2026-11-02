#[derive(Default)]
struct Cfg { a: i32, b: i32 }
fn main() {
    let c = Cfg::default();
    let d = Cfg { a: 5, ..Default::default() };
    println!("{}", c.a + c.b + d.a + d.b);
}
