struct Point{x:i64,y:i64}
fn main(){ let p=Point{x:7,y:13}; let Point{x,y}=p; println!("{}", x*100+y); }
