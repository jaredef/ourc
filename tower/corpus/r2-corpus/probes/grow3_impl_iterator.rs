struct Counter{ n:i64, max:i64 }
impl Iterator for Counter{ type Item=i64; fn next(&mut self)->Option<i64>{ if self.n<self.max { self.n+=1; Some(self.n) } else { None } } }
fn main(){ let c=Counter{n:0,max:5}; let s:i64=c.sum(); println!("{}", s); }
