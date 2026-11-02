fn f(a:i64,b:i64)->i64{ match (a,b) { (0,_)=>1, (_,0)=>2, (x,y) if x==y =>3, _=>4 } }
fn main(){ let mut s=0i64; let ps=[(0,5),(5,0),(3,3),(2,7)]; for &(a,b) in ps.iter() { s=s*10+f(a,b); } println!("{}", s); }
