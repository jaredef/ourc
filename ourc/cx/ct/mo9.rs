trait Shape { fn area(&self) -> i64; }
struct Sq(i64);
impl Shape for Sq { fn area(&self) -> i64 { self.0 * self.0 } }
fn main() { let d: Vec<Box<dyn Shape>> = vec![Box::new(Sq(5)), Box::new(Sq(1))]; println!("{}", d.iter().map(|x| x.area()).sum::<i64>()); let e: Vec<Box<dyn Shape>> = vec![Box::new(Sq(2))]; for x in &e { println!("{}", x.area()); } }
