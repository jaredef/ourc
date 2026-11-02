#[derive(PartialEq)]
struct P{x:i64,y:i64}
fn main(){let a=P{x:1,y:2};let b=P{x:1,y:2};println!("{}",if a==b{1}else{0});}
