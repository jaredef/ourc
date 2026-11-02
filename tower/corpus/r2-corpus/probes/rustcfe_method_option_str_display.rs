struct W { s: String }
impl W {
    fn get(&self) -> &str { &self.s }
    fn opt(&self, b: bool) -> Option<&str> { if b { Some(&self.s) } else { None } }
}
fn main() { let w = W { s: "hi".to_string() };
    println!("{}", w.get());
    match w.opt(true) { Some(x) => println!("{}", x), None => println!("none") }
    match w.opt(false) { Some(x) => println!("{}", x), None => println!("none") }
}
