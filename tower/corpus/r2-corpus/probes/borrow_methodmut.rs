struct C{n:i64}
impl C{fn inc(&mut self){self.n+=1;}}
fn main(){let mut c=C{n:5};c.inc();c.inc();println!("{}",c.n);}
