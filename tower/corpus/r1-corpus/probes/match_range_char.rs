fn cls(c:char)->i32{ match c { 'a'..='z'=>1, '0'..='9'=>2, _=>0 } }
fn main(){ println!("{}",cls('m')+cls('7')*10+cls('#')*100); }
