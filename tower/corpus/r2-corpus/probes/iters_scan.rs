fn main(){let s:i64=(1..5).scan(0,|acc,x|{*acc+=x;Some(*acc)}).sum();println!("{}",s);}
