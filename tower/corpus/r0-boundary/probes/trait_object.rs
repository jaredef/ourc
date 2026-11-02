trait Shape{ fn area(&self)->i32; }
struct Sq(i32); struct Rect(i32,i32);
impl Shape for Sq{ fn area(&self)->i32{ self.0*self.0 } }
impl Shape for Rect{ fn area(&self)->i32{ self.0*self.1 } }
fn main(){ let v:Vec<Box<dyn Shape>>=vec![Box::new(Sq(3)),Box::new(Rect(2,5))]; let mut t=0; for s in &v { t+=s.area(); } println!("{}",t); }
