use std::ops::Sub;
struct V(i64);
impl Sub for V{type Output=V;fn sub(self,o:V)->V{V(self.0-o.0)}}
fn main(){let r=V(10)-V(3);println!("{}",r.0);}
