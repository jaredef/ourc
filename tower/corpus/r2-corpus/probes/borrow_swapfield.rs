struct P{x:i64,y:i64}
fn main(){let mut p=P{x:1,y:2};std::mem::swap(&mut p.x,&mut p.y);println!("{}",p.x*10+p.y);}
