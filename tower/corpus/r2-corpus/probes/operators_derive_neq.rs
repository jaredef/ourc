#[derive(PartialEq)]
struct P{x:i64}
fn main(){let a=P{x:1};let b=P{x:2};println!("{}",if a!=b{1}else{0});}
