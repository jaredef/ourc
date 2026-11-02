#[derive(PartialEq,Eq,PartialOrd,Ord)]
struct P{x:i64,y:i64}
fn main(){let a=P{x:2,y:1};let b=P{x:1,y:9};println!("{}",if a>b{1}else{0});}
