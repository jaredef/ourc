#[derive(PartialEq)] enum E { A, B, Eof }
fn conv(n: u8) -> u8 { n }
fn main(){ let _x = conv(5); let e = E::Eof; println!("{}", if e == E::Eof { 1 } else { 0 }); }
