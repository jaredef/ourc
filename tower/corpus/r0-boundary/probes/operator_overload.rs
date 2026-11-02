use std::ops::Add;
#[derive(Clone,Copy)] struct V{x:i32,y:i32}
impl Add for V{ type Output=V; fn add(self,o:V)->V{ V{x:self.x+o.x,y:self.y+o.y} } }
fn main(){ let a=V{x:1,y:2}; let b=V{x:3,y:4}; let c=a+b; println!("{} {}", c.x, c.y); }
