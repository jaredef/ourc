#[derive(Clone)]
struct P{x:i64,y:i64,z:i64}
fn main(){ let a=P{x:1,y:2,z:3}; let b=P{y:20,..a.clone()}; println!("{}",b.x+b.y+b.z); }
