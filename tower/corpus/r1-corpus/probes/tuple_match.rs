fn f(a:i32,b:i32)->i32{ match (a,b) { (0,_)=>1, (_,0)=>2, (x,y)=>x+y } }
fn main(){ println!("{}",f(0,5)+f(5,0)+f(3,4)); }
