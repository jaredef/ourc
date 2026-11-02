#[derive(PartialEq)]
enum E{A,B,C}
fn main(){let x=E::B;println!("{}",if x==E::B{1}else{0});}
