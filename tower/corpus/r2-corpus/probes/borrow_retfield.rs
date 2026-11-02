struct P{x:i64,y:i64}
fn getx(p:&P)->&i64{&p.x}
fn main(){let p=P{x:42,y:1};println!("{}",*getx(&p));}
