use std::ops::{Add,Mul};
#[derive(Clone,Copy)]
struct V(i64);
impl Add for V{type Output=V;fn add(self,o:V)->V{V(self.0+o.0)}}
impl Mul for V{type Output=V;fn mul(self,o:V)->V{V(self.0*o.0)}}
fn main(){let r=V(2)*V(3)+V(4);println!("{}",r.0);}
