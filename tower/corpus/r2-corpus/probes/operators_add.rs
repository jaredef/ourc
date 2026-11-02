use std::ops::Add;
#[derive(Clone,Copy)]
struct P{x:i64,y:i64}
impl Add for P{type Output=P;fn add(self,o:P)->P{P{x:self.x+o.x,y:self.y+o.y}}}
fn main(){let a=P{x:1,y:2}+P{x:3,y:4};println!("{}",a.x+a.y);}
