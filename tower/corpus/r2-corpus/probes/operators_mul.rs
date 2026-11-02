use std::ops::Mul;
struct S(i64);
impl Mul for S{type Output=S;fn mul(self,o:S)->S{S(self.0*o.0)}}
fn main(){let r=S(6)*S(7);println!("{}",r.0);}
