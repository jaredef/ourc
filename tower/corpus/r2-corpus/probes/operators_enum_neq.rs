#[derive(PartialEq)]
enum E{A,B}
fn main(){let x=E::A;println!("{}",if x!=E::B{1}else{0});}
