use std::ops::Add;
struct P{x:i32}
impl Add for P { type Output=P; fn add(self,o:P)->P{ P{x:self.x+o.x} } }
fn main(){ let c=P{x:1}+P{x:2}; println!("{}",c.x); }
