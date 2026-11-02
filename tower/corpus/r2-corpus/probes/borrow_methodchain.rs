struct C{n:i64}
impl C{fn add(&mut self,v:i64)->&mut C{self.n+=v;self}}
fn main(){let mut c=C{n:0};c.add(3).add(4);println!("{}",c.n);}
