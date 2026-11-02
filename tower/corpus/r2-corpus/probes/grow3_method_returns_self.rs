#[derive(Copy,Clone)]
struct P{x:i64,y:i64}
impl P{ fn shift(self,d:i64)->Self{ P{x:self.x+d,y:self.y+d} } }
fn main(){ let p=P{x:1,y:2}.shift(10).shift(5); println!("{}", p.x+p.y); }
