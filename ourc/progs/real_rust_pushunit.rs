fn main() { let cs="a_b_c"; let b=cs.as_bytes(); let mut txt=String::from(""); let mut k=0;
 while k<b.len() { if b[k]!=95 { txt.push(b[k] as char); } k=k+1; }
 let mut s=String::from(""); if txt.len()>0 { s.push_str(&txt); }
 println!("{} {}", txt, s); }
