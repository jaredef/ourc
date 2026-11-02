struct P{x:i64}
fn bump(p:&mut P){p.x+=7;}
fn main(){let mut p=P{x:3};bump(&mut p);println!("{}",p.x);}
