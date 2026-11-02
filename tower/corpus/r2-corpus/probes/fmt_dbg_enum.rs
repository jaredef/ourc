#[derive(Debug)]
enum E { A, B(i32) }
fn main(){ println!("{:?}", E::B(5)); }
