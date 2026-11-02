struct Pt{x:i32,y:i32}
fn main(){ let v=vec![Pt{x:1,y:2},Pt{x:3,y:4}]; let mut t=0; for p in &v { t+=p.x+p.y; } println!("{}",t); }
