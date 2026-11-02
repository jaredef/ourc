use std::ops::Neg;
struct N(i64);
impl Neg for N{type Output=N;fn neg(self)->N{N(-self.0)}}
fn main(){let r=-N(5);println!("{}",r.0);}
