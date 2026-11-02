struct P{x:i32,y:i32}
impl P{ fn sum(&self)->i32{ self.x+self.y } }
fn main(){ let p=P{x:3,y:4}; println!("{}", p.sum()); }
