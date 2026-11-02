#[derive(Copy,Clone)]
struct Pt{x:i64,y:i64}
fn main(){ let a=[Pt{x:1,y:2},Pt{x:3,y:4},Pt{x:5,y:6}]; let mut s=0i64; for p in a.iter() { s+=p.x*p.y; } println!("{}", s); }
