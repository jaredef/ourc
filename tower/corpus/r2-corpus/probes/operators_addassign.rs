use std::ops::AddAssign;
struct C{v:i64}
impl AddAssign for C{fn add_assign(&mut self,o:C){self.v+=o.v;}}
fn main(){let mut a=C{v:5};a+=C{v:3};println!("{}",a.v);}
