#[derive(Debug)] enum Tok { Num(i64), Op { c: char, prec: u8 } }
#[derive(Debug)] struct P { x: i32, y: i32 }
fn main() { println!("{:#?}", Tok::Op { c: '*', prec: 2 }); let t = Tok::Op { c: '+', prec: 1 }; println!("{:#?}", t); println!("{:#?}", P { x: 1, y: 2 }); println!("{:#?}", Tok::Num(3)); }
