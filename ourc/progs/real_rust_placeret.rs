fn grow(acc:&mut String, w:&str){ acc.push_str(w); }
fn main(){ let mut a=String::from(""); grow(&mut a,"x"); println!("[{}]",a); }
