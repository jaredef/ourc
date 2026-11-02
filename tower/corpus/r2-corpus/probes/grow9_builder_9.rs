struct Config { a: i64, b: i64, c: i64 }
impl Config {
    fn new() -> Config { Config { a: 0, b: 0, c: 0 } }
    fn set_a(&mut self, x: i64) -> &mut Config { self.a = x; self }
    fn set_b(&mut self, x: i64) -> &mut Config { self.b = x; self }
    fn set_c(&mut self, x: i64) -> &mut Config { self.c = x; self }
}
fn main() {
    let mut c = Config::new();
    c.set_a(1).set_b(2).set_c(3);
    println!("{}", c.a * 100 + c.b * 10 + c.c);
}
