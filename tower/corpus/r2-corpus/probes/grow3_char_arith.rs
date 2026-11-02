fn main(){ let mut s=String::new(); for i in 0..5u8 { let c=(b'a'+i) as char; s.push(c); } println!("{}", s); }
