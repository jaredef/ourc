#[derive(PartialEq,Eq,PartialOrd,Ord)]
struct P{x:i64,y:i64}
fn main(){let a=P{x:1,y:2};let b=P{x:1,y:3};println!("{}",if a<b{1}else{0});}
