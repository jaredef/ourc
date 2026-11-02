fn main(){let mut it=(1..4).peekable();let p=*it.peek().unwrap();let s:i64=it.sum();println!("{}",p+s);}
