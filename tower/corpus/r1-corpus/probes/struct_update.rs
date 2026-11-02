#[derive(Clone,Copy)] struct P{x:i32,y:i32,z:i32}
fn main(){ let a=P{x:1,y:2,z:3}; let b=P{y:20,..a}; println!("{} {} {}",b.x,b.y,b.z); }
